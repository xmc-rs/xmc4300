#[doc = "Reader of register TIMESTAMP_STATUS"]
pub type R = crate::R<u32, super::TIMESTAMP_STATUS>;
#[doc = "Reader of field `TSSOVF`"]
pub type TSSOVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTARGT`"]
pub type TSTARGT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTRGTERR`"]
pub type TSTRGTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTARGT1`"]
pub type TSTARGT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTRGTERR1`"]
pub type TSTRGTERR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTARGT2`"]
pub type TSTARGT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTRGTERR2`"]
pub type TSTRGTERR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTARGT3`"]
pub type TSTARGT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTRGTERR3`"]
pub type TSTRGTERR3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached"]
    #[inline(always)]
    pub fn tstargt(&self) -> TSTARGT_R {
        TSTARGT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr(&self) -> TSTRGTERR_R {
        TSTRGTERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timestamp Target Time Reached for Target Time PPS1"]
    #[inline(always)]
    pub fn tstargt1(&self) -> TSTARGT1_R {
        TSTARGT1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr1(&self) -> TSTRGTERR1_R {
        TSTRGTERR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timestamp Target Time Reached for Target Time PPS2"]
    #[inline(always)]
    pub fn tstargt2(&self) -> TSTARGT2_R {
        TSTARGT2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr2(&self) -> TSTRGTERR2_R {
        TSTRGTERR2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timestamp Target Time Reached for Target Time PPS3"]
    #[inline(always)]
    pub fn tstargt3(&self) -> TSTARGT3_R {
        TSTARGT3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr3(&self) -> TSTRGTERR3_R {
        TSTRGTERR3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
