#[doc = "Register `DSR` reader"]
pub struct R(crate::R<DSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSR` writer"]
pub struct W(crate::W<DSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSR_SPEC>;
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
impl From<crate::W<DSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSC` reader - Destination scatter count"]
pub struct DSC_R(crate::FieldReader<u16, u16>);
impl DSC_R {
    pub(crate) fn new(bits: u16) -> Self {
        DSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSC` writer - Destination scatter count"]
pub struct DSC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Field `DSI` reader - Destination scatter interval"]
pub struct DSI_R(crate::FieldReader<u32, u32>);
impl DSI_R {
    pub(crate) fn new(bits: u32) -> Self {
        DSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI` writer - Destination scatter interval"]
pub struct DSI_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    pub fn dsc(&self) -> DSC_R {
        DSC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    pub fn dsi(&self) -> DSI_R {
        DSI_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    pub fn dsc(&mut self) -> DSC_W {
        DSC_W { w: self }
    }
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    pub fn dsi(&mut self) -> DSI_W {
        DSI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination Scatter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr](index.html) module"]
pub struct DSR_SPEC;
impl crate::RegisterSpec for DSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsr::R](R) reader structure"]
impl crate::Readable for DSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsr::W](W) writer structure"]
impl crate::Writable for DSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSR to value 0"]
impl crate::Resettable for DSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
