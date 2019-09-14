#[doc = "Reader of register PDI_ERR_COUNT"]
pub type R = crate::R<u8, super::PDI_ERR_COUNT>;
#[doc = "Reader of field `PDI_ERROR_COUNTER`"]
pub type PDI_ERROR_COUNTER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PDI Error counter"]
    #[inline(always)]
    pub fn pdi_error_counter(&self) -> PDI_ERROR_COUNTER_R {
        PDI_ERROR_COUNTER_R::new((self.bits & 0xff) as u8)
    }
}
