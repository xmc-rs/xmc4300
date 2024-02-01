#[doc = "Register `ARGUMENT1` reader"]
pub type R = crate::R<ARGUMENT1_SPEC>;
#[doc = "Register `ARGUMENT1` writer"]
pub type W = crate::W<ARGUMENT1_SPEC>;
#[doc = "Field `ARGUMENT1` reader - Command Argument"]
pub type ARGUMENT1_R = crate::FieldReader<u32>;
#[doc = "Field `ARGUMENT1` writer - Command Argument"]
pub type ARGUMENT1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn argument1(&self) -> ARGUMENT1_R {
        ARGUMENT1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    #[must_use]
    pub fn argument1(&mut self) -> ARGUMENT1_W<ARGUMENT1_SPEC> {
        ARGUMENT1_W::new(self, 0)
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
#[doc = "Argument1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argument1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argument1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARGUMENT1_SPEC;
impl crate::RegisterSpec for ARGUMENT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argument1::R`](R) reader structure"]
impl crate::Readable for ARGUMENT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`argument1::W`](W) writer structure"]
impl crate::Writable for ARGUMENT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARGUMENT1 to value 0"]
impl crate::Resettable for ARGUMENT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
