#[doc = "Reader of register AHB_STATUS"]
pub type R = crate::R<u32, super::AHB_STATUS>;
#[doc = "Reader of field `AHBMS`"]
pub type AHBMS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - AHB Master Status"]
    #[inline(always)]
    pub fn ahbms(&self) -> AHBMS_R {
        AHBMS_R::new((self.bits & 0x01) != 0)
    }
}
