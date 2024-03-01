#[doc = "Register `LINE0` reader"]
pub type R = crate::R<Line0Spec>;
#[doc = "Register `LINE0` writer"]
pub type W = crate::W<Line0Spec>;
#[doc = "Field `LINE_0` reader - Output on LINE\\[x\\]"]
pub type Line0R = crate::FieldReader;
#[doc = "Field `LINE_0` writer - Output on LINE\\[x\\]"]
pub type Line0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_1` reader - Output on LINE\\[x\\]"]
pub type Line1R = crate::FieldReader;
#[doc = "Field `LINE_1` writer - Output on LINE\\[x\\]"]
pub type Line1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_2` reader - Output on LINE\\[x\\]"]
pub type Line2R = crate::FieldReader;
#[doc = "Field `LINE_2` writer - Output on LINE\\[x\\]"]
pub type Line2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_3` reader - Output on LINE\\[x\\]"]
pub type Line3R = crate::FieldReader;
#[doc = "Field `LINE_3` writer - Output on LINE\\[x\\]"]
pub type Line3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_0(&self) -> Line0R {
        Line0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_1(&self) -> Line1R {
        Line1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_2(&self) -> Line2R {
        Line2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_3(&self) -> Line3R {
        Line3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_0(&mut self) -> Line0W<Line0Spec> {
        Line0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_1(&mut self) -> Line1W<Line0Spec> {
        Line1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_2(&mut self) -> Line2W<Line0Spec> {
        Line2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_3(&mut self) -> Line3W<Line0Spec> {
        Line3W::new(self, 24)
    }
}
#[doc = "Line Pattern Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`line0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`line0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Line0Spec;
impl crate::RegisterSpec for Line0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`line0::R`](R) reader structure"]
impl crate::Readable for Line0Spec {}
#[doc = "`write(|w| ..)` method takes [`line0::W`](W) writer structure"]
impl crate::Writable for Line0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINE0 to value 0"]
impl crate::Resettable for Line0Spec {
    const RESET_VALUE: u32 = 0;
}
