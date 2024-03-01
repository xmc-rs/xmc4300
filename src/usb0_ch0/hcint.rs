#[doc = "Register `HCINT` reader"]
pub type R = crate::R<HcintSpec>;
#[doc = "Register `HCINT` writer"]
pub type W = crate::W<HcintSpec>;
#[doc = "Field `XferCompl` reader - Transfer Completed"]
pub type XferComplR = crate::BitReader;
#[doc = "Field `XferCompl` writer - Transfer Completed"]
pub type XferComplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ChHltd` reader - Channel Halted"]
pub type ChHltdR = crate::BitReader;
#[doc = "Field `ChHltd` writer - Channel Halted"]
pub type ChHltdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBErr` reader - AHB Error"]
pub type AhberrR = crate::BitReader;
#[doc = "Field `AHBErr` writer - AHB Error"]
pub type AhberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL Response Received Interrupt"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - STALL Response Received Interrupt"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK Response Received Interrupt"]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - NAK Response Received Interrupt"]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK Response Received/Transmitted Interrupt"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - ACK Response Received/Transmitted Interrupt"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - NYET Response Received Interrupt"]
pub type NyetR = crate::BitReader;
#[doc = "Field `NYET` writer - NYET Response Received Interrupt"]
pub type NyetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XactErr` reader - Transaction Error"]
pub type XactErrR = crate::BitReader;
#[doc = "Field `XactErr` writer - Transaction Error"]
pub type XactErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BblErr` reader - Babble Error"]
pub type BblErrR = crate::BitReader;
#[doc = "Field `BblErr` writer - Babble Error"]
pub type BblErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FrmOvrun` reader - Frame Overrun"]
pub type FrmOvrunR = crate::BitReader;
#[doc = "Field `FrmOvrun` writer - Frame Overrun"]
pub type FrmOvrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DataTglErr` reader - Data Toggle Error"]
pub type DataTglErrR = crate::BitReader;
#[doc = "Field `DataTglErr` writer - Data Toggle Error"]
pub type DataTglErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub type BnaintrR = crate::BitReader;
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub type BnaintrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCS_XACT_ERR` reader - Excessive Transaction Error"]
pub type XcsXactErrR = crate::BitReader;
#[doc = "Field `XCS_XACT_ERR` writer - Excessive Transaction Error"]
pub type XcsXactErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LST_ROLLIntr` reader - Descriptor rollover interrupt"]
pub type DescLstRollintrR = crate::BitReader;
#[doc = "Field `DESC_LST_ROLLIntr` writer - Descriptor rollover interrupt"]
pub type DescLstRollintrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfer_compl(&self) -> XferComplR {
        XferComplR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn ch_hltd(&self) -> ChHltdR {
        ChHltdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AhberrR {
        AhberrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NyetR {
        NyetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xact_err(&self) -> XactErrR {
        XactErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bbl_err(&self) -> BblErrR {
        BblErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    pub fn frm_ovrun(&self) -> FrmOvrunR {
        FrmOvrunR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    pub fn data_tgl_err(&self) -> DataTglErrR {
        DataTglErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BnaintrR {
        BnaintrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Excessive Transaction Error"]
    #[inline(always)]
    pub fn xcs_xact_err(&self) -> XcsXactErrR {
        XcsXactErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt"]
    #[inline(always)]
    pub fn desc_lst_rollintr(&self) -> DescLstRollintrR {
        DescLstRollintrR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl(&mut self) -> XferComplW<HcintSpec> {
        XferComplW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    #[must_use]
    pub fn ch_hltd(&mut self) -> ChHltdW<HcintSpec> {
        ChHltdW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AhberrW<HcintSpec> {
        AhberrW::new(self, 2)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<HcintSpec> {
        StallW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NakW<HcintSpec> {
        NakW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<HcintSpec> {
        AckW::new(self, 5)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NyetW<HcintSpec> {
        NyetW::new(self, 6)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    #[must_use]
    pub fn xact_err(&mut self) -> XactErrW<HcintSpec> {
        XactErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    #[must_use]
    pub fn bbl_err(&mut self) -> BblErrW<HcintSpec> {
        BblErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn frm_ovrun(&mut self) -> FrmOvrunW<HcintSpec> {
        FrmOvrunW::new(self, 9)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    #[must_use]
    pub fn data_tgl_err(&mut self) -> DataTglErrW<HcintSpec> {
        DataTglErrW::new(self, 10)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BnaintrW<HcintSpec> {
        BnaintrW::new(self, 11)
    }
    #[doc = "Bit 12 - Excessive Transaction Error"]
    #[inline(always)]
    #[must_use]
    pub fn xcs_xact_err(&mut self) -> XcsXactErrW<HcintSpec> {
        XcsXactErrW::new(self, 12)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn desc_lst_rollintr(&mut self) -> DescLstRollintrW<HcintSpec> {
        DescLstRollintrW::new(self, 13)
    }
}
#[doc = "Host Channel Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcintSpec;
impl crate::RegisterSpec for HcintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint::R`](R) reader structure"]
impl crate::Readable for HcintSpec {}
#[doc = "`write(|w| ..)` method takes [`hcint::W`](W) writer structure"]
impl crate::Writable for HcintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINT to value 0"]
impl crate::Resettable for HcintSpec {
    const RESET_VALUE: u32 = 0;
}
