#[doc = "Register `CLKMXSTAT` reader"]
pub struct R(crate::R<CLKMXSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKMXSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKMXSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKMXSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status of System Clock Multiplexing Upon Source Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCLKMUX_A {
    #[doc = "1: fOFI clock active"]
    CONST_X1 = 1,
    #[doc = "2: fPLL clock active"]
    CONST_1X = 2,
}
impl From<SYSCLKMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLKMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCLKMUX` reader - Status of System Clock Multiplexing Upon Source Switching"]
pub struct SYSCLKMUX_R(crate::FieldReader<u8, SYSCLKMUX_A>);
impl SYSCLKMUX_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYSCLKMUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCLKMUX_A> {
        match self.bits {
            1 => Some(SYSCLKMUX_A::CONST_X1),
            2 => Some(SYSCLKMUX_A::CONST_1X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_X1`"]
    #[inline(always)]
    pub fn is_const_x1(&self) -> bool {
        **self == SYSCLKMUX_A::CONST_X1
    }
    #[doc = "Checks if the value of the field is `CONST_1X`"]
    #[inline(always)]
    pub fn is_const_1x(&self) -> bool {
        **self == SYSCLKMUX_A::CONST_1X
    }
}
impl core::ops::Deref for SYSCLKMUX_R {
    type Target = crate::FieldReader<u8, SYSCLKMUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Status of System Clock Multiplexing Upon Source Switching"]
    #[inline(always)]
    pub fn sysclkmux(&self) -> SYSCLKMUX_R {
        SYSCLKMUX_R::new((self.bits & 0x03) as u8)
    }
}
#[doc = "Clock Multiplexing Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkmxstat](index.html) module"]
pub struct CLKMXSTAT_SPEC;
impl crate::RegisterSpec for CLKMXSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkmxstat::R](R) reader structure"]
impl crate::Readable for CLKMXSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLKMXSTAT to value 0"]
impl crate::Resettable for CLKMXSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
