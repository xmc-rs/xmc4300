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
pub type SUSP_STS_R = crate::BitReader<bool>;
#[doc = "Field `EnumSpd` reader - Enumerated Speed"]
pub type ENUM_SPD_R = crate::FieldReader<u8, ENUM_SPD_A>;
#[doc = "Enumerated Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENUM_SPD_A {
    #[doc = "3: Full speed (PHY clock is running at 48 MHz)"]
    VALUE4 = 3,
}
impl From<ENUM_SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUM_SPD_A) -> Self {
        variant as _
    }
}
impl ENUM_SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENUM_SPD_A> {
        match self.bits {
            3 => Some(ENUM_SPD_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ENUM_SPD_A::VALUE4
    }
}
#[doc = "Field `ErrticErr` reader - Erratic Error"]
pub type ERRTIC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SOFFN` reader - Frame Number of the Received SOF"]
pub type SOFFN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn susp_sts(&self) -> SUSP_STS_R {
        SUSP_STS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enum_spd(&self) -> ENUM_SPD_R {
        ENUM_SPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errtic_err(&self) -> ERRTIC_ERR_R {
        ERRTIC_ERR_R::new(((self.bits >> 3) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0x02;
}
