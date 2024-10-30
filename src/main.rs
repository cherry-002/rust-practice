use regex::Regex;
use std::{any::Any, io, iter};

fn main() {
    println!("Rust cli calculator!\nSupports simple operations\nWaiting for your input...\n\x1b[3mExample: 2+2\x1b[0m\n");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("There was an error reading the value");

        let regex = Regex::new(r"(\d+|[+\-*/^])").unwrap();
        let result: Vec<&str> = regex.find_iter(&input).map(|mat| mat.as_str()).collect();

        fn seperate_input(result: Vec<&str>) -> (Vec<String>, Vec<String>) {
            let mut operators = Vec::new();
            let mut numbers = Vec::new();

            for (i, v) in result.iter().enumerate() {
                if i % 2 == 1 {
                    operators.push(v.to_string());
                } else {
                    numbers.push(v.to_string());
                };
            }
            (operators, numbers)
        }

        let (operators, number) = seperate_input(result.clone());

        for operator in operators {

        } 

        println!("Result: {result:?}");
    }
}
