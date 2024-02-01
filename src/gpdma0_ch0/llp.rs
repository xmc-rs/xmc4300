#[doc = "Register `LLP` reader"]
pub type R = crate::R<LLP_SPEC>;
#[doc = "Register `LLP` writer"]
pub type W = crate::W<LLP_SPEC>;
#[doc = "Field `LOC` reader - Starting Address In Memory"]
pub type LOC_R = crate::FieldReader<u32>;
#[doc = "Field `LOC` writer - Starting Address In Memory"]
pub type LOC_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Starting Address In Memory"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Starting Address In Memory"]
    #[inline(always)]
    #[must_use]
    pub fn loc(&mut self) -> LOC_W<LLP_SPEC> {
        LOC_W::new(self, 2)
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
#[doc = "Linked List Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLP_SPEC;
impl crate::RegisterSpec for LLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp::R`](R) reader structure"]
impl crate::Readable for LLP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`llp::W`](W) writer structure"]
impl crate::Writable for LLP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LLP to value 0"]
impl crate::Resettable for LLP_SPEC {
    const RESET_VALUE: u32 = 0;
}
