extern crate csv;

use std::io;

fn main() {
    let mut work_product = String::new();

    println!("Enter the work product you ware working on. Example: S1234, DE1234");

    io::stdin()
        .read_line(&mut work_product)
        .expect("Failed to read work product");

    let work_product = work_product.trim();

    let mut number_of_test_cases = String::new();

    println!("How many test cases do you need?");

    io::stdin()
        .read_line(&mut number_of_test_cases)
        .expect("Failed to read number of test cases");
    
    let number_of_test_cases = number_of_test_cases.trim();

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

    let mut test_type = match test_type.to_uppercase() {
        m => String::from("Manual"),
        f => String::from("Functional"),
        r => String::from("Regression"),
        a => String::from("Automatic"),
    };

    let test_type = test_type.trim();

    let mut test_folder = String::new();

    println!("What is the test folder of the product?");

    io::stdin()
        .read_line(&mut test_folder)
        .expect("Failed to read test folder");
    
    let test_folder = test_folder.trim();

    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&[work_product, number_of_test_cases, test_type, test_folder])
        .expect("Failed to write record");
    wtr.flush()
        .expect("Failed to write record");
}
