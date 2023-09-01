#[doc = "Register `TARGET_TIME_NANOSECONDS` reader"]
pub type R = crate::R<TARGET_TIME_NANOSECONDS_SPEC>;
#[doc = "Register `TARGET_TIME_NANOSECONDS` writer"]
pub type W = crate::W<TARGET_TIME_NANOSECONDS_SPEC>;
#[doc = "Field `TTSLO` reader - Target Timestamp Low Register"]
pub type TTSLO_R = crate::FieldReader<u32>;
#[doc = "Field `TTSLO` writer - Target Timestamp Low Register"]
pub type TTSLO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
#[doc = "Field `TRGTBUSY` reader - Target Time Register Busy"]
pub type TRGTBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&self) -> TTSLO_R {
        TTSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TRGTBUSY_R {
        TRGTBUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    #[must_use]
    pub fn ttslo(&mut self) -> TTSLO_W<TARGET_TIME_NANOSECONDS_SPEC, 0> {
        TTSLO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Target Time Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_time_nanoseconds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_time_nanoseconds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET_TIME_NANOSECONDS_SPEC;
impl crate::RegisterSpec for TARGET_TIME_NANOSECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_time_nanoseconds::R`](R) reader structure"]
impl crate::Readable for TARGET_TIME_NANOSECONDS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target_time_nanoseconds::W`](W) writer structure"]
impl crate::Writable for TARGET_TIME_NANOSECONDS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET_TIME_NANOSECONDS to value 0"]
impl crate::Resettable for TARGET_TIME_NANOSECONDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
