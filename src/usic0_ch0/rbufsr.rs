#[doc = "Register `RBUFSR` reader"]
pub type R = crate::R<RBUFSR_SPEC>;
#[doc = "Field `WLEN` reader - Received Data Word Length in RBUF or RBUFD"]
pub type WLEN_R = crate::FieldReader;
#[doc = "Field `SOF` reader - Start of Frame in RBUF or RBUFD"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `PAR` reader - Protocol-Related Argument in RBUF or RBUFD"]
pub type PAR_R = crate::BitReader;
#[doc = "Field `PERR` reader - Protocol-related Error in RBUF or RBUFD"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `RDV0` reader - Receive Data Valid in RBUF or RBUFD"]
pub type RDV0_R = crate::BitReader;
#[doc = "Field `RDV1` reader - Receive Data Valid in RBUF or RBUFD"]
pub type RDV1_R = crate::BitReader;
#[doc = "Field `DS` reader - Data Source of RBUF or RBUFD"]
pub type DS_R = crate::BitReader;
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
#[doc = "Receiver Buffer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbufsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBUFSR_SPEC;
impl crate::RegisterSpec for RBUFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbufsr::R`](R) reader structure"]
impl crate::Readable for RBUFSR_SPEC {}
#[doc = "`reset()` method sets RBUFSR to value 0"]
impl crate::Resettable for RBUFSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
