#[doc = "Register `MSIMASK` reader"]
pub type R = crate::R<MSIMASK_SPEC>;
#[doc = "Register `MSIMASK` writer"]
pub type W = crate::W<MSIMASK_SPEC>;
#[doc = "Field `IM` reader - Message Index Mask"]
pub type IM_R = crate::FieldReader<u32>;
#[doc = "Field `IM` writer - Message Index Mask"]
pub type IM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Index Mask"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Index Mask"]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> IM_W<MSIMASK_SPEC> {
        IM_W::new(self, 0)
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
#[doc = "Message Index Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msimask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msimask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSIMASK_SPEC;
impl crate::RegisterSpec for MSIMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msimask::R`](R) reader structure"]
impl crate::Readable for MSIMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msimask::W`](W) writer structure"]
impl crate::Writable for MSIMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSIMASK to value 0"]
impl crate::Resettable for MSIMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
