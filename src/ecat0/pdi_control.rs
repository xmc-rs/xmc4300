#[doc = "Reader of register PDI_CONTROL"]
pub type R = crate::R<u8, super::PDI_CONTROL>;
#[doc = "On-chip bus clock\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PDI_A {
    #[doc = "0: Interface deactivated (no PDI)"]
    VALUE1 = 0,
    #[doc = "128: On-chip Bus"]
    VALUE2 = 128,
}
impl From<PDI_A> for u8 {
    #[inline(always)]
    fn from(variant: PDI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PDI`"]
pub type PDI_R = crate::R<u8, PDI_A>;
impl PDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PDI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PDI_A::VALUE1),
            128 => Val(PDI_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDI_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:7 - On-chip bus clock"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new((self.bits & 0xff) as u8)
    }
}
