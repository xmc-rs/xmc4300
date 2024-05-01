#[doc = "Register `IOCR4` reader"]
pub type R = crate::R<Iocr4Spec>;
#[doc = "Register `IOCR4` writer"]
pub type W = crate::W<Iocr4Spec>;
#[doc = "Field `PC4` reader - Port Control for Port n Pin 4 to 7"]
pub type Pc4R = crate::FieldReader;
#[doc = "Field `PC4` writer - Port Control for Port n Pin 4 to 7"]
pub type Pc4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC5` reader - Port Control for Port n Pin 4 to 7"]
pub type Pc5R = crate::FieldReader;
#[doc = "Field `PC5` writer - Port Control for Port n Pin 4 to 7"]
pub type Pc5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC6` reader - Port Control for Port n Pin 4 to 7"]
pub type Pc6R = crate::FieldReader;
#[doc = "Field `PC6` writer - Port Control for Port n Pin 4 to 7"]
pub type Pc6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC7` reader - Port Control for Port n Pin 4 to 7"]
pub type Pc7R = crate::FieldReader;
#[doc = "Field `PC7` writer - Port Control for Port n Pin 4 to 7"]
pub type Pc7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&self) -> Pc4R {
        Pc4R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&self) -> Pc5R {
        Pc5R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&self) -> Pc6R {
        Pc6R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&self) -> Pc7R {
        Pc7R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc4(&mut self) -> Pc4W<Iocr4Spec> {
        Pc4W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc5(&mut self) -> Pc5W<Iocr4Spec> {
        Pc5W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc6(&mut self) -> Pc6W<Iocr4Spec> {
        Pc6W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc7(&mut self) -> Pc7W<Iocr4Spec> {
        Pc7W::new(self, 27)
    }
}
#[doc = "Port 1 Input/Output Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocr4Spec;
impl crate::RegisterSpec for Iocr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr4::R`](R) reader structure"]
impl crate::Readable for Iocr4Spec {}
#[doc = "`write(|w| ..)` method takes [`iocr4::W`](W) writer structure"]
impl crate::Writable for Iocr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR4 to value 0"]
impl crate::Resettable for Iocr4Spec {
    const RESET_VALUE: u32 = 0;
}
