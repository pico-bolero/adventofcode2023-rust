pub fn process(_input: &str) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("50");
        assert_eq!(1, result);
    }
}
