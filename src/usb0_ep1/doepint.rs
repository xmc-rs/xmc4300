#[doc = "Register `DOEPINT` reader"]
pub type R = crate::R<DOEPINT_SPEC>;
#[doc = "Register `DOEPINT` writer"]
pub type W = crate::W<DOEPINT_SPEC>;
#[doc = "Field `XferCompl` reader - Transfer Completed Interrupt"]
pub type XFER_COMPL_R = crate::BitReader;
#[doc = "Field `XferCompl` writer - Transfer Completed Interrupt"]
pub type XFER_COMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDisbld` reader - Endpoint Disabled Interrupt"]
pub type EPDISBLD_R = crate::BitReader;
#[doc = "Field `EPDisbld` writer - Endpoint Disabled Interrupt"]
pub type EPDISBLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBErr` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBErr` writer - AHB Error"]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetUp` reader - SETUP Phase Done"]
pub type SET_UP_R = crate::BitReader;
#[doc = "Field `SetUp` writer - SETUP Phase Done"]
pub type SET_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTknEPdis` reader - OUT Token Received When Endpoint Disabled"]
pub type OUTTKN_EPDIS_R = crate::BitReader;
#[doc = "Field `OUTTknEPdis` writer - OUT Token Received When Endpoint Disabled"]
pub type OUTTKN_EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StsPhseRcvd` reader - Status Phase Received For Control Write"]
pub type STS_PHSE_RCVD_R = crate::BitReader;
#[doc = "Field `StsPhseRcvd` writer - Status Phase Received For Control Write"]
pub type STS_PHSE_RCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Back2BackSETup` reader - Back-to-Back SETUP Packets Received"]
pub type BACK2BACK_SETUP_R = crate::BitReader;
#[doc = "Field `Back2BackSETup` writer - Back-to-Back SETUP Packets Received"]
pub type BACK2BACK_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_R = crate::BitReader;
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PktDrpSts` reader - Packet Dropped Status"]
pub type PKT_DRP_STS_R = crate::BitReader;
#[doc = "Field `PktDrpSts` writer - Packet Dropped Status"]
pub type PKT_DRP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BbleErrIntrpt` reader - BbleErr (Babble Error) interrupt"]
pub type BBLE_ERR_INTRPT_R = crate::BitReader;
#[doc = "Field `BbleErrIntrpt` writer - BbleErr (Babble Error) interrupt"]
pub type BBLE_ERR_INTRPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKIntrpt` reader - NAK interrupt"]
pub type NAKINTRPT_R = crate::BitReader;
#[doc = "Field `NAKIntrpt` writer - NAK interrupt"]
pub type NAKINTRPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETIntrpt` reader - NYET interrupt"]
pub type NYETINTRPT_R = crate::BitReader;
#[doc = "Field `NYETIntrpt` writer - NYET interrupt"]
pub type NYETINTRPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfer_compl(&self) -> XFER_COMPL_R {
        XFER_COMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EPDISBLD_R {
        EPDISBLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP Phase Done"]
    #[inline(always)]
    pub fn set_up(&self) -> SET_UP_R {
        SET_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtkn_epdis(&self) -> OUTTKN_EPDIS_R {
        OUTTKN_EPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    pub fn sts_phse_rcvd(&self) -> STS_PHSE_RCVD_R {
        STS_PHSE_RCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2back_setup(&self) -> BACK2BACK_SETUP_R {
        BACK2BACK_SETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Dropped Status"]
    #[inline(always)]
    pub fn pkt_drp_sts(&self) -> PKT_DRP_STS_R {
        PKT_DRP_STS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BbleErr (Babble Error) interrupt"]
    #[inline(always)]
    pub fn bble_err_intrpt(&self) -> BBLE_ERR_INTRPT_R {
        BBLE_ERR_INTRPT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NAKINTRPT_R {
        NAKINTRPT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyetintrpt(&self) -> NYETINTRPT_R {
        NYETINTRPT_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl(&mut self) -> XFER_COMPL_W<DOEPINT_SPEC> {
        XFER_COMPL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<DOEPINT_SPEC> {
        EPDISBLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<DOEPINT_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - SETUP Phase Done"]
    #[inline(always)]
    #[must_use]
    pub fn set_up(&mut self) -> SET_UP_W<DOEPINT_SPEC> {
        SET_UP_W::new(self, 3)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn outtkn_epdis(&mut self) -> OUTTKN_EPDIS_W<DOEPINT_SPEC> {
        OUTTKN_EPDIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    #[must_use]
    pub fn sts_phse_rcvd(&mut self) -> STS_PHSE_RCVD_W<DOEPINT_SPEC> {
        STS_PHSE_RCVD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    #[must_use]
    pub fn back2back_setup(&mut self) -> BACK2BACK_SETUP_W<DOEPINT_SPEC> {
        BACK2BACK_SETUP_W::new(self, 6)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BNAINTR_W<DOEPINT_SPEC> {
        BNAINTR_W::new(self, 9)
    }
    #[doc = "Bit 11 - Packet Dropped Status"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_drp_sts(&mut self) -> PKT_DRP_STS_W<DOEPINT_SPEC> {
        PKT_DRP_STS_W::new(self, 11)
    }
    #[doc = "Bit 12 - BbleErr (Babble Error) interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bble_err_intrpt(&mut self) -> BBLE_ERR_INTRPT_W<DOEPINT_SPEC> {
        BBLE_ERR_INTRPT_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W<DOEPINT_SPEC> {
        NAKINTRPT_W::new(self, 13)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyetintrpt(&mut self) -> NYETINTRPT_W<DOEPINT_SPEC> {
        NYETINTRPT_W::new(self, 14)
    }
}
#[doc = "Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT_SPEC;
impl crate::RegisterSpec for DOEPINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint::R`](R) reader structure"]
impl crate::Readable for DOEPINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepint::W`](W) writer structure"]
impl crate::Writable for DOEPINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT to value 0x80"]
impl crate::Resettable for DOEPINT_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
