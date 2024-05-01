#[doc = "Register `GPR1` reader"]
pub type R = crate::R<Gpr1Spec>;
#[doc = "Register `GPR1` writer"]
pub type W = crate::W<Gpr1Spec>;
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
    pub fn dat(&mut self) -> DatW<Gpr1Spec> {
        DatW::new(self, 0)
    }
}
#[doc = "General Purpose Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpr1Spec;
impl crate::RegisterSpec for Gpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpr1::R`](R) reader structure"]
impl crate::Readable for Gpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpr1::W`](W) writer structure"]
impl crate::Writable for Gpr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPR1 to value 0"]
impl crate::Resettable for Gpr1Spec {
    const RESET_VALUE: u32 = 0;
}
