#[doc = "Register `DC2R` reader"]
pub type R = crate::R<DC2R_SPEC>;
#[doc = "Register `DC2R` writer"]
pub type W = crate::W<DC2R_SPEC>;
#[doc = "Field `DT2R` reader - Rise Value for Dead Time of Channel 2"]
pub type DT2R_R = crate::FieldReader;
#[doc = "Field `DT2R` writer - Rise Value for Dead Time of Channel 2"]
pub type DT2R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DT2F` reader - Fall Value for Dead Time of Channel 2"]
pub type DT2F_R = crate::FieldReader;
#[doc = "Field `DT2F` writer - Fall Value for Dead Time of Channel 2"]
pub type DT2F_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2r(&self) -> DT2R_R {
        DT2R_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2f(&self) -> DT2F_R {
        DT2F_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn dt2r(&mut self) -> DT2R_W<DC2R_SPEC> {
        DT2R_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn dt2f(&mut self) -> DT2F_W<DC2R_SPEC> {
        DT2F_W::new(self, 8)
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
#[doc = "Channel 2 Dead Time Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC2R_SPEC;
impl crate::RegisterSpec for DC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc2r::R`](R) reader structure"]
impl crate::Readable for DC2R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc2r::W`](W) writer structure"]
impl crate::Writable for DC2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC2R to value 0"]
impl crate::Resettable for DC2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
