#[doc = "Register `DC_SYS_TIME_DIFF` reader"]
pub struct R(crate::R<DC_SYS_TIME_DIFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SYS_TIME_DIFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SYS_TIME_DIFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SYS_TIME_DIFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIME_DIF` reader - Mean difference between local copy of System Time and received System Time values"]
pub type TIME_DIF_R = crate::FieldReader<u32>;
#[doc = "Field `CPY` reader - Local copy of System Time"]
pub type CPY_R = crate::BitReader<CPY_A>;
#[doc = "Local copy of System Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPY_A {
    #[doc = "0: Greater than or equal received System Time"]
    VALUE1 = 0,
    #[doc = "1: Smaller than received System Time"]
    VALUE2 = 1,
}
impl From<CPY_A> for bool {
    #[inline(always)]
    fn from(variant: CPY_A) -> Self {
        variant as u8 != 0
    }
}
impl CPY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPY_A {
        match self.bits {
            false => CPY_A::VALUE1,
            true => CPY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CPY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CPY_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:30 - Mean difference between local copy of System Time and received System Time values"]
    #[inline(always)]
    pub fn time_dif(&self) -> TIME_DIF_R {
        TIME_DIF_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Local copy of System Time"]
    #[inline(always)]
    pub fn cpy(&self) -> CPY_R {
        CPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "System Time Difference\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time_diff](index.html) module"]
pub struct DC_SYS_TIME_DIFF_SPEC;
impl crate::RegisterSpec for DC_SYS_TIME_DIFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_sys_time_diff::R](R) reader structure"]
impl crate::Readable for DC_SYS_TIME_DIFF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_SYS_TIME_DIFF to value 0"]
impl crate::Resettable for DC_SYS_TIME_DIFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
