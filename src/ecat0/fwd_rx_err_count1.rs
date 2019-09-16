#[doc = "Reader of register FWD_RX_ERR_COUNT1"]
pub type R = crate::R<u8, super::FWD_RX_ERR_COUNT1>;
#[doc = "Reader of field `FORW_ERROR`"]
pub type FORW_ERROR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Forwarded error counter of Port y"]
    #[inline(always)]
    pub fn forw_error(&self) -> FORW_ERROR_R {
        FORW_ERROR_R::new((self.bits & 0xff) as u8)
    }
}
