#[doc = "Register `SYSTEM_TIME_HIGHER_WORD_SECONDS` reader"]
pub type R = crate::R<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>;
#[doc = "Register `SYSTEM_TIME_HIGHER_WORD_SECONDS` writer"]
pub type W = crate::W<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>;
#[doc = "Field `TSHWR` reader - Timestamp Higher Word Register"]
pub type TSHWR_R = crate::FieldReader<u16>;
#[doc = "Field `TSHWR` writer - Timestamp Higher Word Register"]
pub type TSHWR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register"]
    #[inline(always)]
    pub fn tshwr(&self) -> TSHWR_R {
        TSHWR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register"]
    #[inline(always)]
    #[must_use]
    pub fn tshwr(&mut self) -> TSHWR_W<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC> {
        TSHWR_W::new(self, 0)
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
#[doc = "System Time - Higher Word Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_higher_word_seconds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_time_higher_word_seconds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC;
impl crate::RegisterSpec for SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_time_higher_word_seconds::R`](R) reader structure"]
impl crate::Readable for SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`system_time_higher_word_seconds::W`](W) writer structure"]
impl crate::Writable for SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSTEM_TIME_HIGHER_WORD_SECONDS to value 0"]
impl crate::Resettable for SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
