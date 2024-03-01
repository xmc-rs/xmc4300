#[doc = "Register `PLLCON1` reader"]
pub type R = crate::R<Pllcon1Spec>;
#[doc = "Register `PLLCON1` writer"]
pub type W = crate::W<Pllcon1Spec>;
#[doc = "Field `K1DIV` reader - K1-Divider Value"]
pub type K1divR = crate::FieldReader;
#[doc = "Field `K1DIV` writer - K1-Divider Value"]
pub type K1divW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NDIV` reader - N-Divider Value"]
pub type NdivR = crate::FieldReader;
#[doc = "Field `NDIV` writer - N-Divider Value"]
pub type NdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `K2DIV` reader - K2-Divider Value"]
pub type K2divR = crate::FieldReader;
#[doc = "Field `K2DIV` writer - K2-Divider Value"]
pub type K2divW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PDIV` reader - P-Divider Value"]
pub type PdivR = crate::FieldReader;
#[doc = "Field `PDIV` writer - P-Divider Value"]
pub type PdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    pub fn k1div(&self) -> K1divR {
        K1divR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&self) -> NdivR {
        NdivR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    pub fn k2div(&self) -> K2divR {
        K2divR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&self) -> PdivR {
        PdivR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - K1-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn k1div(&mut self) -> K1divW<Pllcon1Spec> {
        K1divW::new(self, 0)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn ndiv(&mut self) -> NdivW<Pllcon1Spec> {
        NdivW::new(self, 8)
    }
    #[doc = "Bits 16:22 - K2-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn k2div(&mut self) -> K2divW<Pllcon1Spec> {
        K2divW::new(self, 16)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PdivW<Pllcon1Spec> {
        PdivW::new(self, 24)
    }
}
#[doc = "PLL Configuration 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pllcon1Spec;
impl crate::RegisterSpec for Pllcon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcon1::R`](R) reader structure"]
impl crate::Readable for Pllcon1Spec {}
#[doc = "`write(|w| ..)` method takes [`pllcon1::W`](W) writer structure"]
impl crate::Writable for Pllcon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCON1 to value 0"]
impl crate::Resettable for Pllcon1Spec {
    const RESET_VALUE: u32 = 0;
}
