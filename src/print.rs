pub fn run() {
    println!("test from print.rs");

    println!("{} is {}", "rust", "fun");

    println!("{0} is {1}, really {1}", "rust", "fun");

    println!("{name} is {test}", name = "John", test = "Doe");

    println!("Binary {:b} Hex {:x} Octo {:o}", 10, 10, 10);

    println!("{:?}", (123, "abc", true)); // for debug stuff

    println!("1 + 2 = {}", 1+2);
}