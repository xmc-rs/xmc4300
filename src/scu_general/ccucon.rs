#[doc = "Register `CCUCON` reader"]
pub type R = crate::R<CcuconSpec>;
#[doc = "Register `CCUCON` writer"]
pub type W = crate::W<CcuconSpec>;
#[doc = "Field `GSC40` reader - Global Start Control CCU40"]
pub type Gsc40R = crate::BitReader;
#[doc = "Field `GSC40` writer - Global Start Control CCU40"]
pub type Gsc40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSC41` reader - Global Start Control CCU41"]
pub type Gsc41R = crate::BitReader;
#[doc = "Field `GSC41` writer - Global Start Control CCU41"]
pub type Gsc41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSC80` reader - Global Start Control CCU80"]
pub type Gsc80R = crate::BitReader;
#[doc = "Field `GSC80` writer - Global Start Control CCU80"]
pub type Gsc80W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> Gsc40R {
        Gsc40R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&self) -> Gsc41R {
        Gsc41R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&self) -> Gsc80R {
        Gsc80R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    #[must_use]
    pub fn gsc40(&mut self) -> Gsc40W<CcuconSpec> {
        Gsc40W::new(self, 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    #[must_use]
    pub fn gsc41(&mut self) -> Gsc41W<CcuconSpec> {
        Gsc41W::new(self, 1)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    #[must_use]
    pub fn gsc80(&mut self) -> Gsc80W<CcuconSpec> {
        Gsc80W::new(self, 8)
    }
}
#[doc = "CCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccucon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccucon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcuconSpec;
impl crate::RegisterSpec for CcuconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccucon::R`](R) reader structure"]
impl crate::Readable for CcuconSpec {}
#[doc = "`write(|w| ..)` method takes [`ccucon::W`](W) writer structure"]
impl crate::Writable for CcuconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCUCON to value 0"]
impl crate::Resettable for CcuconSpec {
    const RESET_VALUE: u32 = 0;
}
