use std::vec;

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

fn ch8() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let num = vec![1, 2, 3];

    println!("s{:?}", v);
    println!("{:?}", num);

    let third: &i32 = &v[2];
    println!("{}", third);

    match num.get(2) {
        Some(third) => println!("{}", third),
        None => println!("None, that means v.get -> Option<&T>"),
    }

    for i in &mut v {
        println!("{}", i);
        *i += 100;
    }
    v[0] += 1;
    v.pop();
    for i in &v {
        println!("{}", i);
    }
}

fn main() {
    ch8();
}
