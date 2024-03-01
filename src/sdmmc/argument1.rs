#[doc = "Register `ARGUMENT1` reader"]
pub type R = crate::R<Argument1Spec>;
#[doc = "Register `ARGUMENT1` writer"]
pub type W = crate::W<Argument1Spec>;
#[doc = "Field `ARGUMENT1` reader - Command Argument"]
pub type Argument1R = crate::FieldReader<u32>;
#[doc = "Field `ARGUMENT1` writer - Command Argument"]
pub type Argument1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn argument1(&self) -> Argument1R {
        Argument1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    #[must_use]
    pub fn argument1(&mut self) -> Argument1W<Argument1Spec> {
        Argument1W::new(self, 0)
    }
}
#[doc = "Argument1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argument1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argument1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Argument1Spec;
impl crate::RegisterSpec for Argument1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argument1::R`](R) reader structure"]
impl crate::Readable for Argument1Spec {}
#[doc = "`write(|w| ..)` method takes [`argument1::W`](W) writer structure"]
impl crate::Writable for Argument1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARGUMENT1 to value 0"]
impl crate::Resettable for Argument1Spec {
    const RESET_VALUE: u32 = 0;
}
