use crate::pallet::*;

impl<T: Config> Pallet<T> {
    #[cfg(feature = "try-runtime")]
    pub(crate) fn do_try_state() -> Result<(), sp_runtime::TryRuntimeError> {
        // TODO: Write here your try_state logic if needed
        Ok(())
    }
}
