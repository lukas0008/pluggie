use abi_stable::StableAbi;

pub trait Event: StableAbi + Sized + Send + Sync + 'static {
    const NAME: [u8; 32];
}
