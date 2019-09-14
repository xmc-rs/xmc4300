#[doc = "Reader of register RBUFD"]
pub type R = crate::R<u32, super::RBUFD>;
#[doc = "Reader of field `DSR`"]
pub type DSR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data from Shift Register"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new((self.bits & 0xffff) as u16)
    }
}
