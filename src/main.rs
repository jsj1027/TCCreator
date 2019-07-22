use std::io::stdin;

fn main() {
    let wp_array = ["Story", "Defect"];
    let wp_array_examples = ["S", "D"];

    let type_array = [
        "Acceptance",
        "Regression",
        "Manual",
        "User Interface",
        "Usability",
    ];
    let type_array_examples = ["A", "R", "M", "UI", "U"];

    let mut work_product;

    println!(
        "What is the work product? {:?} Examples:{:?}",
        wp_array, wp_array_examples
    );

    stdin()
        .read_line(&mut work_product)
        .expect("Failed to read work product");

    let work_product = work_product.trim().to_uppercase();

    // if wp_array_examples.contains(& work_product) {}

    // println!("What is the type of test that you are writing? ({},{}) Examples:({},{})",);
}
//testfolder
//Type Acceptance Regression Manual UserInterface Usability
//Method Manual Automated
//Project Elements
