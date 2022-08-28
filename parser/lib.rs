use lexer;

pub fn parse() -> bool{
    lexer::analyse();
    println!("Parsing...");
    true
}

fn main(){
    parse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = parse();
        assert_eq!(result, true);
    }
}
