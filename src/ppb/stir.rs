#[doc = "Register `STIR` writer"]
pub type W = crate::W<STIR_SPEC>;
#[doc = "Field `INTID` writer - Interrupt ID of the interrupt to trigger"]
pub type INTID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl W {
    #[doc = "Bits 0:8 - Interrupt ID of the interrupt to trigger"]
    #[inline(always)]
    #[must_use]
    pub fn intid(&mut self) -> INTID_W<STIR_SPEC, 0> {
        INTID_W::new(self)
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
#[doc = "Software Trigger Interrupt Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STIR_SPEC;
impl crate::RegisterSpec for STIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`stir::W`](W) writer structure"]
impl crate::Writable for STIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIR to value 0"]
impl crate::Resettable for STIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
