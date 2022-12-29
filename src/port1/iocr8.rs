#[doc = "Register `IOCR8` reader"]
pub struct R(crate::R<IOCR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR8` writer"]
pub struct W(crate::W<IOCR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR8_SPEC>;
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
impl From<crate::W<IOCR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC8` reader - Port Control for Port n Pin 8 to 11"]
pub type PC8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC8` writer - Port Control for Port n Pin 8 to 11"]
pub type PC8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR8_SPEC, u8, u8, 5, O>;
#[doc = "Field `PC9` reader - Port Control for Port n Pin 8 to 11"]
pub type PC9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC9` writer - Port Control for Port n Pin 8 to 11"]
pub type PC9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR8_SPEC, u8, u8, 5, O>;
#[doc = "Field `PC10` reader - Port Control for Port n Pin 8 to 11"]
pub type PC10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC10` writer - Port Control for Port n Pin 8 to 11"]
pub type PC10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR8_SPEC, u8, u8, 5, O>;
#[doc = "Field `PC11` reader - Port Control for Port n Pin 8 to 11"]
pub type PC11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC11` writer - Port Control for Port n Pin 8 to 11"]
pub type PC11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR8_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&self) -> PC8_R {
        PC8_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&self) -> PC9_R {
        PC9_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&self) -> PC10_R {
        PC10_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&self) -> PC11_R {
        PC11_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc8(&mut self) -> PC8_W<3> {
        PC8_W::new(self)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc9(&mut self) -> PC9_W<11> {
        PC9_W::new(self)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc10(&mut self) -> PC10_W<19> {
        PC10_W::new(self)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc11(&mut self) -> PC11_W<27> {
        PC11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Input/Output Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr8](index.html) module"]
pub struct IOCR8_SPEC;
impl crate::RegisterSpec for IOCR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr8::R](R) reader structure"]
impl crate::Readable for IOCR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr8::W](W) writer structure"]
impl crate::Writable for IOCR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCR8 to value 0"]
impl crate::Resettable for IOCR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
