#[doc = "Register `HPTXFSIZ` reader"]
pub struct R(crate::R<HPTXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPTXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPTXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPTXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPTXFSIZ` writer"]
pub struct W(crate::W<HPTXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPTXFSIZ_SPEC>;
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
impl From<crate::W<HPTXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPTXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTxFStAddr` reader - Host Periodic TxFIFO Start Address"]
pub type PTX_FST_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTxFStAddr` writer - Host Periodic TxFIFO Start Address"]
pub type PTX_FST_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPTXFSIZ_SPEC, u16, u16, 16, O>;
#[doc = "Field `PTxFSize` reader - Host Periodic TxFIFO Depth"]
pub type PTX_FSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTxFSize` writer - Host Periodic TxFIFO Depth"]
pub type PTX_FSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPTXFSIZ_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptx_fst_addr(&self) -> PTX_FST_ADDR_R {
        PTX_FST_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptx_fsize(&self) -> PTX_FSIZE_R {
        PTX_FSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_fst_addr(&mut self) -> PTX_FST_ADDR_W<0> {
        PTX_FST_ADDR_W::new(self)
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_fsize(&mut self) -> PTX_FSIZE_W<16> {
        PTX_FSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Periodic Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptxfsiz](index.html) module"]
pub struct HPTXFSIZ_SPEC;
impl crate::RegisterSpec for HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hptxfsiz::R](R) reader structure"]
impl crate::Readable for HPTXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hptxfsiz::W](W) writer structure"]
impl crate::Writable for HPTXFSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0100_012a"]
impl crate::Resettable for HPTXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_012a;
}
