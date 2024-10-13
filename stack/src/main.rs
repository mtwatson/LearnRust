use std::collections::HashMap;

use stack::Stack;

pub mod stack;

fn tokenize(the_string: &str) -> Vec<&str>
{
    the_string.split(" ")
              .filter(|&x| !x.is_empty())
              .collect::<Vec<&str>>()
}

fn is_operator(token: &str) -> bool
{
    let operators = vec!["+", "-", "*", "/", "(", ")"];
    operators.contains(&token)
}

fn is_operand(token: &str) -> bool { !is_operator(token) }

fn get_precedence(token: &str) -> i32
{
    let precedence_map = HashMap::from([("(", 0), ("+", 1), ("-", 1), ("*", 2), ("/", 2)]);
    *precedence_map.get(token).unwrap()
}

fn infix_to_postfix(infix_expression: &str) -> String
{
    let mut postfix_tokens = Vec::<&str>::new();
    let tokens = tokenize(&infix_expression);
    let mut the_stack = Stack::<&str>::new();

    for token in tokens
    {
        if is_operand(token)
        {
            postfix_tokens.push(token);
        }
        else
        {
            match token
            {
                "(" => the_stack.push(token),
                ")" =>
                {
                    while let Some(top_item) = the_stack.pop()
                    {
                        if top_item == "("
                        {
                            break;
                        }
                        postfix_tokens.push(&top_item);
                    }
                }
                _ =>
                {
                    if the_stack.is_empty()
                    {
                        the_stack.push(token);
                    }
                    else
                    {
                        while let Some(top_item) = the_stack.top()
                        {
                            if get_precedence(top_item) < get_precedence(token)
                            {
                                break;
                            }
                            postfix_tokens.push(top_item);
                            the_stack.pop();
                        }
                        the_stack.push(token);
                    }
                }
            }
        }
    }

    while let Some(top_item) = the_stack.pop()
    {
        postfix_tokens.push(top_item);
    }

    postfix_tokens.join(" ")
}

fn postfix_calculate(postfix_expression: &String) -> Option<i64>
{
    let tokens = tokenize(&postfix_expression);
    let mut the_stack = Stack::<i64>::new();
    for token in tokens
    {
        if is_operand(token)
        {
            if let Ok(value) = token.parse::<i64>()
            {
                the_stack.push(value);
            }
        }
        else
        {
            if the_stack.size() > 1
            {
                if let Some(second) = the_stack.pop()
                {
                    if let Some(first) = the_stack.pop()
                    {
                        let result = match token
                        {
                            "+" => first + second,
                            "-" => first - second,
                            "*" => first * second,
                            "/" => first / second,
                            _ => 0, // bad error handling
                        };
                        the_stack.push(result);
                    }
                }
            }
        }
    }
    the_stack.pop()
}

fn main()
{
    let infix_expression = "1 + 2 * ( 3 - 4 ) * ( 5 + 6 * 7 ) - 8";
    let postfix_expression = infix_to_postfix(infix_expression);
    println!("{}", postfix_expression);
    println!("{}", postfix_calculate(&postfix_expression).unwrap());
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_infix_to_postfix()
    {
        assert_eq!(infix_to_postfix("1 + 2 * ( 3 - 4 ) * ( 5 + 6 * 7 ) - 8"),
                   "1 2 3 4 - * 5 6 7 * + * + 8 -");
        assert_eq!(infix_to_postfix("1 + 2 * 3 - 4"), "1 2 3 * + 4 -");
    }

    #[test]
    fn test_postfix_calculate()
    {
        assert_eq!(postfix_calculate(&infix_to_postfix("1 + 2 * ( 3 - 4 ) * ( 5 + 6 * 7 ) - 8")),
                   Some(-101));
        assert_eq!(postfix_calculate(&infix_to_postfix("1 + 2 * 3 - 4")),
                   Some(3));
    }
}
