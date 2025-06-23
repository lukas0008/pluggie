pub trait Event: Sized {
    const NAME: [u8; 32];
}
