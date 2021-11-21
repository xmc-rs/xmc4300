#[doc = "Register `IOCR0` reader"]
pub struct R(crate::R<IOCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR0` writer"]
pub struct W(crate::W<IOCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR0_SPEC>;
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
impl From<crate::W<IOCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC0` reader - Port Control for Port n Pin 0 to 3"]
pub struct PC0_R(crate::FieldReader<u8, u8>);
impl PC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC0` writer - Port Control for Port n Pin 0 to 3"]
pub struct PC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `PC1` reader - Port Control for Port n Pin 0 to 3"]
pub struct PC1_R(crate::FieldReader<u8, u8>);
impl PC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC1` writer - Port Control for Port n Pin 0 to 3"]
pub struct PC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `PC2` reader - Port Control for Port n Pin 0 to 3"]
pub struct PC2_R(crate::FieldReader<u8, u8>);
impl PC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC2` writer - Port Control for Port n Pin 0 to 3"]
pub struct PC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `PC3` reader - Port Control for Port n Pin 0 to 3"]
pub struct PC3_R(crate::FieldReader<u8, u8>);
impl PC3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC3` writer - Port Control for Port n Pin 0 to 3"]
pub struct PC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&self) -> PC0_R {
        PC0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&self) -> PC1_R {
        PC1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&self) -> PC2_R {
        PC2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&self) -> PC3_R {
        PC3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&mut self) -> PC0_W {
        PC0_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&mut self) -> PC1_W {
        PC1_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&mut self) -> PC2_W {
        PC2_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&mut self) -> PC3_W {
        PC3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 14 Input/Output Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr0](index.html) module"]
pub struct IOCR0_SPEC;
impl crate::RegisterSpec for IOCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr0::R](R) reader structure"]
impl crate::Readable for IOCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr0::W](W) writer structure"]
impl crate::Writable for IOCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCR0 to value 0"]
impl crate::Resettable for IOCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
