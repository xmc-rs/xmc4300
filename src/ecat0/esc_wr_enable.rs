#[doc = "Register `ESC_WR_ENABLE` reader"]
pub struct R(crate::R<ESC_WR_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_WR_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_WR_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_WR_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ESC_WR_PROT` reader - Write protection enabled"]
pub struct ESC_WR_PROT_R(crate::FieldReader<bool, bool>);
impl ESC_WR_PROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESC_WR_PROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESC_WR_PROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Write protection enabled"]
    #[inline(always)]
    pub fn esc_wr_prot(&self) -> ESC_WR_PROT_R {
        ESC_WR_PROT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "ESC Write Enable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_wr_enable](index.html) module"]
pub struct ESC_WR_ENABLE_SPEC;
impl crate::RegisterSpec for ESC_WR_ENABLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [esc_wr_enable::R](R) reader structure"]
impl crate::Readable for ESC_WR_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESC_WR_ENABLE to value 0"]
impl crate::Resettable for ESC_WR_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
