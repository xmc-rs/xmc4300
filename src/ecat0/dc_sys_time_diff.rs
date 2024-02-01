#[doc = "Register `DC_SYS_TIME_DIFF` reader"]
pub type R = crate::R<DC_SYS_TIME_DIFF_SPEC>;
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
    pub const fn variant(&self) -> CPY_A {
        match self.bits {
            false => CPY_A::VALUE1,
            true => CPY_A::VALUE2,
        }
    }
    #[doc = "Greater than or equal received System Time"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CPY_A::VALUE1
    }
    #[doc = "Smaller than received System Time"]
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
#[doc = "System Time Difference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sys_time_diff::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SYS_TIME_DIFF_SPEC;
impl crate::RegisterSpec for DC_SYS_TIME_DIFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sys_time_diff::R`](R) reader structure"]
impl crate::Readable for DC_SYS_TIME_DIFF_SPEC {}
#[doc = "`reset()` method sets DC_SYS_TIME_DIFF to value 0"]
impl crate::Resettable for DC_SYS_TIME_DIFF_SPEC {
    const RESET_VALUE: u32 = 0;
}
