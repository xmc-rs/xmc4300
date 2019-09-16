#[doc = "Reader of register WDTSTS"]
pub type R = crate::R<u32, super::WDTSTS>;
#[doc = "Reader of field `ALMS`"]
pub type ALMS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Pre-warning Alarm"]
    #[inline(always)]
    pub fn alms(&self) -> ALMS_R {
        ALMS_R::new((self.bits & 0x01) != 0)
    }
}
