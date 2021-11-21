#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "PARITY ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARERR_A {
    #[doc = "0: No Error"]
    VALUE1 = 0,
    #[doc = "1: Parity Error in User or Process RAM"]
    VALUE2 = 1,
}
impl From<PARERR_A> for bool {
    #[inline(always)]
    fn from(variant: PARERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARERR` reader - PARITY ERROR"]
pub struct PARERR_R(crate::FieldReader<bool, PARERR_A>);
impl PARERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARERR_A {
        match self.bits {
            false => PARERR_A::VALUE1,
            true => PARERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PARERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PARERR_A::VALUE2
    }
}
impl core::ops::Deref for PARERR_R {
    type Target = crate::FieldReader<bool, PARERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PARITY ERROR"]
    #[inline(always)]
    pub fn parerr(&self) -> PARERR_R {
        PARERR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "ECAT0 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
