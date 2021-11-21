#[doc = "Register `TIM` reader"]
pub struct R(crate::R<TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM` reader - Timer Value"]
pub struct TIM_R(crate::FieldReader<u32, u32>);
impl TIM_R {
    pub(crate) fn new(bits: u32) -> Self {
        TIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "WDT Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim](index.html) module"]
pub struct TIM_SPEC;
impl crate::RegisterSpec for TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim::R](R) reader structure"]
impl crate::Readable for TIM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIM to value 0"]
impl crate::Resettable for TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
