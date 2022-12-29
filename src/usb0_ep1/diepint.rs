#[doc = "Register `DIEPINT` reader"]
pub struct R(crate::R<DIEPINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT` writer"]
pub struct W(crate::W<DIEPINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DIEPINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferCompl` reader - Transfer Completed Interrupt"]
pub type XFER_COMPL_R = crate::BitReader<bool>;
#[doc = "Field `XferCompl` writer - Transfer Completed Interrupt"]
pub type XFER_COMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT_SPEC, bool, O>;
#[doc = "Field `EPDisbld` reader - Endpoint Disabled Interrupt"]
pub type EPDISBLD_R = crate::BitReader<bool>;
#[doc = "Field `EPDisbld` writer - Endpoint Disabled Interrupt"]
pub type EPDISBLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT_SPEC, bool, O>;
#[doc = "Field `AHBErr` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBErr` writer - AHB Error"]
pub type AHBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT_SPEC, bool, O>;
#[doc = "Field `TimeOUT` reader - Timeout Condition"]
pub type TIME_OUT_R = crate::BitReader<bool>;
#[doc = "Field `TimeOUT` writer - Timeout Condition"]
pub type TIME_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT_SPEC, bool, O>;
#[doc = "Field `INTknTXFEmp` reader - IN Token Received When TxFIFO is Empty"]
pub type INTKN_TXFEMP_R = crate::BitReader<bool>;
#[doc = "Field `INTknTXFEmp` writer - IN Token Received When TxFIFO is Empty"]
pub type INTKN_TXFEMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT_SPEC, bool, O>;
#[doc = "Field `INEPNakEff` reader - IN Endpoint NAK Effective"]
pub type INEPNAK_EFF_R = crate::BitReader<bool>;
#[doc = "Field `INEPNakEff` writer - IN Endpoint NAK Effective"]
pub type INEPNAK_EFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT_SPEC, bool, O>;
#[doc = "Field `TxFEmp` reader - Transmit FIFO Empty"]
pub type TX_FEMP_R = crate::BitReader<bool>;
#[doc = "Field `BNAIntr` reader - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_R = crate::BitReader<bool>;
#[doc = "Field `BNAIntr` writer - BNA (Buffer Not Available) Interrupt"]
pub type BNAINTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT_SPEC, bool, O>;
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
    pub fn xfer_compl(&mut self) -> XFER_COMPL_W<0> {
        XFER_COMPL_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<1> {
        EPDISBLD_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<2> {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    #[must_use]
    pub fn time_out(&mut self) -> TIME_OUT_W<3> {
        TIME_OUT_W::new(self)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkn_txfemp(&mut self) -> INTKN_TXFEMP_W<4> {
        INTKN_TXFEMP_W::new(self)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    #[must_use]
    pub fn inepnak_eff(&mut self) -> INEPNAK_EFF_W<6> {
        INEPNAK_EFF_W::new(self)
    }
    #[doc = "Bit 9 - BNA (Buffer Not Available) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BNAINTR_W<9> {
        BNAINTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint](index.html) module"]
pub struct DIEPINT_SPEC;
impl crate::RegisterSpec for DIEPINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint::R](R) reader structure"]
impl crate::Readable for DIEPINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint::W](W) writer structure"]
impl crate::Writable for DIEPINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT to value 0x80"]
impl crate::Resettable for DIEPINT_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
