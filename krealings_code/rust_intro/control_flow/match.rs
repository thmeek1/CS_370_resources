// from std::result::Result
//pub enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}

use text_io::read;
use std::io::{self, Write};

fn main() {
    // `match` behaves like a traditional `switch statement`

    print!("Please enter a value >");
    io::stdout().flush().unwrap();
    let num = read!();
    match num {
        // Match a single value
        0 => {
            println!("Zero");
        }
        // Match a range
        10..=100 => {
            println!("Something between ten and one hundred");
        }
        // Match multiples
        1 | 3 | 5 | 7 | 9 => {
            println!("Odd number between zero and ten");
        }
        // Catch-all case, covers everything else
        _ => {
            println!("Something else");
        }
    }

    // `match` can be used to declare a variable, as well
    print!("Please enter how many burgers you ate >");
    io::stdout().flush().unwrap(); // flush resturns a std::io::Result<T, Error>
                                   // unwrap acknowledges that return, but essentially ignores it.
    let num_burgers = read!();
    let sentence = match num_burgers {
        0 => "I wasn't hungry",
        1 | 2 | 3 => "I ate a couple burgers and am full",
        4..=8 => "I ate too many burgers",
        _ => "I'm going to be sick",
    };

    println!("{}", sentence);
}
