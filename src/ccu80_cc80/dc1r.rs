#[doc = "Register `DC1R` reader"]
pub type R = crate::R<Dc1rSpec>;
#[doc = "Register `DC1R` writer"]
pub type W = crate::W<Dc1rSpec>;
#[doc = "Field `DT1R` reader - Rise Value for Dead Time of Channel 1"]
pub type Dt1rR = crate::FieldReader;
#[doc = "Field `DT1R` writer - Rise Value for Dead Time of Channel 1"]
pub type Dt1rW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DT1F` reader - Fall Value for Dead Time of Channel 1"]
pub type Dt1fR = crate::FieldReader;
#[doc = "Field `DT1F` writer - Fall Value for Dead Time of Channel 1"]
pub type Dt1fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1r(&self) -> Dt1rR {
        Dt1rR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 1"]
    #[inline(always)]
    pub fn dt1f(&self) -> Dt1fR {
        Dt1fR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rise Value for Dead Time of Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dt1r(&mut self) -> Dt1rW<Dc1rSpec> {
        Dt1rW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fall Value for Dead Time of Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dt1f(&mut self) -> Dt1fW<Dc1rSpec> {
        Dt1fW::new(self, 8)
    }
}
#[doc = "Channel 1 Dead Time Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dc1rSpec;
impl crate::RegisterSpec for Dc1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc1r::R`](R) reader structure"]
impl crate::Readable for Dc1rSpec {}
#[doc = "`write(|w| ..)` method takes [`dc1r::W`](W) writer structure"]
impl crate::Writable for Dc1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC1R to value 0"]
impl crate::Resettable for Dc1rSpec {
    const RESET_VALUE: u32 = 0;
}
