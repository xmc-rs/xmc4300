#[doc = "Register `DIEPTXF1` reader"]
pub struct R(crate::R<DIEPTXF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF1` writer"]
pub struct W(crate::W<DIEPTXF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF1_SPEC>;
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
impl From<crate::W<DIEPTXF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPnTxFStAddr` reader - IN Endpoint FIFOn Transmit RAM Start Address"]
pub type INEPN_TX_FST_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPnTxFStAddr` writer - IN Endpoint FIFOn Transmit RAM Start Address"]
pub type INEPN_TX_FST_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF1_SPEC, u16, u16, 16, O>;
#[doc = "Field `INEPnTxFDep` reader - IN Endpoint TxFIFO Depth"]
pub type INEPN_TX_FDEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPnTxFDep` writer - IN Endpoint TxFIFO Depth"]
pub type INEPN_TX_FDEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepn_tx_fst_addr(&self) -> INEPN_TX_FST_ADDR_R {
        INEPN_TX_FST_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepn_tx_fdep(&self) -> INEPN_TX_FDEP_R {
        INEPN_TX_FDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn inepn_tx_fst_addr(&mut self) -> INEPN_TX_FST_ADDR_W<0> {
        INEPN_TX_FST_ADDR_W::new(self)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn inepn_tx_fdep(&mut self) -> INEPN_TX_FDEP_W<16> {
        INEPN_TX_FDEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf1](index.html) module"]
pub struct DIEPTXF1_SPEC;
impl crate::RegisterSpec for DIEPTXF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf1::R](R) reader structure"]
impl crate::Readable for DIEPTXF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf1::W](W) writer structure"]
impl crate::Writable for DIEPTXF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF1 to value 0x0100_012a"]
impl crate::Resettable for DIEPTXF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_012a;
}
