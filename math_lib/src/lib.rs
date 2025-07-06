pub fn add(n1: i64, n2: i64) -> i64 {
    return n1 + n2;
}

pub fn sub(n1: i64, n2: i64) -> i64 {
    return n1 - n2;
}

pub fn mul(n1: i64, n2: i64) -> i64 {
    return n1 * n2;
}

pub fn divide(n1: i64, n2: i64) -> Result<i64, String> {
    match n2 {
        0 => Err(String::from("Division by zero is not ok")),
        _ => Ok(n1 / n2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
