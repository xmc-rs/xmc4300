#[doc = "Register `DC1R` reader"]
pub type R = crate::R<DC1R_SPEC>;
#[doc = "Register `DC1R` writer"]
pub type W = crate::W<DC1R_SPEC>;
#[doc = "Field `DT1R` reader - Rise Value for Dead Time of Channel 1"]
pub type DT1R_R = crate::FieldReader;
#[doc = "Field `DT1R` writer - Rise Value for Dead Time of Channel 1"]
pub type DT1R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DT1F` reader - Fall Value for Dead Time of Channel 1"]
pub type DT1F_R = crate::FieldReader;
#[doc = "Field `DT1F` writer - Fall Value for Dead Time of Channel 1"]
pub type DT1F_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1r(&self) -> DT1R_R {
        DT1R_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1f(&self) -> DT1F_R {
        DT1F_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dt1r(&mut self) -> DT1R_W<DC1R_SPEC> {
        DT1R_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dt1f(&mut self) -> DT1F_W<DC1R_SPEC> {
        DT1F_W::new(self, 8)
    }
}
#[doc = "Channel 1 Dead Time Values\n\nYou can [`read`](crate::Reg::read) this register and get [`dc1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC1R_SPEC;
impl crate::RegisterSpec for DC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc1r::R`](R) reader structure"]
impl crate::Readable for DC1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc1r::W`](W) writer structure"]
impl crate::Writable for DC1R_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC1R to value 0"]
impl crate::Resettable for DC1R_SPEC {
    const RESET_VALUE: u32 = 0;
}
