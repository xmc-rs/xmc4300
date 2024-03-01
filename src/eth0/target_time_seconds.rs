#[doc = "Register `TARGET_TIME_SECONDS` reader"]
pub type R = crate::R<TargetTimeSecondsSpec>;
#[doc = "Register `TARGET_TIME_SECONDS` writer"]
pub type W = crate::W<TargetTimeSecondsSpec>;
#[doc = "Field `TSTR` reader - Target Time Seconds Register"]
pub type TstrR = crate::FieldReader<u32>;
#[doc = "Field `TSTR` writer - Target Time Seconds Register"]
pub type TstrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstr(&self) -> TstrR {
        TstrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    #[must_use]
    pub fn tstr(&mut self) -> TstrW<TargetTimeSecondsSpec> {
        TstrW::new(self, 0)
    }
}
#[doc = "Target Time Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_time_seconds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_time_seconds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TargetTimeSecondsSpec;
impl crate::RegisterSpec for TargetTimeSecondsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_time_seconds::R`](R) reader structure"]
impl crate::Readable for TargetTimeSecondsSpec {}
#[doc = "`write(|w| ..)` method takes [`target_time_seconds::W`](W) writer structure"]
impl crate::Writable for TargetTimeSecondsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARGET_TIME_SECONDS to value 0"]
impl crate::Resettable for TargetTimeSecondsSpec {
    const RESET_VALUE: u32 = 0;
}
