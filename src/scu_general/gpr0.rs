#[doc = "Register `GPR0` reader"]
pub type R = crate::R<GPR0_SPEC>;
#[doc = "Register `GPR0` writer"]
pub type W = crate::W<GPR0_SPEC>;
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
    pub fn dat(&mut self) -> DAT_W<GPR0_SPEC> {
        DAT_W::new(self, 0)
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
#[doc = "General Purpose Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPR0_SPEC;
impl crate::RegisterSpec for GPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpr0::R`](R) reader structure"]
impl crate::Readable for GPR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpr0::W`](W) writer structure"]
impl crate::Writable for GPR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR0 to value 0"]
impl crate::Resettable for GPR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
