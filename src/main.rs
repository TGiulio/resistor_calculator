use std::io;
fn main() {
    let mut resistors:Vec<f32> = Vec::new();
    while resistors.len() == 0 {
        let mut list: String = String::new();
        println!("write a list of resistor values separated by a space");
        match io::stdin().read_line(&mut list){
            Ok(_size) => {
                let v: Vec<&str> = list.split_whitespace().collect();
                for n in v{
                    match n.parse::<f32>() {
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

fn calculate_parallel(resistors: &Vec<f32>) -> f32 {
    let mut denominator: f32  = 0.0;
    for res in resistors {
        denominator += 1.0/(*res ) ;
    }
    1.0/denominator
}

fn calculate_series(resistors: &Vec<f32>) -> f32 {
    resistors.iter().fold(0.0, |acc, x| acc + x)
}

#[test]
fn test_parallel() -> (){
    assert_eq!(calculate_parallel(&vec![100.0, 100.0]), 50.0);
}

#[test]
fn test_series() -> (){
    assert_eq!(calculate_series(&vec![100.0, 100.0]), 200.0)
}
