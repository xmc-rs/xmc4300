#[doc = "Register `CHECK` reader"]
pub type R = crate::R<CHECK_SPEC>;
#[doc = "Register `CHECK` writer"]
pub type W = crate::W<CHECK_SPEC>;
#[doc = "Field `CHECK` reader - CHECK Register"]
pub type CHECK_R = crate::FieldReader<u32>;
#[doc = "Field `CHECK` writer - CHECK Register"]
pub type CHECK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CHECK Register"]
    #[inline(always)]
    pub fn check(&self) -> CHECK_R {
        CHECK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CHECK Register"]
    #[inline(always)]
    pub fn check(&mut self) -> CHECK_W<CHECK_SPEC> {
        CHECK_W::new(self, 0)
    }
}
#[doc = "CRC Check Register\n\nYou can [`read`](crate::Reg::read) this register and get [`check::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`check::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHECK_SPEC;
impl crate::RegisterSpec for CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`check::R`](R) reader structure"]
impl crate::Readable for CHECK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`check::W`](W) writer structure"]
impl crate::Writable for CHECK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHECK to value 0"]
impl crate::Resettable for CHECK_SPEC {
    const RESET_VALUE: u32 = 0;
}
