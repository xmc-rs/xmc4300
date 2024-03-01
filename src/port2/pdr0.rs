#[doc = "Register `PDR0` reader"]
pub type R = crate::R<Pdr0Spec>;
#[doc = "Register `PDR0` writer"]
pub type W = crate::W<Pdr0Spec>;
#[doc = "Field `PD0` reader - Pad Driver Mode for Pn.0"]
pub type Pd0R = crate::FieldReader;
#[doc = "Field `PD0` writer - Pad Driver Mode for Pn.0"]
pub type Pd0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD1` reader - Pad Driver Mode for Pn.1"]
pub type Pd1R = crate::FieldReader;
#[doc = "Field `PD1` writer - Pad Driver Mode for Pn.1"]
pub type Pd1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD2` reader - Pad Driver Mode for Pn.2"]
pub type Pd2R = crate::FieldReader;
#[doc = "Field `PD2` writer - Pad Driver Mode for Pn.2"]
pub type Pd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD3` reader - Pad Driver Mode for Pn.3"]
pub type Pd3R = crate::FieldReader;
#[doc = "Field `PD3` writer - Pad Driver Mode for Pn.3"]
pub type Pd3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD4` reader - Pad Driver Mode for Pn.4"]
pub type Pd4R = crate::FieldReader;
#[doc = "Field `PD4` writer - Pad Driver Mode for Pn.4"]
pub type Pd4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD5` reader - Pad Driver Mode for Pn.5"]
pub type Pd5R = crate::FieldReader;
#[doc = "Field `PD5` writer - Pad Driver Mode for Pn.5"]
pub type Pd5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD6` reader - Pad Driver Mode for Pn.6"]
pub type Pd6R = crate::FieldReader;
#[doc = "Field `PD6` writer - Pad Driver Mode for Pn.6"]
pub type Pd6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD7` reader - Pad Driver Mode for Pn.7"]
pub type Pd7R = crate::FieldReader;
#[doc = "Field `PD7` writer - Pad Driver Mode for Pn.7"]
pub type Pd7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    pub fn pd0(&self) -> Pd0R {
        Pd0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    pub fn pd1(&self) -> Pd1R {
        Pd1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    pub fn pd2(&self) -> Pd2R {
        Pd2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    pub fn pd3(&self) -> Pd3R {
        Pd3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    pub fn pd4(&self) -> Pd4R {
        Pd4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    pub fn pd5(&self) -> Pd5R {
        Pd5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    pub fn pd6(&self) -> Pd6R {
        Pd6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    pub fn pd7(&self) -> Pd7R {
        Pd7R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> Pd0W<Pdr0Spec> {
        Pd0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> Pd1W<Pdr0Spec> {
        Pd1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> Pd2W<Pdr0Spec> {
        Pd2W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> Pd3W<Pdr0Spec> {
        Pd3W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> Pd4W<Pdr0Spec> {
        Pd4W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> Pd5W<Pdr0Spec> {
        Pd5W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> Pd6W<Pdr0Spec> {
        Pd6W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> Pd7W<Pdr0Spec> {
        Pd7W::new(self, 28)
    }
}
#[doc = "Port 2 Pad Driver Mode 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdr0Spec;
impl crate::RegisterSpec for Pdr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdr0::R`](R) reader structure"]
impl crate::Readable for Pdr0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdr0::W`](W) writer structure"]
impl crate::Writable for Pdr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDR0 to value 0x2222_2222"]
impl crate::Resettable for Pdr0Spec {
    const RESET_VALUE: u32 = 0x2222_2222;
}
