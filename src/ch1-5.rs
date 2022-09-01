use rand::Rng;
use std::char::MAX;
use std::cmp::Ordering;
use std::io;

// VSCode + Rust shortcut
// Run: Ctrl+R Ctrl+R
// Save Ctrl+S

fn main() {
    ch6();
}

fn ch2() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret number is {}", secret_number);

    loop {
        println!("Pls input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Frain to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Big"),
            Ordering::Equal => {
                println!("Same");
                break;
            }
        }
    }
}

fn ch3() {
    let x = 2;
    let mut c = 0;
    const MAX_POINT: u32 = 100_000;
    loop {
        let x = 1;

        c += add(c);

        if c > 10 {
            break;
        }
    }
    println!("{} {} {}", x, c, MAX_POINT);

    c = 0;
    while c != 10 {
        c = add(c);
    }
    println!("{}", c);

    // let内でif
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number);

    // loops in list
    let q = [1, 2, 3, 4, 5, 6];
    for element in q {
        println!("{}", element);
    }

    //
}

fn add(x: i32) -> i32 {
    x + 1
}

fn ch4() {
    //
    let s = String::from("hello");
    let mut s_mut = String::from("hello");
    s_mut.push_str(", world!");
    println!("{}", s_mut);

    copying();

    owner_ship();
}

fn copying() {
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let s1 = String::from("hello"); // ptr, len, capacity
                                    // let s2 = s1;
                                    // これではs1とs2が同じポインタを指してしまう
                                    // s2を解放するとs1が解放することになり、バグを起こす
                                    // そのためこれはダメになる.
    let s2 = s1.clone();
    println!("{} {}", s1, s2);

    let x1 = 100;
    let x2 = x1;
    println!("{} {}", x1, x2);
    // int, float, bool, char, tuple(int, float, bool, char)だけcopy可能

    //
}

fn owner_ship() {
    let s = String::from("hello");
    take_owership(s);
    // println!("{}", s);
    // ここではsはもう使用できない.
    // 代入でも関数にmoveされる.

    let x = 111;
    makes_copy(x);
    println!("{} ", x);
    // これはcopyなので使用可能.

    let words: char = 'c';
    makes_copy_char(words);
    println!("{}", words);
    // charなのでcopyになる.
}

fn take_owership(s: String) {
    println!("{}", s);
}
fn makes_copy(x: i32) {
    println!("{}", x);
}

fn makes_copy_char(c: char) {
    println!("{}", c);
}

fn ch4_5() {
    let s1 = String::from("hello");
    let len = calc_length(&s1); //参照する.
    println!("{} {}", s1, len);

    let mut s1 = String::from("hello");
    //mutを直接指定すると可変になる.
    add_san(&mut s1);
    println!("{}", s1);

    let r1 = &mut s1;
    // let r2 = &mut s1; //2回目の借用はできない.
    // 複数個作るとポインタが同時にアクセスするなどのバグを回避できる。
    println!("{} ", r1);

    let mut s1 = String::from("this is it.");
    {
        let rr1 = &mut s1;
        println!("{}", rr1);
    } // rr1はここでdropする.
    let rr2 = &mut s1;
    println!("{}", rr2);

    // 文字列スライス
    let s = String::from("hello my sister");
    let h = &s[0..5];
    println!("{}", h);
    let slice = &s[1..];
    println!("{}", slice);

    let mojiretu = "this is mojiretu literal";
    println!("{}", &mojiretu[0..5]);

    let q = [1, 2, 3, 4, 5];
    let slice = &q[0..3];
    for num in slice {
        println!("{}", num);
    }

    //
}

fn calc_length(s: &String) -> usize {
    //&Stringによってsを借用する.
    s.len()
}

fn add_san(s: &mut String) {
    s.push_str(" san");
}

fn ch5() {
    //可変なStruct
    let mut user1 = User {
        email: String::from("thisistestmail@gmail.comm"),
        username: String::from("whoareyou"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.username);
    user1.username = String::from("whodunit");
    println!("{}", user1.username);

    let mut user2 = build_user(
        "thisisanotheruser@gmail.comm".to_string(),
        "man".to_string(),
    );
    println!("{}", user2.username);

    // Point struct
    let x = Point(0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // 略記ができる.
    // 本来なら
    // username: username,
    // email: email,
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

// tupleみたいな構造体
struct Point(i32, i32);

fn ch5_2() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 100,
    };

    let rect3 = Rectangle::square(100);

    println!("{:#?}", rect3);
    println!("{}", rect1.area());
    println!("{}", rect1.can_hold(&rect2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Rectangle構造体上のarea method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // selfを引数にとらない関数
    // 生成できる(String::fromと似てる)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
