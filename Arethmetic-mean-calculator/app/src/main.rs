use std::io;

fn main() {
    let mut list: Vec<i32>= Vec::new();

    loop {
        let mut input = String::new();

        println!("Введите число (или \"stop\" для подсчета среднего арифметического)");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => println!("Ошибка - {}", e)
        }

        let input = input.trim();

        if input.eq("stop") {
            break;
        }

        match input.parse::<i32>() {
            Ok(num) => list.push(num),
            Err(e) => println!("Ошибка - {}, вероятно вы ввели что-то не то", e),
        }
    }

    // println!("{:?}", &list);

    if !list.is_empty() {
        let avg = find_avg(&list);
        println!("Среднее арифмитечиское - {}", avg);
    } else {
        println!("Вы должны написать хотябы два числа перед подсчетом");
    }

}

fn find_avg(l: &Vec<i32>) -> f64 {
    let mut sum = 0;

    for el in l {
        sum += el
    }

    let length = (l.len()) as f64;
    let sum = sum as f64;

    sum / length
}
