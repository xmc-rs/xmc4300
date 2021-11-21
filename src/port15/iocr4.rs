#[doc = "Register `IOCR4` reader"]
pub struct R(crate::R<IOCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR4` writer"]
pub struct W(crate::W<IOCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR4_SPEC>;
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
impl From<crate::W<IOCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC4` reader - Port Control for Port n Pin 4 to 7"]
pub struct PC4_R(crate::FieldReader<u8, u8>);
impl PC4_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC4` writer - Port Control for Port n Pin 4 to 7"]
pub struct PC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `PC5` reader - Port Control for Port n Pin 4 to 7"]
pub struct PC5_R(crate::FieldReader<u8, u8>);
impl PC5_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC5` writer - Port Control for Port n Pin 4 to 7"]
pub struct PC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `PC6` reader - Port Control for Port n Pin 4 to 7"]
pub struct PC6_R(crate::FieldReader<u8, u8>);
impl PC6_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC6` writer - Port Control for Port n Pin 4 to 7"]
pub struct PC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `PC7` reader - Port Control for Port n Pin 4 to 7"]
pub struct PC7_R(crate::FieldReader<u8, u8>);
impl PC7_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC7` writer - Port Control for Port n Pin 4 to 7"]
pub struct PC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&self) -> PC4_R {
        PC4_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&self) -> PC5_R {
        PC5_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&mut self) -> PC4_W {
        PC4_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&mut self) -> PC5_W {
        PC5_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&mut self) -> PC6_W {
        PC6_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&mut self) -> PC7_W {
        PC7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 15 Input/Output Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr4](index.html) module"]
pub struct IOCR4_SPEC;
impl crate::RegisterSpec for IOCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr4::R](R) reader structure"]
impl crate::Readable for IOCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr4::W](W) writer structure"]
impl crate::Writable for IOCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCR4 to value 0"]
impl crate::Resettable for IOCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
