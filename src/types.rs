use crate::*;

/// An alias referencing the RuntimeOrigin config type.
pub type RuntimeLocalOrigin<T> = <T as Config>::RuntimeOrigin;

/// An alias referencing the native currency's balance of an account.
pub type BalanceOf<T> =
    <<T as Config>::Currency as fungible::Inspect<<T as frame_system::Config>::AccountId>>::Balance;

/// A fancy struct that can be used to store information together in storage. It's storing some bytes chosen and the Block number when it's registered SCALE compact encoded. This struct will be linked to a AccountId via MyStorageMap
#[derive(TypeInfo, Encode, Decode, MaxEncodedLen, PartialEq, RuntimeDebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct AccountInfo<T: Config> {
    // Awesome bytes
    pub my_bytes: [u8; 10],
    // The block where the account registered the phrase
    #[codec(compact)]
    pub block_joined: BlockNumberFor<T>,
}
