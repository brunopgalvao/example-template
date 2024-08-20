use crate::{self as pallet_example_template, *};

pub use frame::{
    deps::{frame_support::runtime, sp_io::TestExternalities},
    runtime::prelude::*,
    testing_prelude::*,
    traits::fungible::Mutate,
};

type Block = MockBlock<Test>;

type Balance = u64;

#[runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type Balances = pallet_balances;

    #[runtime::pallet_index(2)]
    pub type Example_template = pallet_example_template;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountData = pallet_balances::AccountData<Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type AccountStore = System;
}
#[derive_impl(pallet_example_template::config_preludes::TestDefaultConfig)]
impl pallet_example_template::Config for Test {
    type Currency = Balances;
}

pub(crate) struct StateBuilder {
    balances: Vec<(<Test as frame_system::Config>::AccountId, Balance)>,
}

impl Default for StateBuilder {
    fn default() -> Self {
        Self { balances: vec![] }
    }
}

impl StateBuilder {
    /// This function helps to construct a initial state where some accounts have balances
    fn add_balance(
        mut self,
        who: <Test as frame_system::Config>::AccountId,
        amount: Balance,
    ) -> Self {
        self.balances.push((who, amount));
        self
    }

    pub(crate) fn build_and_execute(self, test: impl FnOnce() -> ()) {
        let mut ext: TestExternalities = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .into();

        // Test setup
        ext.execute_with(|| {
            System::set_block_number(1);
            self.balances.iter().for_each(|(who, amount)| {
                <Test as Config>::Currency::set_balance(who, *amount);
            })
        });

        ext.execute_with(test);

        // Test assertions
        ext.execute_with(|| {});
    }
}

pub fn new_test_ext() -> TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
