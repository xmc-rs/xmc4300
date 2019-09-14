#[doc = "Reader of register PDI_EXT_CONFIG"]
pub type R = crate::R<u16, super::PDI_EXT_CONFIG>;
#[doc = "Read Prefetch Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_PREF_A {
    #[doc = "0: 4 cycles"]
    VALUE1,
    #[doc = "1: 1 cycle (typical)"]
    VALUE2,
    #[doc = "2: 2 cycles"]
    VALUE3,
}
impl From<R_PREF_A> for u8 {
    #[inline(always)]
    fn from(variant: R_PREF_A) -> Self {
        match variant {
            R_PREF_A::VALUE1 => 0,
            R_PREF_A::VALUE2 => 1,
            R_PREF_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `R_Pref`"]
pub type R_PREF_R = crate::R<u8, R_PREF_A>;
impl R_PREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, R_PREF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(R_PREF_A::VALUE1),
            1 => Val(R_PREF_A::VALUE2),
            2 => Val(R_PREF_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == R_PREF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == R_PREF_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == R_PREF_A::VALUE3
    }
}
#[doc = "On-chip Sub Type for AXI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUB_TYPE_A {
    #[doc = "0: AXI3"]
    VALUE1,
    #[doc = "1: AXI4"]
    VALUE2,
    #[doc = "2: AXI4 Lite"]
    VALUE3,
}
impl From<SUB_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SUB_TYPE_A) -> Self {
        match variant {
            SUB_TYPE_A::VALUE1 => 0,
            SUB_TYPE_A::VALUE2 => 1,
            SUB_TYPE_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `SUB_TYPE`"]
pub type SUB_TYPE_R = crate::R<u8, SUB_TYPE_A>;
impl SUB_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUB_TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUB_TYPE_A::VALUE1),
            1 => Val(SUB_TYPE_A::VALUE2),
            2 => Val(SUB_TYPE_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUB_TYPE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUB_TYPE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUB_TYPE_A::VALUE3
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
