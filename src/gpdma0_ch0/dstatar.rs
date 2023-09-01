#[doc = "Register `DSTATAR` reader"]
pub type R = crate::R<DSTATAR_SPEC>;
#[doc = "Register `DSTATAR` writer"]
pub type W = crate::W<DSTATAR_SPEC>;
#[doc = "Field `DSTATAR` reader - Destination Status Address"]
pub type DSTATAR_R = crate::FieldReader<u32>;
#[doc = "Field `DSTATAR` writer - Destination Status Address"]
pub type DSTATAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination Status Address"]
    #[inline(always)]
    pub fn dstatar(&self) -> DSTATAR_R {
        DSTATAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Status Address"]
    #[inline(always)]
    #[must_use]
    pub fn dstatar(&mut self) -> DSTATAR_W<DSTATAR_SPEC, 0> {
        DSTATAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Destination Status Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstatar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstatar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTATAR_SPEC;
impl crate::RegisterSpec for DSTATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstatar::R`](R) reader structure"]
impl crate::Readable for DSTATAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dstatar::W`](W) writer structure"]
impl crate::Writable for DSTATAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSTATAR to value 0"]
impl crate::Resettable for DSTATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
