#[doc = "Register `ESC_RESET_ECAT` reader"]
pub struct R(crate::R<WRITEMODE_ESC_RESET_ECAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITEMODE_ESC_RESET_ECAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITEMODE_ESC_RESET_ECAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITEMODE_ESC_RESET_ECAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESET_CMD` reader - Reset commands issued by EtherCAt Master"]
pub struct RESET_CMD_R(crate::FieldReader<u8, u8>);
impl RESET_CMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESET_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_CMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Reset commands issued by EtherCAt Master"]
    #[inline(always)]
    pub fn reset_cmd(&self) -> RESET_CMD_R {
        RESET_CMD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "ESC Reset ECAT \\[WRITE Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writemode_esc_reset_ecat](index.html) module"]
pub struct WRITEMODE_ESC_RESET_ECAT_SPEC;
impl crate::RegisterSpec for WRITEMODE_ESC_RESET_ECAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [writemode_esc_reset_ecat::R](R) reader structure"]
impl crate::Readable for WRITEMODE_ESC_RESET_ECAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESC_RESET_ECAT to value 0"]
impl crate::Resettable for WRITEMODE_ESC_RESET_ECAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
