#[doc = "Register `IOCR12` reader"]
pub type R = crate::R<IOCR12_SPEC>;
#[doc = "Register `IOCR12` writer"]
pub type W = crate::W<IOCR12_SPEC>;
#[doc = "Field `PC12` reader - Port Control for Port n Pin 12 to 15"]
pub type PC12_R = crate::FieldReader;
#[doc = "Field `PC12` writer - Port Control for Port n Pin 12 to 15"]
pub type PC12_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC13` reader - Port Control for Port n Pin 12 to 15"]
pub type PC13_R = crate::FieldReader;
#[doc = "Field `PC13` writer - Port Control for Port n Pin 12 to 15"]
pub type PC13_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC14` reader - Port Control for Port n Pin 12 to 15"]
pub type PC14_R = crate::FieldReader;
#[doc = "Field `PC14` writer - Port Control for Port n Pin 12 to 15"]
pub type PC14_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC15` reader - Port Control for Port n Pin 12 to 15"]
pub type PC15_R = crate::FieldReader;
#[doc = "Field `PC15` writer - Port Control for Port n Pin 12 to 15"]
pub type PC15_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&self) -> PC12_R {
        PC12_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&self) -> PC13_R {
        PC13_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&self) -> PC14_R {
        PC14_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&self) -> PC15_R {
        PC15_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc12(&mut self) -> PC12_W<IOCR12_SPEC> {
        PC12_W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc13(&mut self) -> PC13_W<IOCR12_SPEC> {
        PC13_W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc14(&mut self) -> PC14_W<IOCR12_SPEC> {
        PC14_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc15(&mut self) -> PC15_W<IOCR12_SPEC> {
        PC15_W::new(self, 27)
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
#[doc = "Port 0 Input/Output Control Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOCR12_SPEC;
impl crate::RegisterSpec for IOCR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr12::R`](R) reader structure"]
impl crate::Readable for IOCR12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iocr12::W`](W) writer structure"]
impl crate::Writable for IOCR12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCR12 to value 0"]
impl crate::Resettable for IOCR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
