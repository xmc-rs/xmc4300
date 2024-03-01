#[doc = "Register `PDR1` reader"]
pub type R = crate::R<Pdr1Spec>;
#[doc = "Register `PDR1` writer"]
pub type W = crate::W<Pdr1Spec>;
#[doc = "Field `PD8` reader - Pad Driver Mode for Pn.8"]
pub type Pd8R = crate::FieldReader;
#[doc = "Field `PD8` writer - Pad Driver Mode for Pn.8"]
pub type Pd8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD9` reader - Pad Driver Mode for Pn.9"]
pub type Pd9R = crate::FieldReader;
#[doc = "Field `PD9` writer - Pad Driver Mode for Pn.9"]
pub type Pd9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD10` reader - Pad Driver Mode for Pn.10"]
pub type Pd10R = crate::FieldReader;
#[doc = "Field `PD10` writer - Pad Driver Mode for Pn.10"]
pub type Pd10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD11` reader - Pad Driver Mode for Pn.11"]
pub type Pd11R = crate::FieldReader;
#[doc = "Field `PD11` writer - Pad Driver Mode for Pn.11"]
pub type Pd11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD12` reader - Pad Driver Mode for Pn.12"]
pub type Pd12R = crate::FieldReader;
#[doc = "Field `PD12` writer - Pad Driver Mode for Pn.12"]
pub type Pd12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD13` reader - Pad Driver Mode for Pn.13"]
pub type Pd13R = crate::FieldReader;
#[doc = "Field `PD13` writer - Pad Driver Mode for Pn.13"]
pub type Pd13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD14` reader - Pad Driver Mode for Pn.14"]
pub type Pd14R = crate::FieldReader;
#[doc = "Field `PD14` writer - Pad Driver Mode for Pn.14"]
pub type Pd14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD15` reader - Pad Driver Mode for Pn.15"]
pub type Pd15R = crate::FieldReader;
#[doc = "Field `PD15` writer - Pad Driver Mode for Pn.15"]
pub type Pd15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    pub fn pd8(&self) -> Pd8R {
        Pd8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    pub fn pd9(&self) -> Pd9R {
        Pd9R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    pub fn pd10(&self) -> Pd10R {
        Pd10R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    pub fn pd11(&self) -> Pd11R {
        Pd11R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    pub fn pd12(&self) -> Pd12R {
        Pd12R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    pub fn pd13(&self) -> Pd13R {
        Pd13R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    pub fn pd14(&self) -> Pd14R {
        Pd14R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    pub fn pd15(&self) -> Pd15R {
        Pd15R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> Pd8W<Pdr1Spec> {
        Pd8W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> Pd9W<Pdr1Spec> {
        Pd9W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> Pd10W<Pdr1Spec> {
        Pd10W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> Pd11W<Pdr1Spec> {
        Pd11W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> Pd12W<Pdr1Spec> {
        Pd12W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> Pd13W<Pdr1Spec> {
        Pd13W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> Pd14W<Pdr1Spec> {
        Pd14W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> Pd15W<Pdr1Spec> {
        Pd15W::new(self, 28)
    }
}
#[doc = "Port 0 Pad Driver Mode 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdr1Spec;
impl crate::RegisterSpec for Pdr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdr1::R`](R) reader structure"]
impl crate::Readable for Pdr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdr1::W`](W) writer structure"]
impl crate::Writable for Pdr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDR1 to value 0x2222_2222"]
impl crate::Resettable for Pdr1Spec {
    const RESET_VALUE: u32 = 0x2222_2222;
}
