pub fn run() {
    let name = "Joe";
    let mut age = 123;

    age = 321;

    println!("{} is {} y.o.", name, age);

    // const
    const ID: i32 = 123;
    println!("id is {}", ID);

    // multiple vars
    let (name, age) = ("John", 123);
    println!("{} is {}", name, age);
}