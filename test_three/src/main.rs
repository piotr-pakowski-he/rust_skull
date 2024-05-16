use cnsl::{self, readln};
use named_tuple::named_tuple;

named_tuple!(
    #[derive(Copy, Clone)]
    struct  pociag<'a>
    {
        typ: &'a str,
        code_name: &'a str,
        max_speed: i32,
        is_passazerski: bool 
    }
);

fn g() -> (i32, String) {
    (3,"".to_string())
}

fn main () {

    let mut s: pociag = ("EN57", "KIBEL", 40, true).into();
    let aaa = ("aaa", 111, 222);
    let (x,y,z) = aaa;    
    s.set_typ("andrzej");
    
    let _a = readln!("Witaj strudzony pielgrzymie!");
    let tab_pociag = [s; 5];
    let tab_z = [tab_pociag; 10];

    let _wek = vec![1,2,3];
    let mut wek1: Vec<pociag> = vec![];
        
    let v1 = readln!("AAA");

    wek1.push(("EN57", "KIBEL", readln!("AAA").parse().unwrap(), true).into());
    wek1.push((v1.as_ref(), "KIBEL", readln!("AAA").parse().unwrap(), true).into());

    for i in wek1.iter()
    {
        tab_z[0][0].code_name();
        println!("Kołd name: {}", i.code_name());
    }

    // for i in wek1
    // {
    //     tab_z[0][0].code_name();
    //     println!("Kołd name: {}", i.code_name());
    // }

    
    wek1.push(("EN57", "KIBEL", readln!("AAA").parse().unwrap(), true).into());

//     let p = ( 1, String::from("aaa"), true);
//     let mut pociag = (String::from("EN57"), "KIBEL", 40, true);

//     print!("Typ pociagu: {} code name: {} max speed:{}  pasazerski{}", pociag.0, pociag.1, pociag.2, pociag.3);

//     pociag.0 = String::from("value");
// 12
//     print!("Typ pociagu: {} code name: {} max speed:{}  pasazerski{}", pociag.0, pociag.1, pociag.2, pociag.3);

}
