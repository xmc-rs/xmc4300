#[doc = "Register `DIEPMSK` reader"]
pub type R = crate::R<DIEPMSK_SPEC>;
#[doc = "Register `DIEPMSK` writer"]
pub type W = crate::W<DIEPMSK_SPEC>;
#[doc = "Field `XferComplMsk` reader - Transfer Completed Interrupt Mask"]
pub type XFER_COMPL_MSK_R = crate::BitReader;
#[doc = "Field `XferComplMsk` writer - Transfer Completed Interrupt Mask"]
pub type XFER_COMPL_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDisbldMsk` reader - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLD_MSK_R = crate::BitReader;
#[doc = "Field `EPDisbldMsk` writer - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLD_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBErrMsk` reader - AHB Error Mask"]
pub type AHBERR_MSK_R = crate::BitReader;
#[doc = "Field `AHBErrMsk` writer - AHB Error Mask"]
pub type AHBERR_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TimeOUTMsk` reader - Timeout Condition Mask"]
pub type TIME_OUTMSK_R = crate::BitReader;
#[doc = "Field `TimeOUTMsk` writer - Timeout Condition Mask"]
pub type TIME_OUTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTknTXFEmpMsk` reader - IN Token Received When TxFIFO Empty Mask"]
pub type INTKN_TXFEMP_MSK_R = crate::BitReader;
#[doc = "Field `INTknTXFEmpMsk` writer - IN Token Received When TxFIFO Empty Mask"]
pub type INTKN_TXFEMP_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNakEffMsk` reader - IN Endpoint NAK Effective Mask"]
pub type INEPNAK_EFF_MSK_R = crate::BitReader;
#[doc = "Field `INEPNakEffMsk` writer - IN Endpoint NAK Effective Mask"]
pub type INEPNAK_EFF_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxfifoUndrnMsk` reader - Fifo Underrun Mask"]
pub type TXFIFO_UNDRN_MSK_R = crate::BitReader;
#[doc = "Field `TxfifoUndrnMsk` writer - Fifo Underrun Mask"]
pub type TXFIFO_UNDRN_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAInIntrMsk` reader - BNA Interrupt Mask"]
pub type BNAIN_INTR_MSK_R = crate::BitReader;
#[doc = "Field `BNAInIntrMsk` writer - BNA Interrupt Mask"]
pub type BNAIN_INTR_MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMsk` reader - NAK interrupt Mask"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMsk` writer - NAK interrupt Mask"]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&self) -> XFER_COMPL_MSK_R {
        XFER_COMPL_MSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbld_msk(&self) -> EPDISBLD_MSK_R {
        EPDISBLD_MSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberr_msk(&self) -> AHBERR_MSK_R {
        AHBERR_MSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    pub fn time_outmsk(&self) -> TIME_OUTMSK_R {
        TIME_OUTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn intkn_txfemp_msk(&self) -> INTKN_TXFEMP_MSK_R {
        INTKN_TXFEMP_MSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    pub fn inepnak_eff_msk(&self) -> INEPNAK_EFF_MSK_R {
        INEPNAK_EFF_MSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    pub fn txfifo_undrn_msk(&self) -> TXFIFO_UNDRN_MSK_R {
        TXFIFO_UNDRN_MSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA Interrupt Mask"]
    #[inline(always)]
    pub fn bnain_intr_msk(&self) -> BNAIN_INTR_MSK_R {
        BNAIN_INTR_MSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl_msk(&mut self) -> XFER_COMPL_MSK_W<DIEPMSK_SPEC> {
        XFER_COMPL_MSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld_msk(&mut self) -> EPDISBLD_MSK_W<DIEPMSK_SPEC> {
        EPDISBLD_MSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr_msk(&mut self) -> AHBERR_MSK_W<DIEPMSK_SPEC> {
        AHBERR_MSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Condition Mask"]
    #[inline(always)]
    #[must_use]
    pub fn time_outmsk(&mut self) -> TIME_OUTMSK_W<DIEPMSK_SPEC> {
        TIME_OUTMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn intkn_txfemp_msk(&mut self) -> INTKN_TXFEMP_MSK_W<DIEPMSK_SPEC> {
        INTKN_TXFEMP_MSK_W::new(self, 4)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnak_eff_msk(&mut self) -> INEPNAK_EFF_MSK_W<DIEPMSK_SPEC> {
        INEPNAK_EFF_MSK_W::new(self, 6)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_undrn_msk(&mut self) -> TXFIFO_UNDRN_MSK_W<DIEPMSK_SPEC> {
        TXFIFO_UNDRN_MSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bnain_intr_msk(&mut self) -> BNAIN_INTR_MSK_W<DIEPMSK_SPEC> {
        BNAIN_INTR_MSK_W::new(self, 9)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<DIEPMSK_SPEC> {
        NAKMSK_W::new(self, 13)
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
#[doc = "Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepmsk::R`](R) reader structure"]
impl crate::Readable for DIEPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure"]
impl crate::Writable for DIEPMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DIEPMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
