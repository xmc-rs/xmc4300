#[doc = "Reader of register RX_FIFO_OVERFLOW_FRAMES"]
pub type R = crate::R<u32, super::RX_FIFO_OVERFLOW_FRAMES>;
#[doc = "Reader of field `RXFIFOOVFL`"]
pub type RXFIFOOVFL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received frames missed because of FIFO overflow."]
    #[inline(always)]
    pub fn rxfifoovfl(&self) -> RXFIFOOVFL_R {
        RXFIFOOVFL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
