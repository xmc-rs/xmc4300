#[doc = "Register `RESPONSE0` reader"]
pub struct R(crate::R<RESPONSE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESPONSE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESPONSE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESPONSE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPONSE1` reader - Response1"]
pub struct RESPONSE1_R(crate::FieldReader<u16, u16>);
impl RESPONSE1_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESPONSE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESPONSE0` reader - Response0"]
pub struct RESPONSE0_R(crate::FieldReader<u16, u16>);
impl RESPONSE0_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESPONSE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31 - Response1"]
    #[inline(always)]
    pub fn response1(&self) -> RESPONSE1_R {
        RESPONSE1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Response0"]
    #[inline(always)]
    pub fn response0(&self) -> RESPONSE0_R {
        RESPONSE0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Response 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [response0](index.html) module"]
pub struct RESPONSE0_SPEC;
impl crate::RegisterSpec for RESPONSE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [response0::R](R) reader structure"]
impl crate::Readable for RESPONSE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESPONSE0 to value 0"]
impl crate::Resettable for RESPONSE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
