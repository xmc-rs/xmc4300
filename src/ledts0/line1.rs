#[doc = "Register `LINE1` reader"]
pub type R = crate::R<Line1Spec>;
#[doc = "Register `LINE1` writer"]
pub type W = crate::W<Line1Spec>;
#[doc = "Field `LINE_4` reader - Output on LINE\\[x\\]"]
pub type Line4R = crate::FieldReader;
#[doc = "Field `LINE_4` writer - Output on LINE\\[x\\]"]
pub type Line4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_5` reader - Output on LINE\\[x\\]"]
pub type Line5R = crate::FieldReader;
#[doc = "Field `LINE_5` writer - Output on LINE\\[x\\]"]
pub type Line5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_6` reader - Output on LINE\\[x\\]"]
pub type Line6R = crate::FieldReader;
#[doc = "Field `LINE_6` writer - Output on LINE\\[x\\]"]
pub type Line6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_A` reader - Output on LINE\\[x\\]"]
pub type LineAR = crate::FieldReader;
#[doc = "Field `LINE_A` writer - Output on LINE\\[x\\]"]
pub type LineAW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_4(&self) -> Line4R {
        Line4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_5(&self) -> Line5R {
        Line5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_6(&self) -> Line6R {
        Line6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_a(&self) -> LineAR {
        LineAR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_4(&mut self) -> Line4W<Line1Spec> {
        Line4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_5(&mut self) -> Line5W<Line1Spec> {
        Line5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_6(&mut self) -> Line6W<Line1Spec> {
        Line6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_a(&mut self) -> LineAW<Line1Spec> {
        LineAW::new(self, 24)
    }
}
#[doc = "Line Pattern Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`line1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`line1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Line1Spec;
impl crate::RegisterSpec for Line1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`line1::R`](R) reader structure"]
impl crate::Readable for Line1Spec {}
#[doc = "`write(|w| ..)` method takes [`line1::W`](W) writer structure"]
impl crate::Writable for Line1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINE1 to value 0"]
impl crate::Resettable for Line1Spec {
    const RESET_VALUE: u32 = 0;
}
