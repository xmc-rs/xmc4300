#[doc = "Register `CPUID` reader"]
pub struct R(crate::R<CPUID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Revision number\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVISION_A {
    #[doc = "1: Patch 1"]
    VALUE1 = 1,
}
impl From<REVISION_A> for u8 {
    #[inline(always)]
    fn from(variant: REVISION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Revision` reader - Revision number"]
pub struct REVISION_R(crate::FieldReader<u8, REVISION_A>);
impl REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REVISION_A> {
        match self.bits {
            1 => Some(REVISION_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == REVISION_A::VALUE1
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, REVISION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Part number of the processor\n\nValue on reset: 3108"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum PARTNO_A {
    #[doc = "3108: Cortex-M4"]
    VALUE1 = 3108,
}
impl From<PARTNO_A> for u16 {
    #[inline(always)]
    fn from(variant: PARTNO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PartNo` reader - Part number of the processor"]
pub struct PARTNO_R(crate::FieldReader<u16, PARTNO_A>);
impl PARTNO_R {
    pub(crate) fn new(bits: u16) -> Self {
        PARTNO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARTNO_A> {
        match self.bits {
            3108 => Some(PARTNO_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PARTNO_A::VALUE1
    }
}
impl core::ops::Deref for PARTNO_R {
    type Target = crate::FieldReader<u16, PARTNO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Constant` reader - Reads as 0xF"]
pub struct CONSTANT_R(crate::FieldReader<u8, u8>);
impl CONSTANT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONSTANT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONSTANT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Variant number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VARIANT_A {
    #[doc = "0: Revision 0"]
    VALUE1 = 0,
}
impl From<VARIANT_A> for u8 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Variant` reader - Variant number"]
pub struct VARIANT_R(crate::FieldReader<u8, VARIANT_A>);
impl VARIANT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VARIANT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VARIANT_A> {
        match self.bits {
            0 => Some(VARIANT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VARIANT_A::VALUE1
    }
}
impl core::ops::Deref for VARIANT_R {
    type Target = crate::FieldReader<u8, VARIANT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Implementer code\n\nValue on reset: 65"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IMPLEMENTER_A {
    #[doc = "65: ARM"]
    VALUE1 = 65,
}
impl From<IMPLEMENTER_A> for u8 {
    #[inline(always)]
    fn from(variant: IMPLEMENTER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Implementer` reader - Implementer code"]
pub struct IMPLEMENTER_R(crate::FieldReader<u8, IMPLEMENTER_A>);
impl IMPLEMENTER_R {
    pub(crate) fn new(bits: u8) -> Self {
        IMPLEMENTER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IMPLEMENTER_A> {
        match self.bits {
            65 => Some(IMPLEMENTER_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == IMPLEMENTER_A::VALUE1
    }
}
impl core::ops::Deref for IMPLEMENTER_R {
    type Target = crate::FieldReader<u8, IMPLEMENTER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Revision number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Part number of the processor"]
    #[inline(always)]
    pub fn part_no(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Reads as 0xF"]
    #[inline(always)]
    pub fn constant(&self) -> CONSTANT_R {
        CONSTANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Variant number"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer code"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CPUID Base Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](index.html) module"]
pub struct CPUID_SPEC;
impl crate::RegisterSpec for CPUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuid::R](R) reader structure"]
impl crate::Readable for CPUID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPUID to value 0x410f_c241"]
impl crate::Resettable for CPUID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x410f_c241
    }
}
