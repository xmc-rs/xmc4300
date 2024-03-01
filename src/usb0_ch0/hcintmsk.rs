#[doc = "Register `HCINTMSK` reader"]
pub type R = crate::R<HcintmskSpec>;
#[doc = "Register `HCINTMSK` writer"]
pub type W = crate::W<HcintmskSpec>;
#[doc = "Field `XferComplMsk` reader - Transfer Completed Mask"]
pub type XferComplMskR = crate::BitReader;
#[doc = "Field `XferComplMsk` writer - Transfer Completed Mask"]
pub type XferComplMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ChHltdMsk` reader - Channel Halted Mask"]
pub type ChHltdMskR = crate::BitReader;
#[doc = "Field `ChHltdMsk` writer - Channel Halted Mask"]
pub type ChHltdMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBErrMsk` reader - AHB Error Mask"]
pub type AhberrMskR = crate::BitReader;
#[doc = "Field `AHBErrMsk` writer - AHB Error Mask"]
pub type AhberrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StallMsk` reader - STALL Response Received Interrupt Mask"]
pub type StallMskR = crate::BitReader;
#[doc = "Field `StallMsk` writer - STALL Response Received Interrupt Mask"]
pub type StallMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NakMsk` reader - NAK Response Received Interrupt Mask"]
pub type NakMskR = crate::BitReader;
#[doc = "Field `NakMsk` writer - NAK Response Received Interrupt Mask"]
pub type NakMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AckMsk` reader - ACK Response Received/Transmitted Interrupt Mask"]
pub type AckMskR = crate::BitReader;
#[doc = "Field `AckMsk` writer - ACK Response Received/Transmitted Interrupt Mask"]
pub type AckMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NyetMsk` reader - NYET Response Received Interrupt Mask"]
pub type NyetMskR = crate::BitReader;
#[doc = "Field `NyetMsk` writer - NYET Response Received Interrupt Mask"]
pub type NyetMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XactErrMsk` reader - Transaction Error Mask"]
pub type XactErrMskR = crate::BitReader;
#[doc = "Field `XactErrMsk` writer - Transaction Error Mask"]
pub type XactErrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BblErrMsk` reader - Babble Error Mask"]
pub type BblErrMskR = crate::BitReader;
#[doc = "Field `BblErrMsk` writer - Babble Error Mask"]
pub type BblErrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FrmOvrunMsk` reader - Frame Overrun Mask"]
pub type FrmOvrunMskR = crate::BitReader;
#[doc = "Field `FrmOvrunMsk` writer - Frame Overrun Mask"]
pub type FrmOvrunMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DataTglErrMsk` reader - Data Toggle Error Mask"]
pub type DataTglErrMskR = crate::BitReader;
#[doc = "Field `DataTglErrMsk` writer - Data Toggle Error Mask"]
pub type DataTglErrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAIntrMsk` reader - BNA (Buffer Not Available) Interrupt mask register"]
pub type BnaintrMskR = crate::BitReader;
#[doc = "Field `BNAIntrMsk` writer - BNA (Buffer Not Available) Interrupt mask register"]
pub type BnaintrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LST_ROLLIntrMsk` reader - Descriptor rollover interrupt Mask register"]
pub type DescLstRollintrMskR = crate::BitReader;
#[doc = "Field `DESC_LST_ROLLIntrMsk` writer - Descriptor rollover interrupt Mask register"]
pub type DescLstRollintrMskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&self) -> XferComplMskR {
        XferComplMskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    pub fn ch_hltd_msk(&self) -> ChHltdMskR {
        ChHltdMskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberr_msk(&self) -> AhberrMskR {
        AhberrMskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn stall_msk(&self) -> StallMskR {
        StallMskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nak_msk(&self) -> NakMskR {
        NakMskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    pub fn ack_msk(&self) -> AckMskR {
        AckMskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nyet_msk(&self) -> NyetMskR {
        NyetMskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    pub fn xact_err_msk(&self) -> XactErrMskR {
        XactErrMskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    pub fn bbl_err_msk(&self) -> BblErrMskR {
        BblErrMskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    pub fn frm_ovrun_msk(&self) -> FrmOvrunMskR {
        FrmOvrunMskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    pub fn data_tgl_err_msk(&self) -> DataTglErrMskR {
        DataTglErrMskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt mask register"]
    #[inline(always)]
    pub fn bnaintr_msk(&self) -> BnaintrMskR {
        BnaintrMskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt Mask register"]
    #[inline(always)]
    pub fn desc_lst_rollintr_msk(&self) -> DescLstRollintrMskR {
        DescLstRollintrMskR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl_msk(&mut self) -> XferComplMskW<HcintmskSpec> {
        XferComplMskW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch_hltd_msk(&mut self) -> ChHltdMskW<HcintmskSpec> {
        ChHltdMskW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr_msk(&mut self) -> AhberrMskW<HcintmskSpec> {
        AhberrMskW::new(self, 2)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn stall_msk(&mut self) -> StallMskW<HcintmskSpec> {
        StallMskW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nak_msk(&mut self) -> NakMskW<HcintmskSpec> {
        NakMskW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ack_msk(&mut self) -> AckMskW<HcintmskSpec> {
        AckMskW::new(self, 5)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyet_msk(&mut self) -> NyetMskW<HcintmskSpec> {
        NyetMskW::new(self, 6)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xact_err_msk(&mut self) -> XactErrMskW<HcintmskSpec> {
        XactErrMskW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bbl_err_msk(&mut self) -> BblErrMskW<HcintmskSpec> {
        BblErrMskW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    #[must_use]
    pub fn frm_ovrun_msk(&mut self) -> FrmOvrunMskW<HcintmskSpec> {
        FrmOvrunMskW::new(self, 9)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn data_tgl_err_msk(&mut self) -> DataTglErrMskW<HcintmskSpec> {
        DataTglErrMskW::new(self, 10)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt mask register"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr_msk(&mut self) -> BnaintrMskW<HcintmskSpec> {
        BnaintrMskW::new(self, 11)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt Mask register"]
    #[inline(always)]
    #[must_use]
    pub fn desc_lst_rollintr_msk(&mut self) -> DescLstRollintrMskW<HcintmskSpec> {
        DescLstRollintrMskW::new(self, 13)
    }
}
#[doc = "Host Channel Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcintmskSpec;
impl crate::RegisterSpec for HcintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk::R`](R) reader structure"]
impl crate::Readable for HcintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk::W`](W) writer structure"]
impl crate::Writable for HcintmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINTMSK to value 0"]
impl crate::Resettable for HcintmskSpec {
    const RESET_VALUE: u32 = 0;
}
