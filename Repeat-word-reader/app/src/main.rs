use std::{collections::HashMap, io};

fn main() {
    println!("Считыватель повторных слов");
    println!("Напишите \"exit\" чтобы выйти");

    loop {
        println!("\nВведите слова");
        let mut text = String::new();

        match io::stdin().read_line(&mut text) {
            Ok(_) => {},
            Err(e) => println!("Ошибка - {}", e)
        }

        if text.trim() == "exit" {
            break;
        }

        text = text.to_lowercase();

        let mut count_map = HashMap::new();

        for w in text.split_whitespace() {
            let count = count_map.entry(w).or_insert(0);
            *count += 1;
        }

        println!("\n");
        let mut found = false;
        for (word, count) in &count_map {
            if *count >= 2 {
                println!("{}: {} раз(а)", word, count);
                found = true;
            }
        }

        if !found {
                println!("Нет повторяющихся слов.");
            }
    }
}
