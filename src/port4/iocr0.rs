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
pub type PC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC0` writer - Port Control for Port n Pin 0 to 3"]
pub type PC0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `PC1` reader - Port Control for Port n Pin 0 to 3"]
pub type PC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC1` writer - Port Control for Port n Pin 0 to 3"]
pub type PC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `PC2` reader - Port Control for Port n Pin 0 to 3"]
pub type PC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC2` writer - Port Control for Port n Pin 0 to 3"]
pub type PC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `PC3` reader - Port Control for Port n Pin 0 to 3"]
pub type PC3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC3` writer - Port Control for Port n Pin 0 to 3"]
pub type PC3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR0_SPEC, u8, u8, 5, O>;
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
    #[must_use]
    pub fn pc0(&mut self) -> PC0_W<3> {
        PC0_W::new(self)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc1(&mut self) -> PC1_W<11> {
        PC1_W::new(self)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc2(&mut self) -> PC2_W<19> {
        PC2_W::new(self)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc3(&mut self) -> PC3_W<27> {
        PC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 4 Input/Output Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr0](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCR0 to value 0"]
impl crate::Resettable for IOCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
