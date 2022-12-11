use std::str::FromStr;

pub type Result<T> = std::result::Result<T, &'static str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variable {
    Const(i64),
    Old
}

impl FromStr for Variable {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self> {
        if s == "old" {
            Ok(Variable::Old)
        } else {
            let c = s.parse().map_err(|_| "cannot parse a constant")?;
            Ok(Variable::Const(c))
        }
    }
}

impl Variable {
    fn get_or(&self, old: i64) -> i64 {
        match self {
            Variable::Const(c) => *c,
            Variable::Old => old
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Variable, Variable),
    Mult(Variable, Variable),
}

impl FromStr for Operation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        let a = parts[0].parse()?;
        let b = parts[2].parse()?;
        match parts[1] {
            "+" => Ok(Operation::Add(a, b)),
            "*" => Ok(Operation::Mult(a, b)),
            _ => Err("unknown operation")
        }
    }
}

impl Operation {
    fn apply(&self, old: i64) -> i64 {
        use Operation::*;
        match self {
            Add(a, b) => a.get_or(old) + b.get_or(old),
            Mult(a, b) => a.get_or(old) * b.get_or(old)
        }
    }
}

#[derive(Debug)]
struct Test {
    t: i64,
}

impl Test {
    fn check(&self, x: i64) -> bool {
        (x % self.t) == 0
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Operation,
    test: Test,
    if_true: usize,
    if_false: usize,
    count: i64
}

impl Monkey {
    fn from<S: AsRef<str>>(lines: &[S]) -> Result<Self> {
        let items = lines[1].as_ref().split(": ").last().unwrap()
            .split(", ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let op = lines[2].as_ref().split(" = ").last().unwrap().parse()?;
        let test = Test { t: lines[3].as_ref().rsplit(' ').next().unwrap().parse().unwrap() } ;
        let if_true = lines[4].as_ref().rsplit(' ').next().unwrap().parse().unwrap();
        let if_false = lines[5].as_ref().rsplit(' ').next().unwrap().parse().unwrap();
        Ok(Self{items, op, test, if_true, if_false, count: 0})
    }
}

fn parse_monkeys<S: AsRef<str>>(lines: &[S]) -> Vec<Monkey> {
    lines.chunks(7).map(|l| Monkey::from(l).unwrap()).collect()
}

fn round(monkeys: &mut [Monkey], divisor: i64, common_divisor: i64) {
    for i in 0..monkeys.len() {
        let monk = &mut monkeys[i];
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();
        for item in &monk.items {
            let new_item = monk.op.apply(*item) / divisor % common_divisor;
            if monk.test.check(new_item) {
                true_items.push(new_item);
            } else {
                false_items.push(new_item);
            }
            monk.count += 1;
        }
        monk.items.clear();
        let true_idx = monk.if_true;
        let false_idx = monk.if_false;
        monkeys[true_idx].items.extend(true_items);
        monkeys[false_idx].items.extend(false_items);
    }
}

pub fn task1<S: AsRef<str>>(lines: &[S], n: u32, divisor: i32) -> Result<i64> {
    let mut monkeys = parse_monkeys(lines);
    let commont_divisor = monkeys.iter().map(|m| m.test.t).product();
    for _ in 0..n {
        round(&mut monkeys, divisor as i64,commont_divisor);
    }
    monkeys.sort_by_key(|m| - m.count);
    Ok(monkeys.iter().take(2).map(|m| m.count as i64).product())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test1() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok(10605), task1(&lines, 20, 3));
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok(2713310158), task1(&lines, 10000, 1));
    }
}
