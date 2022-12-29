#[doc = "Register `EVRSTAT` reader"]
pub struct R(crate::R<EVRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OV13` reader - Regulator Overvoltage for 1.3 V"]
pub type OV13_R = crate::BitReader<OV13_A>;
#[doc = "Regulator Overvoltage for 1.3 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OV13_A {
    #[doc = "0: No overvoltage condition"]
    CONST_0 = 0,
    #[doc = "1: Regulator is in overvoltage"]
    CONST_1 = 1,
}
impl From<OV13_A> for bool {
    #[inline(always)]
    fn from(variant: OV13_A) -> Self {
        variant as u8 != 0
    }
}
impl OV13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OV13_A {
        match self.bits {
            false => OV13_A::CONST_0,
            true => OV13_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == OV13_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == OV13_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 1 - Regulator Overvoltage for 1.3 V"]
    #[inline(always)]
    pub fn ov13(&self) -> OV13_R {
        OV13_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "EVR Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evrstat](index.html) module"]
pub struct EVRSTAT_SPEC;
impl crate::RegisterSpec for EVRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evrstat::R](R) reader structure"]
impl crate::Readable for EVRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EVRSTAT to value 0"]
impl crate::Resettable for EVRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
