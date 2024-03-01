#[doc = "Register `DIEPINT0` reader"]
pub type R = crate::R<Diepint0Spec>;
#[doc = "Register `DIEPINT0` writer"]
pub type W = crate::W<Diepint0Spec>;
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
#[doc = "Field `TimeOUT` reader - Timeout Condition"]
pub type TimeOutR = crate::BitReader;
#[doc = "Field `TimeOUT` writer - Timeout Condition"]
pub type TimeOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTknTXFEmp` reader - IN Token Received When TxFIFO is Empty"]
pub type IntknTxfempR = crate::BitReader;
#[doc = "Field `INTknTXFEmp` writer - IN Token Received When TxFIFO is Empty"]
pub type IntknTxfempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNakEff` reader - IN Endpoint NAK Effective"]
pub type InepnakEffR = crate::BitReader;
#[doc = "Field `INEPNakEff` writer - IN Endpoint NAK Effective"]
pub type InepnakEffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFEmp` reader - Transmit FIFO Empty"]
pub type TxFempR = crate::BitReader;
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub type BnaintrR = crate::BitReader;
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub type BnaintrW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    pub fn time_out(&self) -> TimeOutR {
        TimeOutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkn_txfemp(&self) -> IntknTxfempR {
        IntknTxfempR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnak_eff(&self) -> InepnakEffR {
        InepnakEffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn tx_femp(&self) -> TxFempR {
        TxFempR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BnaintrR {
        BnaintrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl(&mut self) -> XferComplW<Diepint0Spec> {
        XferComplW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EpdisbldW<Diepint0Spec> {
        EpdisbldW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AhberrW<Diepint0Spec> {
        AhberrW::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    #[must_use]
    pub fn time_out(&mut self) -> TimeOutW<Diepint0Spec> {
        TimeOutW::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkn_txfemp(&mut self) -> IntknTxfempW<Diepint0Spec> {
        IntknTxfempW::new(self, 4)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    #[must_use]
    pub fn inepnak_eff(&mut self) -> InepnakEffW<Diepint0Spec> {
        InepnakEffW::new(self, 6)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BnaintrW<Diepint0Spec> {
        BnaintrW::new(self, 9)
    }
}
#[doc = "Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepint0Spec;
impl crate::RegisterSpec for Diepint0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint0::R`](R) reader structure"]
impl crate::Readable for Diepint0Spec {}
#[doc = "`write(|w| ..)` method takes [`diepint0::W`](W) writer structure"]
impl crate::Writable for Diepint0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT0 to value 0x80"]
impl crate::Resettable for Diepint0Spec {
    const RESET_VALUE: u32 = 0x80;
}
