use std::io;
use regex::Regex;

// enum CircuitType{
//     Series(Vec<f32>),
//     Parallel(Vec<f32>)
// }

fn main() {
    let mut circuit: String = String::new();
    println!("write a circuit of resistor values separated by a space");
    match io::stdin().read_line(&mut circuit){
        Ok(_size) => {
            match parse_input(circuit.trim()){
                Ok(res) => {
                    println!("The resistance of {} is {} Ω", circuit, res);
                },
                Err(err) => {println!("Something went wrong: {}", err)}
            }
        }
        Err(error) => {
            println!("error: {}", error);
        }
                  
    }
}

fn parse_input(input: &str) -> Result<f32, &'static str >{
    let open_parentheses = Regex::new(r"\(").unwrap();
    let closed_parentheses = Regex::new(r"\)").unwrap();

    let simple_series = Regex::new(r"s\(([^\(\)]*)\)").unwrap();
    let simple_parallel = Regex::new(r"p\(([^\(\)]*)\)").unwrap();

    let closed_match: Vec<regex::Captures> = closed_parentheses.captures_iter(input).collect();
    let open_match: Vec<regex::Captures> = open_parentheses.captures_iter(input).collect();

    if open_match.len() != closed_match.len() {
        return Err("the circuit is not valid, please check")
    }

    let mut circuit: String = input.to_owned();
    while circuit.parse::<f32>().is_err() {
        let mut go_on: bool = false;
        let series = simple_series.captures(&circuit);
        if !series.is_none() {
            go_on = true;
            let series = series.unwrap();
            let values: Vec<f32> = series[1].split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect(); // get the resistace values
            circuit = circuit.replace(&series[0], &calculate_series(&values).to_string());
        }
        let parallel = simple_parallel.captures(&circuit);
        if !parallel.is_none() {
            go_on = true;
            let parallel = parallel.unwrap();
            let values: Vec<f32> = parallel[1].split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect(); // get the resistace values
            circuit = circuit.replace(&parallel[0], &calculate_parallel(&values).to_string());
        }
        if !go_on {
            return Err("the circuit is not valid, please check");
        }
    }

    Ok(circuit.parse::<f32>().unwrap())
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

#[test]
fn test_input() -> (){
    assert_eq!(950.0, parse_input("s(200 500 p(500 500))").unwrap());
}
