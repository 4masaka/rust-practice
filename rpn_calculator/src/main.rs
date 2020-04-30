enum Operator {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

enum OperationElement {
    Operator(Operator),
    Operand(f32)
}

fn tokenize(formula_string: &str) -> Vec<OperationElement> {
    formula_string.split_whitespace()
    .map(|s| {
        match s {
            "+" => OperationElement::Operator(Operator::Addition),
            "-" => OperationElement::Operator(Operator::Substraction),
            "*" => OperationElement::Operator(Operator::Multiplication),
            "/" => OperationElement::Operator(Operator::Division),
            operand => OperationElement::Operand(
                operand.parse::<f32>().unwrap())
        }
    })
    .into_iter()
    .collect::<Vec<OperationElement>>()
}

fn calculate_rpn(formula_tokens: Vec<OperationElement>) -> f32 {
    let mut stack:Vec<f32> = Vec::new();
    for token in formula_tokens {
        match token {
            OperationElement::Operator(operator) => {
                let operand2: f32 = stack.pop().expect("a");
                let operand1: f32 = stack.pop().expect("b");
                let result = match operator {
                    Operator::Addition => operand1 + operand2,
                    Operator::Substraction => operand1 - operand2,
                    Operator::Multiplication => operand1 * operand2,
                    Operator::Division => operand1 / operand2
                };
                stack.push(result);
            },
            OperationElement::Operand(value) => stack.push(value)
        }
    }
    return stack.pop().expect("c")
}
fn main() {
    println!("{}", calculate_rpn(tokenize("1 2 3 + *")));
}
