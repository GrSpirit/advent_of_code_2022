use super::tools;
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal error")]
    #[allow(unused)]
    Internal,
    #[error("Parse error")]
    ParseError,
    #[error("Srialize error")]
    SerializeError(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn decrypt(s: &str) -> Result<i64> {
    let mapped: Option<Vec<i64>> = s.bytes().rev().map(|b| {
        match b {
            b'0' => Some(0),
            b'1' => Some(1),
            b'2' => Some(2),
            b'-' => Some(-1),
            b'=' => Some(-2),
            _ => None
        }
    }).collect();

    mapped.map(|m| 
        m.iter().fold((0, 1), |(res, mult), d| {
            (res + d * mult, mult * 5)
        }).0
    ).ok_or(Error::ParseError)
}

fn encrypt(x: i64) -> Result<String> {
    if x == 0 {
        return Ok(String::from("0"));
    }
    let mut result = tools::DivIterator::new(x, 5)
    .map(|rem|
        match rem {
            r if r < 3 => b'0' + r,
            3 => b'=',
            _ => b'-'
        }
    ).collect::<Vec<_>>();
    result.reverse();
    Ok(String::from_utf8(result)?)
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<String> {
    encrypt(lines.iter().map(|s| decrypt(s.as_ref()).unwrap()).sum())
}

pub fn task2<S: AsRef<str>>(_lines: &[S]) -> Result<String> {
    Ok(String::from("ho-ho-ho"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    const ENC_DATA: &[(i64, &str)] = &[(1, "1"), (2, "2"), (3, "1="), (4, "1-"), (5, "10"), (6, "11"), (7, "12"), (8, "2="),
    (9, "2-"), (10, "20"), (15, "1=0"), (20, "1-0"), (2022, "1=11-2"), (12345, "1-0---0"), (314159265, "1121-1110-1=0")];

    #[test]
    fn test_decrypt() {
        for (x, s) in ENC_DATA {
            assert_eq!(Ok(*x), decrypt(s));
        }
    }

    #[test]
    fn test_encrypt() {
        for (x, s) in ENC_DATA {
            assert_eq!(encrypt(*x), Ok(s.to_string()));
        }
    }

    #[test]
    fn test1() {
        let lines = DATA.split('\n').collect::<Vec<_>>();
        assert_eq!(Ok("2=-1=0".to_string()), task1(&lines));
    }

    #[test]
    fn test2() {
        let lines = DATA.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(Ok("ho-ho-ho".to_string()), task2(&lines));
    }
}
