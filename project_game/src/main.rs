use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn input<'a>(label: &'a str) -> String {
  print!("{label}: ");
  io::stdout().flush();
  return read_input();
}

fn read_input() -> String {
  let mut buffer = String::new();
  let handler = io::stdin();
  let result = handler.read_line(&mut buffer);

  return match result {
    Ok(size) => buffer,
    Err(any) => String::new(),
  };
}

fn main() {
    println!("Adivinhe o número!");
    let num_secreto = rand::thread_rng().gen_range(1..=100);
    loop{
        let guess = input("Por favor digite seu número");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            
        };
        println!("Seu palpite: {guess}");

        match guess.cmp(&num_secreto) {
            Ordering::Less => println!("Seu palpite foi menor!"),
            Ordering::Greater => println!("Seu palpite foi maior!"),
            Ordering::Equal =>{
                println!("Você acertou!!");
                break;
            } 
        }
    }
}
