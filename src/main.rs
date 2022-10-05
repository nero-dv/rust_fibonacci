use std::io;

fn main() {
    let mut n1: u128 = 0;
    let mut n2: u128 = 1;
    let mut count: u128 = 0;

    loop {
        println!("Enter terms for Fibonacci sequence.\nConstraints: Greater than 0, less than 186");
        let mut nterm: String = String::new();

        io::stdin()
            .read_line(&mut nterm)
            .expect("Failed to read line.");

        let nterm: u128 = match nterm.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if nterm == 1 {
            println!("Fibonacci sequence up to {nterm}:\n{n1}");
            break;
        } else {
            println!("Fibonacci sequence:");
            if nterm > 185 {
                println!("Number must be less than 186!");
                break;
            }
            while count < nterm {
                let run: u128 = count + 1;
                println!("Run: {run}, nth sequence: {n1}");
                let nth: u128 = n1 + n2;
                n1 = n2;
                n2 = nth;
                count += 1;
            }
            break;
        }
    }
}
