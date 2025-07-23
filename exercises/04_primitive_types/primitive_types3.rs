fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let mut a = String::from("I Love Sharon");

    for _ in 0..1009 {
        a.push_str(" I Love Sharon");
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
