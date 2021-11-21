#[doc = "Register `RBUFSR` reader"]
pub struct R(crate::R<RBUFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBUFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBUFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBUFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WLEN` reader - Received Data Word Length in RBUF or RBUFD"]
pub struct WLEN_R(crate::FieldReader<u8, u8>);
impl WLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        WLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF` reader - Start of Frame in RBUF or RBUFD"]
pub struct SOF_R(crate::FieldReader<bool, bool>);
impl SOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAR` reader - Protocol-Related Argument in RBUF or RBUFD"]
pub struct PAR_R(crate::FieldReader<bool, bool>);
impl PAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERR` reader - Protocol-related Error in RBUF or RBUFD"]
pub struct PERR_R(crate::FieldReader<bool, bool>);
impl PERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDV0` reader - Receive Data Valid in RBUF or RBUFD"]
pub struct RDV0_R(crate::FieldReader<bool, bool>);
impl RDV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDV1` reader - Receive Data Valid in RBUF or RBUFD"]
pub struct RDV1_R(crate::FieldReader<bool, bool>);
impl RDV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS` reader - Data Source of RBUF or RBUFD"]
pub struct DS_R(crate::FieldReader<bool, bool>);
impl DS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Received Data Word Length in RBUF or RBUFD"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Start of Frame in RBUF or RBUFD"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Protocol-Related Argument in RBUF or RBUFD"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protocol-related Error in RBUF or RBUFD"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Receive Data Valid in RBUF or RBUFD"]
    #[inline(always)]
    pub fn rdv0(&self) -> RDV0_R {
        RDV0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Data Valid in RBUF or RBUFD"]
    #[inline(always)]
    pub fn rdv1(&self) -> RDV1_R {
        RDV1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Data Source of RBUF or RBUFD"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Receiver Buffer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbufsr](index.html) module"]
pub struct RBUFSR_SPEC;
impl crate::RegisterSpec for RBUFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbufsr::R](R) reader structure"]
impl crate::Readable for RBUFSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RBUFSR to value 0"]
impl crate::Resettable for RBUFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
