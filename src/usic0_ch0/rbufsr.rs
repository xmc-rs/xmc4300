#[doc = "Reader of register RBUFSR"]
pub type R = crate::R<u32, super::RBUFSR>;
#[doc = "Reader of field `WLEN`"]
pub type WLEN_R = crate::R<u8, u8>;
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PAR`"]
pub type PAR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDV0`"]
pub type RDV0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDV1`"]
pub type RDV1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<bool, bool>;
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
