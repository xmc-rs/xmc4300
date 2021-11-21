#[doc = "Register `VERSION` reader"]
pub struct R(crate::R<VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SNPSVER` reader - Synopsys-defined Version (3.7)"]
pub struct SNPSVER_R(crate::FieldReader<u8, u8>);
impl SNPSVER_R {
    pub(crate) fn new(bits: u8) -> Self {
        SNPSVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNPSVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USERVER` reader - User-defined Version (Configured with the coreConsultant)"]
pub struct USERVER_R(crate::FieldReader<u8, u8>);
impl USERVER_R {
    pub(crate) fn new(bits: u8) -> Self {
        USERVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USERVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Synopsys-defined Version (3.7)"]
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - User-defined Version (Configured with the coreConsultant)"]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](index.html) module"]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [version::R](R) reader structure"]
impl crate::Readable for VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VERSION to value 0x1037"]
impl crate::Resettable for VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1037
    }
}
