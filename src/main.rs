use std::io;
fn main() {
    let mut resistors:Vec<u64> = Vec::new();
    while resistors.len() == 0 {
        let mut list: String = String::new();
        println!("write a list of resistor values separated by a space");
        match io::stdin().read_line(&mut list){
            Ok(_size) => {
                let v: Vec<&str> = list.split_whitespace().collect();
                for n in v{
                    match n.parse::<u64>() {
                        Ok(num) => {
                            resistors.push(num);
                        }
                        Err(error) => {
                            println!("error: {}", error);
                            resistors = vec![];
                            break;
                        }
                    }
                }
            }
            Err(error) => println!("error: {error}")
        };
    }
    println!("The parallel of {:?} is {} Ω", resistors, calculate_parallel(&resistors));
}

fn calculate_parallel(resistors: &Vec<u64>) -> f64 {
    let mut denominator: f64 = 0.0;
    for res in resistors {
        denominator += 1.0/(*res as f64) ;
    }
    1.0/denominator
}


