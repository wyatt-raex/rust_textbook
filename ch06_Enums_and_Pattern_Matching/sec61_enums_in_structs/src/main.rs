// Create enum containing the kinds of IP addresses: IPV4 or IPV6
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // Create instances of each variant of IpAddrKind
    let _four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: six,
        address: String::from("::1"),
    };
}

// Function taking enum IpAddrKind as a parameter
fn route(ip_kind: IpAddrKind) {}
