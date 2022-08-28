use lexer;

pub fn parse() {
    lexer::analyse();
    println!("Parsing...");
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
