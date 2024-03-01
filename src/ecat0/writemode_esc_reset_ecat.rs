#[doc = "Register `ESC_RESET_ECAT` reader"]
pub type R = crate::R<WritemodeEscResetEcatSpec>;
#[doc = "Field `RESET_CMD` reader - Reset commands issued by EtherCAt Master"]
pub type ResetCmdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reset commands issued by EtherCAt Master"]
    #[inline(always)]
    pub fn reset_cmd(&self) -> ResetCmdR {
        ResetCmdR::new(self.bits)
    }
}
#[doc = "ESC Reset ECAT \\[WRITE Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writemode_esc_reset_ecat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritemodeEscResetEcatSpec;
impl crate::RegisterSpec for WritemodeEscResetEcatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`writemode_esc_reset_ecat::R`](R) reader structure"]
impl crate::Readable for WritemodeEscResetEcatSpec {}
#[doc = "`reset()` method sets ESC_RESET_ECAT to value 0"]
impl crate::Resettable for WritemodeEscResetEcatSpec {
    const RESET_VALUE: u8 = 0;
}
