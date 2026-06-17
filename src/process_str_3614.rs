// https://leetcode.com/problems/process-string-with-special-operations-ii

use std::{cmp::Reverse, collections::VecDeque};

use crate::process_str_3614::Operator::Repeat;

enum Operator {
    Char(u8),
    Repeat(u16),
    Reverse,
    RemoveFirst,
    RemoveLast,
}

enum Expression {
    Text(VecDeque<u8>),
    Operation(Operator, Box<Expression>),
    Sequence(Box<Expression>, Box<Expression>),
}

impl Operator {
    fn apply(self, mut operand: Box<Expression>) -> Box<Expression> {
        use Operator::*;
        use Expression::*;
        match self {
            Char(c) => {
                if let Text(mut v) = *operand {
                    v.push_back(c);
                    operand
                } else {
                    Box::new(Sequence ( operand, Box::new(Text (VecDeque::from([c])))))
                }
            }
            _ => Box::new(Operation ( self, operand))
        }
    }
}

impl Expression {
    fn len(&self) -> usize {
        todo!()
    }
    fn get(&self, idx: usize) -> u8 {
        use Expression::*;
        match self {
            Text(deq) => *deq.get(idx).unwrap_or(&b'.'),
            Operation(operator, expression) => {
                match operator {
                    Operator::Char(c) => unreachable!(),
                    Repeat(_) => todo!(),
                    Operator::Reverse => todo!(),
                    Operator::RemoveFirst => todo!(),
                    Operator::RemoveLast => todo!(),
                }
            },
            Sequence(expression, expression1) => {
                let len = expression.len();
                if len > idx {
                    expression.get(idx)
                } else {
                    expression1.get(idx - len)
                }
            },
        }
    }
}

pub fn process_str(s: String, k: i64) -> char {

}
