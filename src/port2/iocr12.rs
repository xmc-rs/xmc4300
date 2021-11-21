#[doc = "Register `IOCR12` reader"]
pub struct R(crate::R<IOCR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR12` writer"]
pub struct W(crate::W<IOCR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR12_SPEC>;
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
impl From<crate::W<IOCR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC12` reader - Port Control for Port n Pin 12 to 15"]
pub struct PC12_R(crate::FieldReader<u8, u8>);
impl PC12_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC12` writer - Port Control for Port n Pin 12 to 15"]
pub struct PC12_W<'a> {
    w: &'a mut W,
}
impl<'a> PC12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `PC13` reader - Port Control for Port n Pin 12 to 15"]
pub struct PC13_R(crate::FieldReader<u8, u8>);
impl PC13_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC13` writer - Port Control for Port n Pin 12 to 15"]
pub struct PC13_W<'a> {
    w: &'a mut W,
}
impl<'a> PC13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `PC14` reader - Port Control for Port n Pin 12 to 15"]
pub struct PC14_R(crate::FieldReader<u8, u8>);
impl PC14_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC14` writer - Port Control for Port n Pin 12 to 15"]
pub struct PC14_W<'a> {
    w: &'a mut W,
}
impl<'a> PC14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `PC15` reader - Port Control for Port n Pin 12 to 15"]
pub struct PC15_R(crate::FieldReader<u8, u8>);
impl PC15_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC15` writer - Port Control for Port n Pin 12 to 15"]
pub struct PC15_W<'a> {
    w: &'a mut W,
}
impl<'a> PC15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&self) -> PC12_R {
        PC12_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&self) -> PC13_R {
        PC13_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&self) -> PC14_R {
        PC14_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&self) -> PC15_R {
        PC15_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&mut self) -> PC12_W {
        PC12_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&mut self) -> PC13_W {
        PC13_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&mut self) -> PC14_W {
        PC14_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&mut self) -> PC15_W {
        PC15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Input/Output Control Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr12](index.html) module"]
pub struct IOCR12_SPEC;
impl crate::RegisterSpec for IOCR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr12::R](R) reader structure"]
impl crate::Readable for IOCR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr12::W](W) writer structure"]
impl crate::Writable for IOCR12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCR12 to value 0"]
impl crate::Resettable for IOCR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
