#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CPUID_SPEC>;
#[doc = "Field `Revision` reader - Revision number"]
pub type REVISION_R = crate::FieldReader<REVISION_A>;
#[doc = "Revision number\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for REVISION_A {
    type Ux = u8;
}
impl REVISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REVISION_A> {
        match self.bits {
            1 => Some(REVISION_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Patch 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REVISION_A::VALUE1
    }
}
#[doc = "Field `PartNo` reader - Part number of the processor"]
pub type PART_NO_R = crate::FieldReader<PART_NO_A>;
#[doc = "Part number of the processor\n\nValue on reset: 3108"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PART_NO_A {
    #[doc = "3108: Cortex-M4"]
    VALUE1 = 3108,
}
impl From<PART_NO_A> for u16 {
    #[inline(always)]
    fn from(variant: PART_NO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PART_NO_A {
    type Ux = u16;
}
impl PART_NO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PART_NO_A> {
        match self.bits {
            3108 => Some(PART_NO_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Cortex-M4"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PART_NO_A::VALUE1
    }
}
#[doc = "Field `Constant` reader - Reads as 0xF"]
pub type CONSTANT_R = crate::FieldReader;
#[doc = "Field `Variant` reader - Variant number"]
pub type VARIANT_R = crate::FieldReader<VARIANT_A>;
#[doc = "Variant number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for VARIANT_A {
    type Ux = u8;
}
impl VARIANT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VARIANT_A> {
        match self.bits {
            0 => Some(VARIANT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Revision 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VARIANT_A::VALUE1
    }
}
#[doc = "Field `Implementer` reader - Implementer code"]
pub type IMPLEMENTER_R = crate::FieldReader<IMPLEMENTER_A>;
#[doc = "Implementer code\n\nValue on reset: 65"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for IMPLEMENTER_A {
    type Ux = u8;
}
impl IMPLEMENTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IMPLEMENTER_A> {
        match self.bits {
            65 => Some(IMPLEMENTER_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "ARM"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IMPLEMENTER_A::VALUE1
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
    pub fn part_no(&self) -> PART_NO_R {
        PART_NO_R::new(((self.bits >> 4) & 0x0fff) as u16)
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
#[doc = "CPUID Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUID_SPEC;
impl crate::RegisterSpec for CPUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CPUID_SPEC {}
#[doc = "`reset()` method sets CPUID to value 0x410f_c241"]
impl crate::Resettable for CPUID_SPEC {
    const RESET_VALUE: u32 = 0x410f_c241;
}
