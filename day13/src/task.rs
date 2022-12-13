use std::ops::Index;
use std::slice::SliceIndex;
use std::str::{self, FromStr};
use std::cmp::{PartialOrd, Ord, Ordering, PartialEq, Eq};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("Internal Error")]
    #[allow(unused)]
    Internal,
    #[error("Wrong list format")]
    WrongListFormat,
    #[error("Parse error")]
    ParseError,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Error::ParseError
    }
}

#[derive(Debug, Clone)]
enum ListNode {
    Val(i32),
    List(List),
}

#[derive(Debug, Clone, Default)]
struct List {
    data: Vec<ListNode>,
}

macro_rules! list {
    [$($x: expr),*] => {{
        let mut new_list = List::new();
        $(
            new_list.push($x);
        )*
        new_list
    }};
}

impl List {
    fn new() -> Self {
        Default::default()
    }
    fn push(&mut self, x: ListNode) {
        self.data.push(x);
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for List {}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut i = 0;
        while i < self.len() && i < other.len() {
            let cmp_res = match &self[i] {
                ListNode::List(sublist1) => {
                    match &other[i] {
                        ListNode::List(sublist2) => {
                            sublist1.cmp(&sublist2)
                        },
                        ListNode::Val(x2) => {
                            let sublist2 = list![ListNode::Val(*x2)];
                            sublist1.cmp(&sublist2)
                        }
                    }
                },
                ListNode::Val(x1) => {
                    match &other[i] {
                        ListNode::Val(x2) if x1 < x2 => Ordering::Less,
                        ListNode::Val(x2) if x1 > x2 => Ordering::Greater,
                        ListNode::List(sublist2) => {
                            let sublist1 = list![ListNode::Val(*x1)];
                            sublist1.cmp(&sublist2)
                        },
                        _ => Ordering::Equal
                    }
                }
            };
            if cmp_res != Ordering::Equal {
                return cmp_res
            }
            i += 1;
        }
        if self.len() == other.len() {
            Ordering::Equal
        } else {
            if i == self.len() { Ordering::Less } else { Ordering::Greater }
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Idx> Index<Idx> for List
where Idx: SliceIndex<[ListNode], Output = ListNode> {
    type Output = ListNode;
    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output {
        self.data.index(index)
    }
}

impl FromStr for List {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        let mut i = 0;
        let mut list = List::new();
        let mut stack = Vec::new();
        while i < s.len() - 1 {
            match s[i] {
                b'0'..=b'9' => {
                    let l = s.iter().skip(i).position(|x| !x.is_ascii_digit()).ok_or(Error::WrongListFormat)?;
                    let node = ListNode::Val(str::from_utf8(&s[i..i+l]).unwrap().parse()?);
                    list.push(node);
                    i += l;
                },
                b'[' => {
                    stack.push(list);
                    list = List::new();
                    i += 1;
                },
                b']' => {
                    if let Some(mut last) = stack.pop() {
                        last.push(ListNode::List(list));
                        list = last;
                    }
                    i += 1;
                },
                b',' => {
                    i += 1;
                }
                _ => return Err(Error::WrongListFormat)
            };
        }
        return Ok(list);
    }
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    Ok(
        lines.chunks(3).enumerate()
        .map(|(i, chunk)| (i, chunk[0].as_ref().parse::<List>().unwrap(), chunk[1].as_ref().parse::<List>().unwrap()))
        .filter(|(_, list1, list2)| list1 < list2)
        .map(|(i, _, _)| i as u32 + 1).sum()
    )
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32, Error> {
    let mut lists = lines.iter().map(|l| l.as_ref()).filter(|l| !l.is_empty()).map(|l| l.parse::<List>().unwrap()).collect::<Vec<_>>();
    let marker1 = list![ListNode::List(list![ListNode::Val(2)])];
    let marker2 = list![ListNode::List(list![ListNode::Val(6)])];
    lists.push(marker1.clone());
    lists.push(marker2.clone());
    lists.sort();
    let p1 = lists.iter().position(|m| m == &marker1).unwrap() + 1;
    let p2 = lists.iter().position(|m| m == &marker2).unwrap() + 1;
    Ok(p1 as u32 * p2 as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &'static str = "[1,1,3,1,1]
    [1,1,5,1,1]

    [[1],[2,3,4]]
    [[1],4]

    [9]
    [[8,7,6]]

    [[4,4],4,4]
    [[4,4],4,4,4]

    [7,7,7,7]
    [7,7,7]

    []
    [3]

    [[[]]]
    [[]]

    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test1() {
        let lines = DATA.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok(13), task1(&lines));
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok(140), task2(&lines));
    }
}
