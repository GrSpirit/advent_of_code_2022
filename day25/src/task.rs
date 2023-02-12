#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Internal error")]
    #[allow(unused)]
    Internal,
    #[error("Parse error")]
    ParseError,
    #[error("Srialize error")]
    SerializeError,
}

fn decrypt(s: &str) -> Result<i64, Error> {
    let mut result = 0;
    let mut muiltiplier = 1;
    for b in s.bytes().rev() {
        let d = match b {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'-' => -1,
            b'=' => -2,
            _ => return Err(Error::ParseError)
        };
        result += d * muiltiplier;
        muiltiplier *= 5;
    }
    Ok(result)
}

fn encrypt(mut x: i64) -> Result<String, Error> {
    let mut s = Vec::new();
    if x == 0 {
        return Ok(String::from("0"));
    }
    while x != 0 {
        let rem = (x % 5) as u8;
        x /= 5;
        if rem < 3 {
            s.push(b'0' + rem);
        } else if rem == 3 {
            s.push(b'=');
            x += 1;
        } else {
            s.push(b'-');
            x += 1;
        }
    }
    s.reverse();
    String::from_utf8(s).map_err(|_| Error::SerializeError)
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<String, Error> {
    encrypt(lines.iter().map(|s| decrypt(s.as_ref()).unwrap()).sum())
}

pub fn task2<S: AsRef<str>>(_lines: &[S]) -> Result<String, Error> {
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
