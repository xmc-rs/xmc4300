#[doc = "Register `MIDR` reader"]
pub struct R(crate::R<MIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODR` reader - Module Revision"]
pub struct MODR_R(crate::FieldReader<u8, u8>);
impl MODR_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODT` reader - Module Type"]
pub struct MODT_R(crate::FieldReader<u8, u8>);
impl MODT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODN` reader - Module Number"]
pub struct MODN_R(crate::FieldReader<u16, u16>);
impl MODN_R {
    pub(crate) fn new(bits: u16) -> Self {
        MODN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn modr(&self) -> MODR_R {
        MODR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn modt(&self) -> MODT_R {
        MODT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn modn(&self) -> MODN_R {
        MODN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midr](index.html) module"]
pub struct MIDR_SPEC;
impl crate::RegisterSpec for MIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midr::R](R) reader structure"]
impl crate::Readable for MIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIDR to value 0x00a7_c000"]
impl crate::Resettable for MIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00a7_c000
    }
}
