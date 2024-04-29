#[doc = "Register `DTC` reader"]
pub type R = crate::R<DTC_SPEC>;
#[doc = "Register `DTC` writer"]
pub type W = crate::W<DTC_SPEC>;
#[doc = "Dead Time Enable for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTE1_A {
    #[doc = "0: Dead Time for channel 1 is disabled"]
    VALUE1 = 0,
    #[doc = "1: Dead Time for channel 1 is enabled"]
    VALUE2 = 1,
}
impl From<DTE1_A> for bool {
    #[inline(always)]
    fn from(variant: DTE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE1` reader - Dead Time Enable for Channel 1"]
pub type DTE1_R = crate::BitReader<DTE1_A>;
impl DTE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTE1_A {
        match self.bits {
            false => DTE1_A::VALUE1,
            true => DTE1_A::VALUE2,
        }
    }
    #[doc = "Dead Time for channel 1 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DTE1_A::VALUE1
    }
    #[doc = "Dead Time for channel 1 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTE1_A::VALUE2
    }
}
#[doc = "Field `DTE1` writer - Dead Time Enable for Channel 1"]
pub type DTE1_W<'a, REG> = crate::BitWriter<'a, REG, DTE1_A>;
impl<'a, REG> DTE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for channel 1 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DTE1_A::VALUE1)
    }
    #[doc = "Dead Time for channel 1 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DTE1_A::VALUE2)
    }
}
#[doc = "Dead Time Enable for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTE2_A {
    #[doc = "0: Dead Time for channel 2 is disabled"]
    VALUE1 = 0,
    #[doc = "1: Dead Time for channel 2 is enabled"]
    VALUE2 = 1,
}
impl From<DTE2_A> for bool {
    #[inline(always)]
    fn from(variant: DTE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE2` reader - Dead Time Enable for Channel 2"]
pub type DTE2_R = crate::BitReader<DTE2_A>;
impl DTE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTE2_A {
        match self.bits {
            false => DTE2_A::VALUE1,
            true => DTE2_A::VALUE2,
        }
    }
    #[doc = "Dead Time for channel 2 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DTE2_A::VALUE1
    }
    #[doc = "Dead Time for channel 2 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTE2_A::VALUE2
    }
}
#[doc = "Field `DTE2` writer - Dead Time Enable for Channel 2"]
pub type DTE2_W<'a, REG> = crate::BitWriter<'a, REG, DTE2_A>;
impl<'a, REG> DTE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for channel 2 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DTE2_A::VALUE1)
    }
    #[doc = "Dead Time for channel 2 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DTE2_A::VALUE2)
    }
}
#[doc = "Dead Time Enable for CC8yST1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN1_A {
    #[doc = "0: Dead Time for CC8yST1 path is disabled"]
    VALUE1 = 0,
    #[doc = "1: Dead Time for CC8yST1 path is enabled"]
    VALUE2 = 1,
}
impl From<DCEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DCEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN1` reader - Dead Time Enable for CC8yST1"]
pub type DCEN1_R = crate::BitReader<DCEN1_A>;
impl DCEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCEN1_A {
        match self.bits {
            false => DCEN1_A::VALUE1,
            true => DCEN1_A::VALUE2,
        }
    }
    #[doc = "Dead Time for CC8yST1 path is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCEN1_A::VALUE1
    }
    #[doc = "Dead Time for CC8yST1 path is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCEN1_A::VALUE2
    }
}
#[doc = "Field `DCEN1` writer - Dead Time Enable for CC8yST1"]
pub type DCEN1_W<'a, REG> = crate::BitWriter<'a, REG, DCEN1_A>;
impl<'a, REG> DCEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for CC8yST1 path is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN1_A::VALUE1)
    }
    #[doc = "Dead Time for CC8yST1 path is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN1_A::VALUE2)
    }
}
#[doc = "Dead Time Enable for inverted CC8yST1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN2_A {
    #[doc = "0: Dead Time for inverted CC8yST1 path is disabled"]
    VALUE1 = 0,
    #[doc = "1: Dead Time for inverted CC8yST1 path is enabled"]
    VALUE2 = 1,
}
impl From<DCEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DCEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN2` reader - Dead Time Enable for inverted CC8yST1"]
pub type DCEN2_R = crate::BitReader<DCEN2_A>;
impl DCEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCEN2_A {
        match self.bits {
            false => DCEN2_A::VALUE1,
            true => DCEN2_A::VALUE2,
        }
    }
    #[doc = "Dead Time for inverted CC8yST1 path is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCEN2_A::VALUE1
    }
    #[doc = "Dead Time for inverted CC8yST1 path is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCEN2_A::VALUE2
    }
}
#[doc = "Field `DCEN2` writer - Dead Time Enable for inverted CC8yST1"]
pub type DCEN2_W<'a, REG> = crate::BitWriter<'a, REG, DCEN2_A>;
impl<'a, REG> DCEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for inverted CC8yST1 path is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN2_A::VALUE1)
    }
    #[doc = "Dead Time for inverted CC8yST1 path is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN2_A::VALUE2)
    }
}
#[doc = "Dead Time Enable for CC8yST2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN3_A {
    #[doc = "0: Dead Time for CC8yST2 path is disabled"]
    VALUE1 = 0,
    #[doc = "1: Dead Time for CC8yST2 path is enabled"]
    VALUE2 = 1,
}
impl From<DCEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DCEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN3` reader - Dead Time Enable for CC8yST2"]
pub type DCEN3_R = crate::BitReader<DCEN3_A>;
impl DCEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCEN3_A {
        match self.bits {
            false => DCEN3_A::VALUE1,
            true => DCEN3_A::VALUE2,
        }
    }
    #[doc = "Dead Time for CC8yST2 path is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCEN3_A::VALUE1
    }
    #[doc = "Dead Time for CC8yST2 path is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCEN3_A::VALUE2
    }
}
#[doc = "Field `DCEN3` writer - Dead Time Enable for CC8yST2"]
pub type DCEN3_W<'a, REG> = crate::BitWriter<'a, REG, DCEN3_A>;
impl<'a, REG> DCEN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for CC8yST2 path is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN3_A::VALUE1)
    }
    #[doc = "Dead Time for CC8yST2 path is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN3_A::VALUE2)
    }
}
#[doc = "Dead Time Enable for inverted CC8yST2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN4_A {
    #[doc = "0: Dead Time for inverted CC8yST2 path is disabled"]
    VALUE1 = 0,
    #[doc = "1: Dead Time for inverted CC8yST2 path is enabled"]
    VALUE2 = 1,
}
impl From<DCEN4_A> for bool {
    #[inline(always)]
    fn from(variant: DCEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN4` reader - Dead Time Enable for inverted CC8yST2"]
pub type DCEN4_R = crate::BitReader<DCEN4_A>;
impl DCEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCEN4_A {
        match self.bits {
            false => DCEN4_A::VALUE1,
            true => DCEN4_A::VALUE2,
        }
    }
    #[doc = "Dead Time for inverted CC8yST2 path is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCEN4_A::VALUE1
    }
    #[doc = "Dead Time for inverted CC8yST2 path is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCEN4_A::VALUE2
    }
}
#[doc = "Field `DCEN4` writer - Dead Time Enable for inverted CC8yST2"]
pub type DCEN4_W<'a, REG> = crate::BitWriter<'a, REG, DCEN4_A>;
impl<'a, REG> DCEN4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for inverted CC8yST2 path is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN4_A::VALUE1)
    }
    #[doc = "Dead Time for inverted CC8yST2 path is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN4_A::VALUE2)
    }
}
#[doc = "Dead Time clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTCC_A {
    #[doc = "0: ftclk"]
    VALUE1 = 0,
    #[doc = "1: ftclk/2"]
    VALUE2 = 1,
    #[doc = "2: ftclk/4"]
    VALUE3 = 2,
    #[doc = "3: ftclk/8"]
    VALUE4 = 3,
}
impl From<DTCC_A> for u8 {
    #[inline(always)]
    fn from(variant: DTCC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTCC_A {
    type Ux = u8;
}
impl crate::IsEnum for DTCC_A {}
#[doc = "Field `DTCC` reader - Dead Time clock control"]
pub type DTCC_R = crate::FieldReader<DTCC_A>;
impl DTCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTCC_A {
        match self.bits {
            0 => DTCC_A::VALUE1,
            1 => DTCC_A::VALUE2,
            2 => DTCC_A::VALUE3,
            3 => DTCC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "ftclk"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DTCC_A::VALUE1
    }
    #[doc = "ftclk/2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTCC_A::VALUE2
    }
    #[doc = "ftclk/4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DTCC_A::VALUE3
    }
    #[doc = "ftclk/8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DTCC_A::VALUE4
    }
}
#[doc = "Field `DTCC` writer - Dead Time clock control"]
pub type DTCC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DTCC_A, crate::Safe>;
impl<'a, REG> DTCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ftclk"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DTCC_A::VALUE1)
    }
    #[doc = "ftclk/2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DTCC_A::VALUE2)
    }
    #[doc = "ftclk/4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DTCC_A::VALUE3)
    }
    #[doc = "ftclk/8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DTCC_A::VALUE4)
    }
}
impl R {
    #[doc = "Bit 0 - Dead Time Enable for Channel 1"]
    #[inline(always)]
    pub fn dte1(&self) -> DTE1_R {
        DTE1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dead Time Enable for Channel 2"]
    #[inline(always)]
    pub fn dte2(&self) -> DTE2_R {
        DTE2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dead Time Enable for CC8yST1"]
    #[inline(always)]
    pub fn dcen1(&self) -> DCEN1_R {
        DCEN1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dead Time Enable for inverted CC8yST1"]
    #[inline(always)]
    pub fn dcen2(&self) -> DCEN2_R {
        DCEN2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Dead Time Enable for CC8yST2"]
    #[inline(always)]
    pub fn dcen3(&self) -> DCEN3_R {
        DCEN3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dead Time Enable for inverted CC8yST2"]
    #[inline(always)]
    pub fn dcen4(&self) -> DCEN4_R {
        DCEN4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Dead Time clock control"]
    #[inline(always)]
    pub fn dtcc(&self) -> DTCC_R {
        DTCC_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dead Time Enable for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dte1(&mut self) -> DTE1_W<DTC_SPEC> {
        DTE1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Dead Time Enable for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn dte2(&mut self) -> DTE2_W<DTC_SPEC> {
        DTE2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Dead Time Enable for CC8yST1"]
    #[inline(always)]
    #[must_use]
    pub fn dcen1(&mut self) -> DCEN1_W<DTC_SPEC> {
        DCEN1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Dead Time Enable for inverted CC8yST1"]
    #[inline(always)]
    #[must_use]
    pub fn dcen2(&mut self) -> DCEN2_W<DTC_SPEC> {
        DCEN2_W::new(self, 3)
    }
    #[doc = "Bit 4 - Dead Time Enable for CC8yST2"]
    #[inline(always)]
    #[must_use]
    pub fn dcen3(&mut self) -> DCEN3_W<DTC_SPEC> {
        DCEN3_W::new(self, 4)
    }
    #[doc = "Bit 5 - Dead Time Enable for inverted CC8yST2"]
    #[inline(always)]
    #[must_use]
    pub fn dcen4(&mut self) -> DCEN4_W<DTC_SPEC> {
        DCEN4_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Dead Time clock control"]
    #[inline(always)]
    #[must_use]
    pub fn dtcc(&mut self) -> DTCC_W<DTC_SPEC> {
        DTCC_W::new(self, 6)
    }
}
#[doc = "Dead Time Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTC_SPEC;
impl crate::RegisterSpec for DTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtc::R`](R) reader structure"]
impl crate::Readable for DTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtc::W`](W) writer structure"]
impl crate::Writable for DTC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTC to value 0"]
impl crate::Resettable for DTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
