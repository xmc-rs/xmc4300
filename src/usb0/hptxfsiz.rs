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
pub struct PTXFSTADDR_R(crate::FieldReader<u16, u16>);
impl PTXFSTADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        PTXFSTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFSTADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTxFStAddr` writer - Host Periodic TxFIFO Start Address"]
pub struct PTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `PTxFSize` reader - Host Periodic TxFIFO Depth"]
pub struct PTXFSIZE_R(crate::FieldReader<u16, u16>);
impl PTXFSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        PTXFSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTxFSize` writer - Host Periodic TxFIFO Depth"]
pub struct PTXFSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptx_fst_addr(&self) -> PTXFSTADDR_R {
        PTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptx_fsize(&self) -> PTXFSIZE_R {
        PTXFSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptx_fst_addr(&mut self) -> PTXFSTADDR_W {
        PTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptx_fsize(&mut self) -> PTXFSIZE_W {
        PTXFSIZE_W { w: self }
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
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0100_012a"]
impl crate::Resettable for HPTXFSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_012a
    }
}
