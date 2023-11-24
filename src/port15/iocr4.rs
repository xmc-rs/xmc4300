#[doc = "Register `IOCR4` reader"]
pub type R = crate::R<IOCR4_SPEC>;
#[doc = "Register `IOCR4` writer"]
pub type W = crate::W<IOCR4_SPEC>;
#[doc = "Field `PC4` reader - Port Control for Port n Pin 4 to 7"]
pub type PC4_R = crate::FieldReader;
#[doc = "Field `PC4` writer - Port Control for Port n Pin 4 to 7"]
pub type PC4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC5` reader - Port Control for Port n Pin 4 to 7"]
pub type PC5_R = crate::FieldReader;
#[doc = "Field `PC5` writer - Port Control for Port n Pin 4 to 7"]
pub type PC5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC6` reader - Port Control for Port n Pin 4 to 7"]
pub type PC6_R = crate::FieldReader;
#[doc = "Field `PC6` writer - Port Control for Port n Pin 4 to 7"]
pub type PC6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC7` reader - Port Control for Port n Pin 4 to 7"]
pub type PC7_R = crate::FieldReader;
#[doc = "Field `PC7` writer - Port Control for Port n Pin 4 to 7"]
pub type PC7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&self) -> PC4_R {
        PC4_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&self) -> PC5_R {
        PC5_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc4(&mut self) -> PC4_W<IOCR4_SPEC> {
        PC4_W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc5(&mut self) -> PC5_W<IOCR4_SPEC> {
        PC5_W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc6(&mut self) -> PC6_W<IOCR4_SPEC> {
        PC6_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc7(&mut self) -> PC7_W<IOCR4_SPEC> {
        PC7_W::new(self, 27)
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
#[doc = "Port 15 Input/Output Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOCR4_SPEC;
impl crate::RegisterSpec for IOCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr4::R`](R) reader structure"]
impl crate::Readable for IOCR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iocr4::W`](W) writer structure"]
impl crate::Writable for IOCR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCR4 to value 0"]
impl crate::Resettable for IOCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
