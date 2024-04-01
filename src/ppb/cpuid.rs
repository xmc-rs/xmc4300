#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CpuidSpec>;
#[doc = "Revision number\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revision {
    #[doc = "1: Patch 1"]
    Value1 = 1,
}
impl From<Revision> for u8 {
    #[inline(always)]
    fn from(variant: Revision) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revision {
    type Ux = u8;
}
impl crate::IsEnum for Revision {}
#[doc = "Field `Revision` reader - Revision number"]
pub type RevisionR = crate::FieldReader<Revision>;
impl RevisionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revision> {
        match self.bits {
            1 => Some(Revision::Value1),
            _ => None,
        }
    }
    #[doc = "Patch 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Revision::Value1
    }
}
#[doc = "Part number of the processor\n\nValue on reset: 3108"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PartNo {
    #[doc = "3108: Cortex-M4"]
    Value1 = 3108,
}
impl From<PartNo> for u16 {
    #[inline(always)]
    fn from(variant: PartNo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PartNo {
    type Ux = u16;
}
impl crate::IsEnum for PartNo {}
#[doc = "Field `PartNo` reader - Part number of the processor"]
pub type PartNoR = crate::FieldReader<PartNo>;
impl PartNoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PartNo> {
        match self.bits {
            3108 => Some(PartNo::Value1),
            _ => None,
        }
    }
    #[doc = "Cortex-M4"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PartNo::Value1
    }
}
#[doc = "Field `Constant` reader - Reads as 0xF"]
pub type ConstantR = crate::FieldReader;
#[doc = "Variant number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Variant {
    #[doc = "0: Revision 0"]
    Value1 = 0,
}
impl From<Variant> for u8 {
    #[inline(always)]
    fn from(variant: Variant) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Variant {
    type Ux = u8;
}
impl crate::IsEnum for Variant {}
#[doc = "Field `Variant` reader - Variant number"]
pub type VariantR = crate::FieldReader<Variant>;
impl VariantR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Variant> {
        match self.bits {
            0 => Some(Variant::Value1),
            _ => None,
        }
    }
    #[doc = "Revision 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Variant::Value1
    }
}
#[doc = "Implementer code\n\nValue on reset: 65"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Implementer {
    #[doc = "65: ARM"]
    Value1 = 65,
}
impl From<Implementer> for u8 {
    #[inline(always)]
    fn from(variant: Implementer) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Implementer {
    type Ux = u8;
}
impl crate::IsEnum for Implementer {}
#[doc = "Field `Implementer` reader - Implementer code"]
pub type ImplementerR = crate::FieldReader<Implementer>;
impl ImplementerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Implementer> {
        match self.bits {
            65 => Some(Implementer::Value1),
            _ => None,
        }
    }
    #[doc = "ARM"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Implementer::Value1
    }
}
impl R {
    #[doc = "Bits 0:3 - Revision number"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Part number of the processor"]
    #[inline(always)]
    pub fn part_no(&self) -> PartNoR {
        PartNoR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Reads as 0xF"]
    #[inline(always)]
    pub fn constant(&self) -> ConstantR {
        ConstantR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Variant number"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer code"]
    #[inline(always)]
    pub fn implementer(&self) -> ImplementerR {
        ImplementerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CPUID Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuidSpec;
impl crate::RegisterSpec for CpuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CpuidSpec {}
#[doc = "`reset()` method sets CPUID to value 0x410f_c241"]
impl crate::Resettable for CpuidSpec {
    const RESET_VALUE: u32 = 0x410f_c241;
}
