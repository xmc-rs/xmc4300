#[doc = "Register `PLLCON2` reader"]
pub type R = crate::R<Pllcon2Spec>;
#[doc = "Register `PLLCON2` writer"]
pub type W = crate::W<Pllcon2Spec>;
#[doc = "P-Divider Input Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pinsel {
    #[doc = "0: PLL external oscillator selected"]
    Const0 = 0,
    #[doc = "1: Backup clock fofi selected"]
    Const1 = 1,
}
impl From<Pinsel> for bool {
    #[inline(always)]
    fn from(variant: Pinsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINSEL` reader - P-Divider Input Selection"]
pub type PinselR = crate::BitReader<Pinsel>;
impl PinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pinsel {
        match self.bits {
            false => Pinsel::Const0,
            true => Pinsel::Const1,
        }
    }
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pinsel::Const0
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pinsel::Const1
    }
}
#[doc = "Field `PINSEL` writer - P-Divider Input Selection"]
pub type PinselW<'a, REG> = crate::BitWriter<'a, REG, Pinsel>;
impl<'a, REG> PinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pinsel::Const0)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pinsel::Const1)
    }
}
#[doc = "K1-Divider Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum K1insel {
    #[doc = "0: PLL external oscillator selected"]
    Const0 = 0,
    #[doc = "1: Backup clock fofi selected"]
    Const1 = 1,
}
impl From<K1insel> for bool {
    #[inline(always)]
    fn from(variant: K1insel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `K1INSEL` reader - K1-Divider Input Selection"]
pub type K1inselR = crate::BitReader<K1insel>;
impl K1inselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> K1insel {
        match self.bits {
            false => K1insel::Const0,
            true => K1insel::Const1,
        }
    }
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == K1insel::Const0
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == K1insel::Const1
    }
}
#[doc = "Field `K1INSEL` writer - K1-Divider Input Selection"]
pub type K1inselW<'a, REG> = crate::BitWriter<'a, REG, K1insel>;
impl<'a, REG> K1inselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(K1insel::Const0)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(K1insel::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline(always)]
    pub fn pinsel(&self) -> PinselR {
        PinselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline(always)]
    pub fn k1insel(&self) -> K1inselR {
        K1inselR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pinsel(&mut self) -> PinselW<Pllcon2Spec> {
        PinselW::new(self, 0)
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn k1insel(&mut self) -> K1inselW<Pllcon2Spec> {
        K1inselW::new(self, 8)
    }
}
#[doc = "PLL Configuration 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pllcon2Spec;
impl crate::RegisterSpec for Pllcon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcon2::R`](R) reader structure"]
impl crate::Readable for Pllcon2Spec {}
#[doc = "`write(|w| ..)` method takes [`pllcon2::W`](W) writer structure"]
impl crate::Writable for Pllcon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCON2 to value 0x01"]
impl crate::Resettable for Pllcon2Spec {
    const RESET_VALUE: u32 = 0x01;
}
