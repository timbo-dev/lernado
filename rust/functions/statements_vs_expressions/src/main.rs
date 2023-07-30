fn main() {
    let _y = 6; // This is a statement because don't return anything

    //let x = (let y = 6); // This will produce an error because a statement don't return anything

    // The let is a statement
    // The curly brackets is a expression because don't have the semicolon at the end
    let y = {
        let x = 3;
        x + 1 // Don't have the semicolon at the end
    };

    println!("The value of y is: {y}");
}

// This is a statement
fn another_function() {
    println!("Do something...");
}
