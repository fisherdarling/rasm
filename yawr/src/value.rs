#[cfg(test)]
pub mod tests {
    use crate::types::Value;
    use crate::binop;


    #[test]
    fn add() {
        let a = Value::I32(5);
        let b = Value::I32(10);

        // let res = binop!(a, b, |a, b| a + b).unwrap();

        // assert_eq!(res, Value::I32(15));
    }
}