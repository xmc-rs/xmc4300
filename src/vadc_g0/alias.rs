#[doc = "Register `ALIAS` reader"]
pub type R = crate::R<ALIAS_SPEC>;
#[doc = "Register `ALIAS` writer"]
pub type W = crate::W<ALIAS_SPEC>;
#[doc = "Field `ALIAS0` reader - Alias Value for CH0 Conversion Requests"]
pub type ALIAS0_R = crate::FieldReader;
#[doc = "Field `ALIAS0` writer - Alias Value for CH0 Conversion Requests"]
pub type ALIAS0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ALIAS1` reader - Alias Value for CH1 Conversion Requests"]
pub type ALIAS1_R = crate::FieldReader;
#[doc = "Field `ALIAS1` writer - Alias Value for CH1 Conversion Requests"]
pub type ALIAS1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    pub fn alias0(&self) -> ALIAS0_R {
        ALIAS0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    pub fn alias1(&self) -> ALIAS1_R {
        ALIAS1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alias Value for CH0 Conversion Requests"]
    #[inline(always)]
    #[must_use]
    pub fn alias0(&mut self) -> ALIAS0_W<ALIAS_SPEC> {
        ALIAS0_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Alias Value for CH1 Conversion Requests"]
    #[inline(always)]
    #[must_use]
    pub fn alias1(&mut self) -> ALIAS1_W<ALIAS_SPEC> {
        ALIAS1_W::new(self, 8)
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
#[doc = "Alias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALIAS_SPEC;
impl crate::RegisterSpec for ALIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alias::R`](R) reader structure"]
impl crate::Readable for ALIAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alias::W`](W) writer structure"]
impl crate::Writable for ALIAS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALIAS to value 0x0100"]
impl crate::Resettable for ALIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
