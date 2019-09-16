#[doc = "Reader of register OUTDR"]
pub type R = crate::R<u32, super::OUTDR>;
#[doc = "Reader of field `DSR`"]
pub type DSR_R = crate::R<u16, u16>;
#[doc = "Reader of field `RCI`"]
pub type RCI_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Data from Shift Register"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Receive Control Information from Shift Register"]
    #[inline(always)]
    pub fn rci(&self) -> RCI_R {
        RCI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
