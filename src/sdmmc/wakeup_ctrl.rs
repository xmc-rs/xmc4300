#[doc = "Register `WAKEUP_CTRL` reader"]
pub type R = crate::R<WakeupCtrlSpec>;
#[doc = "Register `WAKEUP_CTRL` writer"]
pub type W = crate::W<WakeupCtrlSpec>;
#[doc = "Wakeup Event Enable On Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupEventEnInt {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<WakeupEventEnInt> for bool {
    #[inline(always)]
    fn from(variant: WakeupEventEnInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INT` reader - Wakeup Event Enable On Card Interrupt"]
pub type WakeupEventEnIntR = crate::BitReader<WakeupEventEnInt>;
impl WakeupEventEnIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupEventEnInt {
        match self.bits {
            false => WakeupEventEnInt::Value1,
            true => WakeupEventEnInt::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WakeupEventEnInt::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WakeupEventEnInt::Value2
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INT` writer - Wakeup Event Enable On Card Interrupt"]
pub type WakeupEventEnIntW<'a, REG> = crate::BitWriter<'a, REG, WakeupEventEnInt>;
impl<'a, REG> WakeupEventEnIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupEventEnInt::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupEventEnInt::Value2)
    }
}
#[doc = "Wakeup Event Enable On SD Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupEventEnIns {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<WakeupEventEnIns> for bool {
    #[inline(always)]
    fn from(variant: WakeupEventEnIns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INS` reader - Wakeup Event Enable On SD Card Insertion"]
pub type WakeupEventEnInsR = crate::BitReader<WakeupEventEnIns>;
impl WakeupEventEnInsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupEventEnIns {
        match self.bits {
            false => WakeupEventEnIns::Value1,
            true => WakeupEventEnIns::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WakeupEventEnIns::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WakeupEventEnIns::Value2
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INS` writer - Wakeup Event Enable On SD Card Insertion"]
pub type WakeupEventEnInsW<'a, REG> = crate::BitWriter<'a, REG, WakeupEventEnIns>;
impl<'a, REG> WakeupEventEnInsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupEventEnIns::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupEventEnIns::Value2)
    }
}
#[doc = "Wakeup Event Enable On SD Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupEventEnRem {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<WakeupEventEnRem> for bool {
    #[inline(always)]
    fn from(variant: WakeupEventEnRem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_REM` reader - Wakeup Event Enable On SD Card Removal"]
pub type WakeupEventEnRemR = crate::BitReader<WakeupEventEnRem>;
impl WakeupEventEnRemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupEventEnRem {
        match self.bits {
            false => WakeupEventEnRem::Value1,
            true => WakeupEventEnRem::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WakeupEventEnRem::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WakeupEventEnRem::Value2
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_REM` writer - Wakeup Event Enable On SD Card Removal"]
pub type WakeupEventEnRemW<'a, REG> = crate::BitWriter<'a, REG, WakeupEventEnRem>;
impl<'a, REG> WakeupEventEnRemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupEventEnRem::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupEventEnRem::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wakeup_event_en_int(&self) -> WakeupEventEnIntR {
        WakeupEventEnIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wakeup_event_en_ins(&self) -> WakeupEventEnInsR {
        WakeupEventEnInsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wakeup_event_en_rem(&self) -> WakeupEventEnRemR {
        WakeupEventEnRemR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_event_en_int(&mut self) -> WakeupEventEnIntW<WakeupCtrlSpec> {
        WakeupEventEnIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_event_en_ins(&mut self) -> WakeupEventEnInsW<WakeupCtrlSpec> {
        WakeupEventEnInsW::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_event_en_rem(&mut self) -> WakeupEventEnRemW<WakeupCtrlSpec> {
        WakeupEventEnRemW::new(self, 2)
    }
}
#[doc = "Wake-up Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupCtrlSpec;
impl crate::RegisterSpec for WakeupCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wakeup_ctrl::R`](R) reader structure"]
impl crate::Readable for WakeupCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeup_ctrl::W`](W) writer structure"]
impl crate::Writable for WakeupCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WAKEUP_CTRL to value 0"]
impl crate::Resettable for WakeupCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
