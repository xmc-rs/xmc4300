#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RUN_LED {
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
#[doc = "Possible values of the field `LED_CODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LED_CODER {
    #[doc = "OFF - Init State"]
    VALUE1,
    #[doc = "Flash - SafeOp)"]
    VALUE2,
    #[doc = "Blinking - PreOp"]
    VALUE3,
    #[doc = "Flickering - Bootstrap"]
    VALUE4,
    #[doc = "On - Operational"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LED_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LED_CODER::VALUE1 => 0,
            LED_CODER::VALUE2 => 1,
            LED_CODER::VALUE3 => 13,
            LED_CODER::VALUE4 => 14,
            LED_CODER::VALUE5 => 15,
            LED_CODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LED_CODER {
        match value {
            0 => LED_CODER::VALUE1,
            1 => LED_CODER::VALUE2,
            13 => LED_CODER::VALUE3,
            14 => LED_CODER::VALUE4,
            15 => LED_CODER::VALUE5,
            i => LED_CODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LED_CODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LED_CODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LED_CODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LED_CODER::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == LED_CODER::VALUE5
    }
}
#[doc = "Possible values of the field `EN_OVERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_OVERRR {
    #[doc = "Override disable"]
    VALUE1,
    #[doc = "Override enable"]
    VALUE2,
}
impl EN_OVERRR {
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
            EN_OVERRR::VALUE1 => false,
            EN_OVERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_OVERRR {
        match value {
            false => EN_OVERRR::VALUE1,
            true => EN_OVERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EN_OVERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EN_OVERRR::VALUE2
    }
}
#[doc = "Values that can be written to the field `LED_CODE`"]
pub enum LED_CODEW {
    #[doc = "OFF - Init State"]
    VALUE1,
    #[doc = "Flash - SafeOp)"]
    VALUE2,
    #[doc = "Blinking - PreOp"]
    VALUE3,
    #[doc = "Flickering - Bootstrap"]
    VALUE4,
    #[doc = "On - Operational"]
    VALUE5,
}
impl LED_CODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LED_CODEW::VALUE1 => 0,
            LED_CODEW::VALUE2 => 1,
            LED_CODEW::VALUE3 => 13,
            LED_CODEW::VALUE4 => 14,
            LED_CODEW::VALUE5 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LED_CODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LED_CODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LED_CODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "OFF - Init State"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LED_CODEW::VALUE1)
    }
    #[doc = "Flash - SafeOp)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LED_CODEW::VALUE2)
    }
    #[doc = "Blinking - PreOp"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LED_CODEW::VALUE3)
    }
    #[doc = "Flickering - Bootstrap"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(LED_CODEW::VALUE4)
    }
    #[doc = "On - Operational"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(LED_CODEW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN_OVERR`"]
pub enum EN_OVERRW {
    #[doc = "Override disable"]
    VALUE1,
    #[doc = "Override enable"]
    VALUE2,
}
impl EN_OVERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_OVERRW::VALUE1 => false,
            EN_OVERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_OVERRW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_OVERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_OVERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Override disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EN_OVERRW::VALUE1)
    }
    #[doc = "Override enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EN_OVERRW::VALUE2)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - LED Code"]
    #[inline]
    pub fn led_code(&self) -> LED_CODER {
        LED_CODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline]
    pub fn en_overr(&self) -> EN_OVERRR {
        EN_OVERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 14 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - LED Code"]
    #[inline]
    pub fn led_code(&mut self) -> _LED_CODEW {
        _LED_CODEW { w: self }
    }
    #[doc = "Bit 4 - Enable Override"]
    #[inline]
    pub fn en_overr(&mut self) -> _EN_OVERRW {
        _EN_OVERRW { w: self }
    }
}
