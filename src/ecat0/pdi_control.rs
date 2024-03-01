#[doc = "Register `PDI_CONTROL` reader"]
pub type R = crate::R<PdiControlSpec>;
#[doc = "On-chip bus clock\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pdi {
    #[doc = "0: Interface deactivated (no PDI)"]
    Value1 = 0,
    #[doc = "128: On-chip Bus"]
    Value2 = 128,
}
impl From<Pdi> for u8 {
    #[inline(always)]
    fn from(variant: Pdi) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pdi {
    type Ux = u8;
}
#[doc = "Field `PDI` reader - On-chip bus clock"]
pub type PdiR = crate::FieldReader<Pdi>;
impl PdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pdi> {
        match self.bits {
            0 => Some(Pdi::Value1),
            128 => Some(Pdi::Value2),
            _ => None,
        }
    }
    #[doc = "Interface deactivated (no PDI)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdi::Value1
    }
    #[doc = "On-chip Bus"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdi::Value2
    }
}
impl R {
    #[doc = "Bits 0:7 - On-chip bus clock"]
    #[inline(always)]
    pub fn pdi(&self) -> PdiR {
        PdiR::new(self.bits)
    }
}
#[doc = "PDI Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdi_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdiControlSpec;
impl crate::RegisterSpec for PdiControlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdi_control::R`](R) reader structure"]
impl crate::Readable for PdiControlSpec {}
#[doc = "`reset()` method sets PDI_CONTROL to value 0x80"]
impl crate::Resettable for PdiControlSpec {
    const RESET_VALUE: u8 = 0x80;
}
