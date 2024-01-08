mod common;
#[cfg(test)]
mod lifetime_test {
    use crate::common;
    #[test]
    #[ignore]
    #[should_panic]
    fn test_longest() {
        common::setup();
        assert_eq!(2+2, 4);
        panic!("hhh");
    }
}



