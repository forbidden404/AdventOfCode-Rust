use std::collections::HashMap;
use std::path::Path;

pub fn main_2015_7() {
    let path = Path::new("src/_2015/inputs/day07.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    process(&input);
}

#[derive(Clone)]
enum Op {
    Assign,
    And,
    Or,
    LShift,
    RShift,
    Not,
}
#[derive(Clone)]
struct OperationParsing {
    output_variable: String,
    operation: Op,
    input_variables: [String; 2], 
}

impl OperationParsing {
    fn new(output_variable: String, operation: Op, input_variables: [String; 2]) -> OperationParsing {
        OperationParsing { output_variable, operation, input_variables }
    }

    fn parse(input: &str) -> OperationParsing {
        let words: Vec<&str> = input.split(&[' ', '-', '>'][..]).filter(|x| *x != "").collect();
        let operation: Op;
        let mut input_variables = [String::from(""), String::from("")];
        if words.len() == 2 {
            operation = Op::Assign;
            input_variables[0] = words[0].to_string();
        } else if words.len() == 3 {
            operation = Op::Not;
            input_variables[0] = words[1].to_string();
        } else {
            match words[1] {
                "AND" => operation = Op::And,
                "OR" => operation = Op::Or,
                "LSHIFT" => operation = Op::LShift,
                "RSHIFT" => operation = Op::RShift,
                input => panic!("Invalid operation: `{}`", input),
            }

            input_variables[0] = words[0].to_string();
            input_variables[1] = words[2].to_string();
        }

        let output_variable = words.last().unwrap().to_string();
        OperationParsing::new(output_variable, operation, input_variables)
    }
}

type Cache = HashMap<String, isize>;
type Variables = HashMap<String, OperationParsing>;
fn evaluate(cache: &mut Cache, variables: &Variables, variable: &String) -> isize {
    if let Some(cached_value) = cache.get(variable) {
        return *cached_value;
    } else {
        let result: isize;
        let a: isize;
        let b: isize;
        let current = variables.get(variable).unwrap();
        let lhs = &current.input_variables[0];
        let rhs = &current.input_variables[1];
        match current.operation {
            Op::And => {
                a = lhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, lhs));

                b = rhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, rhs));

                result = a & b;
            }
            Op::Or => {
                a = lhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, lhs));

                b = rhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, rhs));

                result = a | b;
            }
            Op::Not => {
                a = lhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, lhs));

                result = !a;
            }
            Op::Assign => {
                result = lhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, lhs));
            }
            Op::LShift => {
                a = lhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, lhs));

                b = rhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, rhs));

                result = a << b;
            }
            Op::RShift => {
                a = lhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, lhs));

                b = rhs.parse::<isize>()
                    .unwrap_or_else(|_| evaluate(cache, variables, rhs));

                result = a >> b;
            }
        }

        cache.insert(variable.to_string(), result);
        return result;
    } 
}

fn process(input: &str) {
    let mut evaluated_values = HashMap::new();
    let mut variables = HashMap::new();

    for line in input.lines() {
        let operation_parsing = OperationParsing::parse(line);
        variables.insert(operation_parsing.output_variable.clone(), operation_parsing.clone());
    }

    let a = evaluate(&mut evaluated_values, &variables, &String::from("a"));
    println!("The signal provided to wire a was {}", a);

    evaluated_values.clear();
    variables.insert(String::from("b"), OperationParsing::parse(&format!("{} -> b", a)));

    let a = evaluate(&mut evaluated_values, &variables, &String::from("a"));
    println!("The signal provided to wire a was {} in part two", a);
}