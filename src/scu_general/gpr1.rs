#[doc = "Register `GPR1` reader"]
pub type R = crate::R<GPR1_SPEC>;
#[doc = "Register `GPR1` writer"]
pub type W = crate::W<GPR1_SPEC>;
#[doc = "Field `DAT` reader - User Data"]
pub type DAT_R = crate::FieldReader<u32>;
#[doc = "Field `DAT` writer - User Data"]
pub type DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<GPR1_SPEC> {
        DAT_W::new(self, 0)
    }
}
#[doc = "General Purpose Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPR1_SPEC;
impl crate::RegisterSpec for GPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpr1::R`](R) reader structure"]
impl crate::Readable for GPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpr1::W`](W) writer structure"]
impl crate::Writable for GPR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPR1 to value 0"]
impl crate::Resettable for GPR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
