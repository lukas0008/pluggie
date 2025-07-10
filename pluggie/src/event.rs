use abi_stable::StableAbi;

pub trait Event: StableAbi + Sized + Send + Sync + 'static {
    const NAME: &'static str;
    const NAME_HASH: [u8; 32] = sha2_const::Sha256::new()
        .update(Self::NAME.as_bytes())
        .finalize();
}
