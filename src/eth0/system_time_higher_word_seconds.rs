#[doc = "Register `SYSTEM_TIME_HIGHER_WORD_SECONDS` reader"]
pub type R = crate::R<SystemTimeHigherWordSecondsSpec>;
#[doc = "Register `SYSTEM_TIME_HIGHER_WORD_SECONDS` writer"]
pub type W = crate::W<SystemTimeHigherWordSecondsSpec>;
#[doc = "Field `TSHWR` reader - Timestamp Higher Word Register"]
pub type TshwrR = crate::FieldReader<u16>;
#[doc = "Field `TSHWR` writer - Timestamp Higher Word Register"]
pub type TshwrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register"]
    #[inline(always)]
    pub fn tshwr(&self) -> TshwrR {
        TshwrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register"]
    #[inline(always)]
    #[must_use]
    pub fn tshwr(&mut self) -> TshwrW<SystemTimeHigherWordSecondsSpec> {
        TshwrW::new(self, 0)
    }
}
#[doc = "System Time - Higher Word Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_higher_word_seconds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_time_higher_word_seconds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemTimeHigherWordSecondsSpec;
impl crate::RegisterSpec for SystemTimeHigherWordSecondsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_time_higher_word_seconds::R`](R) reader structure"]
impl crate::Readable for SystemTimeHigherWordSecondsSpec {}
#[doc = "`write(|w| ..)` method takes [`system_time_higher_word_seconds::W`](W) writer structure"]
impl crate::Writable for SystemTimeHigherWordSecondsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTEM_TIME_HIGHER_WORD_SECONDS to value 0"]
impl crate::Resettable for SystemTimeHigherWordSecondsSpec {
    const RESET_VALUE: u32 = 0;
}
