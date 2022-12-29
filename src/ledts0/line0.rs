#[doc = "Register `LINE0` reader"]
pub struct R(crate::R<LINE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINE0` writer"]
pub struct W(crate::W<LINE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINE0_SPEC>;
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
impl From<crate::W<LINE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE_0` reader - Output on LINE\\[x\\]"]
pub type LINE_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_0` writer - Output on LINE\\[x\\]"]
pub type LINE_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LINE0_SPEC, u8, u8, 8, O>;
#[doc = "Field `LINE_1` reader - Output on LINE\\[x\\]"]
pub type LINE_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_1` writer - Output on LINE\\[x\\]"]
pub type LINE_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LINE0_SPEC, u8, u8, 8, O>;
#[doc = "Field `LINE_2` reader - Output on LINE\\[x\\]"]
pub type LINE_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_2` writer - Output on LINE\\[x\\]"]
pub type LINE_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LINE0_SPEC, u8, u8, 8, O>;
#[doc = "Field `LINE_3` reader - Output on LINE\\[x\\]"]
pub type LINE_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_3` writer - Output on LINE\\[x\\]"]
pub type LINE_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LINE0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_0(&self) -> LINE_0_R {
        LINE_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_1(&self) -> LINE_1_R {
        LINE_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_2(&self) -> LINE_2_R {
        LINE_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_3(&self) -> LINE_3_R {
        LINE_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_0(&mut self) -> LINE_0_W<0> {
        LINE_0_W::new(self)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_1(&mut self) -> LINE_1_W<8> {
        LINE_1_W::new(self)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_2(&mut self) -> LINE_2_W<16> {
        LINE_2_W::new(self)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_3(&mut self) -> LINE_3_W<24> {
        LINE_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Pattern Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [line0](index.html) module"]
pub struct LINE0_SPEC;
impl crate::RegisterSpec for LINE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [line0::R](R) reader structure"]
impl crate::Readable for LINE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [line0::W](W) writer structure"]
impl crate::Writable for LINE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINE0 to value 0"]
impl crate::Resettable for LINE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
