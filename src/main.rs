fn ch6() {
    let home = IpAddr::V4(127, 0, 0, 1);

    println!("{:#?}", home);
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    ch6();
}
