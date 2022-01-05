use super::*;

#[test]
fn error_works(){
    new_test_ext().execute_with(|| {
        assert_err!(
            TestingPallet::add_value(Origin::signed(1), 51),
            "value must be <= maximum add amount constant"
        );
    })
}

#[test]
fn test_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(
            TestingPallet::add_value(Origin::signed(1), 10)
        );
    })
}

#[test]
fn test_should_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(
            TestingPallet::add_value(Origin::signed(1), 100)
        );
    })
}
