use dhcp4r::{options, packet, server};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::ops::Add;
use std::process;
use std::thread;
use std::time::{Duration, Instant};

use std::error::Error;

// Server configuration
const SERVER_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 32, 100);
const IP_START: [u8; 4] = [192, 168, 32, 61];
const SUBNET_MASK: Ipv4Addr = Ipv4Addr::new(255, 255, 255, 0);
const DNS_IPS: [Ipv4Addr; 2] = [
    // Google DNS servers
    Ipv4Addr::new(8, 8, 8, 8),
    Ipv4Addr::new(4, 4, 4, 4),
];
const ROUTER_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 32, 100);
const LEASE_DURATION_SECS: u32 = 7200;
const LEASE_NUM: u32 = 1;

// Derived constants
const IP_START_NUM: u32 = bytes_u32!(IP_START);

pub fn dhcp_server_runner(trimmed_line: &str) -> Result<(), Box<dyn Error>> {
    // Split and collect all whitespace-separated tokens
    let args: Vec<&str> = trimmed_line.split_whitespace().collect();

    // Validate we have the token token: [start or stop]
    if args.is_empty() {
        return Err(
            "Incorrect amount of arguments: Usage: minimal_dchp_server <offered_ip>".into(),
        );
    }

    let offered_ip = args[0];

    println!("{}", offered_ip);

    // Run server in another thread
    thread::spawn(|| {
        if let Err(e) = run_dhcp_server() {
            eprintln!("DHCP server failed: {}", e);
        }
        eprintln!("Cannot continue without DHCP server. Exiting.");
        process::exit(1); // Kill entire program
    });

    Ok(())
}

struct MyServer {
    leases: HashMap<Ipv4Addr, ([u8; 6], Instant)>,
    last_lease: u32,
    lease_duration: Duration,
}

impl server::Handler for MyServer {
    fn handle_request(&mut self, server: &server::Server, in_packet: packet::Packet) {
        match in_packet.message_type() {
            Ok(options::MessageType::Discover) => {
                // Prefer client's choice if available
                if let Some(options::DhcpOption::RequestedIpAddress(addr)) =
                    in_packet.option(options::REQUESTED_IP_ADDRESS)
                {
                    let addr = *addr;
                    if self.available(&in_packet.chaddr, &addr) {
                        reply(server, options::MessageType::Offer, in_packet, &addr);
                        return;
                    }
                }
                // Otherwise prefer existing (including expired if available)
                if let Some(ip) = self.current_lease(&in_packet.chaddr) {
                    reply(server, options::MessageType::Offer, in_packet, &ip);
                    return;
                }
                // Otherwise choose a free ip if available
                for _ in 0..LEASE_NUM {
                    self.last_lease = (self.last_lease + 1) % LEASE_NUM;
                    if self.available(
                        &in_packet.chaddr,
                        &((IP_START_NUM + &self.last_lease).into()),
                    ) {
                        reply(
                            server,
                            options::MessageType::Offer,
                            in_packet,
                            &((IP_START_NUM + &self.last_lease).into()),
                        );
                        break;
                    }
                }
            }

            Ok(options::MessageType::Request) => {
                // Ignore requests to alternative DHCP server
                if !server.for_this_server(&in_packet) {
                    return;
                }
                let req_ip = match in_packet.option(options::REQUESTED_IP_ADDRESS) {
                    Some(options::DhcpOption::RequestedIpAddress(x)) => *x,
                    _ => in_packet.ciaddr,
                };
                if !&self.available(&in_packet.chaddr, &req_ip) {
                    nak(server, in_packet, "Requested IP not available");
                    return;
                }
                self.leases.insert(
                    req_ip,
                    (in_packet.chaddr, Instant::now().add(self.lease_duration)),
                );
                reply(server, options::MessageType::Ack, in_packet, &req_ip);
            }

            Ok(options::MessageType::Release) | Ok(options::MessageType::Decline) => {
                // Ignore requests to alternative DHCP server
                if !server.for_this_server(&in_packet) {
                    return;
                }
                if let Some(ip) = self.current_lease(&in_packet.chaddr) {
                    self.leases.remove(&ip);
                }
            }

            // TODO - not necessary but support for dhcp4r::INFORM might be nice
            _ => {}
        }
    }
}

impl MyServer {
    fn available(&self, chaddr: &[u8; 6], addr: &Ipv4Addr) -> bool {
        let pos: u32 = (*addr).into();
        pos >= IP_START_NUM
            && pos < IP_START_NUM + LEASE_NUM
            && match self.leases.get(addr) {
                Some(x) => x.0 == *chaddr || Instant::now().gt(&x.1),
                None => true,
            }
    }

    fn current_lease(&self, chaddr: &[u8; 6]) -> Option<Ipv4Addr> {
        for (i, v) in &self.leases {
            if &v.0 == chaddr {
                return Some(*i);
            }
        }
        return None;
    }
}

fn reply(
    s: &server::Server,
    msg_type: options::MessageType,
    req_packet: packet::Packet,
    offer_ip: &Ipv4Addr,
) {
    let _ = s.reply(
        msg_type,
        vec![
            options::DhcpOption::IpAddressLeaseTime(LEASE_DURATION_SECS),
            options::DhcpOption::SubnetMask(SUBNET_MASK),
            options::DhcpOption::Router(vec![ROUTER_IP]),
            options::DhcpOption::DomainNameServer(DNS_IPS.to_vec()),
        ],
        *offer_ip,
        req_packet,
    );
}

fn nak(s: &server::Server, req_packet: packet::Packet, message: &str) {
    let _ = s.reply(
        options::MessageType::Nak,
        vec![options::DhcpOption::Message(message.to_string())],
        Ipv4Addr::new(0, 0, 0, 0),
        req_packet,
    );
}

/// Get the IPv4 address of a network interface by name, only if it's connected.
/// Returns `Some(ipv4_string)` or `None` if not found / not connected.
fn get_ipv4_address(interface_name: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let adapters = ipconfig::get_adapters()?;

    for adapter in adapters {
        if adapter.friendly_name() == interface_name {
            // Look for IPv4, regardless of oper_status
            for ip in adapter.ip_addresses() {
                if let IpAddr::V4(ipv4) = ip {
                    return Ok(Some(ipv4.to_string()));
                }
            }
            return Ok(None); // adapter found but no IPv4
        }
    }

    Ok(None) // no adapter by that name
}

fn run_dhcp_server() -> Result<(), Box<dyn std::error::Error>> {
    let nic_name = "Ethernet10"; // NIC where sensor is connected

    if let Some(ip) = get_ipv4_address(nic_name)? {
        println!("Successfully found {}'s IPv4 address: {}", nic_name, ip);
        let socket_str = format!("{}:67", ip);
        let socket = UdpSocket::bind(socket_str).unwrap();
        socket.set_broadcast(true).unwrap();

        let ms = MyServer {
            leases: HashMap::new(),
            last_lease: 0,
            lease_duration: Duration::new(LEASE_DURATION_SECS as u64, 0),
        };
        println!("DHCP server is running.");
        server::Server::serve(socket, SERVER_IP, ms);
    } else {
        return Err(format!(
            "Network interface '{}' has no active IPv4 address",
            nic_name
        )
        .into());
    }

    Ok(())
}
