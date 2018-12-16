#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::HOST_CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CARD_DET_SIGNAL_DETECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_DET_SIGNAL_DETECTR {
    #[doc = "SDCD is selected (for normal use)"]
    VALUE1,
    #[doc = "The card detect test level is selected"]
    VALUE2,
}
impl CARD_DET_SIGNAL_DETECTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CARD_DET_SIGNAL_DETECTR::VALUE1 => false,
            CARD_DET_SIGNAL_DETECTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_DET_SIGNAL_DETECTR {
        match value {
            false => CARD_DET_SIGNAL_DETECTR::VALUE1,
            true => CARD_DET_SIGNAL_DETECTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_DET_SIGNAL_DETECTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_DET_SIGNAL_DETECTR::VALUE2
    }
}
#[doc = "Possible values of the field `CARD_DETECT_TEST_LEVEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_DETECT_TEST_LEVELR {
    #[doc = "No Card"]
    VALUE1,
    #[doc = "Card Inserted"]
    VALUE2,
}
impl CARD_DETECT_TEST_LEVELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CARD_DETECT_TEST_LEVELR::VALUE1 => false,
            CARD_DETECT_TEST_LEVELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_DETECT_TEST_LEVELR {
        match value {
            false => CARD_DETECT_TEST_LEVELR::VALUE1,
            true => CARD_DETECT_TEST_LEVELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CARD_DETECT_TEST_LEVELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CARD_DETECT_TEST_LEVELR::VALUE2
    }
}
#[doc = "Possible values of the field `SD_8BIT_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SD_8BIT_MODER {
    #[doc = "Bus Width is selected by Data Transfer Width"]
    VALUE1,
    #[doc = "8-bit Bus Width"]
    VALUE2,
}
impl SD_8BIT_MODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SD_8BIT_MODER::VALUE1 => false,
            SD_8BIT_MODER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SD_8BIT_MODER {
        match value {
            false => SD_8BIT_MODER::VALUE1,
            true => SD_8BIT_MODER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SD_8BIT_MODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SD_8BIT_MODER::VALUE2
    }
}
#[doc = "Possible values of the field `HIGH_SPEED_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIGH_SPEED_ENR {
    #[doc = "Normal Speed Mode"]
    VALUE1,
    #[doc = "High Speed Mode"]
    VALUE2,
}
impl HIGH_SPEED_ENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HIGH_SPEED_ENR::VALUE1 => false,
            HIGH_SPEED_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIGH_SPEED_ENR {
        match value {
            false => HIGH_SPEED_ENR::VALUE1,
            true => HIGH_SPEED_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIGH_SPEED_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIGH_SPEED_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `DATA_TX_WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_TX_WIDTHR {
    #[doc = "1 bit mode"]
    VALUE1,
    #[doc = "4-bit mode"]
    VALUE2,
}
impl DATA_TX_WIDTHR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DATA_TX_WIDTHR::VALUE1 => false,
            DATA_TX_WIDTHR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_TX_WIDTHR {
        match value {
            false => DATA_TX_WIDTHR::VALUE1,
            true => DATA_TX_WIDTHR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATA_TX_WIDTHR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATA_TX_WIDTHR::VALUE2
    }
}
#[doc = "Possible values of the field `LED_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LED_CTRLR {
    #[doc = "LED off"]
    VALUE1,
    #[doc = "LED on"]
    VALUE2,
}
impl LED_CTRLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LED_CTRLR::VALUE1 => false,
            LED_CTRLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LED_CTRLR {
        match value {
            false => LED_CTRLR::VALUE1,
            true => LED_CTRLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LED_CTRLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LED_CTRLR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CARD_DET_SIGNAL_DETECT`"]
pub enum CARD_DET_SIGNAL_DETECTW {
    #[doc = "SDCD is selected (for normal use)"]
    VALUE1,
    #[doc = "The card detect test level is selected"]
    VALUE2,
}
impl CARD_DET_SIGNAL_DETECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_DET_SIGNAL_DETECTW::VALUE1 => false,
            CARD_DET_SIGNAL_DETECTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_DET_SIGNAL_DETECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_DET_SIGNAL_DETECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_DET_SIGNAL_DETECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SDCD is selected (for normal use)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_DET_SIGNAL_DETECTW::VALUE1)
    }
    #[doc = "The card detect test level is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_DET_SIGNAL_DETECTW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CARD_DETECT_TEST_LEVEL`"]
pub enum CARD_DETECT_TEST_LEVELW {
    #[doc = "No Card"]
    VALUE1,
    #[doc = "Card Inserted"]
    VALUE2,
}
impl CARD_DETECT_TEST_LEVELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_DETECT_TEST_LEVELW::VALUE1 => false,
            CARD_DETECT_TEST_LEVELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_DETECT_TEST_LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_DETECT_TEST_LEVELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_DETECT_TEST_LEVELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Card"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CARD_DETECT_TEST_LEVELW::VALUE1)
    }
    #[doc = "Card Inserted"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CARD_DETECT_TEST_LEVELW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SD_8BIT_MODE`"]
pub enum SD_8BIT_MODEW {
    #[doc = "Bus Width is selected by Data Transfer Width"]
    VALUE1,
    #[doc = "8-bit Bus Width"]
    VALUE2,
}
impl SD_8BIT_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SD_8BIT_MODEW::VALUE1 => false,
            SD_8BIT_MODEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SD_8BIT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SD_8BIT_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SD_8BIT_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus Width is selected by Data Transfer Width"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SD_8BIT_MODEW::VALUE1)
    }
    #[doc = "8-bit Bus Width"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SD_8BIT_MODEW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIGH_SPEED_EN`"]
pub enum HIGH_SPEED_ENW {
    #[doc = "Normal Speed Mode"]
    VALUE1,
    #[doc = "High Speed Mode"]
    VALUE2,
}
impl HIGH_SPEED_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIGH_SPEED_ENW::VALUE1 => false,
            HIGH_SPEED_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIGH_SPEED_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HIGH_SPEED_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIGH_SPEED_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Speed Mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIGH_SPEED_ENW::VALUE1)
    }
    #[doc = "High Speed Mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIGH_SPEED_ENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA_TX_WIDTH`"]
pub enum DATA_TX_WIDTHW {
    #[doc = "1 bit mode"]
    VALUE1,
    #[doc = "4-bit mode"]
    VALUE2,
}
impl DATA_TX_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_TX_WIDTHW::VALUE1 => false,
            DATA_TX_WIDTHW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_TX_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_TX_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_TX_WIDTHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 bit mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATA_TX_WIDTHW::VALUE1)
    }
    #[doc = "4-bit mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATA_TX_WIDTHW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LED_CTRL`"]
pub enum LED_CTRLW {
    #[doc = "LED off"]
    VALUE1,
    #[doc = "LED on"]
    VALUE2,
}
impl LED_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LED_CTRLW::VALUE1 => false,
            LED_CTRLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LED_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LED_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LED_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LED off"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LED_CTRLW::VALUE1)
    }
    #[doc = "LED on"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LED_CTRLW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 7 - Card detect signal detetction"]
    #[inline]
    pub fn card_det_signal_detect(&self) -> CARD_DET_SIGNAL_DETECTR {
        CARD_DET_SIGNAL_DETECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline]
    pub fn card_detect_test_level(&self) -> CARD_DETECT_TEST_LEVELR {
        CARD_DETECT_TEST_LEVELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline]
    pub fn sd_8bit_mode(&self) -> SD_8BIT_MODER {
        SD_8BIT_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline]
    pub fn high_speed_en(&self) -> HIGH_SPEED_ENR {
        HIGH_SPEED_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Data Transfer Width (SD1 or SD4)"]
    #[inline]
    pub fn data_tx_width(&self) -> DATA_TX_WIDTHR {
        DATA_TX_WIDTHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 0 - LED Control"]
    #[inline]
    pub fn led_ctrl(&self) -> LED_CTRLR {
        LED_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - Card detect signal detetction"]
    #[inline]
    pub fn card_det_signal_detect(&mut self) -> _CARD_DET_SIGNAL_DETECTW {
        _CARD_DET_SIGNAL_DETECTW { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline]
    pub fn card_detect_test_level(&mut self) -> _CARD_DETECT_TEST_LEVELW {
        _CARD_DETECT_TEST_LEVELW { w: self }
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline]
    pub fn sd_8bit_mode(&mut self) -> _SD_8BIT_MODEW {
        _SD_8BIT_MODEW { w: self }
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline]
    pub fn high_speed_en(&mut self) -> _HIGH_SPEED_ENW {
        _HIGH_SPEED_ENW { w: self }
    }
    #[doc = "Bit 1 - Data Transfer Width (SD1 or SD4)"]
    #[inline]
    pub fn data_tx_width(&mut self) -> _DATA_TX_WIDTHW {
        _DATA_TX_WIDTHW { w: self }
    }
    #[doc = "Bit 0 - LED Control"]
    #[inline]
    pub fn led_ctrl(&mut self) -> _LED_CTRLW {
        _LED_CTRLW { w: self }
    }
}
