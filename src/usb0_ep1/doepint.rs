#[doc = "Register `DOEPINT` reader"]
pub type R = crate::R<DoepintSpec>;
#[doc = "Register `DOEPINT` writer"]
pub type W = crate::W<DoepintSpec>;
#[doc = "Field `XferCompl` reader - Transfer Completed Interrupt"]
pub type XferComplR = crate::BitReader;
#[doc = "Field `XferCompl` writer - Transfer Completed Interrupt"]
pub type XferComplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDisbld` reader - Endpoint Disabled Interrupt"]
pub type EpdisbldR = crate::BitReader;
#[doc = "Field `EPDisbld` writer - Endpoint Disabled Interrupt"]
pub type EpdisbldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBErr` reader - AHB Error"]
pub type AhberrR = crate::BitReader;
#[doc = "Field `AHBErr` writer - AHB Error"]
pub type AhberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetUp` reader - SETUP Phase Done"]
pub type SetUpR = crate::BitReader;
#[doc = "Field `SetUp` writer - SETUP Phase Done"]
pub type SetUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTknEPdis` reader - OUT Token Received When Endpoint Disabled"]
pub type OuttknEpdisR = crate::BitReader;
#[doc = "Field `OUTTknEPdis` writer - OUT Token Received When Endpoint Disabled"]
pub type OuttknEpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StsPhseRcvd` reader - Status Phase Received For Control Write"]
pub type StsPhseRcvdR = crate::BitReader;
#[doc = "Field `StsPhseRcvd` writer - Status Phase Received For Control Write"]
pub type StsPhseRcvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Back2BackSETup` reader - Back-to-Back SETUP Packets Received"]
pub type Back2backSetupR = crate::BitReader;
#[doc = "Field `Back2BackSETup` writer - Back-to-Back SETUP Packets Received"]
pub type Back2backSetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub type BnaintrR = crate::BitReader;
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub type BnaintrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PktDrpSts` reader - Packet Dropped Status"]
pub type PktDrpStsR = crate::BitReader;
#[doc = "Field `PktDrpSts` writer - Packet Dropped Status"]
pub type PktDrpStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BbleErrIntrpt` reader - BbleErr (Babble Error) interrupt"]
pub type BbleErrIntrptR = crate::BitReader;
#[doc = "Field `BbleErrIntrpt` writer - BbleErr (Babble Error) interrupt"]
pub type BbleErrIntrptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKIntrpt` reader - NAK interrupt"]
pub type NakintrptR = crate::BitReader;
#[doc = "Field `NAKIntrpt` writer - NAK interrupt"]
pub type NakintrptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETIntrpt` reader - NYET interrupt"]
pub type NyetintrptR = crate::BitReader;
#[doc = "Field `NYETIntrpt` writer - NYET interrupt"]
pub type NyetintrptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfer_compl(&self) -> XferComplR {
        XferComplR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EpdisbldR {
        EpdisbldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AhberrR {
        AhberrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP Phase Done"]
    #[inline(always)]
    pub fn set_up(&self) -> SetUpR {
        SetUpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtkn_epdis(&self) -> OuttknEpdisR {
        OuttknEpdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    pub fn sts_phse_rcvd(&self) -> StsPhseRcvdR {
        StsPhseRcvdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2back_setup(&self) -> Back2backSetupR {
        Back2backSetupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BnaintrR {
        BnaintrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Dropped Status"]
    #[inline(always)]
    pub fn pkt_drp_sts(&self) -> PktDrpStsR {
        PktDrpStsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BbleErr (Babble Error) interrupt"]
    #[inline(always)]
    pub fn bble_err_intrpt(&self) -> BbleErrIntrptR {
        BbleErrIntrptR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NakintrptR {
        NakintrptR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyetintrpt(&self) -> NyetintrptR {
        NyetintrptR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl(&mut self) -> XferComplW<DoepintSpec> {
        XferComplW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EpdisbldW<DoepintSpec> {
        EpdisbldW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AhberrW<DoepintSpec> {
        AhberrW::new(self, 2)
    }
    #[doc = "Bit 3 - SETUP Phase Done"]
    #[inline(always)]
    #[must_use]
    pub fn set_up(&mut self) -> SetUpW<DoepintSpec> {
        SetUpW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn outtkn_epdis(&mut self) -> OuttknEpdisW<DoepintSpec> {
        OuttknEpdisW::new(self, 4)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    #[must_use]
    pub fn sts_phse_rcvd(&mut self) -> StsPhseRcvdW<DoepintSpec> {
        StsPhseRcvdW::new(self, 5)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    #[must_use]
    pub fn back2back_setup(&mut self) -> Back2backSetupW<DoepintSpec> {
        Back2backSetupW::new(self, 6)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BnaintrW<DoepintSpec> {
        BnaintrW::new(self, 9)
    }
    #[doc = "Bit 11 - Packet Dropped Status"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_drp_sts(&mut self) -> PktDrpStsW<DoepintSpec> {
        PktDrpStsW::new(self, 11)
    }
    #[doc = "Bit 12 - BbleErr (Babble Error) interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bble_err_intrpt(&mut self) -> BbleErrIntrptW<DoepintSpec> {
        BbleErrIntrptW::new(self, 12)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt(&mut self) -> NakintrptW<DoepintSpec> {
        NakintrptW::new(self, 13)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyetintrpt(&mut self) -> NyetintrptW<DoepintSpec> {
        NyetintrptW::new(self, 14)
    }
}
#[doc = "Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoepintSpec;
impl crate::RegisterSpec for DoepintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint::R`](R) reader structure"]
impl crate::Readable for DoepintSpec {}
#[doc = "`write(|w| ..)` method takes [`doepint::W`](W) writer structure"]
impl crate::Writable for DoepintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT to value 0x80"]
impl crate::Resettable for DoepintSpec {
    const RESET_VALUE: u32 = 0x80;
}
