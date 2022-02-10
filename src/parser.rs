use std::fmt;
use crate::lexer::*;

pub enum NodeVal {
    Add, Sub, Mul, Div, Exp, Fac
}

pub enum LeafVal {
    Int(i32),
    Sym(String),
}

pub enum Node {
    Leaf(LeafVal),
    Node {
        v: NodeVal,
        children: Vec<Node>
    },
}

fn expr_bp(tokens: &mut Lexer, min_prec: i32) -> Node {
    let mut lhs = match tokens.next() {
        v @ (Token::Int(_) | Token::Sym(_))
            => Node::Leaf(LeafVal::from(v)),
        Token::LParen => {
            let lhs = expr_bp(tokens, 0);
            assert_eq!(tokens.next(), Token::RParen);
            lhs
        }
        op @ (Token::Minus | Token::Plus) => {
            let op = NodeVal::from(&op);
            let prec = op.prefix_prec();
            let rhs = expr_bp(tokens, prec);
            Node::Node { v: op, children: vec![rhs] }
        }
        e => panic!("Expected literal, found {e:?}")
    };

    loop {
        let op = match tokens.peek() {
            Token::Eof | Token::RParen => break,
            e @ Token::Int(_) => panic!("Expected operator, found {e:?}"),
            op => NodeVal::from(op),
        };

        if let Some(lhs_prec) = op.postfix_prec() {
            if lhs_prec < min_prec {
                break;
            }

            tokens.next();

            lhs = Node::Node { v: op, children: vec![lhs] };
            continue;
        }

        let (lhs_prec, rhs_prec) = op.infix_prec();
        if lhs_prec < min_prec {
            break;
        }

        tokens.next();

        let rhs = expr_bp(tokens, rhs_prec);

        lhs = Node::Node { v: op, children: vec![lhs, rhs]};
    };

    lhs
}

pub fn expr(s: &[u8]) -> Node {
    let mut lexer = Lexer::new(s);
    expr_bp(&mut lexer, 0)
}

fn fac(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        n => fac(n-1) * n,
    }
}

impl NodeVal {
    pub fn infix_prec(&self) -> (i32, i32) {
        match self {
            NodeVal::Add | NodeVal::Sub => (1,2),
            NodeVal::Mul | NodeVal::Div => (3,4),
            NodeVal::Exp => (8,7),
            _ => panic!(),
        }
    }

    pub fn prefix_prec(&self) -> i32 {
        match self {
            NodeVal::Add | NodeVal::Sub => 5,
                                      _ => panic!(),
        }
    }

    pub fn postfix_prec(&self) -> Option<i32> {
        match self {
            NodeVal::Fac => Some(6),
                       _ => None,
        }
    }

    pub fn apply(&self, args: &[i32]) -> i32 {
        match self {
            NodeVal::Add => args.iter().sum(),
            NodeVal::Sub => {
                match args.len() {
                    1 => -args[0],
                    2 => args[0]-args[1],
                    _ => panic!(),
                }
            },
            NodeVal::Mul => args.iter().product(),
            NodeVal::Div => {
                assert_eq!(args.len(), 2);
                args[0]/args[1]
            },
            NodeVal::Exp => {
                assert_eq!(args.len(), 2);
                args[0].pow(args[1] as u32)
            },
            NodeVal::Fac => {
                assert_eq!(args.len(), 1);
                fac(args[0])
            },
        }
    }
}

impl From<Token> for LeafVal {
    fn from(t: Token) -> Self {
        match t {
            Token::Int(v) => Self::Int(v),
            Token::Sym(v) => Self::Sym(v),
                        _ => panic!(),
        }
    }
}

impl From<&Token> for NodeVal {
    fn from(t: &Token) -> Self {
        match t {
            Token::Plus  => NodeVal::Add,
            Token::Minus => NodeVal::Sub,
            Token::Star  => NodeVal::Mul,
            Token::Slash => NodeVal::Div,
            Token::Caret => NodeVal::Exp,
            Token::Fac   => NodeVal::Fac,
                       _ => panic!(),
        }
    }
}

impl fmt::Display for LeafVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            LeafVal::Int(v) => v.to_string(),
            LeafVal::Sym(v) => v.to_string(),
        })
    }
}

impl fmt::Display for NodeVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            NodeVal::Add => "+",
            NodeVal::Sub => "-",
            NodeVal::Mul => "*",
            NodeVal::Div => "/",
            NodeVal::Exp => "^",
            NodeVal::Fac => "!",
        })
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Leaf(v) => write!(f, "{v}")?,
            Self::Node { v, children } => {
                write!(f, "({}", v)?;
                for i in children {
                    write!(f, " {}", i)?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

#[test]
fn tests() {
    let s = expr(b"1");
    assert_eq!(s.to_string(), "1");

    let s = expr(b"1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

    let s = expr(b"a + b * c * d + e");
    assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

    let s = expr(b"f ^ g ^ h");
    assert_eq!(s.to_string(), "(^ f (^ g h))");

    let s = expr(b" 1 + 2 + f ^ g ^ h * 3 * 4");
    assert_eq!(s.to_string(), "(+ (+ 1 2) (* (* (^ f (^ g h)) 3) 4))");

    let s = expr(b"--1 * 2");
    assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

    let s = expr(b"--f ^ g");
    assert_eq!(s.to_string(), "(- (- (^ f g)))");

    let s = expr(b"-9!");
    assert_eq!(s.to_string(), "(- (! 9))");

    let s = expr(b"f ^ g !");
    assert_eq!(s.to_string(), "(! (^ f g))");

    let s = expr(b"(((0)))");
    assert_eq!(s.to_string(), "0");
}
