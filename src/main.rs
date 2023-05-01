use std::io;
fn main() {
    let mut resistances:Vec<u64> = Vec::new();
    let mut list: String = String::new();
    while resistances.len() == 0 {
        println!("write a list of resistor values separated by a space");
        match io::stdin().read_line(&mut list){
            Ok(_size) => {
                let v: Vec<&str> = list.split_whitespace().collect();
                for n in v{
                    match n.parse::<u64>() {
                        Ok(num) => {
                            resistances.push(num);
                        }
                        Err(error) => {
                            println!("error: {}", error);
                            resistances = vec![];
                            break;
                        }
                    }
                }
            }
            Err(error) => println!("error: {error}")
        };
    }
    println!("The parallel of {:?} is {} Ω", resistances, calculate_parallel(&resistances));
}

fn calculate_parallel(resistances: &Vec<u64>) -> f64 {
    let mut denominator: f64 = 0.0;
    for res in resistances {
        denominator += 1.0/(*res as f64) ;
    }
    1.0/denominator
}


