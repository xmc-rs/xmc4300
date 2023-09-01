#[doc = "Register `CCUCON` reader"]
pub type R = crate::R<CCUCON_SPEC>;
#[doc = "Register `CCUCON` writer"]
pub type W = crate::W<CCUCON_SPEC>;
#[doc = "Field `GSC40` reader - Global Start Control CCU40"]
pub type GSC40_R = crate::BitReader;
#[doc = "Field `GSC40` writer - Global Start Control CCU40"]
pub type GSC40_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GSC41` reader - Global Start Control CCU41"]
pub type GSC41_R = crate::BitReader;
#[doc = "Field `GSC41` writer - Global Start Control CCU41"]
pub type GSC41_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GSC80` reader - Global Start Control CCU80"]
pub type GSC80_R = crate::BitReader;
#[doc = "Field `GSC80` writer - Global Start Control CCU80"]
pub type GSC80_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> GSC40_R {
        GSC40_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&self) -> GSC41_R {
        GSC41_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&self) -> GSC80_R {
        GSC80_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    #[must_use]
    pub fn gsc40(&mut self) -> GSC40_W<CCUCON_SPEC, 0> {
        GSC40_W::new(self)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    #[must_use]
    pub fn gsc41(&mut self) -> GSC41_W<CCUCON_SPEC, 1> {
        GSC41_W::new(self)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    #[must_use]
    pub fn gsc80(&mut self) -> GSC80_W<CCUCON_SPEC, 8> {
        GSC80_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccucon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccucon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCUCON_SPEC;
impl crate::RegisterSpec for CCUCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccucon::R`](R) reader structure"]
impl crate::Readable for CCUCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccucon::W`](W) writer structure"]
impl crate::Writable for CCUCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCUCON to value 0"]
impl crate::Resettable for CCUCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
