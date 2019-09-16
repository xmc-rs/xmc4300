#[doc = "Reader of register RX_OUT_OF_RANGE_TYPE_FRAMES"]
pub type R = crate::R<u32, super::RX_OUT_OF_RANGE_TYPE_FRAMES>;
#[doc = "Reader of field `RXOUTOFRNG`"]
pub type RXOUTOFRNG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received frames with length field not equal to the valid frame size (greater than 1,500 but less than 1,536)."]
    #[inline(always)]
    pub fn rxoutofrng(&self) -> RXOUTOFRNG_R {
        RXOUTOFRNG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
