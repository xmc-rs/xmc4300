#[doc = "Register `SSTATAR` reader"]
pub type R = crate::R<SSTATAR_SPEC>;
#[doc = "Register `SSTATAR` writer"]
pub type W = crate::W<SSTATAR_SPEC>;
#[doc = "Field `SSTATAR` reader - Source Status Address"]
pub type SSTATAR_R = crate::FieldReader<u32>;
#[doc = "Field `SSTATAR` writer - Source Status Address"]
pub type SSTATAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    pub fn sstatar(&self) -> SSTATAR_R {
        SSTATAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Status Address"]
    #[inline(always)]
    #[must_use]
    pub fn sstatar(&mut self) -> SSTATAR_W<SSTATAR_SPEC, 0> {
        SSTATAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Source Status Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstatar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstatar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSTATAR_SPEC;
impl crate::RegisterSpec for SSTATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstatar::R`](R) reader structure"]
impl crate::Readable for SSTATAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sstatar::W`](W) writer structure"]
impl crate::Writable for SSTATAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSTATAR to value 0"]
impl crate::Resettable for SSTATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
