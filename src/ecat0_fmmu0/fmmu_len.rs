#[doc = "Reader of register FMMU_LEN"]
pub type R = crate::R<u16, super::FMMU_LEN>;
#[doc = "Reader of field `OFFSET`"]
pub type OFFSET_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Offset from the first logical FMMU Byte to the last FMMU Byte + 1"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
