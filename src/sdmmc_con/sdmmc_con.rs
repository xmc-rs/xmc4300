#[doc = "Register `SDMMC_CON` reader"]
pub type R = crate::R<SDMMC_CON_SPEC>;
#[doc = "Register `SDMMC_CON` writer"]
pub type W = crate::W<SDMMC_CON_SPEC>;
#[doc = "SDMMC Write Protection Input Multiplexer Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPSEL_A {
    #[doc = "0: P1.1 input pin selected"]
    VALUE1 = 0,
    #[doc = "1: Software bit WPVAL is selected"]
    VALUE2 = 1,
}
impl From<WPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPSEL` reader - SDMMC Write Protection Input Multiplexer Control"]
pub type WPSEL_R = crate::BitReader<WPSEL_A>;
impl WPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WPSEL_A {
        match self.bits {
            false => WPSEL_A::VALUE1,
            true => WPSEL_A::VALUE2,
        }
    }
    #[doc = "P1.1 input pin selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPSEL_A::VALUE1
    }
    #[doc = "Software bit WPVAL is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPSEL_A::VALUE2
    }
}
#[doc = "Field `WPSEL` writer - SDMMC Write Protection Input Multiplexer Control"]
pub type WPSEL_W<'a, REG> = crate::BitWriter<'a, REG, WPSEL_A>;
impl<'a, REG> WPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "P1.1 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WPSEL_A::VALUE1)
    }
    #[doc = "Software bit WPVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WPSEL_A::VALUE2)
    }
}
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPSVAL_A {
    #[doc = "0: No write protection"]
    VALUE1 = 0,
    #[doc = "1: Write protection active"]
    VALUE2 = 1,
}
impl From<WPSVAL_A> for bool {
    #[inline(always)]
    fn from(variant: WPSVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPSVAL` reader - SDMMC Write Protect Software Control"]
pub type WPSVAL_R = crate::BitReader<WPSVAL_A>;
impl WPSVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WPSVAL_A {
        match self.bits {
            false => WPSVAL_A::VALUE1,
            true => WPSVAL_A::VALUE2,
        }
    }
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPSVAL_A::VALUE1
    }
    #[doc = "Write protection active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPSVAL_A::VALUE2
    }
}
#[doc = "Field `WPSVAL` writer - SDMMC Write Protect Software Control"]
pub type WPSVAL_W<'a, REG> = crate::BitWriter<'a, REG, WPSVAL_A>;
impl<'a, REG> WPSVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WPSVAL_A::VALUE1)
    }
    #[doc = "Write protection active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WPSVAL_A::VALUE2)
    }
}
#[doc = "SDMMC Card Detection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDSEL_A {
    #[doc = "0: P1.10 input pin selected"]
    VALUE1 = 0,
    #[doc = "1: Software bit CDSVAL is selected"]
    VALUE2 = 1,
}
impl From<CDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDSEL` reader - SDMMC Card Detection Control"]
pub type CDSEL_R = crate::BitReader<CDSEL_A>;
impl CDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CDSEL_A {
        match self.bits {
            false => CDSEL_A::VALUE1,
            true => CDSEL_A::VALUE2,
        }
    }
    #[doc = "P1.10 input pin selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDSEL_A::VALUE1
    }
    #[doc = "Software bit CDSVAL is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDSEL_A::VALUE2
    }
}
#[doc = "Field `CDSEL` writer - SDMMC Card Detection Control"]
pub type CDSEL_W<'a, REG> = crate::BitWriter<'a, REG, CDSEL_A>;
impl<'a, REG> CDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "P1.10 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CDSEL_A::VALUE1)
    }
    #[doc = "Software bit CDSVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CDSEL_A::VALUE2)
    }
}
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDSVAL_A {
    #[doc = "0: No card detected"]
    VALUE1 = 0,
    #[doc = "1: Card detected"]
    VALUE2 = 1,
}
impl From<CDSVAL_A> for bool {
    #[inline(always)]
    fn from(variant: CDSVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDSVAL` reader - SDMMC Write Protect Software Control"]
pub type CDSVAL_R = crate::BitReader<CDSVAL_A>;
impl CDSVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CDSVAL_A {
        match self.bits {
            false => CDSVAL_A::VALUE1,
            true => CDSVAL_A::VALUE2,
        }
    }
    #[doc = "No card detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDSVAL_A::VALUE1
    }
    #[doc = "Card detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDSVAL_A::VALUE2
    }
}
#[doc = "Field `CDSVAL` writer - SDMMC Write Protect Software Control"]
pub type CDSVAL_W<'a, REG> = crate::BitWriter<'a, REG, CDSVAL_A>;
impl<'a, REG> CDSVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No card detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CDSVAL_A::VALUE1)
    }
    #[doc = "Card detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CDSVAL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    pub fn wpsel(&self) -> WPSEL_R {
        WPSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn wpsval(&self) -> WPSVAL_R {
        WPSVAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    pub fn cdsel(&self) -> CDSEL_R {
        CDSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn cdsval(&self) -> CDSVAL_R {
        CDSVAL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    #[must_use]
    pub fn wpsel(&mut self) -> WPSEL_W<SDMMC_CON_SPEC> {
        WPSEL_W::new(self, 0)
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    #[must_use]
    pub fn wpsval(&mut self) -> WPSVAL_W<SDMMC_CON_SPEC> {
        WPSVAL_W::new(self, 4)
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    #[must_use]
    pub fn cdsel(&mut self) -> CDSEL_W<SDMMC_CON_SPEC> {
        CDSEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    #[must_use]
    pub fn cdsval(&mut self) -> CDSVAL_W<SDMMC_CON_SPEC> {
        CDSVAL_W::new(self, 20)
    }
}
#[doc = "SDMMC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_CON_SPEC;
impl crate::RegisterSpec for SDMMC_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_con::R`](R) reader structure"]
impl crate::Readable for SDMMC_CON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_con::W`](W) writer structure"]
impl crate::Writable for SDMMC_CON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_CON to value 0"]
impl crate::Resettable for SDMMC_CON_SPEC {
    const RESET_VALUE: u32 = 0;
}
