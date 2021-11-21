#[doc = "Register `DSTS` reader"]
pub struct R(crate::R<DSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SuspSts` reader - Suspend Status"]
pub struct SUSPSTS_R(crate::FieldReader<bool, bool>);
impl SUSPSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enumerated Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENUMSPD_A {
    #[doc = "3: Full speed (PHY clock is running at 48 MHz)"]
    VALUE4 = 3,
}
impl From<ENUMSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUMSPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EnumSpd` reader - Enumerated Speed"]
pub struct ENUMSPD_R(crate::FieldReader<u8, ENUMSPD_A>);
impl ENUMSPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENUMSPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENUMSPD_A> {
        match self.bits {
            3 => Some(ENUMSPD_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ENUMSPD_A::VALUE4
    }
}
impl core::ops::Deref for ENUMSPD_R {
    type Target = crate::FieldReader<u8, ENUMSPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ErrticErr` reader - Erratic Error"]
pub struct ERRTICERR_R(crate::FieldReader<bool, bool>);
impl ERRTICERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRTICERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRTICERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFFN` reader - Frame Number of the Received SOF"]
pub struct SOFFN_R(crate::FieldReader<u16, u16>);
impl SOFFN_R {
    pub(crate) fn new(bits: u16) -> Self {
        SOFFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFFN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn susp_sts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enum_spd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errtic_err(&self) -> ERRTICERR_R {
        ERRTICERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:21 - Frame Number of the Received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
#[doc = "Device Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsts](index.html) module"]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsts::R](R) reader structure"]
impl crate::Readable for DSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
