fn ch6() {
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{:#?}", home);
    println!("{}", check_v4(home));

    let my_coin = Coin::Dime;
    let my_value = value_in_cents(my_coin);
    println!("{}", my_value);
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn check_v4(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(u1, u2, u3, u4) => {
            println!("{}", u4);
            true
        }
        IpAddr::V6(name) => false,
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 2,
        Coin::Nickel => 3,
        Coin::Quarter => 4,
    }
}

fn main() {
    ch6();
}
