#[doc = "Register `IOCR8` reader"]
pub type R = crate::R<Iocr8Spec>;
#[doc = "Register `IOCR8` writer"]
pub type W = crate::W<Iocr8Spec>;
#[doc = "Field `PC8` reader - Port Control for Port n Pin 8 to 11"]
pub type Pc8R = crate::FieldReader;
#[doc = "Field `PC8` writer - Port Control for Port n Pin 8 to 11"]
pub type Pc8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC9` reader - Port Control for Port n Pin 8 to 11"]
pub type Pc9R = crate::FieldReader;
#[doc = "Field `PC9` writer - Port Control for Port n Pin 8 to 11"]
pub type Pc9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC10` reader - Port Control for Port n Pin 8 to 11"]
pub type Pc10R = crate::FieldReader;
#[doc = "Field `PC10` writer - Port Control for Port n Pin 8 to 11"]
pub type Pc10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC11` reader - Port Control for Port n Pin 8 to 11"]
pub type Pc11R = crate::FieldReader;
#[doc = "Field `PC11` writer - Port Control for Port n Pin 8 to 11"]
pub type Pc11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&self) -> Pc8R {
        Pc8R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&self) -> Pc9R {
        Pc9R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&self) -> Pc10R {
        Pc10R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&self) -> Pc11R {
        Pc11R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc8(&mut self) -> Pc8W<Iocr8Spec> {
        Pc8W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc9(&mut self) -> Pc9W<Iocr8Spec> {
        Pc9W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc10(&mut self) -> Pc10W<Iocr8Spec> {
        Pc10W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc11(&mut self) -> Pc11W<Iocr8Spec> {
        Pc11W::new(self, 27)
    }
}
#[doc = "Port 1 Input/Output Control Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocr8Spec;
impl crate::RegisterSpec for Iocr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr8::R`](R) reader structure"]
impl crate::Readable for Iocr8Spec {}
#[doc = "`write(|w| ..)` method takes [`iocr8::W`](W) writer structure"]
impl crate::Writable for Iocr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR8 to value 0"]
impl crate::Resettable for Iocr8Spec {
    const RESET_VALUE: u32 = 0;
}
