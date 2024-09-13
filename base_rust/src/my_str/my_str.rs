fn my_print() {
    println!("hello string")
}

pub fn new_str() -> String {
    my_print();
    let s = String::from("hello");
    s
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_str() {
        assert_eq!(new_str(), "hello".to_string());
        let mut s = String::new();
        s = "hello".to_string();
        assert_eq!(new_str(), s);

        // assert_eq!(
        //     Err(CreationError::Negative),
        //     PositiveNonzeroInteger::new(-10)
        // );
        // assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}
