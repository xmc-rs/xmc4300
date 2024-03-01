#[doc = "Register `TIMESTAMP_STATUS` reader"]
pub type R = crate::R<TimestampStatusSpec>;
#[doc = "Field `TSSOVF` reader - Timestamp Seconds Overflow"]
pub type TssovfR = crate::BitReader;
#[doc = "Field `TSTARGT` reader - Timestamp Target Time Reached"]
pub type TstargtR = crate::BitReader;
#[doc = "Field `TSTRGTERR` reader - Timestamp Target Time Error"]
pub type TstrgterrR = crate::BitReader;
#[doc = "Field `TSTARGT1` reader - Timestamp Target Time Reached for Target Time PPS1"]
pub type Tstargt1R = crate::BitReader;
#[doc = "Field `TSTRGTERR1` reader - Timestamp Target Time Error"]
pub type Tstrgterr1R = crate::BitReader;
#[doc = "Field `TSTARGT2` reader - Timestamp Target Time Reached for Target Time PPS2"]
pub type Tstargt2R = crate::BitReader;
#[doc = "Field `TSTRGTERR2` reader - Timestamp Target Time Error"]
pub type Tstrgterr2R = crate::BitReader;
#[doc = "Field `TSTARGT3` reader - Timestamp Target Time Reached for Target Time PPS3"]
pub type Tstargt3R = crate::BitReader;
#[doc = "Field `TSTRGTERR3` reader - Timestamp Target Time Error"]
pub type Tstrgterr3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow"]
    #[inline(always)]
    pub fn tssovf(&self) -> TssovfR {
        TssovfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached"]
    #[inline(always)]
    pub fn tstargt(&self) -> TstargtR {
        TstargtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr(&self) -> TstrgterrR {
        TstrgterrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp Target Time Reached for Target Time PPS1"]
    #[inline(always)]
    pub fn tstargt1(&self) -> Tstargt1R {
        Tstargt1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr1(&self) -> Tstrgterr1R {
        Tstrgterr1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timestamp Target Time Reached for Target Time PPS2"]
    #[inline(always)]
    pub fn tstargt2(&self) -> Tstargt2R {
        Tstargt2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr2(&self) -> Tstrgterr2R {
        Tstrgterr2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timestamp Target Time Reached for Target Time PPS3"]
    #[inline(always)]
    pub fn tstargt3(&self) -> Tstargt3R {
        Tstargt3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr3(&self) -> Tstrgterr3R {
        Tstrgterr3R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Timestamp Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampStatusSpec;
impl crate::RegisterSpec for TimestampStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_status::R`](R) reader structure"]
impl crate::Readable for TimestampStatusSpec {}
#[doc = "`reset()` method sets TIMESTAMP_STATUS to value 0"]
impl crate::Resettable for TimestampStatusSpec {
    const RESET_VALUE: u32 = 0;
}
