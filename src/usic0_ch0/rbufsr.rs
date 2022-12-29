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
pub type WLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOF` reader - Start of Frame in RBUF or RBUFD"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `PAR` reader - Protocol-Related Argument in RBUF or RBUFD"]
pub type PAR_R = crate::BitReader<bool>;
#[doc = "Field `PERR` reader - Protocol-related Error in RBUF or RBUFD"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `RDV0` reader - Receive Data Valid in RBUF or RBUFD"]
pub type RDV0_R = crate::BitReader<bool>;
#[doc = "Field `RDV1` reader - Receive Data Valid in RBUF or RBUFD"]
pub type RDV1_R = crate::BitReader<bool>;
#[doc = "Field `DS` reader - Data Source of RBUF or RBUFD"]
pub type DS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Received Data Word Length in RBUF or RBUFD"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Start of Frame in RBUF or RBUFD"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol-Related Argument in RBUF or RBUFD"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protocol-related Error in RBUF or RBUFD"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Data Valid in RBUF or RBUFD"]
    #[inline(always)]
    pub fn rdv0(&self) -> RDV0_R {
        RDV0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Data Valid in RBUF or RBUFD"]
    #[inline(always)]
    pub fn rdv1(&self) -> RDV1_R {
        RDV1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Source of RBUF or RBUFD"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 15) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
