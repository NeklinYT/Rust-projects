// #[derive(Debug)]
use std::io;

struct Triangle {
    cat1: f32,
    cat2: f32
}

impl Triangle {
    fn find_hyp(&self) -> f32 {
        (self.cat1 * self.cat1 + self.cat2 * self.cat2).sqrt()
    }
}

fn main() {
    println!("Калькулятор гипотенузы по 2 катетам");
    println!("Напишите \"exit\" чтобы завершить программу");

    loop {
    let mut c1: String = String::new();
    let mut c2: String = String::new();

    println!("Введите катет 1: ");
    match io::stdin().read_line(&mut c1) {
        Ok(_) => {},
        Err(e) => println!("Ошибка ввода - {}", e)
    }

    if c1.trim() == "exit" || c1.trim() == "Exit" {
        println!("Выход из программы...");
        break;
    }

    println!("Введите катет 2: ");
    match io::stdin().read_line(&mut c2) {
        Ok(_) => {},
        Err(e) => println!("Ошибка ввода - {}", e)
    }

    let cat1: f32 = c1.trim().parse().unwrap();
    let cat2: f32 = c2.trim().parse().unwrap();

    let tr = Triangle {
        cat1,
        cat2
    };

    let hyp = tr.find_hyp();
    println!("\nГипотенуза равна: {}", hyp);
    }
}
