#[doc = "Register `HDSET` writer"]
pub type W = crate::W<HdsetSpec>;
#[doc = "Wake-up Pin Event Positive Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epev {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Set wake-up event"]
    Const1 = 1,
}
impl From<Epev> for bool {
    #[inline(always)]
    fn from(variant: Epev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` writer - Wake-up Pin Event Positive Edge Set"]
pub type EpevW<'a, REG> = crate::BitWriter<'a, REG, Epev>;
impl<'a, REG> EpevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Epev::Const0)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Epev::Const1)
    }
}
#[doc = "Wake-up Pin Event Negative Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enev {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Set wake-up event"]
    Const1 = 1,
}
impl From<Enev> for bool {
    #[inline(always)]
    fn from(variant: Enev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` writer - Wake-up Pin Event Negative Edge Set"]
pub type EnevW<'a, REG> = crate::BitWriter<'a, REG, Enev>;
impl<'a, REG> EnevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enev::Const0)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enev::Const1)
    }
}
#[doc = "RTC Event Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcev {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Set wake-up event"]
    Const1 = 1,
}
impl From<Rtcev> for bool {
    #[inline(always)]
    fn from(variant: Rtcev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` writer - RTC Event Set"]
pub type RtcevW<'a, REG> = crate::BitWriter<'a, REG, Rtcev>;
impl<'a, REG> RtcevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcev::Const0)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcev::Const1)
    }
}
#[doc = "ULP WDG Alarm Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdg {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Set watchdog alarm"]
    Const1 = 1,
}
impl From<Ulpwdg> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` writer - ULP WDG Alarm Set"]
pub type UlpwdgW<'a, REG> = crate::BitWriter<'a, REG, Ulpwdg>;
impl<'a, REG> UlpwdgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdg::Const0)
    }
    #[doc = "Set watchdog alarm"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdg::Const1)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Set"]
    #[inline(always)]
    #[must_use]
    pub fn epev(&mut self) -> EpevW<HdsetSpec> {
        EpevW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Set"]
    #[inline(always)]
    #[must_use]
    pub fn enev(&mut self) -> EnevW<HdsetSpec> {
        EnevW::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Event Set"]
    #[inline(always)]
    #[must_use]
    pub fn rtcev(&mut self) -> RtcevW<HdsetSpec> {
        RtcevW::new(self, 2)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Set"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdg(&mut self) -> UlpwdgW<HdsetSpec> {
        UlpwdgW::new(self, 3)
    }
}
#[doc = "Hibernate Domain Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdsetSpec;
impl crate::RegisterSpec for HdsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdset::W`](W) writer structure"]
impl crate::Writable for HdsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDSET to value 0"]
impl crate::Resettable for HdsetSpec {
    const RESET_VALUE: u32 = 0;
}
