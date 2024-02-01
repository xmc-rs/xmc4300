#[doc = "Register `ESC_RESET_PDI` reader"]
pub type R = crate::R<WRITEMODE_ESC_RESET_PDI_SPEC>;
#[doc = "Field `RESET_CMD` reader - Reset commands issued by XMC4700"]
pub type RESET_CMD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reset commands issued by XMC4700"]
    #[inline(always)]
    pub fn reset_cmd(&self) -> RESET_CMD_R {
        RESET_CMD_R::new(self.bits)
    }
}
#[doc = "ESC Reset PDI \\[WRITE Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writemode_esc_reset_pdi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRITEMODE_ESC_RESET_PDI_SPEC;
impl crate::RegisterSpec for WRITEMODE_ESC_RESET_PDI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`writemode_esc_reset_pdi::R`](R) reader structure"]
impl crate::Readable for WRITEMODE_ESC_RESET_PDI_SPEC {}
#[doc = "`reset()` method sets ESC_RESET_PDI to value 0"]
impl crate::Resettable for WRITEMODE_ESC_RESET_PDI_SPEC {
    const RESET_VALUE: u8 = 0;
}
