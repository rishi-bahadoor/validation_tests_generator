use std::fs;
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use pcap::{Capture, ConnectionStatus, Device};
use std::net::IpAddr;

const HOST_IP: IpAddr = IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 32, 100));
const PCAP_DIR: &str = "pcaps";

// TODO: Implement an interface flag for (TDB)[default],[short],[long] timeout
// constants and based on the input flag, let the code decide when to stop the
// pcap capture.
const MAX_PCAP_CAPTURE_TIME_S: u64 = 600; // 600 seconds (5 mins)

fn connected_ethernet_device() -> Option<Device> {
    let all_devices = match Device::list() {
        Ok(devices) => devices,
        Err(e) => {
            print_warn_ln!("Failed to list devices: {}", e);
            return None;
        }
    };

    let device = match all_devices
        .into_iter()
        .find(|device| device.addresses.iter().any(|addr| addr.addr == HOST_IP))
    {
        Some(d) => d,
        None => {
            print_warn_ln!("No device found with IP {}.", HOST_IP);
            print_help_ln!(
                "Check your system [Network Connections / Adapters] configuration for IP: {}.",
                HOST_IP
            );
            return None;
        }
    };

    if device.flags.connection_status != ConnectionStatus::Connected {
        print_warn_ln!("Device with IP {} is not connected.", HOST_IP);
        print_help_ln!("Check the ethernet hardware connection.");
        return None;
    }

    Some(device)
}

pub struct PcapInstance {
    test_name: String,
    stop_flag: Arc<AtomicBool>,
    thread_1_handle: Option<JoinHandle<()>>,
    thread_2_handle: Option<JoinHandle<()>>,
    skip: bool,
}

impl PcapInstance {
    pub fn new(test_name: &str) -> Self {
        let dir = PathBuf::from(PCAP_DIR);
        let mut skip = false;

        if let Err(e) = fs::create_dir_all(&dir) {
            print_warn_ln!("Failed to create {} directory: {}", PCAP_DIR, e);
            skip = true;
        }

        let pcap_path = dir.join(format!("{}.pcap", test_name));
        if pcap_path.exists() {
            if let Err(e) = fs::remove_file(&pcap_path) {
                print_warn_ln!("Failed to remove existing pcap file. {}", e);
                skip = true;
            }
        }

        PcapInstance {
            test_name: test_name.to_string(),
            stop_flag: Arc::new(AtomicBool::new(false)),
            thread_1_handle: None,
            thread_2_handle: None,
            skip,
        }
    }

    pub fn start(&mut self) {
        if self.skip {
            print_warn_ln!("Skipping pcap...");
            return;
        }
        let main_device = match connected_ethernet_device() {
            Some(d) => d,
            None => {
                self.skip = true;
                print_warn_ln!(" Skipping pcap...");
                return;
            }
        };

        println!("[PCAP] Capture started for: {}", self.test_name);

        // Share the same stop_flag for both threads
        let timer_flag = Arc::clone(&self.stop_flag);
        let capture_flag = Arc::clone(&self.stop_flag);

        // 1) Timer thread: waits, then stops the capture
        let thread_1_handle = thread::spawn(move || {
            let timeout = Duration::from_secs(MAX_PCAP_CAPTURE_TIME_S);
            let sleep_chunk = Duration::from_secs(1);
            let mut elapsed = Duration::ZERO;

            while elapsed < timeout {
                // If stop() was called, break out immediately
                if timer_flag.load(Ordering::Acquire) {
                    return;
                }
                thread::sleep(sleep_chunk);
                elapsed += sleep_chunk;
            }
            // Timeout expired—signal the capture thread to stop
            timer_flag.store(true, Ordering::Release);
        });

        // 2) Capture thread: runs until stop_flag becomes true
        let name = self.test_name.clone();
        let path = PathBuf::from(PCAP_DIR).join(format!("{}.pcap", name));
        let thread_2_handle = thread::spawn(move || {
            let mut cap = Capture::from_device(main_device)
                .unwrap()
                .promisc(true)
                .open()
                .unwrap();

            let mut cap_save_file = match cap.savefile(&path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("[ERROR] Failed to create pcap saver: {}", e);
                    return;
                }
            };

            while !capture_flag.load(Ordering::Acquire) {
                match cap.next_packet() {
                    Ok(packet) => {
                        cap_save_file.write(&packet);
                    }
                    Err(_) => {
                        // TODO: log packet drops.
                    }
                }
            }
        });

        self.thread_1_handle = Some(thread_1_handle);
        self.thread_2_handle = Some(thread_2_handle);
    }

    // Stops capture, joins both threads, and prints the output location.
    pub fn stop(&mut self) {
        if self.skip {
            print_warn_ln!(
                "Pcap capture was skipped due to some errors in the capture process. This does not affect the testing process or test results."
            );
            return;
        }

        // Signal both threads to stop
        self.stop_flag.store(true, Ordering::Release);

        // Join the timer thread
        if let Some(handle) = self.thread_1_handle.take() {
            if let Err(e) = handle.join() {
                print_warn_ln!("Failed to join timer thread: {:?}", e);
            }
        }

        // Join the capture thread
        if let Some(handle) = self.thread_2_handle.take() {
            if let Err(e) = handle.join() {
                print_warn_ln!("Failed to join capture thread: {:?}", e);
            }
        }

        println!(
            "[PCAP] Capture complete. Saved to: {}/{}.pcap",
            PCAP_DIR, &self.test_name
        );
    }
}
