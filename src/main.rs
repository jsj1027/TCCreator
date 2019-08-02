use std::io;
use std::process;

fn main() {
    let mut work_product = String::new();

    println!("Enter the work product you ware working on. Example: S1234, DE1234");

    io::stdin()
        .read_line(&mut work_product)
        .expect("Failed to read work product");

    let mut number_of_test_cases = String::new();

    println!("How many test cases do you need?");

    io::stdin()
        .read_line(&mut number_of_test_cases)
        .expect("Failed to read number of test cases");

    let number_of_test_cases: u32 = match number_of_test_cases.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number, program is exiting");
            process::exit(1)
        }
    };

    let mut test_type = String::new();

    println!("What type of test are you conducting? Enter M, F, R, or A.");
    println!("These mean certain test types. Examples: Manual, Functional, Regression, Automatic");

    io::stdin()
        .read_line(&mut test_type)
        .expect("Failed to read test type");
    
    let m = String::from("M");
    let f = String::from("F");
    let a = String::from("A");
    let r = String::from("R");

    let test_type = match test_type.to_uppercase() {
        m => "Manual",
        f => "Functional",
        r => "Regression",
        a => "Automatic",
    };

    println!("{} - {} - {}", work_product, number_of_test_cases, test_type);
}
