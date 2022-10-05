use std::io;

fn main() {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut count = 0;

    loop {
        println!("Enter terms for Fibonacci sequence (under 46 as this program is designed to work with unsigned integers up to 32 bits in size)");
        let mut nterm: String = String::new();

        io::stdin()
            .read_line(&mut nterm)
            .expect("Failed to read line.");

        let nterm: u32 = match nterm.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if nterm == 1 {
            println!("Fibonacci sequence up to {nterm}:\n{n1}");
            break;
        } else {
            println!("Fibonacci sequence:");
            while count < nterm {
                let run: u32 = count + 1;
                println!("Run: {run}, nth sequence: {n1}");
                let nth: u32 = n1 + n2;
                n1 = n2;
                n2 = nth;
                count += 1;
            }
            break;
        }
    }
}
