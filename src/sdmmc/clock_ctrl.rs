#[doc = "Register `CLOCK_CTRL` reader"]
pub type R = crate::R<ClockCtrlSpec>;
#[doc = "Register `CLOCK_CTRL` writer"]
pub type W = crate::W<ClockCtrlSpec>;
#[doc = "Internal Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InternalClockEn {
    #[doc = "0: Stop"]
    Value1 = 0,
    #[doc = "1: Oscillate"]
    Value2 = 1,
}
impl From<InternalClockEn> for bool {
    #[inline(always)]
    fn from(variant: InternalClockEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTERNAL_CLOCK_EN` reader - Internal Clock Enable"]
pub type InternalClockEnR = crate::BitReader<InternalClockEn>;
impl InternalClockEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InternalClockEn {
        match self.bits {
            false => InternalClockEn::Value1,
            true => InternalClockEn::Value2,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == InternalClockEn::Value1
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == InternalClockEn::Value2
    }
}
#[doc = "Field `INTERNAL_CLOCK_EN` writer - Internal Clock Enable"]
pub type InternalClockEnW<'a, REG> = crate::BitWriter<'a, REG, InternalClockEn>;
impl<'a, REG> InternalClockEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(InternalClockEn::Value1)
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(InternalClockEn::Value2)
    }
}
#[doc = "Internal Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InternalClockStable {
    #[doc = "0: Not Ready"]
    Value1 = 0,
    #[doc = "1: Ready"]
    Value2 = 1,
}
impl From<InternalClockStable> for bool {
    #[inline(always)]
    fn from(variant: InternalClockStable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTERNAL_CLOCK_STABLE` reader - Internal Clock Stable"]
pub type InternalClockStableR = crate::BitReader<InternalClockStable>;
impl InternalClockStableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InternalClockStable {
        match self.bits {
            false => InternalClockStable::Value1,
            true => InternalClockStable::Value2,
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == InternalClockStable::Value1
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == InternalClockStable::Value2
    }
}
#[doc = "SD Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdclockEn {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<SdclockEn> for bool {
    #[inline(always)]
    fn from(variant: SdclockEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDCLOCK_EN` reader - SD Clock Enable"]
pub type SdclockEnR = crate::BitReader<SdclockEn>;
impl SdclockEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdclockEn {
        match self.bits {
            false => SdclockEn::Value1,
            true => SdclockEn::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SdclockEn::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SdclockEn::Value2
    }
}
#[doc = "Field `SDCLOCK_EN` writer - SD Clock Enable"]
pub type SdclockEnW<'a, REG> = crate::BitWriter<'a, REG, SdclockEn>;
impl<'a, REG> SdclockEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SdclockEn::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SdclockEn::Value2)
    }
}
#[doc = "SDCLK Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SdclkFreqSel {
    #[doc = "0: base clock(10MHz-63MHz)"]
    Value1 = 0,
    #[doc = "1: base clock divided by 2"]
    Value2 = 1,
    #[doc = "16: base clock divided by 32"]
    Value3 = 16,
    #[doc = "2: base clock divided by 4"]
    Value4 = 2,
    #[doc = "4: base clock divided by 8"]
    Value5 = 4,
    #[doc = "8: base clock divided by 16"]
    Value6 = 8,
    #[doc = "32: base clock divided by 64"]
    Value7 = 32,
    #[doc = "64: base clock divided by 128"]
    Value8 = 64,
    #[doc = "128: base clock divided by 256"]
    Value9 = 128,
}
impl From<SdclkFreqSel> for u8 {
    #[inline(always)]
    fn from(variant: SdclkFreqSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SdclkFreqSel {
    type Ux = u8;
}
#[doc = "Field `SDCLK_FREQ_SEL` reader - SDCLK Frequency Select"]
pub type SdclkFreqSelR = crate::FieldReader<SdclkFreqSel>;
impl SdclkFreqSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SdclkFreqSel> {
        match self.bits {
            0 => Some(SdclkFreqSel::Value1),
            1 => Some(SdclkFreqSel::Value2),
            16 => Some(SdclkFreqSel::Value3),
            2 => Some(SdclkFreqSel::Value4),
            4 => Some(SdclkFreqSel::Value5),
            8 => Some(SdclkFreqSel::Value6),
            32 => Some(SdclkFreqSel::Value7),
            64 => Some(SdclkFreqSel::Value8),
            128 => Some(SdclkFreqSel::Value9),
            _ => None,
        }
    }
    #[doc = "base clock(10MHz-63MHz)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SdclkFreqSel::Value1
    }
    #[doc = "base clock divided by 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SdclkFreqSel::Value2
    }
    #[doc = "base clock divided by 32"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SdclkFreqSel::Value3
    }
    #[doc = "base clock divided by 4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SdclkFreqSel::Value4
    }
    #[doc = "base clock divided by 8"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SdclkFreqSel::Value5
    }
    #[doc = "base clock divided by 16"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SdclkFreqSel::Value6
    }
    #[doc = "base clock divided by 64"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SdclkFreqSel::Value7
    }
    #[doc = "base clock divided by 128"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == SdclkFreqSel::Value8
    }
    #[doc = "base clock divided by 256"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == SdclkFreqSel::Value9
    }
}
#[doc = "Field `SDCLK_FREQ_SEL` writer - SDCLK Frequency Select"]
pub type SdclkFreqSelW<'a, REG> = crate::FieldWriter<'a, REG, 8, SdclkFreqSel>;
impl<'a, REG> SdclkFreqSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "base clock(10MHz-63MHz)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SdclkFreqSel::Value1)
    }
    #[doc = "base clock divided by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SdclkFreqSel::Value2)
    }
    #[doc = "base clock divided by 32"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SdclkFreqSel::Value3)
    }
    #[doc = "base clock divided by 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SdclkFreqSel::Value4)
    }
    #[doc = "base clock divided by 8"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(SdclkFreqSel::Value5)
    }
    #[doc = "base clock divided by 16"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(SdclkFreqSel::Value6)
    }
    #[doc = "base clock divided by 64"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(SdclkFreqSel::Value7)
    }
    #[doc = "base clock divided by 128"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(SdclkFreqSel::Value8)
    }
    #[doc = "base clock divided by 256"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(SdclkFreqSel::Value9)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn internal_clock_en(&self) -> InternalClockEnR {
        InternalClockEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn internal_clock_stable(&self) -> InternalClockStableR {
        InternalClockStableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclock_en(&self) -> SdclockEnR {
        SdclockEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclk_freq_sel(&self) -> SdclkFreqSelR {
        SdclkFreqSelR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn internal_clock_en(&mut self) -> InternalClockEnW<ClockCtrlSpec> {
        InternalClockEnW::new(self, 0)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdclock_en(&mut self) -> SdclockEnW<ClockCtrlSpec> {
        SdclockEnW::new(self, 2)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_freq_sel(&mut self) -> SdclkFreqSelW<ClockCtrlSpec> {
        SdclkFreqSelW::new(self, 8)
    }
}
#[doc = "Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockCtrlSpec;
impl crate::RegisterSpec for ClockCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clock_ctrl::R`](R) reader structure"]
impl crate::Readable for ClockCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clock_ctrl::W`](W) writer structure"]
impl crate::Writable for ClockCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CLOCK_CTRL to value 0"]
impl crate::Resettable for ClockCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
