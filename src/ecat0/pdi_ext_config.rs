#[doc = "Register `PDI_EXT_CONFIG` reader"]
pub struct R(crate::R<PDI_EXT_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDI_EXT_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDI_EXT_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDI_EXT_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Read Prefetch Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum R_PREF_A {
    #[doc = "0: 4 cycles"]
    VALUE1 = 0,
    #[doc = "1: 1 cycle (typical)"]
    VALUE2 = 1,
    #[doc = "2: 2 cycles"]
    VALUE3 = 2,
}
impl From<R_PREF_A> for u8 {
    #[inline(always)]
    fn from(variant: R_PREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `R_Pref` reader - Read Prefetch Size"]
pub struct R_PREF_R(crate::FieldReader<u8, R_PREF_A>);
impl R_PREF_R {
    pub(crate) fn new(bits: u8) -> Self {
        R_PREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<R_PREF_A> {
        match self.bits {
            0 => Some(R_PREF_A::VALUE1),
            1 => Some(R_PREF_A::VALUE2),
            2 => Some(R_PREF_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == R_PREF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == R_PREF_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == R_PREF_A::VALUE3
    }
}
impl core::ops::Deref for R_PREF_R {
    type Target = crate::FieldReader<u8, R_PREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "On-chip Sub Type for AXI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUB_TYPE_A {
    #[doc = "0: AXI3"]
    VALUE1 = 0,
    #[doc = "1: AXI4"]
    VALUE2 = 1,
    #[doc = "2: AXI4 Lite"]
    VALUE3 = 2,
}
impl From<SUB_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SUB_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SUB_TYPE` reader - On-chip Sub Type for AXI"]
pub struct SUB_TYPE_R(crate::FieldReader<u8, SUB_TYPE_A>);
impl SUB_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUB_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUB_TYPE_A> {
        match self.bits {
            0 => Some(SUB_TYPE_A::VALUE1),
            1 => Some(SUB_TYPE_A::VALUE2),
            2 => Some(SUB_TYPE_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SUB_TYPE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SUB_TYPE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SUB_TYPE_A::VALUE3
    }
}
impl core::ops::Deref for SUB_TYPE_R {
    type Target = crate::FieldReader<u8, SUB_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Read Prefetch Size"]
    #[inline(always)]
    pub fn r_pref(&self) -> R_PREF_R {
        R_PREF_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - On-chip Sub Type for AXI"]
    #[inline(always)]
    pub fn sub_type(&self) -> SUB_TYPE_R {
        SUB_TYPE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
#[doc = "PDI Synchronous Microcontroller extended Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdi_ext_config](index.html) module"]
pub struct PDI_EXT_CONFIG_SPEC;
impl crate::RegisterSpec for PDI_EXT_CONFIG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdi_ext_config::R](R) reader structure"]
impl crate::Readable for PDI_EXT_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDI_EXT_CONFIG to value 0"]
impl crate::Resettable for PDI_EXT_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
