use std::io;

fn main() {
    struct Workproduct {
        name: String,
        example: String,
    }

    let story = Workproduct{
        name: String::from("Story"),
        example: String::from("S12345"),
    };
    
    let defect = Workproduct{
        name: String::from("Defect"),
        example: String::from("DE12345"),
    };
    
    println!("What is the work product? ({},{}) Examples:({},{})", story.name, defect.name, story.example, defect.example);
}
