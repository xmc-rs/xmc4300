#[doc = "Register `TIMESTAMP_STATUS` reader"]
pub type R = crate::R<TIMESTAMP_STATUS_SPEC>;
#[doc = "Field `TSSOVF` reader - Timestamp Seconds Overflow"]
pub type TSSOVF_R = crate::BitReader;
#[doc = "Field `TSTARGT` reader - Timestamp Target Time Reached"]
pub type TSTARGT_R = crate::BitReader;
#[doc = "Field `TSTRGTERR` reader - Timestamp Target Time Error"]
pub type TSTRGTERR_R = crate::BitReader;
#[doc = "Field `TSTARGT1` reader - Timestamp Target Time Reached for Target Time PPS1"]
pub type TSTARGT1_R = crate::BitReader;
#[doc = "Field `TSTRGTERR1` reader - Timestamp Target Time Error"]
pub type TSTRGTERR1_R = crate::BitReader;
#[doc = "Field `TSTARGT2` reader - Timestamp Target Time Reached for Target Time PPS2"]
pub type TSTARGT2_R = crate::BitReader;
#[doc = "Field `TSTRGTERR2` reader - Timestamp Target Time Error"]
pub type TSTRGTERR2_R = crate::BitReader;
#[doc = "Field `TSTARGT3` reader - Timestamp Target Time Reached for Target Time PPS3"]
pub type TSTARGT3_R = crate::BitReader;
#[doc = "Field `TSTRGTERR3` reader - Timestamp Target Time Error"]
pub type TSTRGTERR3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached"]
    #[inline(always)]
    pub fn tstargt(&self) -> TSTARGT_R {
        TSTARGT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr(&self) -> TSTRGTERR_R {
        TSTRGTERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp Target Time Reached for Target Time PPS1"]
    #[inline(always)]
    pub fn tstargt1(&self) -> TSTARGT1_R {
        TSTARGT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr1(&self) -> TSTRGTERR1_R {
        TSTRGTERR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timestamp Target Time Reached for Target Time PPS2"]
    #[inline(always)]
    pub fn tstargt2(&self) -> TSTARGT2_R {
        TSTARGT2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr2(&self) -> TSTRGTERR2_R {
        TSTRGTERR2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timestamp Target Time Reached for Target Time PPS3"]
    #[inline(always)]
    pub fn tstargt3(&self) -> TSTARGT3_R {
        TSTARGT3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr3(&self) -> TSTRGTERR3_R {
        TSTRGTERR3_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Timestamp Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTAMP_STATUS_SPEC;
impl crate::RegisterSpec for TIMESTAMP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_status::R`](R) reader structure"]
impl crate::Readable for TIMESTAMP_STATUS_SPEC {}
#[doc = "`reset()` method sets TIMESTAMP_STATUS to value 0"]
impl crate::Resettable for TIMESTAMP_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
