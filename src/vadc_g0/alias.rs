#[doc = "Register `ALIAS` reader"]
pub type R = crate::R<AliasSpec>;
#[doc = "Register `ALIAS` writer"]
pub type W = crate::W<AliasSpec>;
#[doc = "Field `ALIAS0` reader - Alias Value for CH0 Conversion Requests"]
pub type Alias0R = crate::FieldReader;
#[doc = "Field `ALIAS0` writer - Alias Value for CH0 Conversion Requests"]
pub type Alias0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ALIAS1` reader - Alias Value for CH1 Conversion Requests"]
pub type Alias1R = crate::FieldReader;
#[doc = "Field `ALIAS1` writer - Alias Value for CH1 Conversion Requests"]
pub type Alias1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    pub fn alias0(&self) -> Alias0R {
        Alias0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    pub fn alias1(&self) -> Alias1R {
        Alias1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    #[must_use]
    pub fn alias0(&mut self) -> Alias0W<AliasSpec> {
        Alias0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    #[must_use]
    pub fn alias1(&mut self) -> Alias1W<AliasSpec> {
        Alias1W::new(self, 8)
    }
}
#[doc = "Alias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AliasSpec;
impl crate::RegisterSpec for AliasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alias::R`](R) reader structure"]
impl crate::Readable for AliasSpec {}
#[doc = "`write(|w| ..)` method takes [`alias::W`](W) writer structure"]
impl crate::Writable for AliasSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALIAS to value 0x0100"]
impl crate::Resettable for AliasSpec {
    const RESET_VALUE: u32 = 0x0100;
}
