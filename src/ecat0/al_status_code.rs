#[doc = "Register `AL_STATUS_CODE` reader"]
pub type R = crate::R<AlStatusCodeSpec>;
#[doc = "Register `AL_STATUS_CODE` writer"]
pub type W = crate::W<AlStatusCodeSpec>;
#[doc = "Field `AL_S_CODE` reader - AL Status Code"]
pub type AlSCodeR = crate::FieldReader<u16>;
#[doc = "Field `AL_S_CODE` writer - AL Status Code"]
pub type AlSCodeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - AL Status Code"]
    #[inline(always)]
    pub fn al_s_code(&self) -> AlSCodeR {
        AlSCodeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - AL Status Code"]
    #[inline(always)]
    #[must_use]
    pub fn al_s_code(&mut self) -> AlSCodeW<AlStatusCodeSpec> {
        AlSCodeW::new(self, 0)
    }
}
#[doc = "AL Status Code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_status_code::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_status_code::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlStatusCodeSpec;
impl crate::RegisterSpec for AlStatusCodeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`al_status_code::R`](R) reader structure"]
impl crate::Readable for AlStatusCodeSpec {}
#[doc = "`write(|w| ..)` method takes [`al_status_code::W`](W) writer structure"]
impl crate::Writable for AlStatusCodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets AL_STATUS_CODE to value 0"]
impl crate::Resettable for AlStatusCodeSpec {
    const RESET_VALUE: u16 = 0;
}
