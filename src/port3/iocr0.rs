#[doc = "Register `IOCR0` reader"]
pub type R = crate::R<Iocr0Spec>;
#[doc = "Register `IOCR0` writer"]
pub type W = crate::W<Iocr0Spec>;
#[doc = "Field `PC0` reader - Port Control for Port n Pin 0 to 3"]
pub type Pc0R = crate::FieldReader;
#[doc = "Field `PC0` writer - Port Control for Port n Pin 0 to 3"]
pub type Pc0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC1` reader - Port Control for Port n Pin 0 to 3"]
pub type Pc1R = crate::FieldReader;
#[doc = "Field `PC1` writer - Port Control for Port n Pin 0 to 3"]
pub type Pc1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC2` reader - Port Control for Port n Pin 0 to 3"]
pub type Pc2R = crate::FieldReader;
#[doc = "Field `PC2` writer - Port Control for Port n Pin 0 to 3"]
pub type Pc2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC3` reader - Port Control for Port n Pin 0 to 3"]
pub type Pc3R = crate::FieldReader;
#[doc = "Field `PC3` writer - Port Control for Port n Pin 0 to 3"]
pub type Pc3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&self) -> Pc0R {
        Pc0R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&self) -> Pc1R {
        Pc1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&self) -> Pc2R {
        Pc2R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&self) -> Pc3R {
        Pc3R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc0(&mut self) -> Pc0W<Iocr0Spec> {
        Pc0W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc1(&mut self) -> Pc1W<Iocr0Spec> {
        Pc1W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc2(&mut self) -> Pc2W<Iocr0Spec> {
        Pc2W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc3(&mut self) -> Pc3W<Iocr0Spec> {
        Pc3W::new(self, 27)
    }
}
#[doc = "Port 3 Input/Output Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocr0Spec;
impl crate::RegisterSpec for Iocr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr0::R`](R) reader structure"]
impl crate::Readable for Iocr0Spec {}
#[doc = "`write(|w| ..)` method takes [`iocr0::W`](W) writer structure"]
impl crate::Writable for Iocr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR0 to value 0"]
impl crate::Resettable for Iocr0Spec {
    const RESET_VALUE: u32 = 0;
}
