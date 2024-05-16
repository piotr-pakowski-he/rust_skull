use std::io;

fn main() {

    let mut guess = String::new();

    println!("Hello, world!");
    
    print!("{:?}",guess);

    let mut counter  = 0;
    let result = loop {
        counter = counter + 1;
        if counter == 100 {
            break "STO";
        }
    };

    let result_for = for a in 1..100 {
            if a == 33 {
                break;
            }
    };

    println!("{result}");

    println!("{result_for}");

}
