#[doc = "Register `RCR[%s]` reader"]
pub type R = crate::R<RCR_SPEC>;
#[doc = "Register `RCR[%s]` writer"]
pub type W = crate::W<RCR_SPEC>;
#[doc = "Field `DRCTR` reader - Data Reduction Control"]
pub type DRCTR_R = crate::FieldReader;
#[doc = "Field `DRCTR` writer - Data Reduction Control"]
pub type DRCTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMM` reader - Data Modification Mode"]
pub type DMM_R = crate::FieldReader<DMM_A>;
#[doc = "Data Modification Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMM_A {
    #[doc = "0: Standard data reduction (accumulation)"]
    VALUE1 = 0,
    #[doc = "1: Result filtering mode"]
    VALUE2 = 1,
    #[doc = "2: Difference mode"]
    VALUE3 = 2,
}
impl From<DMM_A> for u8 {
    #[inline(always)]
    fn from(variant: DMM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMM_A {
    type Ux = u8;
}
impl DMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMM_A> {
        match self.bits {
            0 => Some(DMM_A::VALUE1),
            1 => Some(DMM_A::VALUE2),
            2 => Some(DMM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Standard data reduction (accumulation)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMM_A::VALUE1
    }
    #[doc = "Result filtering mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMM_A::VALUE2
    }
    #[doc = "Difference mode"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DMM_A::VALUE3
    }
}
#[doc = "Field `DMM` writer - Data Modification Mode"]
pub type DMM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DMM_A>;
impl<'a, REG> DMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard data reduction (accumulation)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMM_A::VALUE1)
    }
    #[doc = "Result filtering mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMM_A::VALUE2)
    }
    #[doc = "Difference mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DMM_A::VALUE3)
    }
}
#[doc = "Field `WFR` reader - Wait-for-Read Mode Enable"]
pub type WFR_R = crate::BitReader<WFR_A>;
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
#[doc = "Field `FEN` reader - FIFO Mode Enable"]
pub type FEN_R = crate::FieldReader<FEN_A>;
#[doc = "FIFO Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FEN_A {
    #[doc = "0: Separate result register"]
    VALUE1 = 0,
    #[doc = "1: Part of a FIFO structure: copy each new valid result"]
    VALUE2 = 1,
}
impl From<FEN_A> for u8 {
    #[inline(always)]
    fn from(variant: FEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FEN_A {
    type Ux = u8;
}
impl FEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FEN_A> {
        match self.bits {
            0 => Some(FEN_A::VALUE1),
            1 => Some(FEN_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Separate result register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FEN_A::VALUE1
    }
    #[doc = "Part of a FIFO structure: copy each new valid result"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FEN_A::VALUE2
    }
}
#[doc = "Field `FEN` writer - FIFO Mode Enable"]
pub type FEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FEN_A>;
impl<'a, REG> FEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Separate result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FEN_A::VALUE1)
    }
    #[doc = "Part of a FIFO structure: copy each new valid result"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FEN_A::VALUE2)
    }
}
#[doc = "Field `SRGEN` reader - Service Request Generation Enable"]
pub type SRGEN_R = crate::BitReader<SRGEN_A>;
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
    #[doc = "Bits 20:21 - Data Modification Mode"]
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - FIFO Mode Enable"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 25) & 3) as u8)
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
    pub fn drctr(&mut self) -> DRCTR_W<RCR_SPEC> {
        DRCTR_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Data Modification Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmm(&mut self) -> DMM_W<RCR_SPEC> {
        DMM_W::new(self, 20)
    }
    #[doc = "Bit 24 - Wait-for-Read Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wfr(&mut self) -> WFR_W<RCR_SPEC> {
        WFR_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - FIFO Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FEN_W<RCR_SPEC> {
        FEN_W::new(self, 25)
    }
    #[doc = "Bit 31 - Service Request Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srgen(&mut self) -> SRGEN_W<RCR_SPEC> {
        SRGEN_W::new(self, 31)
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
#[doc = "Result Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR[%s]
to value 0"]
impl crate::Resettable for RCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
