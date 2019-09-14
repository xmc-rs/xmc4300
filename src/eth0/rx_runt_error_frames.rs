#[doc = "Reader of register RX_RUNT_ERROR_FRAMES"]
pub type R = crate::R<u32, super::RX_RUNT_ERROR_FRAMES>;
#[doc = "Reader of field `RXRUNTERR`"]
pub type RXRUNTERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with runt error(<64 bytes and CRC error)."]
    #[inline(always)]
    pub fn rxrunterr(&self) -> RXRUNTERR_R {
        RXRUNTERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
