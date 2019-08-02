use std::io;

fn main() {
    let mut work_product = String::new();
    
    println!("Enter the work product you ware working on. Example: S1234, DE1234");

    io::stdin()
        .read_line(&mut work_product)
        .expect("Failed to read work product");

    let mut number_of_test_cases = String::new();

    loop {
        println!("How many test cases do you need?");

        io::stdin()
            .read_line(&mut number_of_test_cases)
            .expect("Failed to read number of test cases");

        let number_of_test_cases: u32 = number_of_test_cases.trim().parse();

        match number_of_test_cases {
            Ok(num) => {break},
            Err(_) => {
                println!("Not a number");
                continue
            },
        };
    }
}