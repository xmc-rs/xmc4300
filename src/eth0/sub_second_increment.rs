#[doc = "Register `SUB_SECOND_INCREMENT` reader"]
pub struct R(crate::R<SUB_SECOND_INCREMENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUB_SECOND_INCREMENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUB_SECOND_INCREMENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUB_SECOND_INCREMENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUB_SECOND_INCREMENT` writer"]
pub struct W(crate::W<SUB_SECOND_INCREMENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUB_SECOND_INCREMENT_SPEC>;
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
impl From<crate::W<SUB_SECOND_INCREMENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUB_SECOND_INCREMENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSINC` reader - Sub-second Increment Value"]
pub type SSINC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSINC` writer - Sub-second Increment Value"]
pub type SSINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUB_SECOND_INCREMENT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sub-second Increment Value"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-second Increment Value"]
    #[inline(always)]
    #[must_use]
    pub fn ssinc(&mut self) -> SSINC_W<0> {
        SSINC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-Second Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sub_second_increment](index.html) module"]
pub struct SUB_SECOND_INCREMENT_SPEC;
impl crate::RegisterSpec for SUB_SECOND_INCREMENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sub_second_increment::R](R) reader structure"]
impl crate::Readable for SUB_SECOND_INCREMENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sub_second_increment::W](W) writer structure"]
impl crate::Writable for SUB_SECOND_INCREMENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SUB_SECOND_INCREMENT to value 0"]
impl crate::Resettable for SUB_SECOND_INCREMENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
