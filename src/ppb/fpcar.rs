#[doc = "Register `FPCAR` reader"]
pub type R = crate::R<FPCAR_SPEC>;
#[doc = "Register `FPCAR` writer"]
pub type W = crate::W<FPCAR_SPEC>;
#[doc = "Field `ADDRESS` reader - Address"]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - Address"]
pub type ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 29, O, u32>;
impl R {
    #[doc = "Bits 3:31 - Address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<FPCAR_SPEC, 3> {
        ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Floating-point Context Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpcar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpcar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPCAR_SPEC;
impl crate::RegisterSpec for FPCAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpcar::R`](R) reader structure"]
impl crate::Readable for FPCAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpcar::W`](W) writer structure"]
impl crate::Writable for FPCAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPCAR to value 0"]
impl crate::Resettable for FPCAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
