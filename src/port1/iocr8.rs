#[doc = "Register `IOCR8` reader"]
pub type R = crate::R<IOCR8_SPEC>;
#[doc = "Register `IOCR8` writer"]
pub type W = crate::W<IOCR8_SPEC>;
#[doc = "Field `PC8` reader - Port Control for Port n Pin 8 to 11"]
pub type PC8_R = crate::FieldReader;
#[doc = "Field `PC8` writer - Port Control for Port n Pin 8 to 11"]
pub type PC8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC9` reader - Port Control for Port n Pin 8 to 11"]
pub type PC9_R = crate::FieldReader;
#[doc = "Field `PC9` writer - Port Control for Port n Pin 8 to 11"]
pub type PC9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC10` reader - Port Control for Port n Pin 8 to 11"]
pub type PC10_R = crate::FieldReader;
#[doc = "Field `PC10` writer - Port Control for Port n Pin 8 to 11"]
pub type PC10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC11` reader - Port Control for Port n Pin 8 to 11"]
pub type PC11_R = crate::FieldReader;
#[doc = "Field `PC11` writer - Port Control for Port n Pin 8 to 11"]
pub type PC11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&self) -> PC8_R {
        PC8_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&self) -> PC9_R {
        PC9_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&self) -> PC10_R {
        PC10_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&self) -> PC11_R {
        PC11_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc8(&mut self) -> PC8_W<IOCR8_SPEC> {
        PC8_W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc9(&mut self) -> PC9_W<IOCR8_SPEC> {
        PC9_W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc10(&mut self) -> PC10_W<IOCR8_SPEC> {
        PC10_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc11(&mut self) -> PC11_W<IOCR8_SPEC> {
        PC11_W::new(self, 27)
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
#[doc = "Port 1 Input/Output Control Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOCR8_SPEC;
impl crate::RegisterSpec for IOCR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr8::R`](R) reader structure"]
impl crate::Readable for IOCR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iocr8::W`](W) writer structure"]
impl crate::Writable for IOCR8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCR8 to value 0"]
impl crate::Resettable for IOCR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
