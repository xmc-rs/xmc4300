#[doc = "Register `HOST_CTRL` reader"]
pub type R = crate::R<HOST_CTRL_SPEC>;
#[doc = "Register `HOST_CTRL` writer"]
pub type W = crate::W<HOST_CTRL_SPEC>;
#[doc = "LED Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LED_CTRL_A {
    #[doc = "0: LED off"]
    VALUE1 = 0,
    #[doc = "1: LED on"]
    VALUE2 = 1,
}
impl From<LED_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: LED_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LED_CTRL` reader - LED Control"]
pub type LED_CTRL_R = crate::BitReader<LED_CTRL_A>;
impl LED_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LED_CTRL_A {
        match self.bits {
            false => LED_CTRL_A::VALUE1,
            true => LED_CTRL_A::VALUE2,
        }
    }
    #[doc = "LED off"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LED_CTRL_A::VALUE1
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LED_CTRL_A::VALUE2
    }
}
#[doc = "Field `LED_CTRL` writer - LED Control"]
pub type LED_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, LED_CTRL_A>;
impl<'a, REG> LED_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LED off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LED_CTRL_A::VALUE1)
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LED_CTRL_A::VALUE2)
    }
}
#[doc = "Data Transfer Width (SD1 or SD4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_TX_WIDTH_A {
    #[doc = "0: 1 bit mode"]
    VALUE1 = 0,
    #[doc = "1: 4-bit mode"]
    VALUE2 = 1,
}
impl From<DATA_TX_WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_TX_WIDTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_TX_WIDTH` reader - Data Transfer Width (SD1 or SD4)"]
pub type DATA_TX_WIDTH_R = crate::BitReader<DATA_TX_WIDTH_A>;
impl DATA_TX_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA_TX_WIDTH_A {
        match self.bits {
            false => DATA_TX_WIDTH_A::VALUE1,
            true => DATA_TX_WIDTH_A::VALUE2,
        }
    }
    #[doc = "1 bit mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATA_TX_WIDTH_A::VALUE1
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATA_TX_WIDTH_A::VALUE2
    }
}
#[doc = "Field `DATA_TX_WIDTH` writer - Data Transfer Width (SD1 or SD4)"]
pub type DATA_TX_WIDTH_W<'a, REG> = crate::BitWriter<'a, REG, DATA_TX_WIDTH_A>;
impl<'a, REG> DATA_TX_WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 bit mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_TX_WIDTH_A::VALUE1)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_TX_WIDTH_A::VALUE2)
    }
}
#[doc = "High Speed Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIGH_SPEED_EN_A {
    #[doc = "0: Normal Speed Mode"]
    VALUE1 = 0,
    #[doc = "1: High Speed Mode"]
    VALUE2 = 1,
}
impl From<HIGH_SPEED_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HIGH_SPEED_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGH_SPEED_EN` reader - High Speed Enable"]
pub type HIGH_SPEED_EN_R = crate::BitReader<HIGH_SPEED_EN_A>;
impl HIGH_SPEED_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIGH_SPEED_EN_A {
        match self.bits {
            false => HIGH_SPEED_EN_A::VALUE1,
            true => HIGH_SPEED_EN_A::VALUE2,
        }
    }
    #[doc = "Normal Speed Mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIGH_SPEED_EN_A::VALUE1
    }
    #[doc = "High Speed Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIGH_SPEED_EN_A::VALUE2
    }
}
#[doc = "Field `HIGH_SPEED_EN` writer - High Speed Enable"]
pub type HIGH_SPEED_EN_W<'a, REG> = crate::BitWriter<'a, REG, HIGH_SPEED_EN_A>;
impl<'a, REG> HIGH_SPEED_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Speed Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIGH_SPEED_EN_A::VALUE1)
    }
    #[doc = "High Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIGH_SPEED_EN_A::VALUE2)
    }
}
#[doc = "Extended Data Transfer Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SD_8BIT_MODE_A {
    #[doc = "0: Bus Width is selected by Data Transfer Width"]
    VALUE1 = 0,
    #[doc = "1: 8-bit Bus Width"]
    VALUE2 = 1,
}
impl From<SD_8BIT_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: SD_8BIT_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SD_8BIT_MODE` reader - Extended Data Transfer Width"]
pub type SD_8BIT_MODE_R = crate::BitReader<SD_8BIT_MODE_A>;
impl SD_8BIT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SD_8BIT_MODE_A {
        match self.bits {
            false => SD_8BIT_MODE_A::VALUE1,
            true => SD_8BIT_MODE_A::VALUE2,
        }
    }
    #[doc = "Bus Width is selected by Data Transfer Width"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SD_8BIT_MODE_A::VALUE1
    }
    #[doc = "8-bit Bus Width"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SD_8BIT_MODE_A::VALUE2
    }
}
#[doc = "Field `SD_8BIT_MODE` writer - Extended Data Transfer Width"]
pub type SD_8BIT_MODE_W<'a, REG> = crate::BitWriter<'a, REG, SD_8BIT_MODE_A>;
impl<'a, REG> SD_8BIT_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus Width is selected by Data Transfer Width"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SD_8BIT_MODE_A::VALUE1)
    }
    #[doc = "8-bit Bus Width"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SD_8BIT_MODE_A::VALUE2)
    }
}
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_DETECT_TEST_LEVEL_A {
    #[doc = "0: No Card"]
    VALUE1 = 0,
    #[doc = "1: Card Inserted"]
    VALUE2 = 1,
}
impl From<CARD_DETECT_TEST_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_DETECT_TEST_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_DETECT_TEST_LEVEL` reader - Card Detect Test Level"]
pub type CARD_DETECT_TEST_LEVEL_R = crate::BitReader<CARD_DETECT_TEST_LEVEL_A>;
impl CARD_DETECT_TEST_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_DETECT_TEST_LEVEL_A {
        match self.bits {
            false => CARD_DETECT_TEST_LEVEL_A::VALUE1,
            true => CARD_DETECT_TEST_LEVEL_A::VALUE2,
        }
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_DETECT_TEST_LEVEL_A::VALUE1
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_DETECT_TEST_LEVEL_A::VALUE2
    }
}
#[doc = "Field `CARD_DETECT_TEST_LEVEL` writer - Card Detect Test Level"]
pub type CARD_DETECT_TEST_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG, CARD_DETECT_TEST_LEVEL_A>;
impl<'a, REG> CARD_DETECT_TEST_LEVEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Card"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_DETECT_TEST_LEVEL_A::VALUE1)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_DETECT_TEST_LEVEL_A::VALUE2)
    }
}
#[doc = "Card detect signal detetction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_DET_SIGNAL_DETECT_A {
    #[doc = "0: SDCD is selected (for normal use)"]
    VALUE1 = 0,
    #[doc = "1: The card detect test level is selected"]
    VALUE2 = 1,
}
impl From<CARD_DET_SIGNAL_DETECT_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_DET_SIGNAL_DETECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_DET_SIGNAL_DETECT` reader - Card detect signal detetction"]
pub type CARD_DET_SIGNAL_DETECT_R = crate::BitReader<CARD_DET_SIGNAL_DETECT_A>;
impl CARD_DET_SIGNAL_DETECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_DET_SIGNAL_DETECT_A {
        match self.bits {
            false => CARD_DET_SIGNAL_DETECT_A::VALUE1,
            true => CARD_DET_SIGNAL_DETECT_A::VALUE2,
        }
    }
    #[doc = "SDCD is selected (for normal use)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CARD_DET_SIGNAL_DETECT_A::VALUE1
    }
    #[doc = "The card detect test level is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CARD_DET_SIGNAL_DETECT_A::VALUE2
    }
}
#[doc = "Field `CARD_DET_SIGNAL_DETECT` writer - Card detect signal detetction"]
pub type CARD_DET_SIGNAL_DETECT_W<'a, REG> = crate::BitWriter<'a, REG, CARD_DET_SIGNAL_DETECT_A>;
impl<'a, REG> CARD_DET_SIGNAL_DETECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDCD is selected (for normal use)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_DET_SIGNAL_DETECT_A::VALUE1)
    }
    #[doc = "The card detect test level is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_DET_SIGNAL_DETECT_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn led_ctrl(&self) -> LED_CTRL_R {
        LED_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width (SD1 or SD4)"]
    #[inline(always)]
    pub fn data_tx_width(&self) -> DATA_TX_WIDTH_R {
        DATA_TX_WIDTH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn high_speed_en(&self) -> HIGH_SPEED_EN_R {
        HIGH_SPEED_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    pub fn sd_8bit_mode(&self) -> SD_8BIT_MODE_R {
        SD_8BIT_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn card_detect_test_level(&self) -> CARD_DETECT_TEST_LEVEL_R {
        CARD_DETECT_TEST_LEVEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card detect signal detetction"]
    #[inline(always)]
    pub fn card_det_signal_detect(&self) -> CARD_DET_SIGNAL_DETECT_R {
        CARD_DET_SIGNAL_DETECT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    #[must_use]
    pub fn led_ctrl(&mut self) -> LED_CTRL_W<HOST_CTRL_SPEC> {
        LED_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data Transfer Width (SD1 or SD4)"]
    #[inline(always)]
    #[must_use]
    pub fn data_tx_width(&mut self) -> DATA_TX_WIDTH_W<HOST_CTRL_SPEC> {
        DATA_TX_WIDTH_W::new(self, 1)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn high_speed_en(&mut self) -> HIGH_SPEED_EN_W<HOST_CTRL_SPEC> {
        HIGH_SPEED_EN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn sd_8bit_mode(&mut self) -> SD_8BIT_MODE_W<HOST_CTRL_SPEC> {
        SD_8BIT_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    #[must_use]
    pub fn card_detect_test_level(&mut self) -> CARD_DETECT_TEST_LEVEL_W<HOST_CTRL_SPEC> {
        CARD_DETECT_TEST_LEVEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Card detect signal detetction"]
    #[inline(always)]
    #[must_use]
    pub fn card_det_signal_detect(&mut self) -> CARD_DET_SIGNAL_DETECT_W<HOST_CTRL_SPEC> {
        CARD_DET_SIGNAL_DETECT_W::new(self, 7)
    }
}
#[doc = "Host Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_CTRL_SPEC;
impl crate::RegisterSpec for HOST_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`host_ctrl::R`](R) reader structure"]
impl crate::Readable for HOST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_ctrl::W`](W) writer structure"]
impl crate::Writable for HOST_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOST_CTRL to value 0"]
impl crate::Resettable for HOST_CTRL_SPEC {
    const RESET_VALUE: u8 = 0;
}
