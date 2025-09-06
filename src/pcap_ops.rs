use std::fs;
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use pcap::{ConnectionStatus, Device};
use std::net::IpAddr;

const HOST_IP: IpAddr = IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 32, 100));

fn connected_ethernet_device() -> Option<Device> {
    let all_devices = match Device::list() {
        Ok(devices) => devices,
        Err(e) => {
            eprintln!("[WARN] Failed to list devices: {}", e);
            return None;
        }
    };

    let device = match all_devices
        .into_iter()
        .find(|device| device.addresses.iter().any(|addr| addr.addr == HOST_IP))
    {
        Some(d) => d,
        None => {
            println!("[WARN] No device found with IP {}.", HOST_IP);
            println!(
                "[HELPER] Check your system [Network Connections / Adapters] configuration for IP: {}.",
                HOST_IP
            );
            return None;
        }
    };

    if device.flags.connection_status != ConnectionStatus::Connected {
        println!("[WARN] Device with IP {} is not connected.", HOST_IP);
        println!("[HELPER] Check the ethernet hardware connection.");
        return None;
    }

    let desc = device.desc.as_deref().unwrap_or("No description available");
    println!(
        "[PCAP] Connected device with IP {}: {} — {}",
        HOST_IP, device.name, desc
    );

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
        let dir = PathBuf::from("pcaps");
        let mut skip = false;

        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("[WARN] Failed to create pcaps directory: {}", e);
            skip = true;
        }

        let pcap_path = dir.join(format!("{}.pcap", test_name));
        if pcap_path.exists() {
            if let Err(e) = fs::remove_file(&pcap_path) {
                eprintln!("[WARN] Failed to remove existing pcap file. {}", e);
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
            println!("[WARN] Skipping pcap...");
            return;
        }
        if connected_ethernet_device().is_none() {
            self.skip = true;
            println!("[WARN] Skipping pcap...");
            return;
        }

        println!("[PCAP] started for: {}", self.test_name);

        let flag = Arc::clone(&self.stop_flag);
        let name = self.test_name.clone();
        let handle = thread::spawn(move || {
            while !flag.load(Ordering::Relaxed) {
                println!("[Thread {}] heartbeat", &name);
                thread::sleep(Duration::from_secs(5));
            }
            println!("[PCAP] received stop signal for: {}", &name);
        });

        self.handle = Some(handle);
    }

    pub fn stop(mut self) {
        if self.skip {
            println!(
                "[WARN] Pcap capture was skipped due to some errors in the capture process. This does not affect the testing process or test results."
            );
            return;
        }

        self.stop_flag.store(true, Ordering::Relaxed);

        if let Some(handle) = self.handle.take() {
            handle.join().expect("[WARN] Failed to join capture thread");
        }
    }
}
