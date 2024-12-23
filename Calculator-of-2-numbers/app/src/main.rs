use std::io;

fn main() {
    println!("Калькулятор 2 чисел");
    println!("Calculator of 2 numbers");
    println!("Напишите \"exit\" чтобы завершить программу");
    println!("Write \"exit\" to finish program");
    loop {
        let mut str1: String = String::new();
        let mut str2: String = String::new();
        let mut action: String = String::new();
        
        println!("Введите первое число: ");
        match io::stdin().read_line(&mut str1) {
            Ok(_) => {},
            Err (e) => println!("Ошибка ввода - {}", e)
        }

        if str1.trim() == "exit" || str1.trim() == "Exit" {
            println!("Выход из программы...");
            break;
        }

        println!("Введите действие: ");
        match io::stdin().read_line(&mut action) {
            Ok(_) => {},
            Err (e) => println!("Ошибка ввода - {}", e)
        }

        println!("Введите второе число: ");
        match io::stdin().read_line(&mut str2) {
            Ok(_) => {},
            Err (e) => println!("Ошибка ввода - {}", e)
        }

        let num1: f64 = str1.trim().parse().unwrap();
        let num2: f64 = str2.trim().parse().unwrap();

        match action.trim() { // trim для удаления пробелов
            "+" => plus(num1, num2),
            "-" => minus(num1, num2),
            "*" => multiply(num1, num2),
            "/" => divide(num1, num2),
            _ => println!("Ошибка ввода")
        } 
    };
}

fn plus(n1: f64, n2: f64) {
    println!("\nРезультат: {}", n1 + n2);
}

fn minus(n1: f64, n2: f64) {
    println!("\nРезультат: {}", n1 - n2);
}

fn multiply(n1: f64, n2: f64) {
    println!("\nРезультат: {}", n1 * n2);
}

fn divide(n1: f64, n2: f64) {
    println!("\nРезультат: {}", n1 / n2);
}