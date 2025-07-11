pub trait Exposable: Clone + Send + Sync + 'static {
    const NAME: &'static str;
    const NAME_HASH: [u8; 32] = sha2_const::Sha256::new()
        .update(Self::NAME.as_bytes())
        .finalize();
}
