use crate::tests::*;
/// A function useful to advance a block :) If your pallet implements on_initialize, this is a good place to run it.
pub fn next_block() {
    System::set_block_number(System::block_number() + 1);
    System::on_initialize(System::block_number());
    // Example_template::on_initialize(System::block_number());
    #[cfg(feature = "try-runtime")]
    Example_template::try_state(System::block_number()).expect("State corrupted");
}

/// A function useful to run til the desired block. If your pallet implements on_finalize, this is a good place to run it.
pub fn run_to_block(n: BlockNumberFor<Test>) {
    while System::block_number() < n {
        if System::block_number() > 1 {
            System::on_finalize(System::block_number());
            // Example_template::on_finalize(System::block_number());
        }
        next_block();
    }
}

/// Typically in tests, AccountId is just a number. With this macro you can give them a friendly ident
/// # Example
/// ```rust
/// give_accounts_names!(alice, bob, charlie);
/// assert_eq!(alice, 1);
/// assert_eq!(bob, 2);
/// assert_eq!(charlie, 3);
/// ```
#[macro_export]
macro_rules! give_accounts_names{
    ($($account: ident),+) => {
        let mut counter: <Test as crate::frame_system::Config>::AccountId = 0;
        $(
            counter +=1;
            let $account = counter;
        )+
    };
}

/// This macro's useful to test that an unsigned origin cannot call an extrinsic requiring a signed origin
#[macro_export]
macro_rules! not_signed_call_triggers_bad_origin_test{
    ($test_name: ident, $extrinsic: ident, $($arg: expr),*) =>{
        #[test]
        fn $test_name(){
            new_test_ext().execute_with(||{
                assert_noop!(Example_template::$extrinsic(RuntimeOrigin::none(), $($arg),*), DispatchError::BadOrigin);
            })
        }
    };
}
