#[doc = "Register `LINE0` reader"]
pub type R = crate::R<LINE0_SPEC>;
#[doc = "Register `LINE0` writer"]
pub type W = crate::W<LINE0_SPEC>;
#[doc = "Field `LINE_0` reader - Output on LINE\\[x\\]"]
pub type LINE_0_R = crate::FieldReader;
#[doc = "Field `LINE_0` writer - Output on LINE\\[x\\]"]
pub type LINE_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_1` reader - Output on LINE\\[x\\]"]
pub type LINE_1_R = crate::FieldReader;
#[doc = "Field `LINE_1` writer - Output on LINE\\[x\\]"]
pub type LINE_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_2` reader - Output on LINE\\[x\\]"]
pub type LINE_2_R = crate::FieldReader;
#[doc = "Field `LINE_2` writer - Output on LINE\\[x\\]"]
pub type LINE_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_3` reader - Output on LINE\\[x\\]"]
pub type LINE_3_R = crate::FieldReader;
#[doc = "Field `LINE_3` writer - Output on LINE\\[x\\]"]
pub type LINE_3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn line_0(&mut self) -> LINE_0_W<LINE0_SPEC> {
        LINE_0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_1(&mut self) -> LINE_1_W<LINE0_SPEC> {
        LINE_1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_2(&mut self) -> LINE_2_W<LINE0_SPEC> {
        LINE_2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Output on LINE\\[x\\]"]
    #[inline(always)]
    #[must_use]
    pub fn line_3(&mut self) -> LINE_3_W<LINE0_SPEC> {
        LINE_3_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Line Pattern Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`line0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`line0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINE0_SPEC;
impl crate::RegisterSpec for LINE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`line0::R`](R) reader structure"]
impl crate::Readable for LINE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`line0::W`](W) writer structure"]
impl crate::Writable for LINE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINE0 to value 0"]
impl crate::Resettable for LINE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
