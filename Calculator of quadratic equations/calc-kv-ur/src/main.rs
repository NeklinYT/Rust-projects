use std::io;

fn main() {

    println!("\"exit\" - для выхода");

    loop {
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    println!("Решить квадратное уравнение");


    println!("Введите a: ");
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => {},
        Err (e) => println!("Ошибка ввода - {}", e)
    }

    println!("Введите b: ");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => {},
        Err (e) => println!("Ошибка ввода - {}", e)
    }

    println!("Введите c: ");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => {},
        Err (e) => println!("Ошибка ввода - {}", e)
    }

    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();

    let d: f64 = (b * b) - 4.0 * (a * c);

    if d > 0.0 {
        let x1: f64 = ((-b) + d.sqrt()) / (2.0 * a);
        let x2: f64 = ((-b) - d.sqrt()) / (2.0 * a);

        println!("Решено, есть 2 корня\nD = {}\nКорень 1 = {}\nКорень 2 = {}", d, x1, x2);
    }

    if d == 0.0 {
        let x: f64 = (-b) / (2.0 * a);

        println!("Решено, есть 1 корень\nD = 0\nКорень = {}", x);
    }

    if d < 0.0 {
        println!("Корней нет т. к. D({}) < 0", d);
    }
}
}
