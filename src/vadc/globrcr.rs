#[doc = "Register `GLOBRCR` reader"]
pub type R = crate::R<GLOBRCR_SPEC>;
#[doc = "Register `GLOBRCR` writer"]
pub type W = crate::W<GLOBRCR_SPEC>;
#[doc = "Data Reduction Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRCTR_A {
    #[doc = "0: Data reduction disabled"]
    VALUE1 = 0,
}
impl From<DRCTR_A> for u8 {
    #[inline(always)]
    fn from(variant: DRCTR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DRCTR_A {
    type Ux = u8;
}
impl crate::IsEnum for DRCTR_A {}
#[doc = "Field `DRCTR` reader - Data Reduction Control"]
pub type DRCTR_R = crate::FieldReader<DRCTR_A>;
impl DRCTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DRCTR_A> {
        match self.bits {
            0 => Some(DRCTR_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Data reduction disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DRCTR_A::VALUE1
    }
}
#[doc = "Field `DRCTR` writer - Data Reduction Control"]
pub type DRCTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DRCTR_A>;
impl<'a, REG> DRCTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data reduction disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DRCTR_A::VALUE1)
    }
}
#[doc = "Wait-for-Read Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WFR_A {
    #[doc = "0: Overwrite mode"]
    VALUE1 = 0,
    #[doc = "1: Wait-for-read mode enabled for this register"]
    VALUE2 = 1,
}
impl From<WFR_A> for bool {
    #[inline(always)]
    fn from(variant: WFR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WFR` reader - Wait-for-Read Mode Enable"]
pub type WFR_R = crate::BitReader<WFR_A>;
impl WFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WFR_A {
        match self.bits {
            false => WFR_A::VALUE1,
            true => WFR_A::VALUE2,
        }
    }
    #[doc = "Overwrite mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WFR_A::VALUE1
    }
    #[doc = "Wait-for-read mode enabled for this register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WFR_A::VALUE2
    }
}
#[doc = "Field `WFR` writer - Wait-for-Read Mode Enable"]
pub type WFR_W<'a, REG> = crate::BitWriter<'a, REG, WFR_A>;
impl<'a, REG> WFR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overwrite mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WFR_A::VALUE1)
    }
    #[doc = "Wait-for-read mode enabled for this register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WFR_A::VALUE2)
    }
}
#[doc = "Service Request Generation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRGEN_A {
    #[doc = "0: No service request"]
    VALUE1 = 0,
    #[doc = "1: Service request after a result event"]
    VALUE2 = 1,
}
impl From<SRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRGEN` reader - Service Request Generation Enable"]
pub type SRGEN_R = crate::BitReader<SRGEN_A>;
impl SRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRGEN_A {
        match self.bits {
            false => SRGEN_A::VALUE1,
            true => SRGEN_A::VALUE2,
        }
    }
    #[doc = "No service request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRGEN_A::VALUE1
    }
    #[doc = "Service request after a result event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRGEN_A::VALUE2
    }
}
#[doc = "Field `SRGEN` writer - Service Request Generation Enable"]
pub type SRGEN_W<'a, REG> = crate::BitWriter<'a, REG, SRGEN_A>;
impl<'a, REG> SRGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No service request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SRGEN_A::VALUE1)
    }
    #[doc = "Service request after a result event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SRGEN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 16:19 - Data Reduction Control"]
    #[inline(always)]
    pub fn drctr(&self) -> DRCTR_R {
        DRCTR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline(always)]
    pub fn srgen(&self) -> SRGEN_R {
        SRGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19 - Data Reduction Control"]
    #[inline(always)]
    #[must_use]
    pub fn drctr(&mut self) -> DRCTR_W<GLOBRCR_SPEC> {
        DRCTR_W::new(self, 16)
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wfr(&mut self) -> WFR_W<GLOBRCR_SPEC> {
        WFR_W::new(self, 24)
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srgen(&mut self) -> SRGEN_W<GLOBRCR_SPEC> {
        SRGEN_W::new(self, 31)
    }
}
#[doc = "Global Result Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`globrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`globrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBRCR_SPEC;
impl crate::RegisterSpec for GLOBRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globrcr::R`](R) reader structure"]
impl crate::Readable for GLOBRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`globrcr::W`](W) writer structure"]
impl crate::Writable for GLOBRCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBRCR to value 0"]
impl crate::Resettable for GLOBRCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
