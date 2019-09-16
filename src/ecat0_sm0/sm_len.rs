#[doc = "Reader of register SM_LEN"]
pub type R = crate::R<u16, super::SM_LEN>;
#[doc = "Reader of field `NO_BYTES`"]
pub type NO_BYTES_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of bytes assigned to SyncManager"]
    #[inline(always)]
    pub fn no_bytes(&self) -> NO_BYTES_R {
        NO_BYTES_R::new((self.bits & 0xffff) as u16)
    }
}
