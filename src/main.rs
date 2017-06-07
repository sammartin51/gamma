extern crate chrono;
#[macro_use] extern crate text_io;

mod tree;
mod task;

fn test() {

    let mut root = tree::Node::new(
        task::Task::new("Root".to_string())
    );

    // zipper now owns the root node
    let mut focus = root.zipper();

    focus.node.add_child(
        tree::Node::new(
            task::Task::new("new child".to_string())
        )
    );

    focus.node.data.start();
}

fn main() {
    // set up everything
    let mut root = tree::Node::new(
        task::Task::new("Root".to_string())
    );

    // zipper now owns the root node
    let mut focus = root.zipper();
    
    // interactive loop
    println!("Welcome to my Rust task manager!");
    // focus up-down left and right on these
    loop {
        let response: String = read!();
        focus = match response.as_ref() {
            "node" => { focus.node.data.print();
                        focus },
            "up" => focus.parent(),
            "down" => focus.child(0),
            _ => println!("try again")
        }
    }
}
