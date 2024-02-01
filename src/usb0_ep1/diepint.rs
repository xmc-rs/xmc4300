#[doc = "Register `DIEPINT` reader"]
pub type R = crate::R<DIEPINT_SPEC>;
#[doc = "Register `DIEPINT` writer"]
pub type W = crate::W<DIEPINT_SPEC>;
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
#[doc = "Field `TimeOUT` reader - Timeout Condition"]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `TimeOUT` writer - Timeout Condition"]
pub type TIME_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTknTXFEmp` reader - IN Token Received When TxFIFO is Empty"]
pub type INTKN_TXFEMP_R = crate::BitReader;
#[doc = "Field `INTknTXFEmp` writer - IN Token Received When TxFIFO is Empty"]
pub type INTKN_TXFEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNakEff` reader - IN Endpoint NAK Effective"]
pub type INEPNAK_EFF_R = crate::BitReader;
#[doc = "Field `INEPNakEff` writer - IN Endpoint NAK Effective"]
pub type INEPNAK_EFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFEmp` reader - Transmit FIFO Empty"]
pub type TX_FEMP_R = crate::BitReader;
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_R = crate::BitReader;
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkn_txfemp(&self) -> INTKN_TXFEMP_R {
        INTKN_TXFEMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnak_eff(&self) -> INEPNAK_EFF_R {
        INEPNAK_EFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn tx_femp(&self) -> TX_FEMP_R {
        TX_FEMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl(&mut self) -> XFER_COMPL_W<DIEPINT_SPEC> {
        XFER_COMPL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<DIEPINT_SPEC> {
        EPDISBLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<DIEPINT_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    #[must_use]
    pub fn time_out(&mut self) -> TIME_OUT_W<DIEPINT_SPEC> {
        TIME_OUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkn_txfemp(&mut self) -> INTKN_TXFEMP_W<DIEPINT_SPEC> {
        INTKN_TXFEMP_W::new(self, 4)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    #[must_use]
    pub fn inepnak_eff(&mut self) -> INEPNAK_EFF_W<DIEPINT_SPEC> {
        INEPNAK_EFF_W::new(self, 6)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BNAINTR_W<DIEPINT_SPEC> {
        BNAINTR_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT_SPEC;
impl crate::RegisterSpec for DIEPINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint::R`](R) reader structure"]
impl crate::Readable for DIEPINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepint::W`](W) writer structure"]
impl crate::Writable for DIEPINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT to value 0x80"]
impl crate::Resettable for DIEPINT_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
