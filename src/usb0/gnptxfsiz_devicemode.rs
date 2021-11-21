#[doc = "Register `GNPTXFSIZ_DEVICEMODE` reader"]
pub struct R(crate::R<GNPTXFSIZ_DEVICEMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GNPTXFSIZ_DEVICEMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GNPTXFSIZ_DEVICEMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GNPTXFSIZ_DEVICEMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GNPTXFSIZ_DEVICEMODE` writer"]
pub struct W(crate::W<GNPTXFSIZ_DEVICEMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GNPTXFSIZ_DEVICEMODE_SPEC>;
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
impl From<crate::W<GNPTXFSIZ_DEVICEMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GNPTXFSIZ_DEVICEMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTxF0StAddr` reader - IN Endpoint FIFO0 Transmit RAM Start Address"]
pub struct INEPTXF0STADDR_R(crate::FieldReader<u16, u16>);
impl INEPTXF0STADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPTXF0STADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPTXF0STADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPTxF0StAddr` writer - IN Endpoint FIFO0 Transmit RAM Start Address"]
pub struct INEPTXF0STADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXF0STADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `INEPTxF0Dep` reader - IN Endpoint TxFIFO 0 Depth"]
pub struct INEPTXF0DEP_R(crate::FieldReader<u16, u16>);
impl INEPTXF0DEP_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPTXF0DEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPTXF0DEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPTxF0Dep` writer - IN Endpoint TxFIFO 0 Depth"]
pub struct INEPTXF0DEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXF0DEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn ineptx_f0st_addr(&self) -> INEPTXF0STADDR_R {
        INEPTXF0STADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn ineptx_f0dep(&self) -> INEPTXF0DEP_R {
        INEPTXF0DEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn ineptx_f0st_addr(&mut self) -> INEPTXF0STADDR_W {
        INEPTXF0STADDR_W { w: self }
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn ineptx_f0dep(&mut self) -> INEPTXF0DEP_W {
        INEPTXF0DEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxfsiz_devicemode](index.html) module"]
pub struct GNPTXFSIZ_DEVICEMODE_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_DEVICEMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gnptxfsiz_devicemode::R](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_DEVICEMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz_devicemode::W](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_DEVICEMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GNPTXFSIZ_DEVICEMODE to value 0x0010_0000"]
impl crate::Resettable for GNPTXFSIZ_DEVICEMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0000
    }
}
