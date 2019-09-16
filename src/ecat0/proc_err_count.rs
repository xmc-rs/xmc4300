#[doc = "Reader of register PROC_ERR_COUNT"]
pub type R = crate::R<u8, super::PROC_ERR_COUNT>;
#[doc = "Reader of field `UNIT_ERROR`"]
pub type UNIT_ERROR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - ECAT Processing Unit error counter"]
    #[inline(always)]
    pub fn unit_error(&self) -> UNIT_ERROR_R {
        UNIT_ERROR_R::new((self.bits & 0xff) as u8)
    }
}
