use rand::Rng;

enum State
{
    None,
    Begin(String),
    Processing(i32), 
    End
}

fn main() {

   let state = &pobierz_status();

    match state {
        State::None => println!("Brak informacji o procesie"),
        State::Begin(name) => println!("Proces rozpoczyna sie {}", name), 
        State::Processing(progress) =>println!("Proces jest podczas procesowania procesu {}", progress),
        State::End => println!("Proces sie konczy"),
    }

    println!("Proces na koncu maina ma status: {} ", state.to_string());
}

fn pobierz_status() -> State
{
     let mut rng = rand::thread_rng();     
     let num = rng.gen_range(0..4);
     let progress = rng.gen_range(0..100);

     match num  
     {         
         1 => return  State::Begin("Witaj Andrzeju".to_string()),
         2 => return  State::Processing(progress),
         3 => return  State::End,
        _ =>  return State::None,
     }
}

impl State {

    fn to_string(&self) -> String {
            match self  {
                State::None => return "None".to_string(),
                State::Begin(_) => return  "Begin".to_string(),
                State::Processing(_) => return  "Processing".to_string(),
                State::End =>  return  "End".to_string(),
            }
    }
    
}   