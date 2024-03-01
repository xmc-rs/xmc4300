#[doc = "Register `GPR0` reader"]
pub type R = crate::R<Gpr0Spec>;
#[doc = "Register `GPR0` writer"]
pub type W = crate::W<Gpr0Spec>;
#[doc = "Field `DAT` reader - User Data"]
pub type DatR = crate::FieldReader<u32>;
#[doc = "Field `DAT` writer - User Data"]
pub type DatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn dat(&self) -> DatR {
        DatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DatW<Gpr0Spec> {
        DatW::new(self, 0)
    }
}
#[doc = "General Purpose Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpr0Spec;
impl crate::RegisterSpec for Gpr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpr0::R`](R) reader structure"]
impl crate::Readable for Gpr0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpr0::W`](W) writer structure"]
impl crate::Writable for Gpr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPR0 to value 0"]
impl crate::Resettable for Gpr0Spec {
    const RESET_VALUE: u32 = 0;
}
