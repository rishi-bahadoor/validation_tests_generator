use std::fs;
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};

use pcap::{Capture, ConnectionStatus, Device};
use std::net::IpAddr;

const HOST_IP: IpAddr = IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 32, 100));
const PCAP_DIR: &str = "pcaps";

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
    handle: Option<JoinHandle<()>>,
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
            handle: None,
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
        let flag = Arc::clone(&self.stop_flag);
        let name = self.test_name.clone();
        let path = PathBuf::from(PCAP_DIR).join(format!("{}.pcap", name));

        let handle = thread::spawn(move || {
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

            while !flag.load(Ordering::Relaxed) {
                match cap.next_packet() {
                    Ok(packet) => {
                        cap_save_file.write(&packet);
                    }
                    Err(_) => {
                        // TODO: log packet drops.
                    }
                }
            }

            println!("[PCAP] Received stop signal for: {}", &name);
            println!(
                "[PCAP] You can find the captured pcap at: {}/{}.pcap",
                PCAP_DIR, &name
            )
        });

        self.handle = Some(handle);
    }

    pub fn stop(mut self) {
        if self.skip {
            print_warn_ln!(
                "Pcap capture was skipped due to some errors in the capture process. This does not affect the testing process or test results."
            );
            return;
        }

        self.stop_flag.store(true, Ordering::Relaxed);

        if let Some(handle) = self.handle.take() {
            if let Err(e) = handle.join() {
                print_warn_ln!("Failed to join capture thread: {:?}", e);
            }
        }
    }
}
