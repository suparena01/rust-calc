use std::io::{self, Write};
use std::cmp::Ordering;

fn main() {
    println!("Bienvenue dans la calculatrice ! ");
 
    print!("Choissisez un nombre : ");
    io::stdout().flush().unwrap();
    let mut _nb1 = String::new();
  

    io::stdin()
        .read_line(&mut _nb1)
        .expect("Error to read this line");

    
    let _nb1: u64 = _nb1.trim().parse().expect("Veuillez rentrer un nombre !");


    print!("Choissisez un deuxième nombre : ");
    io::stdout().flush().unwrap();

    let mut _nb2 = String::new();

    io::stdin()
        .read_line(&mut _nb2)
        .expect("Error to read this line");

    
    let _nb2: u64 = _nb2.trim().parse().expect("Veuillez rentrer un nombre !");

    println!("1) Additionner\n 2) Soustraire\n 3) Multiplier\n 4) Diviser");
    println!("5) Comparer deux nombres.");

    let mut choice = String::new();


    io::stdin()
        .read_line(&mut choice)
        .expect("Error to read this line");
        

    if choice.trim_end() == "1" {
        print!("Resultat {}", _nb1 + _nb2);
        io::stdout().flush().unwrap();
    } else if choice.trim_end() == "2" {
        print!("Resultat {}", _nb1 - _nb2);
        io::stdout().flush().unwrap();
    } else if choice.trim_end() == "3" {
        print!("Resultat {}", _nb1 * _nb2);
        io::stdout().flush().unwrap();
    } else if choice.trim_end() == "4" {
        print!("Resultat {}", _nb1 / _nb2);
        io::stdout().flush().unwrap();    
    } else if choice.trim_end() == "5" {
        match _nb1.cmp(&_nb2) {
            Ordering::Less => println!("{} est plus petit que {}", _nb1, _nb2),
            Ordering::Greater => println!("{} est plus grand que {}", _nb1, _nb2),
            Ordering::Equal => println!("{} est égal à {}", _nb1, _nb2),
        }
    }
      
    


}

