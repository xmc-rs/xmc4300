#[doc = "Register `HOST_CTRL` reader"]
pub type R = crate::R<HostCtrlSpec>;
#[doc = "Register `HOST_CTRL` writer"]
pub type W = crate::W<HostCtrlSpec>;
#[doc = "LED Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LedCtrl {
    #[doc = "0: LED off"]
    Value1 = 0,
    #[doc = "1: LED on"]
    Value2 = 1,
}
impl From<LedCtrl> for bool {
    #[inline(always)]
    fn from(variant: LedCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LED_CTRL` reader - LED Control"]
pub type LedCtrlR = crate::BitReader<LedCtrl>;
impl LedCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LedCtrl {
        match self.bits {
            false => LedCtrl::Value1,
            true => LedCtrl::Value2,
        }
    }
    #[doc = "LED off"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LedCtrl::Value1
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LedCtrl::Value2
    }
}
#[doc = "Field `LED_CTRL` writer - LED Control"]
pub type LedCtrlW<'a, REG> = crate::BitWriter<'a, REG, LedCtrl>;
impl<'a, REG> LedCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LED off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LedCtrl::Value1)
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LedCtrl::Value2)
    }
}
#[doc = "Data Transfer Width (SD1 or SD4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTxWidth {
    #[doc = "0: 1 bit mode"]
    Value1 = 0,
    #[doc = "1: 4-bit mode"]
    Value2 = 1,
}
impl From<DataTxWidth> for bool {
    #[inline(always)]
    fn from(variant: DataTxWidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_TX_WIDTH` reader - Data Transfer Width (SD1 or SD4)"]
pub type DataTxWidthR = crate::BitReader<DataTxWidth>;
impl DataTxWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataTxWidth {
        match self.bits {
            false => DataTxWidth::Value1,
            true => DataTxWidth::Value2,
        }
    }
    #[doc = "1 bit mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DataTxWidth::Value1
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DataTxWidth::Value2
    }
}
#[doc = "Field `DATA_TX_WIDTH` writer - Data Transfer Width (SD1 or SD4)"]
pub type DataTxWidthW<'a, REG> = crate::BitWriter<'a, REG, DataTxWidth>;
impl<'a, REG> DataTxWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 bit mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DataTxWidth::Value1)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DataTxWidth::Value2)
    }
}
#[doc = "High Speed Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighSpeedEn {
    #[doc = "0: Normal Speed Mode"]
    Value1 = 0,
    #[doc = "1: High Speed Mode"]
    Value2 = 1,
}
impl From<HighSpeedEn> for bool {
    #[inline(always)]
    fn from(variant: HighSpeedEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGH_SPEED_EN` reader - High Speed Enable"]
pub type HighSpeedEnR = crate::BitReader<HighSpeedEn>;
impl HighSpeedEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HighSpeedEn {
        match self.bits {
            false => HighSpeedEn::Value1,
            true => HighSpeedEn::Value2,
        }
    }
    #[doc = "Normal Speed Mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HighSpeedEn::Value1
    }
    #[doc = "High Speed Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HighSpeedEn::Value2
    }
}
#[doc = "Field `HIGH_SPEED_EN` writer - High Speed Enable"]
pub type HighSpeedEnW<'a, REG> = crate::BitWriter<'a, REG, HighSpeedEn>;
impl<'a, REG> HighSpeedEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Speed Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HighSpeedEn::Value1)
    }
    #[doc = "High Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HighSpeedEn::Value2)
    }
}
#[doc = "Extended Data Transfer Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sd8bitMode {
    #[doc = "0: Bus Width is selected by Data Transfer Width"]
    Value1 = 0,
    #[doc = "1: 8-bit Bus Width"]
    Value2 = 1,
}
impl From<Sd8bitMode> for bool {
    #[inline(always)]
    fn from(variant: Sd8bitMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SD_8BIT_MODE` reader - Extended Data Transfer Width"]
pub type Sd8bitModeR = crate::BitReader<Sd8bitMode>;
impl Sd8bitModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sd8bitMode {
        match self.bits {
            false => Sd8bitMode::Value1,
            true => Sd8bitMode::Value2,
        }
    }
    #[doc = "Bus Width is selected by Data Transfer Width"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sd8bitMode::Value1
    }
    #[doc = "8-bit Bus Width"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sd8bitMode::Value2
    }
}
#[doc = "Field `SD_8BIT_MODE` writer - Extended Data Transfer Width"]
pub type Sd8bitModeW<'a, REG> = crate::BitWriter<'a, REG, Sd8bitMode>;
impl<'a, REG> Sd8bitModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus Width is selected by Data Transfer Width"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sd8bitMode::Value1)
    }
    #[doc = "8-bit Bus Width"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sd8bitMode::Value2)
    }
}
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardDetectTestLevel {
    #[doc = "0: No Card"]
    Value1 = 0,
    #[doc = "1: Card Inserted"]
    Value2 = 1,
}
impl From<CardDetectTestLevel> for bool {
    #[inline(always)]
    fn from(variant: CardDetectTestLevel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_DETECT_TEST_LEVEL` reader - Card Detect Test Level"]
pub type CardDetectTestLevelR = crate::BitReader<CardDetectTestLevel>;
impl CardDetectTestLevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardDetectTestLevel {
        match self.bits {
            false => CardDetectTestLevel::Value1,
            true => CardDetectTestLevel::Value2,
        }
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardDetectTestLevel::Value1
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardDetectTestLevel::Value2
    }
}
#[doc = "Field `CARD_DETECT_TEST_LEVEL` writer - Card Detect Test Level"]
pub type CardDetectTestLevelW<'a, REG> = crate::BitWriter<'a, REG, CardDetectTestLevel>;
impl<'a, REG> CardDetectTestLevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Card"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CardDetectTestLevel::Value1)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CardDetectTestLevel::Value2)
    }
}
#[doc = "Card detect signal detetction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardDetSignalDetect {
    #[doc = "0: SDCD is selected (for normal use)"]
    Value1 = 0,
    #[doc = "1: The card detect test level is selected"]
    Value2 = 1,
}
impl From<CardDetSignalDetect> for bool {
    #[inline(always)]
    fn from(variant: CardDetSignalDetect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_DET_SIGNAL_DETECT` reader - Card detect signal detetction"]
pub type CardDetSignalDetectR = crate::BitReader<CardDetSignalDetect>;
impl CardDetSignalDetectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardDetSignalDetect {
        match self.bits {
            false => CardDetSignalDetect::Value1,
            true => CardDetSignalDetect::Value2,
        }
    }
    #[doc = "SDCD is selected (for normal use)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardDetSignalDetect::Value1
    }
    #[doc = "The card detect test level is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardDetSignalDetect::Value2
    }
}
#[doc = "Field `CARD_DET_SIGNAL_DETECT` writer - Card detect signal detetction"]
pub type CardDetSignalDetectW<'a, REG> = crate::BitWriter<'a, REG, CardDetSignalDetect>;
impl<'a, REG> CardDetSignalDetectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDCD is selected (for normal use)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CardDetSignalDetect::Value1)
    }
    #[doc = "The card detect test level is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CardDetSignalDetect::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn led_ctrl(&self) -> LedCtrlR {
        LedCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width (SD1 or SD4)"]
    #[inline(always)]
    pub fn data_tx_width(&self) -> DataTxWidthR {
        DataTxWidthR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn high_speed_en(&self) -> HighSpeedEnR {
        HighSpeedEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    pub fn sd_8bit_mode(&self) -> Sd8bitModeR {
        Sd8bitModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn card_detect_test_level(&self) -> CardDetectTestLevelR {
        CardDetectTestLevelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card detect signal detetction"]
    #[inline(always)]
    pub fn card_det_signal_detect(&self) -> CardDetSignalDetectR {
        CardDetSignalDetectR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    #[must_use]
    pub fn led_ctrl(&mut self) -> LedCtrlW<HostCtrlSpec> {
        LedCtrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Transfer Width (SD1 or SD4)"]
    #[inline(always)]
    #[must_use]
    pub fn data_tx_width(&mut self) -> DataTxWidthW<HostCtrlSpec> {
        DataTxWidthW::new(self, 1)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn high_speed_en(&mut self) -> HighSpeedEnW<HostCtrlSpec> {
        HighSpeedEnW::new(self, 2)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn sd_8bit_mode(&mut self) -> Sd8bitModeW<HostCtrlSpec> {
        Sd8bitModeW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    #[must_use]
    pub fn card_detect_test_level(&mut self) -> CardDetectTestLevelW<HostCtrlSpec> {
        CardDetectTestLevelW::new(self, 6)
    }
    #[doc = "Bit 7 - Card detect signal detetction"]
    #[inline(always)]
    #[must_use]
    pub fn card_det_signal_detect(&mut self) -> CardDetSignalDetectW<HostCtrlSpec> {
        CardDetSignalDetectW::new(self, 7)
    }
}
#[doc = "Host Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCtrlSpec;
impl crate::RegisterSpec for HostCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`host_ctrl::R`](R) reader structure"]
impl crate::Readable for HostCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`host_ctrl::W`](W) writer structure"]
impl crate::Writable for HostCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOST_CTRL to value 0"]
impl crate::Resettable for HostCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
