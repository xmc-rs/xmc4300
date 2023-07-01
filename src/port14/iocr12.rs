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
pub type PC12_R = crate::FieldReader;
#[doc = "Field `PC12` writer - Port Control for Port n Pin 12 to 15"]
pub type PC12_W<'a, const O: u8> = crate::FieldWriter<'a, IOCR12_SPEC, 5, O>;
#[doc = "Field `PC13` reader - Port Control for Port n Pin 12 to 15"]
pub type PC13_R = crate::FieldReader;
#[doc = "Field `PC13` writer - Port Control for Port n Pin 12 to 15"]
pub type PC13_W<'a, const O: u8> = crate::FieldWriter<'a, IOCR12_SPEC, 5, O>;
#[doc = "Field `PC14` reader - Port Control for Port n Pin 12 to 15"]
pub type PC14_R = crate::FieldReader;
#[doc = "Field `PC14` writer - Port Control for Port n Pin 12 to 15"]
pub type PC14_W<'a, const O: u8> = crate::FieldWriter<'a, IOCR12_SPEC, 5, O>;
#[doc = "Field `PC15` reader - Port Control for Port n Pin 12 to 15"]
pub type PC15_R = crate::FieldReader;
#[doc = "Field `PC15` writer - Port Control for Port n Pin 12 to 15"]
pub type PC15_W<'a, const O: u8> = crate::FieldWriter<'a, IOCR12_SPEC, 5, O>;
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
    #[must_use]
    pub fn pc12(&mut self) -> PC12_W<3> {
        PC12_W::new(self)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc13(&mut self) -> PC13_W<11> {
        PC13_W::new(self)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc14(&mut self) -> PC14_W<19> {
        PC14_W::new(self)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc15(&mut self) -> PC15_W<27> {
        PC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 14 Input/Output Control Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr12](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCR12 to value 0"]
impl crate::Resettable for IOCR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
