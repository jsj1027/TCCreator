use std::io::stdin;

fn check_in_string(array_example: &[String], string: &String) -> bool {
    let mut check = false;
    for item in array_example.iter() {
        if item == string {
            check = true;
        }
    }
    check
}

fn main() {
    let mut last: [&str] = [];

    let wp_array: [&'static str; 2] = ["Story", "Defect"];
    let wp_array_examples: [&'static str; 2] = ["S", "D"];

    let type_array: [&'static str; 5] = [
        "Acceptance",
        "Regression",
        "Manual",
        "User Interface",
        "Usability",
    ];
    let type_array_examples: [&'static str; 5] = ["A", "R", "M", "UI", "U"];

    let mut work_product = &'static str;

    println!(
        "What is the work product? {} Examples:{}",
        &wp_array, wp_array_examples
    );

    stdin()
        .read_line(&mut work_product)
        .expect("Failed to read work product");

    let mut work_product = work_product.trim().to_uppercase();

    let check = check_in_string(&wp_array_examples[0..1], &work_product);

    if !check {
        println!("Wrong work product");
    }
    else {
        last.push(&work_product);
    }

    println!(
        "What is the type of test? {:?} Examples:{:?}",
        type_array, type_array_examples
    );

    stdin()
        .read_line(&mut work_product)
        .expect("Failed to read test type");

    // if wp_array_examples.contains(& work_product) {}

    // println!("What is the type of test that you are writing? ({},{}) Examples:({},{})",);
}
//testfolder
//Type Acceptance Regression Manual UserInterface Usability
//Method Manual Automated
//Project Elements
