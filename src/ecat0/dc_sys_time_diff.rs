#[doc = "Register `DC_SYS_TIME_DIFF` reader"]
pub type R = crate::R<DcSysTimeDiffSpec>;
#[doc = "Field `TIME_DIF` reader - Mean difference between local copy of System Time and received System Time values"]
pub type TimeDifR = crate::FieldReader<u32>;
#[doc = "Local copy of System Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpy {
    #[doc = "0: Greater than or equal received System Time"]
    Value1 = 0,
    #[doc = "1: Smaller than received System Time"]
    Value2 = 1,
}
impl From<Cpy> for bool {
    #[inline(always)]
    fn from(variant: Cpy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPY` reader - Local copy of System Time"]
pub type CpyR = crate::BitReader<Cpy>;
impl CpyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpy {
        match self.bits {
            false => Cpy::Value1,
            true => Cpy::Value2,
        }
    }
    #[doc = "Greater than or equal received System Time"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cpy::Value1
    }
    #[doc = "Smaller than received System Time"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cpy::Value2
    }
}
impl R {
    #[doc = "Bits 0:30 - Mean difference between local copy of System Time and received System Time values"]
    #[inline(always)]
    pub fn time_dif(&self) -> TimeDifR {
        TimeDifR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Local copy of System Time"]
    #[inline(always)]
    pub fn cpy(&self) -> CpyR {
        CpyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "System Time Difference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sys_time_diff::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcSysTimeDiffSpec;
impl crate::RegisterSpec for DcSysTimeDiffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_sys_time_diff::R`](R) reader structure"]
impl crate::Readable for DcSysTimeDiffSpec {}
#[doc = "`reset()` method sets DC_SYS_TIME_DIFF to value 0"]
impl crate::Resettable for DcSysTimeDiffSpec {
    const RESET_VALUE: u32 = 0;
}
