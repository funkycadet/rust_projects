fn main() {
    println!("Hello, world. Your number is {}", 49.5 + 2.);
    println!("This \
        is \
        a new statement.");
    let num = 12;
    let mee = 15;
    println!("David is aged {}.", num + mee);
    println!("{}", "A" > "a");
    let mut n = 3;
    n += 1;
    if n < 3 {
        println!("{}", n);
    }
    else {
        println!("{}", n);
    }
}