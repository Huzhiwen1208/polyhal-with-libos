use crate::mem::Barrier;

impl Barrier {
    #[inline]
    pub fn complete_sync() {}

    #[inline]
    pub fn ordering_sync() {}
}
