// Definition of Protocol Families (PF_*) constants
// Originally from /usr/include/bits/socket.h

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PF {
    Unspec        = 0,  // Unspecified
    LocalUnixFile = 1,  // Local to host (pipes and file-domain) or POSIX name for PF_LOCAL or Another non-standard name for PF_LOCAL
    Inet          = 2,  // IP protocol family
    Ax25          = 3,  // Amateur Radio AX.25
    Ipx           = 4,  // Novell Internet Protocol
    Appletalk     = 5,  // Appletalk DDP
    Netrom        = 6,  // Amateur radio NetROM
    Bridge        = 7,  // Multiprotocol bridge
    Atmpvc        = 8,  // ATM PVCs
    X25           = 9,  // Reserved for X.25 project
    Inet6         = 10, // IP version 6
    Rose          = 11, // Amateur Radio X.25 PLP
    Decnet        = 12, // Reserved for DECnet project
    Netbeui       = 13, // Reserved for 802.2LLC project
    Security      = 14, // Security callback pseudo AF
    Key           = 15, // PF_KEY key management API
    NetlinkRoute  = 16, // PF_NETLINK or Alias to emulate 4.4BSD
    Packet        = 17, // Packet family
    Ash           = 18, // Ash
    Econet        = 19, // Acorn Econet
    Atmsvc        = 20, // ATM SVCs
    Rds           = 21, // RDS sockets
    Sna           = 22, // Linux SNA Project
    Irda          = 23, // IRDA sockets
    Pppox         = 24, // PPPoX sockets
    Wanpipe       = 25, // Wanpipe API sockets
    Llc           = 26, // Linux LLC
    Ib            = 27, // Native InfiniBand address
    Mpls          = 28, // MPLS
    Can           = 29, // Controller Area Network
    Tipc          = 30, // TIPC sockets
    Bluetooth     = 31, // Bluetooth sockets
    Iucv          = 32, // IUCV sockets
    Rxrpc         = 33, // RxRPC sockets
    Isdn          = 34, // mISDN sockets
    Phonet        = 35, // Phonet sockets
    Ieee802154    = 36, // IEEE 802.15.4 sockets
    Caif          = 37, // CAIF sockets
    Alg           = 38, // Algorithm sockets
    Nfc           = 39, // NFC sockets
    Vsock         = 40, // vSockets
    Kcm           = 41, // Kernel Connection Multiplexor
    Qipcrtr       = 42, // Qualcomm IPC Router
    Smc           = 43, // SMC sockets
    Xdp           = 44, // XDP sockets
    Mctp          = 45, // Management component transport protocol
    Max           = 46, // For now..
}
