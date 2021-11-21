#[doc = "Register `HOST_CTRL` reader"]
pub struct R(crate::R<HOST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_CTRL` writer"]
pub struct W(crate::W<HOST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HOST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Card detect signal detetction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct CARD_DET_SIGNAL_DETECT_R(crate::FieldReader<bool, CARD_DET_SIGNAL_DETECT_A>);
impl CARD_DET_SIGNAL_DETECT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_DET_SIGNAL_DETECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_DET_SIGNAL_DETECT_A {
        match self.bits {
            false => CARD_DET_SIGNAL_DETECT_A::VALUE1,
            true => CARD_DET_SIGNAL_DETECT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CARD_DET_SIGNAL_DETECT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_DET_SIGNAL_DETECT_A::VALUE2
    }
}
impl core::ops::Deref for CARD_DET_SIGNAL_DETECT_R {
    type Target = crate::FieldReader<bool, CARD_DET_SIGNAL_DETECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_DET_SIGNAL_DETECT` writer - Card detect signal detetction"]
pub struct CARD_DET_SIGNAL_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_DET_SIGNAL_DETECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_DET_SIGNAL_DETECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SDCD is selected (for normal use)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_DET_SIGNAL_DETECT_A::VALUE1)
    }
    #[doc = "The card detect test level is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_DET_SIGNAL_DETECT_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct CARD_DETECT_TEST_LEVEL_R(crate::FieldReader<bool, CARD_DETECT_TEST_LEVEL_A>);
impl CARD_DETECT_TEST_LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_DETECT_TEST_LEVEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_DETECT_TEST_LEVEL_A {
        match self.bits {
            false => CARD_DETECT_TEST_LEVEL_A::VALUE1,
            true => CARD_DETECT_TEST_LEVEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CARD_DETECT_TEST_LEVEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_DETECT_TEST_LEVEL_A::VALUE2
    }
}
impl core::ops::Deref for CARD_DETECT_TEST_LEVEL_R {
    type Target = crate::FieldReader<bool, CARD_DETECT_TEST_LEVEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_DETECT_TEST_LEVEL` writer - Card Detect Test Level"]
pub struct CARD_DETECT_TEST_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_DETECT_TEST_LEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_DETECT_TEST_LEVEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_DETECT_TEST_LEVEL_A::VALUE1)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_DETECT_TEST_LEVEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Extended Data Transfer Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct SD_8BIT_MODE_R(crate::FieldReader<bool, SD_8BIT_MODE_A>);
impl SD_8BIT_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SD_8BIT_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD_8BIT_MODE_A {
        match self.bits {
            false => SD_8BIT_MODE_A::VALUE1,
            true => SD_8BIT_MODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SD_8BIT_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SD_8BIT_MODE_A::VALUE2
    }
}
impl core::ops::Deref for SD_8BIT_MODE_R {
    type Target = crate::FieldReader<bool, SD_8BIT_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SD_8BIT_MODE` writer - Extended Data Transfer Width"]
pub struct SD_8BIT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_8BIT_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SD_8BIT_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus Width is selected by Data Transfer Width"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SD_8BIT_MODE_A::VALUE1)
    }
    #[doc = "8-bit Bus Width"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SD_8BIT_MODE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "High Speed Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct HIGH_SPEED_EN_R(crate::FieldReader<bool, HIGH_SPEED_EN_A>);
impl HIGH_SPEED_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIGH_SPEED_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIGH_SPEED_EN_A {
        match self.bits {
            false => HIGH_SPEED_EN_A::VALUE1,
            true => HIGH_SPEED_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HIGH_SPEED_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HIGH_SPEED_EN_A::VALUE2
    }
}
impl core::ops::Deref for HIGH_SPEED_EN_R {
    type Target = crate::FieldReader<bool, HIGH_SPEED_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGH_SPEED_EN` writer - High Speed Enable"]
pub struct HIGH_SPEED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_SPEED_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIGH_SPEED_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Speed Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIGH_SPEED_EN_A::VALUE1)
    }
    #[doc = "High Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIGH_SPEED_EN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Data Transfer Width (SD1 or SD4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct DATA_TX_WIDTH_R(crate::FieldReader<bool, DATA_TX_WIDTH_A>);
impl DATA_TX_WIDTH_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_TX_WIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_TX_WIDTH_A {
        match self.bits {
            false => DATA_TX_WIDTH_A::VALUE1,
            true => DATA_TX_WIDTH_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DATA_TX_WIDTH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DATA_TX_WIDTH_A::VALUE2
    }
}
impl core::ops::Deref for DATA_TX_WIDTH_R {
    type Target = crate::FieldReader<bool, DATA_TX_WIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_TX_WIDTH` writer - Data Transfer Width (SD1 or SD4)"]
pub struct DATA_TX_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TX_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_TX_WIDTH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1 bit mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_TX_WIDTH_A::VALUE1)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_TX_WIDTH_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "LED Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct LED_CTRL_R(crate::FieldReader<bool, LED_CTRL_A>);
impl LED_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LED_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LED_CTRL_A {
        match self.bits {
            false => LED_CTRL_A::VALUE1,
            true => LED_CTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LED_CTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LED_CTRL_A::VALUE2
    }
}
impl core::ops::Deref for LED_CTRL_R {
    type Target = crate::FieldReader<bool, LED_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LED_CTRL` writer - LED Control"]
pub struct LED_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LED_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LED off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LED_CTRL_A::VALUE1)
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LED_CTRL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Card detect signal detetction"]
    #[inline(always)]
    pub fn card_det_signal_detect(&self) -> CARD_DET_SIGNAL_DETECT_R {
        CARD_DET_SIGNAL_DETECT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn card_detect_test_level(&self) -> CARD_DETECT_TEST_LEVEL_R {
        CARD_DETECT_TEST_LEVEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    pub fn sd_8bit_mode(&self) -> SD_8BIT_MODE_R {
        SD_8BIT_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn high_speed_en(&self) -> HIGH_SPEED_EN_R {
        HIGH_SPEED_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width (SD1 or SD4)"]
    #[inline(always)]
    pub fn data_tx_width(&self) -> DATA_TX_WIDTH_R {
        DATA_TX_WIDTH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn led_ctrl(&self) -> LED_CTRL_R {
        LED_CTRL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Card detect signal detetction"]
    #[inline(always)]
    pub fn card_det_signal_detect(&mut self) -> CARD_DET_SIGNAL_DETECT_W {
        CARD_DET_SIGNAL_DETECT_W { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn card_detect_test_level(&mut self) -> CARD_DETECT_TEST_LEVEL_W {
        CARD_DETECT_TEST_LEVEL_W { w: self }
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    pub fn sd_8bit_mode(&mut self) -> SD_8BIT_MODE_W {
        SD_8BIT_MODE_W { w: self }
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn high_speed_en(&mut self) -> HIGH_SPEED_EN_W {
        HIGH_SPEED_EN_W { w: self }
    }
    #[doc = "Bit 1 - Data Transfer Width (SD1 or SD4)"]
    #[inline(always)]
    pub fn data_tx_width(&mut self) -> DATA_TX_WIDTH_W {
        DATA_TX_WIDTH_W { w: self }
    }
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn led_ctrl(&mut self) -> LED_CTRL_W {
        LED_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctrl](index.html) module"]
pub struct HOST_CTRL_SPEC;
impl crate::RegisterSpec for HOST_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [host_ctrl::R](R) reader structure"]
impl crate::Readable for HOST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_ctrl::W](W) writer structure"]
impl crate::Writable for HOST_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_CTRL to value 0"]
impl crate::Resettable for HOST_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
