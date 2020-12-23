#[derive(PartialEq, Eq, Clone, Copy)]
enum OpType {
    Sum,
    Mult
}

enum Expr {
    Value(usize),
    Parens(Box<Expr>),
    Operation(Box<Expr>, OpType, Box<Expr>)
}

#[derive(Eq, PartialEq)]
enum Token {
    Value(usize),
    Operation(OpType),
    OpenParen,
    CloseParen
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Value(num) => write!(f, "{}", num),
            Token::CloseParen => write!(f, " ) "),
            Token::OpenParen => write!(f, " ( "),
            Token::Operation(op) => match op {
                OpType::Mult => write!(f, "*"),
                OpType::Sum => write!(f, "+"),
            }
        }
    }
}

// TODO: Shunting yard Algorithm For Part 2.

impl Expr {
    fn eval(&self) -> Expr {
        use Expr::*;
        match self {
            Value(v) => Value(*v),
            Parens(boxed) => {
                let value = &*boxed;
                value.eval()
            },
            Operation(lhs, op, rhs) => {
                match (lhs.eval(), rhs.eval()) {
                    (Value(lhs_v), Value(rhs_v)) => {
                        let result = match op {
                            OpType::Sum => lhs_v + rhs_v,
                            OpType::Mult => lhs_v * rhs_v
                        };
                        Value(result)
                    },
                    _ => panic!()
                }

            }
        }
    }


    fn eval_fully(&self) -> usize {
        match self.eval() {
            Expr::Value(v) => {v},
            _ => panic!()
        }
    }

    fn eval_gold(&self) -> usize {
        self.eval_fully()
    }
}

fn new_op(lhs: Expr, op: OpType, rhs: Expr) -> Expr {
    Expr::Operation(Box::new(lhs), op, Box::new(rhs))
}

fn test() {
    use OpType::*;
    use Expr::*;

    // let example_1 = new_op(1, Sum, new_op(2, Mult, new_op(3, Sum, new_op(4, Mult, new_op(5, Sum, Expr::Value(6))))));
    let example_1 = new_op(new_op(new_op(new_op(new_op(Value(1), Sum, Value(2)), Mult, Value(3)), Sum, Value(4)), Mult, Value(5)), Sum, Value(6));
    example_1.eval_fully();

    let example_2 = new_op(new_op(Value(1), Sum, Parens(Box::new(new_op(Value(2), Mult, Value(3))) )),
                           Sum,
                           Parens(Box::new(new_op(Value(4), Mult,
                                                  Parens(Box::new(new_op(Value(5), Sum, Value(6)))))))
    );
    example_2.eval_fully();
}

fn get_matching_close_parens(start: usize, tokens: &[Token]) -> usize {
    let mut nested = 1;
    for (index, tok) in tokens[start..].iter().enumerate().skip(1) {
        if *tok == Token::OpenParen { nested += 1; }

        if *tok == Token::CloseParen{
            nested -= 1;
            if nested == 0 {
            return start + index;
        } }
    }
    panic!("No such closing parenthesis. (Start Index; {})", start);
}

fn match_one_node((start, end): (usize, usize), tokens: &[Token]) -> (usize, Expr) {
    use Token::*;
    match tokens[start] {
        Operation(_) => panic!("Invalid syntax: Unexpected operand"),
        CloseParen => panic!("Invalid syntax: Unexpected Close Paren."),
        OpenParen => {
            let close_paren = get_matching_close_parens(start, tokens);
            if close_paren >= end { panic!("Invalid Syntax: Closing Paren not found."); }
            parse_tokens_silver((start + 1, close_paren), tokens)
        },
        Value(value) => (start + 1, Expr::Value(value))
    }
}

fn parse_tokens_silver((start, end): (usize, usize), tokens: &[Token]) -> (usize, Expr) {
    use Token::*;
    let (mut curr_ind, mut curr_expr) = match_one_node((start, end), tokens);

    while curr_ind < end {
        let operator = match &tokens[curr_ind] {
            Operation(op) => op,
            _ => panic!("Expected operand. Did not find operand. Index: {}", curr_ind)
        };

        let (rhs_ind, rhs_expr)= match_one_node((curr_ind + 1, end), tokens);
        curr_ind = rhs_ind;
        curr_expr = Expr::Operation(Box::new(curr_expr), *operator, Box::new(rhs_expr));
    }

    (curr_ind + 1, curr_expr)
}


fn tokenize(s: &str) -> Vec<Token>{
    let s_new: String = s.replace('(', "( ").replace(')', " )");
    let token_strings: Vec<&str> = s_new.split(|c| c == ' ').map(|s| s).collect();
    token_strings.iter().map(
        |s| match *s {
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            "*" => Token::Operation(OpType::Mult),
            "+" => Token::Operation(OpType::Sum),
            _ => Token::Value(s.parse::<usize>().unwrap())
        }).collect()
}

fn evaluate(s: &str) -> usize {
    let tokens = tokenize(s);
    let (_ind, result) = parse_tokens_silver((0, tokens.len()), &tokens);
    result.eval_fully()
}

fn evaluate_gold(s: &str) -> usize{
    let tokens = tokenize(s);
    let (_ind, result) = parse_tokens_silver((0, tokens.len()), &tokens);
    result.eval_gold()
}

pub fn silver(data: &[String]) {
    let result: usize = data.iter().map(|s| evaluate(s)).sum();
    println!("Sum of all values: {}", result);
}

pub fn gold(data: &[String]) {
    let result: usize = data.iter().map(|s| evaluate_gold(s)).sum();
    println!("Sum of all values: {}", result);
}

pub fn setup() -> Vec<String>{
    crate::helper::file_to_vec("src/18_input.txt").unwrap()
}
pub fn day_18_soln() {
    // test();
    // parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
    let data = setup();
    silver(&data); // 6811433855019
    gold(&data);
}
