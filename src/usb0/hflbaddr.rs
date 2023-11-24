#[doc = "Register `HFLBADDR` reader"]
pub type R = crate::R<HFLBADDR_SPEC>;
#[doc = "Register `HFLBADDR` writer"]
pub type W = crate::W<HFLBADDR_SPEC>;
#[doc = "Field `Starting_Address` reader - Starting Address"]
pub type STARTING_ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `Starting_Address` writer - Starting Address"]
pub type STARTING_ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Starting Address"]
    #[inline(always)]
    pub fn starting_address(&self) -> STARTING_ADDRESS_R {
        STARTING_ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Starting Address"]
    #[inline(always)]
    #[must_use]
    pub fn starting_address(&mut self) -> STARTING_ADDRESS_W<HFLBADDR_SPEC> {
        STARTING_ADDRESS_W::new(self, 0)
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
#[doc = "Host Frame List Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hflbaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hflbaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFLBADDR_SPEC;
impl crate::RegisterSpec for HFLBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hflbaddr::R`](R) reader structure"]
impl crate::Readable for HFLBADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hflbaddr::W`](W) writer structure"]
impl crate::Writable for HFLBADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFLBADDR to value 0"]
impl crate::Resettable for HFLBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
