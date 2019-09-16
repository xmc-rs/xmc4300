#[doc = "Reader of register RX_ERR_COUNT1"]
pub type R = crate::R<u16, super::RX_ERR_COUNT1>;
#[doc = "Reader of field `INVALID_FRAME`"]
pub type INVALID_FRAME_R = crate::R<u8, u8>;
#[doc = "Reader of field `RX_ERROR`"]
pub type RX_ERROR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Invalid frame counter of Port y"]
    #[inline(always)]
    pub fn invalid_frame(&self) -> INVALID_FRAME_R {
        INVALID_FRAME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Error counter of Port y"]
    #[inline(always)]
    pub fn rx_error(&self) -> RX_ERROR_R {
        RX_ERROR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
