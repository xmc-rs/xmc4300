#[doc = "Register `SYSTEM_TIME_SECONDS_UPDATE` reader"]
pub type R = crate::R<SystemTimeSecondsUpdateSpec>;
#[doc = "Register `SYSTEM_TIME_SECONDS_UPDATE` writer"]
pub type W = crate::W<SystemTimeSecondsUpdateSpec>;
#[doc = "Field `TSS` reader - Timestamp Second"]
pub type TssR = crate::FieldReader<u32>;
#[doc = "Field `TSS` writer - Timestamp Second"]
pub type TssW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TssW<SystemTimeSecondsUpdateSpec> {
        TssW::new(self, 0)
    }
}
#[doc = "System Time - Seconds Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_seconds_update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_time_seconds_update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemTimeSecondsUpdateSpec;
impl crate::RegisterSpec for SystemTimeSecondsUpdateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_time_seconds_update::R`](R) reader structure"]
impl crate::Readable for SystemTimeSecondsUpdateSpec {}
#[doc = "`write(|w| ..)` method takes [`system_time_seconds_update::W`](W) writer structure"]
impl crate::Writable for SystemTimeSecondsUpdateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTEM_TIME_SECONDS_UPDATE to value 0"]
impl crate::Resettable for SystemTimeSecondsUpdateSpec {
    const RESET_VALUE: u32 = 0;
}
