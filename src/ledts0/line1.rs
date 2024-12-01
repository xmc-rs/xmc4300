#[doc = "Register `LINE1` reader"]
pub type R = crate::R<LINE1_SPEC>;
#[doc = "Register `LINE1` writer"]
pub type W = crate::W<LINE1_SPEC>;
#[doc = "Field `LINE_4` reader - Output on LINE\\[x\\]"]
pub type LINE_4_R = crate::FieldReader;
#[doc = "Field `LINE_4` writer - Output on LINE\\[x\\]"]
pub type LINE_4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_5` reader - Output on LINE\\[x\\]"]
pub type LINE_5_R = crate::FieldReader;
#[doc = "Field `LINE_5` writer - Output on LINE\\[x\\]"]
pub type LINE_5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_6` reader - Output on LINE\\[x\\]"]
pub type LINE_6_R = crate::FieldReader;
#[doc = "Field `LINE_6` writer - Output on LINE\\[x\\]"]
pub type LINE_6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_A` reader - Output on LINE\\[x\\]"]
pub type LINE_A_R = crate::FieldReader;
#[doc = "Field `LINE_A` writer - Output on LINE\\[x\\]"]
pub type LINE_A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn line_4(&mut self) -> LINE_4_W<LINE1_SPEC> {
        LINE_4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_5(&mut self) -> LINE_5_W<LINE1_SPEC> {
        LINE_5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_6(&mut self) -> LINE_6_W<LINE1_SPEC> {
        LINE_6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    pub fn line_a(&mut self) -> LINE_A_W<LINE1_SPEC> {
        LINE_A_W::new(self, 24)
    }
}
#[doc = "Line Pattern Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`line1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`line1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINE1_SPEC;
impl crate::RegisterSpec for LINE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`line1::R`](R) reader structure"]
impl crate::Readable for LINE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`line1::W`](W) writer structure"]
impl crate::Writable for LINE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINE1 to value 0"]
impl crate::Resettable for LINE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
