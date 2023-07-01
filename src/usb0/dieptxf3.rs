#[doc = "Register `DIEPTXF3` reader"]
pub struct R(crate::R<DIEPTXF3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF3` writer"]
pub struct W(crate::W<DIEPTXF3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF3_SPEC>;
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
impl From<crate::W<DIEPTXF3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPnTxFStAddr` reader - IN Endpoint FIFOn Transmit RAM Start Address"]
pub type INEPN_TX_FST_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPnTxFStAddr` writer - IN Endpoint FIFOn Transmit RAM Start Address"]
pub type INEPN_TX_FST_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTXF3_SPEC, 16, O, u16>;
#[doc = "Field `INEPnTxFDep` reader - IN Endpoint TxFIFO Depth"]
pub type INEPN_TX_FDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPnTxFDep` writer - IN Endpoint TxFIFO Depth"]
pub type INEPN_TX_FDEP_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTXF3_SPEC, 16, O, u16>;
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
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf3](index.html) module"]
pub struct DIEPTXF3_SPEC;
impl crate::RegisterSpec for DIEPTXF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf3::R](R) reader structure"]
impl crate::Readable for DIEPTXF3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf3::W](W) writer structure"]
impl crate::Writable for DIEPTXF3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF3 to value 0x0100_032a"]
impl crate::Resettable for DIEPTXF3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_032a;
}
