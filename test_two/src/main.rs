//use cnsl::readln;

use std::{collections::{hash_map, HashMap}, iter::Map};

use cnsl::readln;

#[derive(Debug)]
struct AA
{
     x: i32,
     z: i32
}

impl std::fmt::Display for AA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Pole1: {} Pole2: {}", self.x, self.z)
    }
}

fn main() {

    let mut a: Vec<AA> = vec![];
    let mut b = HashMap::from([(String::from("A"), 1)]);
    let mut c: Vec<i32> = vec![1,23,4,5];
    let result = AA { x: readln!("Podaj liczbe").parse().unwrap() , z: 0 };

    a.push(result);
    b.insert(String::from("l"), 1);


    for i in a.iter()
    {
        print!("from loop {}",i.x);
    }

    println!("Witaj świecie");
    
    let mut t = "aaa".to_string();

    let mut _z: &mut str = &mut t;

    println!("{}", _z);   

    let mut userVal = readln!("Podaj");

    let _z = &mut userVal;


    test(& mut _z[1..4]);

    test(& mut _z[1..4]);

    println!("{}", _z);    

    println!("{:?}", a);    

}

fn test(a : &mut str)
{  
    println!("{a}");   
}


#[derive (Debug, Clone)]
struct Pociag<'a>
{
    has_lokomotywa: bool,
    is_osobowy: bool,
    nazwa:  &'a str
}

#[derive(Debug)]
struct Samolot
{
    nazwa: String,
    predkosc: i32
}

impl<'a> Copy for Pociag <'a> {}

fn main1() {

    let en47 = Pociag  { has_lokomotywa: false, is_osobowy: true, nazwa: "Kał" };
    
    let tu144 = Samolot { nazwa: String::from("Tupolew"), predkosc: 144 };
    
    let mut kibel = ("Osobowy", "Elektryczny", 20);

    kibel.0 = "KUpa";
    
    let tablica_kibli =  [(Pociag { has_lokomotywa: false, is_osobowy: true, nazwa: "Kał" }); 5];
    
    println!("{}-{}", en47.has_lokomotywa, en47.is_osobowy);
    
    println!("{}", en47.nazwa);
    
    println!("{:?}", kibel);
    
    println!("{:?}", kibel.0);
    
    println!("{:?}", kibel.1);
    
    println!("{:?}", kibel.2);
    
    println!("{:?}", tablica_kibli);
    
    println!("{:?}", tu144);
}
