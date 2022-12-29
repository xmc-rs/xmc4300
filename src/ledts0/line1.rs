#[doc = "Register `LINE1` reader"]
pub struct R(crate::R<LINE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINE1` writer"]
pub struct W(crate::W<LINE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINE1_SPEC>;
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
impl From<crate::W<LINE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE_4` reader - Output on LINE\\[x\\]"]
pub type LINE_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_4` writer - Output on LINE\\[x\\]"]
pub type LINE_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LINE1_SPEC, u8, u8, 8, O>;
#[doc = "Field `LINE_5` reader - Output on LINE\\[x\\]"]
pub type LINE_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_5` writer - Output on LINE\\[x\\]"]
pub type LINE_5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LINE1_SPEC, u8, u8, 8, O>;
#[doc = "Field `LINE_6` reader - Output on LINE\\[x\\]"]
pub type LINE_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_6` writer - Output on LINE\\[x\\]"]
pub type LINE_6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LINE1_SPEC, u8, u8, 8, O>;
#[doc = "Field `LINE_A` reader - Output on LINE\\[x\\]"]
pub type LINE_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_A` writer - Output on LINE\\[x\\]"]
pub type LINE_A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LINE1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_4(&self) -> LINE_4_R {
        LINE_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_5(&self) -> LINE_5_R {
        LINE_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_6(&self) -> LINE_6_R {
        LINE_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_a(&self) -> LINE_A_R {
        LINE_A_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_4(&mut self) -> LINE_4_W<0> {
        LINE_4_W::new(self)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_5(&mut self) -> LINE_5_W<8> {
        LINE_5_W::new(self)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_6(&mut self) -> LINE_6_W<16> {
        LINE_6_W::new(self)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_a(&mut self) -> LINE_A_W<24> {
        LINE_A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Pattern Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [line1](index.html) module"]
pub struct LINE1_SPEC;
impl crate::RegisterSpec for LINE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [line1::R](R) reader structure"]
impl crate::Readable for LINE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [line1::W](W) writer structure"]
impl crate::Writable for LINE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINE1 to value 0"]
impl crate::Resettable for LINE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
