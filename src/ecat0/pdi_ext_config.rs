#[doc = "Register `PDI_EXT_CONFIG` reader"]
pub type R = crate::R<PDI_EXT_CONFIG_SPEC>;
#[doc = "Read Prefetch Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for R_PREF_A {
    type Ux = u8;
}
impl crate::IsEnum for R_PREF_A {}
#[doc = "Field `R_Pref` reader - Read Prefetch Size"]
pub type R_PREF_R = crate::FieldReader<R_PREF_A>;
impl R_PREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R_PREF_A> {
        match self.bits {
            0 => Some(R_PREF_A::VALUE1),
            1 => Some(R_PREF_A::VALUE2),
            2 => Some(R_PREF_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "4 cycles"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == R_PREF_A::VALUE1
    }
    #[doc = "1 cycle (typical)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == R_PREF_A::VALUE2
    }
    #[doc = "2 cycles"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == R_PREF_A::VALUE3
    }
}
#[doc = "On-chip Sub Type for AXI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SUB_TYPE_A {
    type Ux = u8;
}
impl crate::IsEnum for SUB_TYPE_A {}
#[doc = "Field `SUB_TYPE` reader - On-chip Sub Type for AXI"]
pub type SUB_TYPE_R = crate::FieldReader<SUB_TYPE_A>;
impl SUB_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SUB_TYPE_A> {
        match self.bits {
            0 => Some(SUB_TYPE_A::VALUE1),
            1 => Some(SUB_TYPE_A::VALUE2),
            2 => Some(SUB_TYPE_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "AXI3"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUB_TYPE_A::VALUE1
    }
    #[doc = "AXI4"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUB_TYPE_A::VALUE2
    }
    #[doc = "AXI4 Lite"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUB_TYPE_A::VALUE3
    }
}
impl R {
    #[doc = "Bits 0:1 - Read Prefetch Size"]
    #[inline(always)]
    pub fn r_pref(&self) -> R_PREF_R {
        R_PREF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - On-chip Sub Type for AXI"]
    #[inline(always)]
    pub fn sub_type(&self) -> SUB_TYPE_R {
        SUB_TYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
}
#[doc = "PDI Synchronous Microcontroller extended Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pdi_ext_config::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDI_EXT_CONFIG_SPEC;
impl crate::RegisterSpec for PDI_EXT_CONFIG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdi_ext_config::R`](R) reader structure"]
impl crate::Readable for PDI_EXT_CONFIG_SPEC {}
#[doc = "`reset()` method sets PDI_EXT_CONFIG to value 0"]
impl crate::Resettable for PDI_EXT_CONFIG_SPEC {
    const RESET_VALUE: u16 = 0;
}
