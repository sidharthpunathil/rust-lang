
enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let kind = IpAddKind::V4(127, 0, 0, 1);
    let loopback = IpAddKind::V6(String::from("::1"));
}

fn route(ip_kind: IpAddKind) {}
