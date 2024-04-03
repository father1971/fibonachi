use std::io;

fn main() {
    
    loop {

        println!("Введите номер последовательности Фибоначи от 1 и выше:");

        let mut int  = String::new();
           io::stdin()
               .read_line(&mut int)
               .expect("Failed to read line");

        let int: u32 = match int.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if int < 1 {
        println!("Не правильный формат")
        }
        break;
    }

    if int == 1 {
        println!("Число фибоначи {int} порядка, равен: 1")
    }
    else {
        println!("Число фибоначи {int} порядка, равен: 1")
    }

    let mut x: u32 = 1;
    let mut y: u32 = 1;
    
   
}
