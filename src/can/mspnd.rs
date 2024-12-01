#[doc = "Register `MSPND[%s]` reader"]
pub type R = crate::R<MSPND_SPEC>;
#[doc = "Register `MSPND[%s]` writer"]
pub type W = crate::W<MSPND_SPEC>;
#[doc = "Field `PND` reader - Message Pending"]
pub type PND_R = crate::FieldReader<u32>;
#[doc = "Field `PND` writer - Message Pending"]
pub type PND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Pending"]
    #[inline(always)]
    pub fn pnd(&self) -> PND_R {
        PND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Pending"]
    #[inline(always)]
    pub fn pnd(&mut self) -> PND_W<MSPND_SPEC> {
        PND_W::new(self, 0)
    }
}
#[doc = "Message Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSPND_SPEC;
impl crate::RegisterSpec for MSPND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspnd::R`](R) reader structure"]
impl crate::Readable for MSPND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mspnd::W`](W) writer structure"]
impl crate::Writable for MSPND_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSPND[%s]
to value 0"]
impl crate::Resettable for MSPND_SPEC {
    const RESET_VALUE: u32 = 0;
}
