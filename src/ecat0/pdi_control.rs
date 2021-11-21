#[doc = "Register `PDI_CONTROL` reader"]
pub struct R(crate::R<PDI_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDI_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDI_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDI_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `PDI` reader - On-chip bus clock"]
pub struct PDI_R(crate::FieldReader<u8, PDI_A>);
impl PDI_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PDI_A> {
        match self.bits {
            0 => Some(PDI_A::VALUE1),
            128 => Some(PDI_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PDI_A::VALUE2
    }
}
impl core::ops::Deref for PDI_R {
    type Target = crate::FieldReader<u8, PDI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - On-chip bus clock"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PDI Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdi_control](index.html) module"]
pub struct PDI_CONTROL_SPEC;
impl crate::RegisterSpec for PDI_CONTROL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pdi_control::R](R) reader structure"]
impl crate::Readable for PDI_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDI_CONTROL to value 0x80"]
impl crate::Resettable for PDI_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
