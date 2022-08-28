pub fn analyse() -> bool{
    println!("Analyzing...");
    true
}

fn main(){
    analyse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = analyse();
        assert_eq!(result, true);
    }
}
