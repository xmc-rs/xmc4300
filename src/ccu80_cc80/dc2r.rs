#[doc = "Register `DC2R` reader"]
pub type R = crate::R<Dc2rSpec>;
#[doc = "Register `DC2R` writer"]
pub type W = crate::W<Dc2rSpec>;
#[doc = "Field `DT2R` reader - Rise Value for Dead Time of Channel 2"]
pub type Dt2rR = crate::FieldReader;
#[doc = "Field `DT2R` writer - Rise Value for Dead Time of Channel 2"]
pub type Dt2rW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DT2F` reader - Fall Value for Dead Time of Channel 2"]
pub type Dt2fR = crate::FieldReader;
#[doc = "Field `DT2F` writer - Fall Value for Dead Time of Channel 2"]
pub type Dt2fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2r(&self) -> Dt2rR {
        Dt2rR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 2"]
    #[inline(always)]
    pub fn dt2f(&self) -> Dt2fR {
        Dt2fR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn dt2r(&mut self) -> Dt2rW<Dc2rSpec> {
        Dt2rW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn dt2f(&mut self) -> Dt2fW<Dc2rSpec> {
        Dt2fW::new(self, 8)
    }
}
#[doc = "Channel 2 Dead Time Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dc2rSpec;
impl crate::RegisterSpec for Dc2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc2r::R`](R) reader structure"]
impl crate::Readable for Dc2rSpec {}
#[doc = "`write(|w| ..)` method takes [`dc2r::W`](W) writer structure"]
impl crate::Writable for Dc2rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC2R to value 0"]
impl crate::Resettable for Dc2rSpec {
    const RESET_VALUE: u32 = 0;
}
