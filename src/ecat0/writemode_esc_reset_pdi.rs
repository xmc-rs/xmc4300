#[doc = "Register `ESC_RESET_PDI` reader"]
pub struct R(crate::R<WRITEMODE_ESC_RESET_PDI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITEMODE_ESC_RESET_PDI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITEMODE_ESC_RESET_PDI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITEMODE_ESC_RESET_PDI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESET_CMD` reader - Reset commands issued by XMC4700"]
pub type RESET_CMD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reset commands issued by XMC4700"]
    #[inline(always)]
    pub fn reset_cmd(&self) -> RESET_CMD_R {
        RESET_CMD_R::new(self.bits)
    }
}
#[doc = "ESC Reset PDI \\[WRITE Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writemode_esc_reset_pdi](index.html) module"]
pub struct WRITEMODE_ESC_RESET_PDI_SPEC;
impl crate::RegisterSpec for WRITEMODE_ESC_RESET_PDI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [writemode_esc_reset_pdi::R](R) reader structure"]
impl crate::Readable for WRITEMODE_ESC_RESET_PDI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESC_RESET_PDI to value 0"]
impl crate::Resettable for WRITEMODE_ESC_RESET_PDI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
