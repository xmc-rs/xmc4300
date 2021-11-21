#[doc = "Register `TIMESTAMP_STATUS` reader"]
pub struct R(crate::R<TIMESTAMP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMESTAMP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMESTAMP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMESTAMP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSOVF` reader - Timestamp Seconds Overflow"]
pub struct TSSOVF_R(crate::FieldReader<bool, bool>);
impl TSSOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTARGT` reader - Timestamp Target Time Reached"]
pub struct TSTARGT_R(crate::FieldReader<bool, bool>);
impl TSTARGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTARGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTARGT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTRGTERR` reader - Timestamp Target Time Error"]
pub struct TSTRGTERR_R(crate::FieldReader<bool, bool>);
impl TSTRGTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTRGTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTRGTERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTARGT1` reader - Timestamp Target Time Reached for Target Time PPS1"]
pub struct TSTARGT1_R(crate::FieldReader<bool, bool>);
impl TSTARGT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTARGT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTARGT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTRGTERR1` reader - Timestamp Target Time Error"]
pub struct TSTRGTERR1_R(crate::FieldReader<bool, bool>);
impl TSTRGTERR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTRGTERR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTRGTERR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTARGT2` reader - Timestamp Target Time Reached for Target Time PPS2"]
pub struct TSTARGT2_R(crate::FieldReader<bool, bool>);
impl TSTARGT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTARGT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTARGT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTRGTERR2` reader - Timestamp Target Time Error"]
pub struct TSTRGTERR2_R(crate::FieldReader<bool, bool>);
impl TSTRGTERR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTRGTERR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTRGTERR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTARGT3` reader - Timestamp Target Time Reached for Target Time PPS3"]
pub struct TSTARGT3_R(crate::FieldReader<bool, bool>);
impl TSTARGT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTARGT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTARGT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTRGTERR3` reader - Timestamp Target Time Error"]
pub struct TSTRGTERR3_R(crate::FieldReader<bool, bool>);
impl TSTRGTERR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTRGTERR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTRGTERR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Timestamp Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timestamp_status](index.html) module"]
pub struct TIMESTAMP_STATUS_SPEC;
impl crate::RegisterSpec for TIMESTAMP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timestamp_status::R](R) reader structure"]
impl crate::Readable for TIMESTAMP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMESTAMP_STATUS to value 0"]
impl crate::Resettable for TIMESTAMP_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
