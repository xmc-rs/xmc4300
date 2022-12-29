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
pub type PC4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC4` writer - Port Control for Port n Pin 4 to 7"]
pub type PC4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `PC5` reader - Port Control for Port n Pin 4 to 7"]
pub type PC5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC5` writer - Port Control for Port n Pin 4 to 7"]
pub type PC5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `PC6` reader - Port Control for Port n Pin 4 to 7"]
pub type PC6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC6` writer - Port Control for Port n Pin 4 to 7"]
pub type PC6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `PC7` reader - Port Control for Port n Pin 4 to 7"]
pub type PC7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC7` writer - Port Control for Port n Pin 4 to 7"]
pub type PC7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR4_SPEC, u8, u8, 5, O>;
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
    #[must_use]
    pub fn pc4(&mut self) -> PC4_W<3> {
        PC4_W::new(self)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc5(&mut self) -> PC5_W<11> {
        PC5_W::new(self)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc6(&mut self) -> PC6_W<19> {
        PC6_W::new(self)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc7(&mut self) -> PC7_W<27> {
        PC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 14 Input/Output Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr4](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCR4 to value 0"]
impl crate::Resettable for IOCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
