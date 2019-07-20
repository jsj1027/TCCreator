use std::io::stdin;

fn main() {
    struct Tc_Var {
        name: String,
        example: String,
    }

    let story = Tc_Var {
        name: String::from("Story"),
        example: String::from("S12345"),
    };

    let defect = Tc_Var {
        name: String::from("Defect"),
        example: String::from("DE12345"),
    };

    let mut work_product = String::new();
    println!(
        "What is the work product? ({},{}) Examples:({},{})",
        story.name, defect.name, story.example, defect.example
    );

    stdin()
        .read_line(&mut work_product)
        .expect("Failed to read work product");

    let manual = String::from("Manual");
    let regression = String::from("Regression");

    println!("What is the type of test that you are writing? ({},{}) Examples:({},{})", );
}
