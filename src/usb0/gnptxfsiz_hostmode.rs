#[doc = "Register `GNPTXFSIZ_HOSTMODE` reader"]
pub struct R(crate::R<GNPTXFSIZ_HOSTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GNPTXFSIZ_HOSTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GNPTXFSIZ_HOSTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GNPTXFSIZ_HOSTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GNPTXFSIZ_HOSTMODE` writer"]
pub struct W(crate::W<GNPTXFSIZ_HOSTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GNPTXFSIZ_HOSTMODE_SPEC>;
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
impl From<crate::W<GNPTXFSIZ_HOSTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GNPTXFSIZ_HOSTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NPTxFStAddr` reader - Non-periodic Transmit RAM Start Address"]
pub struct NPTXFSTADDR_R(crate::FieldReader<u16, u16>);
impl NPTXFSTADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPTXFSTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFSTADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTxFStAddr` writer - Non-periodic Transmit RAM Start Address"]
pub struct NPTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `NPTxFDep` reader - Non-periodic TxFIFO Depth"]
pub struct NPTXFDEP_R(crate::FieldReader<u16, u16>);
impl NPTXFDEP_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPTXFDEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFDEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTxFDep` writer - Non-periodic TxFIFO Depth"]
pub struct NPTXFDEP_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFDEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    pub fn nptx_fst_addr(&self) -> NPTXFSTADDR_R {
        NPTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn nptx_fdep(&self) -> NPTXFDEP_R {
        NPTXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    pub fn nptx_fst_addr(&mut self) -> NPTXFSTADDR_W {
        NPTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn nptx_fdep(&mut self) -> NPTXFDEP_W {
        NPTXFDEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxfsiz_hostmode](index.html) module"]
pub struct GNPTXFSIZ_HOSTMODE_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_HOSTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gnptxfsiz_hostmode::R](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_HOSTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz_hostmode::W](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_HOSTMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GNPTXFSIZ_HOSTMODE to value 0x0010_011a"]
impl crate::Resettable for GNPTXFSIZ_HOSTMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_011a
    }
}
