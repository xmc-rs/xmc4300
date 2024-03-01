#[doc = "Register `DOEPMSK` reader"]
pub type R = crate::R<DoepmskSpec>;
#[doc = "Register `DOEPMSK` writer"]
pub type W = crate::W<DoepmskSpec>;
#[doc = "Field `XferComplMsk` reader - Transfer Completed Interrupt Mask"]
pub type XferComplMskR = crate::BitReader;
#[doc = "Field `XferComplMsk` writer - Transfer Completed Interrupt Mask"]
pub type XferComplMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDisbldMsk` reader - Endpoint Disabled Interrupt Mask"]
pub type EpdisbldMskR = crate::BitReader;
#[doc = "Field `EPDisbldMsk` writer - Endpoint Disabled Interrupt Mask"]
pub type EpdisbldMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBErrMsk` reader - AHB Error"]
pub type AhberrMskR = crate::BitReader;
#[doc = "Field `AHBErrMsk` writer - AHB Error"]
pub type AhberrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetUPMsk` reader - SETUP Phase Done Mask"]
pub type SetUpmskR = crate::BitReader;
#[doc = "Field `SetUPMsk` writer - SETUP Phase Done Mask"]
pub type SetUpmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTknEPdisMsk` reader - OUT Token Received when Endpoint Disabled Mask"]
pub type OuttknEpdisMskR = crate::BitReader;
#[doc = "Field `OUTTknEPdisMsk` writer - OUT Token Received when Endpoint Disabled Mask"]
pub type OuttknEpdisMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Back2BackSETup` reader - Back-to-Back SETUP Packets Received Mask"]
pub type Back2backSetupR = crate::BitReader;
#[doc = "Field `Back2BackSETup` writer - Back-to-Back SETUP Packets Received Mask"]
pub type Back2backSetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OutPktErrMsk` reader - OUT Packet Error Mask"]
pub type OutPktErrMskR = crate::BitReader;
#[doc = "Field `OutPktErrMsk` writer - OUT Packet Error Mask"]
pub type OutPktErrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BnaOutIntrMsk` reader - BNA interrupt Mask"]
pub type BnaOutIntrMskR = crate::BitReader;
#[doc = "Field `BnaOutIntrMsk` writer - BNA interrupt Mask"]
pub type BnaOutIntrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BbleErrMsk` reader - Babble Interrupt Mask"]
pub type BbleErrMskR = crate::BitReader;
#[doc = "Field `BbleErrMsk` writer - Babble Interrupt Mask"]
pub type BbleErrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMsk` reader - NAK Interrupt Mask"]
pub type NakmskR = crate::BitReader;
#[doc = "Field `NAKMsk` writer - NAK Interrupt Mask"]
pub type NakmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETMsk` reader - NYET Interrupt Mask"]
pub type NyetmskR = crate::BitReader;
#[doc = "Field `NYETMsk` writer - NYET Interrupt Mask"]
pub type NyetmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&self) -> XferComplMskR {
        XferComplMskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbld_msk(&self) -> EpdisbldMskR {
        EpdisbldMskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr_msk(&self) -> AhberrMskR {
        AhberrMskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask"]
    #[inline(always)]
    pub fn set_upmsk(&self) -> SetUpmskR {
        SetUpmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask"]
    #[inline(always)]
    pub fn outtkn_epdis_msk(&self) -> OuttknEpdisMskR {
        OuttknEpdisMskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask"]
    #[inline(always)]
    pub fn back2back_setup(&self) -> Back2backSetupR {
        Back2backSetupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    pub fn out_pkt_err_msk(&self) -> OutPktErrMskR {
        OutPktErrMskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt Mask"]
    #[inline(always)]
    pub fn bna_out_intr_msk(&self) -> BnaOutIntrMskR {
        BnaOutIntrMskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Interrupt Mask"]
    #[inline(always)]
    pub fn bble_err_msk(&self) -> BbleErrMskR {
        BbleErrMskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NakmskR {
        NakmskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET Interrupt Mask"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NyetmskR {
        NyetmskR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl_msk(&mut self) -> XferComplMskW<DoepmskSpec> {
        XferComplMskW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld_msk(&mut self) -> EpdisbldMskW<DoepmskSpec> {
        EpdisbldMskW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr_msk(&mut self) -> AhberrMskW<DoepmskSpec> {
        AhberrMskW::new(self, 2)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask"]
    #[inline(always)]
    #[must_use]
    pub fn set_upmsk(&mut self) -> SetUpmskW<DoepmskSpec> {
        SetUpmskW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask"]
    #[inline(always)]
    #[must_use]
    pub fn outtkn_epdis_msk(&mut self) -> OuttknEpdisMskW<DoepmskSpec> {
        OuttknEpdisMskW::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask"]
    #[inline(always)]
    #[must_use]
    pub fn back2back_setup(&mut self) -> Back2backSetupW<DoepmskSpec> {
        Back2backSetupW::new(self, 6)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn out_pkt_err_msk(&mut self) -> OutPktErrMskW<DoepmskSpec> {
        OutPktErrMskW::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bna_out_intr_msk(&mut self) -> BnaOutIntrMskW<DoepmskSpec> {
        BnaOutIntrMskW::new(self, 9)
    }
    #[doc = "Bit 12 - Babble Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bble_err_msk(&mut self) -> BbleErrMskW<DoepmskSpec> {
        BbleErrMskW::new(self, 12)
    }
    #[doc = "Bit 13 - NAK Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NakmskW<DoepmskSpec> {
        NakmskW::new(self, 13)
    }
    #[doc = "Bit 14 - NYET Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyetmsk(&mut self) -> NyetmskW<DoepmskSpec> {
        NyetmskW::new(self, 14)
    }
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoepmskSpec;
impl crate::RegisterSpec for DoepmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepmsk::R`](R) reader structure"]
impl crate::Readable for DoepmskSpec {}
#[doc = "`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure"]
impl crate::Writable for DoepmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DoepmskSpec {
    const RESET_VALUE: u32 = 0;
}
