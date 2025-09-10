import time
import sys
from scapy.all import AsyncSniffer, sendp, Ether, IP, UDP, BOOTP, DHCP, get_if_hwaddr, ARP, srp1

# --- Configuration ---
SERVER_IP = "192.168.32.100"
OFFERED_IP = "192.168.32.102"
LEASE_TIME = 3600
INTERFACE = "Ethernet"  # Change to your NIC name
MAC_LEASE = None        # Tracks current lease (mac)
LEASE_EXPIRY = 0

# --- DHCP Server Class ---
class MinimalDhcpServer:
    def __init__(self, interface=INTERFACE):
        self.interface = interface
        self.sniffer = None

    # Check if the IP is already in use via ARP
    def arp_conflict(self, ip):
        arp_req = ARP(pdst=ip)
        ans = srp1(arp_req, timeout=1, verbose=0, iface=self.interface)
        return ans is not None

    # Build minimal DHCP OFFER or ACK
    def build_reply(self, packet, msg_type):
        return (
            Ether(dst=packet[Ether].src, src=get_if_hwaddr(self.interface)) /
            IP(src=SERVER_IP, dst="255.255.255.255") /
            UDP(sport=67, dport=68) /
            BOOTP(op=2, yiaddr=OFFERED_IP, siaddr=SERVER_IP,
                  chaddr=packet[BOOTP].chaddr, xid=packet[BOOTP].xid) /
            DHCP(options=[("message-type", msg_type),
                          ("server_id", SERVER_IP),
                          ("lease_time", LEASE_TIME),
                          "end"])
        )

    # Handle DHCP packets
    def handle_dhcp(self, packet):
        global MAC_LEASE, LEASE_EXPIRY
        if DHCP in packet and BOOTP in packet:
            options = packet[DHCP].options
            msg_type = None
            requested_ip = None
            for opt in options:
                if isinstance(opt, tuple):
                    if opt[0] == "message-type":
                        msg_type = opt[1]
                    if opt[0] == "requested_addr":
                        requested_ip = opt[1]

            client_mac = packet[Ether].src
            now = time.time()

            # Release expired lease
            if LEASE_EXPIRY < now:
                MAC_LEASE = None

            # Handle DHCP DISCOVER
            if msg_type == 1:  # DISCOVER
                print(f"DISCOVER from {client_mac}")
                if MAC_LEASE is None or MAC_LEASE == client_mac:
                    if self.arp_conflict(OFFERED_IP):
                        print(f"Conflict detected for {OFFERED_IP}, skipping offer")
                        return
                    offer = self.build_reply(packet, 2)  # DHCPOFFER
                    sendp(offer, iface=self.interface, verbose=0)
                    print(f"OFFER sent for {OFFERED_IP}")

            # Handle DHCP REQUEST
            elif msg_type == 3:  # REQUEST
                print(f"REQUEST from {client_mac} for {requested_ip}")
                if requested_ip != OFFERED_IP:
                    print(f"Requested IP does not match pool, ignoring")
                    return
                if MAC_LEASE is None or MAC_LEASE == client_mac:
                    if self.arp_conflict(OFFERED_IP):
                        print(f"Conflict detected for {OFFERED_IP}, cannot assign")
                        return
                    MAC_LEASE = client_mac
                    LEASE_EXPIRY = now + LEASE_TIME
                    ack = self.build_reply(packet, 5)  # DHCPACK
                    sendp(ack, iface=self.interface, verbose=0)
                    print(f"ACK sent for {OFFERED_IP}")

    # Start sniffing in a background thread
    def start(self):
        if self.sniffer and self.sniffer.running:
            return
        self.sniffer = AsyncSniffer(
            filter="udp and (port 67 or 68)",
            prn=self.handle_dhcp,
            iface=self.interface
        )
        self.sniffer.start()
        print("DHCP server started.")

    # Stop sniffing
    def stop(self):
        if self.sniffer:
            self.sniffer.stop()
            print("DHCP server stopped.")

# ---------------- Main ----------------
if __name__ == "__main__":
    server = MinimalDhcpServer()
    try:
        server.start()
        # Self terminate after a set time
        time.sleep(sys.argv[1])
    except:
        server.stop()