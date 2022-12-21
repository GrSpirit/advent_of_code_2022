use std::{collections::HashMap, str::FromStr};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal error")]
    #[allow(unused)]
    Internal,

    #[error("Parse operation error")]
    ParseOperatonError,

    #[error("Wrong operation")]
    WrongOperation,

    #[error("Monkey not found")]
    NoMonkey,
}

#[derive(Debug, Clone, Copy)]
enum OpType { Add, Sub, Mul, Div }

#[derive(Debug, Clone)]
struct Op {
    lhs: String,
    rhs: String,
    op_type: OpType,
}

#[derive(Debug, Clone)]
enum Yell {
    Number(i64),
    Operation(Op)
}

impl FromStr for Op {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        let lhs = parts.get(0).ok_or(Error::ParseOperatonError)?.to_string();
        let rhs = parts.get(2).ok_or(Error::ParseOperatonError)?.to_string();
        let op_type = match parts[1] {
            "+" => OpType::Add,
            "-" => OpType::Sub,
            "*" => OpType::Mul,
            "/" => OpType::Div,
            _ => return Err(Error::WrongOperation)
        };
        Ok(Op{ lhs, rhs, op_type })
    }
}

impl FromStr for Yell {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(|x| Yell::Number(x)).or_else(|_| s.parse::<Op>().map(|op| Yell::Operation(op)))
    }
}

impl Op {
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self.op_type {
            OpType::Add => lhs + rhs,
            OpType::Sub => lhs - rhs,
            OpType::Mul => lhs * rhs,
            OpType::Div => lhs / rhs,
        }
    }
}

fn parse_input<S: AsRef<str>>(lines: &[S]) -> Result<HashMap<String, Yell>, Error> {
    Ok(lines.iter().map(|l| {
        let (name, yell) = l.as_ref().split_once(':').unwrap();
        (name.to_string(), yell.trim().parse::<Yell>().unwrap())
    }).collect())
}

fn compute(monkeys: &HashMap<String, Yell>, key: &str) -> Result<i64, Error> {
    if let Some(yell) = monkeys.get(key) {
        let res = match yell {
            Yell::Number(x) => *x,
            Yell::Operation(op) => op.apply(compute(monkeys, &op.lhs)?, compute(monkeys, &op.rhs)?)
        };
        Ok(res)
    } else {
        Err(Error::NoMonkey)
    }
}

fn compute_with_cache(monkeys: &HashMap<String, Yell>, key: &str, cache: &mut HashMap<String, i64>, human_cache: &mut HashMap<String, bool>) -> Result<(i64, bool), Error> {
    if let Some(yell) = monkeys.get(key) {
        let res = match yell {
            Yell::Number(x) => (*x, key == "humn"),
            Yell::Operation(op) => {
                let (lhs, lhumn) = compute_with_cache(monkeys, &op.lhs, cache, human_cache)?;
                let (rhs, rhumn) = compute_with_cache(monkeys, &op.rhs, cache, human_cache)?;
                (op.apply(lhs, rhs), lhumn || rhumn)
            }
        };
        cache.insert(key.to_string(), res.0);
        human_cache.insert(key.to_string(), res.1);
        Ok(res)
    } else {
        Err(Error::NoMonkey)
    }
}

fn expect(monkeys: &HashMap<String, Yell>, key: &str, expected_num: i64, cache: &HashMap<String, i64>, human_cache: &HashMap<String, bool>) -> Option<i64>{
    let yell = monkeys.get(key).unwrap();
    match yell {
        Yell::Operation(op) => {
            let left_human = *human_cache.get(&op.lhs).unwrap();
            let left_num = *cache.get(&op.lhs).unwrap();
            let right_num = *cache.get(&op.rhs).unwrap();
            match op.op_type {
                OpType::Add => {
                    if left_human {
                        expect(monkeys, &op.lhs, expected_num - right_num, cache, human_cache)
                    } else {
                        expect(monkeys, &op.rhs, expected_num - left_num, cache, human_cache)
                    }
                },
                OpType::Sub => {
                    if left_human {
                        expect(monkeys, &op.lhs, expected_num + right_num, cache, human_cache)
                    } else {
                        expect(monkeys, &op.rhs, left_num - expected_num, cache, human_cache)
                    }
                },
                OpType::Mul => {
                    if left_human {
                        expect(monkeys, &op.lhs, expected_num / right_num, cache, human_cache)
                    } else {
                        expect(monkeys, &op.rhs, expected_num / left_num, cache, human_cache)
                    }
                },
                OpType::Div => {
                    if left_human {
                        expect(monkeys, &op.lhs, expected_num * right_num, cache, human_cache)
                    } else {
                        expect(monkeys, &op.rhs, left_num / expected_num, cache, human_cache)
                    }
                }
            }
        },
        Yell::Number(_) => {
            if *human_cache.get(key).unwrap() {
                Some(expected_num)
            } else {
                None
            }
        }
    }
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i64, Error> {
    let monkeys = parse_input(lines)?;
    compute(&monkeys, "root")
}

pub fn task2<S: AsRef<str>>(lines: &[S], expected_num: i64) -> Result<i64, Error> {
    let monkeys = parse_input(lines)?;
    let mut cache = HashMap::new();
    let mut human_cache = HashMap::new();
    compute_with_cache(&monkeys, "root", &mut cache, &mut human_cache)?;
    Ok(expect(&monkeys, "root", expected_num, &cache, &human_cache).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";

    #[test]
    fn test1() {
        let lines = DATA.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok(152), task1(&lines));
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok(301), task2(&lines, 300));
    }
}
