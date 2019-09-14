#[doc = "Reader of register RX_LENGTH_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RX_LENGTH_ERROR_FRAMES>;
#[doc = "Reader of field `RXLENERR`"]
pub type RXLENERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with length error (Length type field not equal to frame size) for all frames with valid length field."]
    #[inline(always)]
    pub fn rxlenerr(&self) -> RXLENERR_R {
        RXLENERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
