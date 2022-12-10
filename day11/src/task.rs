
pub type Result<T> = std::result::Result<T, &'static str>;

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    Ok(lines.len() as i32)
}

pub fn task2(lines: &[String]) -> Result<i32> {
    Ok(lines.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &'static [&'static str] = &[
        "1", "2", "3"
    ];
    #[test]
    fn test1() {
        assert_eq!(Ok(3), task1(&DATA));
    }
}
