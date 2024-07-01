#[doc = "Register `IOCR0` reader"]
pub type R = crate::R<IOCR0_SPEC>;
#[doc = "Register `IOCR0` writer"]
pub type W = crate::W<IOCR0_SPEC>;
#[doc = "Field `PC0` reader - Port Control for Port n Pin 0 to 3"]
pub type PC0_R = crate::FieldReader;
#[doc = "Field `PC0` writer - Port Control for Port n Pin 0 to 3"]
pub type PC0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC1` reader - Port Control for Port n Pin 0 to 3"]
pub type PC1_R = crate::FieldReader;
#[doc = "Field `PC1` writer - Port Control for Port n Pin 0 to 3"]
pub type PC1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC2` reader - Port Control for Port n Pin 0 to 3"]
pub type PC2_R = crate::FieldReader;
#[doc = "Field `PC2` writer - Port Control for Port n Pin 0 to 3"]
pub type PC2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC3` reader - Port Control for Port n Pin 0 to 3"]
pub type PC3_R = crate::FieldReader;
#[doc = "Field `PC3` writer - Port Control for Port n Pin 0 to 3"]
pub type PC3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&self) -> PC0_R {
        PC0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&self) -> PC1_R {
        PC1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&self) -> PC2_R {
        PC2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&self) -> PC3_R {
        PC3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc0(&mut self) -> PC0_W<IOCR0_SPEC> {
        PC0_W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc1(&mut self) -> PC1_W<IOCR0_SPEC> {
        PC1_W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc2(&mut self) -> PC2_W<IOCR0_SPEC> {
        PC2_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc3(&mut self) -> PC3_W<IOCR0_SPEC> {
        PC3_W::new(self, 27)
    }
}
#[doc = "Port 1 Input/Output Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`iocr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOCR0_SPEC;
impl crate::RegisterSpec for IOCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr0::R`](R) reader structure"]
impl crate::Readable for IOCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iocr0::W`](W) writer structure"]
impl crate::Writable for IOCR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR0 to value 0"]
impl crate::Resettable for IOCR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
