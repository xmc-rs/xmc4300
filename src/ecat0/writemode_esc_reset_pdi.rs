#[doc = "Register `ESC_RESET_PDI` reader"]
pub type R = crate::R<WritemodeEscResetPdiSpec>;
#[doc = "Field `RESET_CMD` reader - Reset commands issued by XMC4700"]
pub type ResetCmdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reset commands issued by XMC4700"]
    #[inline(always)]
    pub fn reset_cmd(&self) -> ResetCmdR {
        ResetCmdR::new(self.bits)
    }
}
#[doc = "ESC Reset PDI \\[WRITE Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writemode_esc_reset_pdi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritemodeEscResetPdiSpec;
impl crate::RegisterSpec for WritemodeEscResetPdiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`writemode_esc_reset_pdi::R`](R) reader structure"]
impl crate::Readable for WritemodeEscResetPdiSpec {}
#[doc = "`reset()` method sets ESC_RESET_PDI to value 0"]
impl crate::Resettable for WritemodeEscResetPdiSpec {
    const RESET_VALUE: u8 = 0;
}
