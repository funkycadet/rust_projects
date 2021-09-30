fn main() {
    /*arrays */
    let x = [4.; 10];
    println!("{}, {}", x[1], x[5]);
    
    /*adding and removing data in an array*/
    let mut x = vec!["This", " is"];
    println!("{}", x.len());
    x.push(" a");
    println!("{}", x.len());
    x.push(" sentence");
    println!("{}", x.len());
    x[0] = "That";

    for i in 0..x.len() {
        print!("{}", x[i]);
    }
    print!("\n");
    
    /*Debug printing, copying arrays and vectors*/
    let mut a1 = [4, 8, 40];
    let a2 = [6, 19, 278];
    println!("{:?}", a1);
    a1 = a2;
    println!("{:?}", a1);

}