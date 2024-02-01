#[doc = "Register `PDR0` reader"]
pub type R = crate::R<PDR0_SPEC>;
#[doc = "Register `PDR0` writer"]
pub type W = crate::W<PDR0_SPEC>;
#[doc = "Field `PD0` reader - Pad Driver Mode for Pn.0"]
pub type PD0_R = crate::FieldReader;
#[doc = "Field `PD0` writer - Pad Driver Mode for Pn.0"]
pub type PD0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD1` reader - Pad Driver Mode for Pn.1"]
pub type PD1_R = crate::FieldReader;
#[doc = "Field `PD1` writer - Pad Driver Mode for Pn.1"]
pub type PD1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD2` reader - Pad Driver Mode for Pn.2"]
pub type PD2_R = crate::FieldReader;
#[doc = "Field `PD2` writer - Pad Driver Mode for Pn.2"]
pub type PD2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD3` reader - Pad Driver Mode for Pn.3"]
pub type PD3_R = crate::FieldReader;
#[doc = "Field `PD3` writer - Pad Driver Mode for Pn.3"]
pub type PD3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD4` reader - Pad Driver Mode for Pn.4"]
pub type PD4_R = crate::FieldReader;
#[doc = "Field `PD4` writer - Pad Driver Mode for Pn.4"]
pub type PD4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD5` reader - Pad Driver Mode for Pn.5"]
pub type PD5_R = crate::FieldReader;
#[doc = "Field `PD5` writer - Pad Driver Mode for Pn.5"]
pub type PD5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD6` reader - Pad Driver Mode for Pn.6"]
pub type PD6_R = crate::FieldReader;
#[doc = "Field `PD6` writer - Pad Driver Mode for Pn.6"]
pub type PD6_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD7` reader - Pad Driver Mode for Pn.7"]
pub type PD7_R = crate::FieldReader;
#[doc = "Field `PD7` writer - Pad Driver Mode for Pn.7"]
pub type PD7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PDR0_SPEC> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PDR0_SPEC> {
        PD1_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PDR0_SPEC> {
        PD2_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PDR0_SPEC> {
        PD3_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<PDR0_SPEC> {
        PD4_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<PDR0_SPEC> {
        PD5_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<PDR0_SPEC> {
        PD6_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<PDR0_SPEC> {
        PD7_W::new(self, 28)
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
#[doc = "Port 3 Pad Driver Mode 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDR0_SPEC;
impl crate::RegisterSpec for PDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdr0::R`](R) reader structure"]
impl crate::Readable for PDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdr0::W`](W) writer structure"]
impl crate::Writable for PDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDR0 to value 0x2222_2222"]
impl crate::Resettable for PDR0_SPEC {
    const RESET_VALUE: u32 = 0x2222_2222;
}
