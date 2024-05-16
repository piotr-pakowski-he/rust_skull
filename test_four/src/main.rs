fn main() {

    println!("Staś się chlasta nożem do ciasta.  też jest rasta!");



    let mut a = String::from("String1");
    a = operacja(a);

    let b = &a;
    let c = &mut a;   

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

}

fn operacja(mut a: String) -> String
{
    a.push_str("dodan stomg"); 
    format!("Dlugosc: {}", a.len())
}
