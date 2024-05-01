#[doc = "Register `DIEPMSK` reader"]
pub type R = crate::R<DiepmskSpec>;
#[doc = "Register `DIEPMSK` writer"]
pub type W = crate::W<DiepmskSpec>;
#[doc = "Field `XferComplMsk` reader - Transfer Completed Interrupt Mask"]
pub type XferComplMskR = crate::BitReader;
#[doc = "Field `XferComplMsk` writer - Transfer Completed Interrupt Mask"]
pub type XferComplMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDisbldMsk` reader - Endpoint Disabled Interrupt Mask"]
pub type EpdisbldMskR = crate::BitReader;
#[doc = "Field `EPDisbldMsk` writer - Endpoint Disabled Interrupt Mask"]
pub type EpdisbldMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBErrMsk` reader - AHB Error Mask"]
pub type AhberrMskR = crate::BitReader;
#[doc = "Field `AHBErrMsk` writer - AHB Error Mask"]
pub type AhberrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TimeOUTMsk` reader - Timeout Condition Mask"]
pub type TimeOutmskR = crate::BitReader;
#[doc = "Field `TimeOUTMsk` writer - Timeout Condition Mask"]
pub type TimeOutmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTknTXFEmpMsk` reader - IN Token Received When TxFIFO Empty Mask"]
pub type IntknTxfempMskR = crate::BitReader;
#[doc = "Field `INTknTXFEmpMsk` writer - IN Token Received When TxFIFO Empty Mask"]
pub type IntknTxfempMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNakEffMsk` reader - IN Endpoint NAK Effective Mask"]
pub type InepnakEffMskR = crate::BitReader;
#[doc = "Field `INEPNakEffMsk` writer - IN Endpoint NAK Effective Mask"]
pub type InepnakEffMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxfifoUndrnMsk` reader - Fifo Underrun Mask"]
pub type TxfifoUndrnMskR = crate::BitReader;
#[doc = "Field `TxfifoUndrnMsk` writer - Fifo Underrun Mask"]
pub type TxfifoUndrnMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAInIntrMsk` reader - BNA Interrupt Mask"]
pub type BnainIntrMskR = crate::BitReader;
#[doc = "Field `BNAInIntrMsk` writer - BNA Interrupt Mask"]
pub type BnainIntrMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMsk` reader - NAK interrupt Mask"]
pub type NakmskR = crate::BitReader;
#[doc = "Field `NAKMsk` writer - NAK interrupt Mask"]
pub type NakmskW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberr_msk(&self) -> AhberrMskR {
        AhberrMskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    pub fn time_outmsk(&self) -> TimeOutmskR {
        TimeOutmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn intkn_txfemp_msk(&self) -> IntknTxfempMskR {
        IntknTxfempMskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    pub fn inepnak_eff_msk(&self) -> InepnakEffMskR {
        InepnakEffMskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    pub fn txfifo_undrn_msk(&self) -> TxfifoUndrnMskR {
        TxfifoUndrnMskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA Interrupt Mask"]
    #[inline(always)]
    pub fn bnain_intr_msk(&self) -> BnainIntrMskR {
        BnainIntrMskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NakmskR {
        NakmskR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl_msk(&mut self) -> XferComplMskW<DiepmskSpec> {
        XferComplMskW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld_msk(&mut self) -> EpdisbldMskW<DiepmskSpec> {
        EpdisbldMskW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr_msk(&mut self) -> AhberrMskW<DiepmskSpec> {
        AhberrMskW::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    #[must_use]
    pub fn time_outmsk(&mut self) -> TimeOutmskW<DiepmskSpec> {
        TimeOutmskW::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn intkn_txfemp_msk(&mut self) -> IntknTxfempMskW<DiepmskSpec> {
        IntknTxfempMskW::new(self, 4)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnak_eff_msk(&mut self) -> InepnakEffMskW<DiepmskSpec> {
        InepnakEffMskW::new(self, 6)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_undrn_msk(&mut self) -> TxfifoUndrnMskW<DiepmskSpec> {
        TxfifoUndrnMskW::new(self, 8)
    }
    #[doc = "Bit 9 - BNA Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bnain_intr_msk(&mut self) -> BnainIntrMskW<DiepmskSpec> {
        BnainIntrMskW::new(self, 9)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NakmskW<DiepmskSpec> {
        NakmskW::new(self, 13)
    }
}
#[doc = "Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepmskSpec;
impl crate::RegisterSpec for DiepmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepmsk::R`](R) reader structure"]
impl crate::Readable for DiepmskSpec {}
#[doc = "`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure"]
impl crate::Writable for DiepmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DiepmskSpec {
    const RESET_VALUE: u32 = 0;
}
