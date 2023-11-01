#[doc = "Register `AL_STATUS_CODE` reader"]
pub type R = crate::R<AL_STATUS_CODE_SPEC>;
#[doc = "Register `AL_STATUS_CODE` writer"]
pub type W = crate::W<AL_STATUS_CODE_SPEC>;
#[doc = "Field `AL_S_CODE` reader - AL Status Code"]
pub type AL_S_CODE_R = crate::FieldReader<u16>;
#[doc = "Field `AL_S_CODE` writer - AL Status Code"]
pub type AL_S_CODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - AL Status Code"]
    #[inline(always)]
    pub fn al_s_code(&self) -> AL_S_CODE_R {
        AL_S_CODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - AL Status Code"]
    #[inline(always)]
    #[must_use]
    pub fn al_s_code(&mut self) -> AL_S_CODE_W<AL_STATUS_CODE_SPEC, 0> {
        AL_S_CODE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AL Status Code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_status_code::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_status_code::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AL_STATUS_CODE_SPEC;
impl crate::RegisterSpec for AL_STATUS_CODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`al_status_code::R`](R) reader structure"]
impl crate::Readable for AL_STATUS_CODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`al_status_code::W`](W) writer structure"]
impl crate::Writable for AL_STATUS_CODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AL_STATUS_CODE to value 0"]
impl crate::Resettable for AL_STATUS_CODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
