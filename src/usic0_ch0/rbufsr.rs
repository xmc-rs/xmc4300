#[doc = "Register `RBUFSR` reader"]
pub type R = crate::R<RbufsrSpec>;
#[doc = "Field `WLEN` reader - Received Data Word Length in RBUF or RBUFD"]
pub type WlenR = crate::FieldReader;
#[doc = "Field `SOF` reader - Start of Frame in RBUF or RBUFD"]
pub type SofR = crate::BitReader;
#[doc = "Field `PAR` reader - Protocol-Related Argument in RBUF or RBUFD"]
pub type ParR = crate::BitReader;
#[doc = "Field `PERR` reader - Protocol-related Error in RBUF or RBUFD"]
pub type PerrR = crate::BitReader;
#[doc = "Field `RDV0` reader - Receive Data Valid in RBUF or RBUFD"]
pub type Rdv0R = crate::BitReader;
#[doc = "Field `RDV1` reader - Receive Data Valid in RBUF or RBUFD"]
pub type Rdv1R = crate::BitReader;
#[doc = "Field `DS` reader - Data Source of RBUF or RBUFD"]
pub type DsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Received Data Word Length in RBUF or RBUFD"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Start of Frame in RBUF or RBUFD"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol-Related Argument in RBUF or RBUFD"]
    #[inline(always)]
    pub fn par(&self) -> ParR {
        ParR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protocol-related Error in RBUF or RBUFD"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Data Valid in RBUF or RBUFD"]
    #[inline(always)]
    pub fn rdv0(&self) -> Rdv0R {
        Rdv0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Data Valid in RBUF or RBUFD"]
    #[inline(always)]
    pub fn rdv1(&self) -> Rdv1R {
        Rdv1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Source of RBUF or RBUFD"]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Receiver Buffer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbufsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbufsrSpec;
impl crate::RegisterSpec for RbufsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbufsr::R`](R) reader structure"]
impl crate::Readable for RbufsrSpec {}
#[doc = "`reset()` method sets RBUFSR to value 0"]
impl crate::Resettable for RbufsrSpec {
    const RESET_VALUE: u32 = 0;
}
