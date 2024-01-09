mod common;

#[cfg(test)]
mod lifetime_test {
    use crate::common;
    #[test]
    #[ignore]
    fn test_longest() {
        common::setup();
        assert_eq!(2+2, 4);
    }

    #[should_panic]
    fn should_panic() {
        panic!("hhh");
    }
}



