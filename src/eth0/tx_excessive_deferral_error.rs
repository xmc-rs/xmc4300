#[doc = "Reader of register TX_EXCESSIVE_DEFERRAL_ERROR"]
pub type R = crate::R<u32, super::TX_EXCESSIVE_DEFERRAL_ERROR>;
#[doc = "Reader of field `TXEXSDEF`"]
pub type TXEXSDEF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of excessive deferral error, that is, frames deferred for more than two max-sized frame times."]
    #[inline(always)]
    pub fn txexsdef(&self) -> TXEXSDEF_R {
        TXEXSDEF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
