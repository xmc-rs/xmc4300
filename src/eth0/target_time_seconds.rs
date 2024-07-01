#[doc = "Register `TARGET_TIME_SECONDS` reader"]
pub type R = crate::R<TARGET_TIME_SECONDS_SPEC>;
#[doc = "Register `TARGET_TIME_SECONDS` writer"]
pub type W = crate::W<TARGET_TIME_SECONDS_SPEC>;
#[doc = "Field `TSTR` reader - Target Time Seconds Register"]
pub type TSTR_R = crate::FieldReader<u32>;
#[doc = "Field `TSTR` writer - Target Time Seconds Register"]
pub type TSTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstr(&self) -> TSTR_R {
        TSTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    #[must_use]
    pub fn tstr(&mut self) -> TSTR_W<TARGET_TIME_SECONDS_SPEC> {
        TSTR_W::new(self, 0)
    }
}
#[doc = "Target Time Seconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`target_time_seconds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target_time_seconds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET_TIME_SECONDS_SPEC;
impl crate::RegisterSpec for TARGET_TIME_SECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_time_seconds::R`](R) reader structure"]
impl crate::Readable for TARGET_TIME_SECONDS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target_time_seconds::W`](W) writer structure"]
impl crate::Writable for TARGET_TIME_SECONDS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARGET_TIME_SECONDS to value 0"]
impl crate::Resettable for TARGET_TIME_SECONDS_SPEC {
    const RESET_VALUE: u32 = 0;
}
