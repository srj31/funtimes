fn main() {}

#[cfg(test)]
mod test {
    use macros::my_vec;
    #[test]
    fn empty_vec() {
        let v1: Vec<()> = my_vec![];
        assert!(v1.is_empty());
    }

    #[test]
    fn one_vec() {
        let v1: Vec<i32> = my_vec![41];
        assert_eq!(v1[0], 41);
    }

    #[test]
    fn two_vec() {
        let v1: Vec<i32> = my_vec![41, 42];
        assert_eq!(v1[0], 41);
        assert_eq!(v1[1], 42);
    }

    #[test]
    fn same_elements() {
        let v1: Vec<i32> = my_vec![41; 2];
        assert_eq!(v1[0], 41);
        assert_eq!(v1[1], 41);
    }
}
