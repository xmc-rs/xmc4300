#[doc = "Register `IOCR12` reader"]
pub type R = crate::R<Iocr12Spec>;
#[doc = "Register `IOCR12` writer"]
pub type W = crate::W<Iocr12Spec>;
#[doc = "Field `PC12` reader - Port Control for Port n Pin 12 to 15"]
pub type Pc12R = crate::FieldReader;
#[doc = "Field `PC12` writer - Port Control for Port n Pin 12 to 15"]
pub type Pc12W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC13` reader - Port Control for Port n Pin 12 to 15"]
pub type Pc13R = crate::FieldReader;
#[doc = "Field `PC13` writer - Port Control for Port n Pin 12 to 15"]
pub type Pc13W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC14` reader - Port Control for Port n Pin 12 to 15"]
pub type Pc14R = crate::FieldReader;
#[doc = "Field `PC14` writer - Port Control for Port n Pin 12 to 15"]
pub type Pc14W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC15` reader - Port Control for Port n Pin 12 to 15"]
pub type Pc15R = crate::FieldReader;
#[doc = "Field `PC15` writer - Port Control for Port n Pin 12 to 15"]
pub type Pc15W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&self) -> Pc12R {
        Pc12R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&self) -> Pc13R {
        Pc13R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&self) -> Pc14R {
        Pc14R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&self) -> Pc15R {
        Pc15R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc12(&mut self) -> Pc12W<Iocr12Spec> {
        Pc12W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc13(&mut self) -> Pc13W<Iocr12Spec> {
        Pc13W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc14(&mut self) -> Pc14W<Iocr12Spec> {
        Pc14W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc15(&mut self) -> Pc15W<Iocr12Spec> {
        Pc15W::new(self, 27)
    }
}
#[doc = "Port 0 Input/Output Control Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocr12Spec;
impl crate::RegisterSpec for Iocr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr12::R`](R) reader structure"]
impl crate::Readable for Iocr12Spec {}
#[doc = "`write(|w| ..)` method takes [`iocr12::W`](W) writer structure"]
impl crate::Writable for Iocr12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR12 to value 0"]
impl crate::Resettable for Iocr12Spec {
    const RESET_VALUE: u32 = 0;
}
