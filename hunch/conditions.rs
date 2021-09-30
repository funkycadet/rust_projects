fn main() {
    /*let mut i = 0;
    while i < 15 {
        i += 1;
        println!("{}", 
            if i % 3 == 0 && i % 5 == 0 {
                "fizzbuzz"
            }
            else if i % 5 == 0 {
                "buzz"
            }
            else if i % 3 == 0 {
                "fizz"
            }
            else {
                continue
            }
        )
    }*/
    /*Program to print multiples of 3 and 5 as fizz, buzz and fizzbuzz */
    let mut c = 0;
    while c < 15 {
        c += 1;
        if c % 3 == 0 && c % 5 == 0 {
            println!("fizzbuzz");
        }
        else if c % 3 == 0 {
            println!("fizz");
        }
        else if c % 5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", c);
        }
    }
    /*Short code to iterate numbers in a for loop */
    for i in 1..11 {
        println!("{}", i*i)
    }

}
