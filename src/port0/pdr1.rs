#[doc = "Register `PDR1` reader"]
pub type R = crate::R<PDR1_SPEC>;
#[doc = "Register `PDR1` writer"]
pub type W = crate::W<PDR1_SPEC>;
#[doc = "Field `PD8` reader - Pad Driver Mode for Pn.8"]
pub type PD8_R = crate::FieldReader;
#[doc = "Field `PD8` writer - Pad Driver Mode for Pn.8"]
pub type PD8_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD9` reader - Pad Driver Mode for Pn.9"]
pub type PD9_R = crate::FieldReader;
#[doc = "Field `PD9` writer - Pad Driver Mode for Pn.9"]
pub type PD9_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD10` reader - Pad Driver Mode for Pn.10"]
pub type PD10_R = crate::FieldReader;
#[doc = "Field `PD10` writer - Pad Driver Mode for Pn.10"]
pub type PD10_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD11` reader - Pad Driver Mode for Pn.11"]
pub type PD11_R = crate::FieldReader;
#[doc = "Field `PD11` writer - Pad Driver Mode for Pn.11"]
pub type PD11_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD12` reader - Pad Driver Mode for Pn.12"]
pub type PD12_R = crate::FieldReader;
#[doc = "Field `PD12` writer - Pad Driver Mode for Pn.12"]
pub type PD12_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD13` reader - Pad Driver Mode for Pn.13"]
pub type PD13_R = crate::FieldReader;
#[doc = "Field `PD13` writer - Pad Driver Mode for Pn.13"]
pub type PD13_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD14` reader - Pad Driver Mode for Pn.14"]
pub type PD14_R = crate::FieldReader;
#[doc = "Field `PD14` writer - Pad Driver Mode for Pn.14"]
pub type PD14_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD15` reader - Pad Driver Mode for Pn.15"]
pub type PD15_R = crate::FieldReader;
#[doc = "Field `PD15` writer - Pad Driver Mode for Pn.15"]
pub type PD15_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<PDR1_SPEC> {
        PD8_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<PDR1_SPEC> {
        PD9_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<PDR1_SPEC> {
        PD10_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<PDR1_SPEC> {
        PD11_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> PD12_W<PDR1_SPEC> {
        PD12_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> PD13_W<PDR1_SPEC> {
        PD13_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<PDR1_SPEC> {
        PD14_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> PD15_W<PDR1_SPEC> {
        PD15_W::new(self, 28)
    }
}
#[doc = "Port 0 Pad Driver Mode 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDR1_SPEC;
impl crate::RegisterSpec for PDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdr1::R`](R) reader structure"]
impl crate::Readable for PDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdr1::W`](W) writer structure"]
impl crate::Writable for PDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDR1 to value 0x2222_2222"]
impl crate::Resettable for PDR1_SPEC {
    const RESET_VALUE: u32 = 0x2222_2222;
}
