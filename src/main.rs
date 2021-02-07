use std::io;
use std::io::Write;

fn main() {
  loop {
    print!("lisp>");
    io::stdout().flush().ok().expect("Cound not flush stdout");
    let mut input_line = String::new();
    match io::stdin().read_line(&mut input_line) {
      Ok(bytes_read) => {
        if bytes_read > 0 {
          println!("{}", process_line(&input_line));
        }else {
          println!("\nBye!");
          break;
        }
      }
      Err(_error) => {
        println!("Error");
        break;
      }
    }
  }
  println!("Hello, world!");
}

#[derive(PartialEq, Debug, Clone)]
enum Category {
  None,
  Number,
  Ident,
  Op,
  Paren
}

struct RawToken {
  text: String,
  category: Category
}

fn lexing(text: &String)-> Vec<RawToken> {  
  let mut results: Vec<RawToken> = vec![];
  let mut cate = Category::None;
  let mut char_bag: Vec<char> = vec![];
  
  fn cut(results: &mut Vec<RawToken>, char_bag: &Vec<char>, cate: &Category) {
    if char_bag.len() > 0 {
      results.push(RawToken {
        text: char_bag.into_iter().collect(),
        category: cate.clone()
      });
    }
  }
  
  for (_i, c) in text.chars().enumerate() {
    match c {
      '0' ..= '9' => {
        match cate {
          Category::Number | Category::Ident => {}
          _ => {
            // TODO
            cut(&mut results, &char_bag, &cate);
            char_bag = vec![];
            cate = Category::Number;
          }
        }
      }
      'a' ..= 'z' | 'A' ..= 'Z' | '_' => {
        match cate {
          Category::Ident => {}
          _ => {
            cut(&mut results, &char_bag, &cate);
            char_bag = vec![];
            cate = Category::Ident;
          }
        }
      }
      '(' | ')' => {
        cut(&mut results, &char_bag, &cate);
        cut(&mut results, &vec![c], &Category::Paren);
        char_bag = vec![];
        
        cate = Category::None;
        continue;
      }
      '+' | '-' | '*' | '/' => {
        cut(&mut results, &char_bag, &cate);
        cut(&mut results, &vec![c], &Category::Op);
        char_bag = vec![];
        
        cate = Category::None;
        continue;
      }
      ' ' => {
        cut(&mut results, &char_bag, &cate);
        char_bag = vec![];
        cate = Category::None;
        continue;
      }
      _ => {}
    }
    
    char_bag.push(c);
  }
  
  cut(&mut results, &char_bag, &cate);
  
  results
}

enum Token {
  Number(i32),
  Ident(String),
  LParen,
  RParen
}

fn preprocess(tokens: Vec<RawToken>)-> Vec<Token> {
  tokens
  .into_iter()
  .map(|token|
    match token.category {
      Category::Number => {
        Token::Number(token.text.parse::<i32>().expect("Illegal number"))
      }
      Category::Ident => {
        Token::Ident(token.text)
      }
      Category::Op => {
        // let op =
        //   match token.text.as_str() {
        //     "+" => Operator::Add,
        //     "-" => Operator::Sub,
        //     "*" => Operator::Mult,
        //     "/" => Operator::Div,
        //     _ => panic!()
        //   };
        // Token::Op(op)
        Token::Ident(token.text)
      }
      Category::Paren => {
        match token.text.as_str() {
          "(" => Token::LParen,
          ")" => Token::RParen,
          _ => panic!()
        }
      }
      Category::None => panic!()
    }
  )
  .collect()
}

struct Stack<T> {
  buffer: Vec<T>,
  size: usize,
}

impl<T> Stack<T> {
  pub fn new() -> Stack<T> {
    Stack {
      buffer: vec![],
      size: 0
    }
  }
  
  pub fn push(&mut self, elem: T) {
    self.buffer.push(elem);
    self.size += 1;
  }
  
  pub fn pop(&mut self) -> T {
    if self.size == 0 {
      panic!();
    }
    self.size -= 1;
    self.buffer.pop().unwrap()
  }
  
  // pub fn pop(&self) -> T {
  //   if self.size == 0 {
  //     panic!();
  //   }
  //   self.size -= 1;
  //   self.buffer[self.size]
  // }
}

#[derive(Debug)]
enum Sexp {
  Int(i32),
  Symbol(String),
  Cons(Box<Sexp>, Box<Sexp>),
  Nil,
}

fn parse_list(tokens: &Vec<Token>, index: usize)-> (Sexp, usize) {
  match &tokens[index] {
    Token::RParen => (Sexp::Nil, index + 1),
    _ => {
      let (car, index) = parse(tokens, index);
      //)までパース
      let (cdr, index) = parse_list(tokens, index);
      (Sexp::Cons(Box::new(car), Box::new(cdr)), index)
    }
  }
}

fn parse(tokens: &Vec<Token>, index: usize) -> (Sexp, usize) {
  match &tokens[index] {
    Token::LParen => {
      parse_list(&tokens, index + 1)
    }
    Token::Ident(s) => {
      (Sexp::Symbol(s.to_string()), index + 1)
    }
    Token::Number(n) => {
      (Sexp::Int(*n), index + 1)
    }
    Token::RParen => panic!()
  }
}

// fn eval_calc(tokens: Vec<Token>) -> i32 {
//   let mut stack = Stack::new();
  
//   for token in tokens {
//     match token {
//       Token::Number(n) => {
//         stack.push(n);
//       }
//       Token::Ident(op) => {
//         // binop
//         let n1 = stack.pop();
//         let n2 = stack.pop();
//         let n = match op.as_str() {
//           "+" => n1 + n2,
//           "-" => n1 - n2,
//           "*" => n1 * n2,
//           "/" => n1 / n2,
//           _ => panic!()
          
//         };
//         stack.push(n);
//       }
//       _ => panic!()
//     }
//   }
  
//   stack.pop()
// }

// struct HashMap {
  
// }

// impl HashMap {
//   pub fn has(&self, key: K)-> bool {
    
//   }
  
//   pub fn get(&self, key: K)-> Option<V> {
    
//   }
// }

fn add(args: Vec<Sexp>)-> Sexp {
  let (n1, n2) = check_bin_int(args);
  Sexp::Int(n1 + n2)
}
fn sub(args: Vec<Sexp>)-> Sexp {
  let (n1, n2) = check_bin_int(args);
  Sexp::Int(n1 - n2)
}
fn mult(args: Vec<Sexp>)-> Sexp {
  let (n1, n2) = check_bin_int(args);
  Sexp::Int(n1 * n2)
}
fn div(args: Vec<Sexp>)-> Sexp {
  let (n1, n2) = check_bin_int(args);
  Sexp::Int(n1 * n2)
}

fn eval_list(sexp: Sexp) -> Vec<Sexp> {
  match sexp {
    Sexp::Cons(car, cdr) => {
      let mut tail = eval_list(*cdr);
      tail.push(eval(*car));
      tail
    }
    Sexp::Nil => vec![],
    _ => panic!()
  }
}

fn check_bin_int(args: Vec<Sexp>)-> (i32, i32) {
  if args.len() != 2 { panic!() }
  let n1 = &args[0];
  let n2 = &args[1];
  
  match (n1, n2) {
    (Sexp::Int(n1), Sexp::Int(n2)) => {
      (*n1, *n2)
    }
    _ => {
      panic!()
    }
  }
}

fn eval_function(car: Sexp, cdr: Vec<Sexp>)-> Sexp {
  match car {
    Sexp::Symbol(s) => {
      let fun = match s.as_str() {
        "+" => add,
        "-" => sub,
        "*" => mult,
        "/" => div,
        _ => panic!()
      };
      fun(cdr)
    }
    _ => panic!()
  }
}

fn is_intrinsic(car: &Sexp)-> bool {
  match car {
    Sexp::Symbol(s) => match s.as_str() {
      "define" => true,
      _ => false
    },
    _ => false
  }
}

// fn eval_intrinsic(car: Sexp, cdr: Vec<Sexp>)-> Sexp {
//   match car {
//     Sexp::Symbol(s) => match s.as_str() {
//       "define" => eval_define(cdr),
//       _ => panic!()
//     }
//     _ => panic!()
//   }
// }

fn eval(sexp: Sexp)-> Sexp {
  match sexp {
    Sexp::Cons(car, cdr) => {
      let car = eval(*car);
      let cdr = eval_list(*cdr);
      if is_intrinsic(&car) {
        // eval_intrinsic(car, cdr)
        panic!()
      } else {
        eval_function(car, cdr)
      }
    }
    _ => sexp
  }
}


fn process_line(input_line: &String)-> String {
  let tokens = lexing(&(String::from(input_line.trim())));
  for raw_token in tokens.iter() {
    println!("RawToken={}, {:?}", raw_token.text, raw_token.category);
  }
  let tokens = preprocess(tokens);
  println!("");
  
  let (sexp, _) = parse(&tokens, 0);
  let result = eval(sexp);
  // to_string(result)
  format!("{:?}", result)
}

