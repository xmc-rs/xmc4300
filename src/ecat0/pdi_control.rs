#[doc = "Register `PDI_CONTROL` reader"]
pub type R = crate::R<PDI_CONTROL_SPEC>;
#[doc = "On-chip bus clock\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for PDI_A {
    type Ux = u8;
}
impl crate::IsEnum for PDI_A {}
#[doc = "Field `PDI` reader - On-chip bus clock"]
pub type PDI_R = crate::FieldReader<PDI_A>;
impl PDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PDI_A> {
        match self.bits {
            0 => Some(PDI_A::VALUE1),
            128 => Some(PDI_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Interface deactivated (no PDI)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDI_A::VALUE1
    }
    #[doc = "On-chip Bus"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDI_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:7 - On-chip bus clock"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(self.bits)
    }
}
#[doc = "PDI Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdi_control::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDI_CONTROL_SPEC;
impl crate::RegisterSpec for PDI_CONTROL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdi_control::R`](R) reader structure"]
impl crate::Readable for PDI_CONTROL_SPEC {}
#[doc = "`reset()` method sets PDI_CONTROL to value 0x80"]
impl crate::Resettable for PDI_CONTROL_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
