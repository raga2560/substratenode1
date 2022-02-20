#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mocks::*;



#[test]
fn doc_submit_test() {
    new_test_ext().execute_with(|| {
        let dochash: Vec<u8> = b"x4ydu6788".to_vec();
        let manager = 11;
        let staff = 10;
        assert_ok!(TestingPallet::document_submit_sel31(Origin::signed(staff),dochash.clone()  ) );

    });
}
