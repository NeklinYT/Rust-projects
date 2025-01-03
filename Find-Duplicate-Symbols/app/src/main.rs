use std::io;



fn main() {
    println!("Напишите \"exit\" чтобы завершить программу");
    loop {
        let mut user_input = String::new();
        println!("Введите слова или символы");
    
        io::stdin().read_line(&mut user_input).expect("Не удалось прочитать строку");

        if user_input.trim().eq_ignore_ascii_case("exit") {
            break;
        }
    
        let list: Vec<&str> = user_input.trim().split_whitespace().collect();
        let dublicates = find_dublicate(&list);

        if !dublicates.is_empty() {
            println!("Повторяющиеся слова или символы: \n{:?}", dublicates);
        } else {
            println!("Дубликатов не найденно");
        }   
    }
}

fn find_dublicate<T>(list: &[T]) -> Vec<T> 
where T: PartialEq + Copy
{
    let mut duplicates: Vec<T> = Vec::new();

    for i in 0..(list.len()) {
        for j in (i + 1)..(list.len()) {
            if list[i] == list [j] {
                if !duplicates.contains(&list[i]) {
                    duplicates.push(list[i]);
                }
            }
        }
    }

    duplicates
} 