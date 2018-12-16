#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMC_TRANSMIT_INTERRUPT_MASK {
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
#[doc = r" Value of the field"]
pub struct TXGBOCTIMR {
    bits: bool,
}
impl TXGBOCTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXGBFRMIMR {
    bits: bool,
}
impl TXGBFRMIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXBCGFIMR {
    bits: bool,
}
impl TXBCGFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXMCGFIMR {
    bits: bool,
}
impl TXMCGFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TX64OCTGBFIMR {
    bits: bool,
}
impl TX64OCTGBFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TX65T127OCTGBFIMR {
    bits: bool,
}
impl TX65T127OCTGBFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TX128T255OCTGBFIMR {
    bits: bool,
}
impl TX128T255OCTGBFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TX256T511OCTGBFIMR {
    bits: bool,
}
impl TX256T511OCTGBFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TX512T1023OCTGBFIMR {
    bits: bool,
}
impl TX512T1023OCTGBFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TX1024TMAXOCTGBFIMR {
    bits: bool,
}
impl TX1024TMAXOCTGBFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXUCGBFIMR {
    bits: bool,
}
impl TXUCGBFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXMCGBFIMR {
    bits: bool,
}
impl TXMCGBFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXBCGBFIMR {
    bits: bool,
}
impl TXBCGBFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXUFLOWERFIMR {
    bits: bool,
}
impl TXUFLOWERFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXSCOLGFIMR {
    bits: bool,
}
impl TXSCOLGFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXMCOLGFIMR {
    bits: bool,
}
impl TXMCOLGFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXDEFFIMR {
    bits: bool,
}
impl TXDEFFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXLATCOLFIMR {
    bits: bool,
}
impl TXLATCOLFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXEXCOLFIMR {
    bits: bool,
}
impl TXEXCOLFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXCARERFIMR {
    bits: bool,
}
impl TXCARERFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXGOCTIMR {
    bits: bool,
}
impl TXGOCTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXGFRMIMR {
    bits: bool,
}
impl TXGFRMIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXEXDEFFIMR {
    bits: bool,
}
impl TXEXDEFFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXPAUSFIMR {
    bits: bool,
}
impl TXPAUSFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXVLANGFIMR {
    bits: bool,
}
impl TXVLANGFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TXOSIZEGFIMR {
    bits: bool,
}
impl TXOSIZEGFIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _TXGBOCTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXGBOCTIMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXGBFRMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXGBFRMIMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXBCGFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBCGFIMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXMCGFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXMCGFIMW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX64OCTGBFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TX64OCTGBFIMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX65T127OCTGBFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TX65T127OCTGBFIMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX128T255OCTGBFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TX128T255OCTGBFIMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX256T511OCTGBFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TX256T511OCTGBFIMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX512T1023OCTGBFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TX512T1023OCTGBFIMW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX1024TMAXOCTGBFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TX1024TMAXOCTGBFIMW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXUCGBFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUCGBFIMW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXMCGBFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXMCGBFIMW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXBCGBFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBCGBFIMW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXUFLOWERFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUFLOWERFIMW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXSCOLGFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSCOLGFIMW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXMCOLGFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXMCOLGFIMW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXDEFFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDEFFIMW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXLATCOLFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXLATCOLFIMW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXEXCOLFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEXCOLFIMW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXCARERFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCARERFIMW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXGOCTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXGOCTIMW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXGFRMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXGFRMIMW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXEXDEFFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEXDEFFIMW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXPAUSFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPAUSFIMW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXVLANGFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXVLANGFIMW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXOSIZEGFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOSIZEGFIMW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline]
    pub fn txgboctim(&self) -> TXGBOCTIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXGBOCTIMR { bits }
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txgbfrmim(&self) -> TXGBFRMIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXGBFRMIMR { bits }
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txbcgfim(&self) -> TXBCGFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXBCGFIMR { bits }
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txmcgfim(&self) -> TXMCGFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXMCGFIMR { bits }
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx64octgbfim(&self) -> TX64OCTGBFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX64OCTGBFIMR { bits }
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx65t127octgbfim(&self) -> TX65T127OCTGBFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX65T127OCTGBFIMR { bits }
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx128t255octgbfim(&self) -> TX128T255OCTGBFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX128T255OCTGBFIMR { bits }
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx256t511octgbfim(&self) -> TX256T511OCTGBFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX256T511OCTGBFIMR { bits }
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx512t1023octgbfim(&self) -> TX512T1023OCTGBFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX512T1023OCTGBFIMR { bits }
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx1024tmaxoctgbfim(&self) -> TX1024TMAXOCTGBFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX1024TMAXOCTGBFIMR { bits }
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txucgbfim(&self) -> TXUCGBFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXUCGBFIMR { bits }
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txmcgbfim(&self) -> TXMCGBFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXMCGBFIMR { bits }
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txbcgbfim(&self) -> TXBCGBFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXBCGBFIMR { bits }
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txuflowerfim(&self) -> TXUFLOWERFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXUFLOWERFIMR { bits }
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txscolgfim(&self) -> TXSCOLGFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXSCOLGFIMR { bits }
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txmcolgfim(&self) -> TXMCOLGFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXMCOLGFIMR { bits }
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txdeffim(&self) -> TXDEFFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXDEFFIMR { bits }
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txlatcolfim(&self) -> TXLATCOLFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXLATCOLFIMR { bits }
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txexcolfim(&self) -> TXEXCOLFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXEXCOLFIMR { bits }
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txcarerfim(&self) -> TXCARERFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXCARERFIMR { bits }
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline]
    pub fn txgoctim(&self) -> TXGOCTIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXGOCTIMR { bits }
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txgfrmim(&self) -> TXGFRMIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXGFRMIMR { bits }
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txexdeffim(&self) -> TXEXDEFFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXEXDEFFIMR { bits }
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txpausfim(&self) -> TXPAUSFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXPAUSFIMR { bits }
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txvlangfim(&self) -> TXVLANGFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXVLANGFIMR { bits }
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txosizegfim(&self) -> TXOSIZEGFIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXOSIZEGFIMR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline]
    pub fn txgboctim(&mut self) -> _TXGBOCTIMW {
        _TXGBOCTIMW { w: self }
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txgbfrmim(&mut self) -> _TXGBFRMIMW {
        _TXGBFRMIMW { w: self }
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txbcgfim(&mut self) -> _TXBCGFIMW {
        _TXBCGFIMW { w: self }
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txmcgfim(&mut self) -> _TXMCGFIMW {
        _TXMCGFIMW { w: self }
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx64octgbfim(&mut self) -> _TX64OCTGBFIMW {
        _TX64OCTGBFIMW { w: self }
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx65t127octgbfim(&mut self) -> _TX65T127OCTGBFIMW {
        _TX65T127OCTGBFIMW { w: self }
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx128t255octgbfim(&mut self) -> _TX128T255OCTGBFIMW {
        _TX128T255OCTGBFIMW { w: self }
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx256t511octgbfim(&mut self) -> _TX256T511OCTGBFIMW {
        _TX256T511OCTGBFIMW { w: self }
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx512t1023octgbfim(&mut self) -> _TX512T1023OCTGBFIMW {
        _TX512T1023OCTGBFIMW { w: self }
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn tx1024tmaxoctgbfim(&mut self) -> _TX1024TMAXOCTGBFIMW {
        _TX1024TMAXOCTGBFIMW { w: self }
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txucgbfim(&mut self) -> _TXUCGBFIMW {
        _TXUCGBFIMW { w: self }
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txmcgbfim(&mut self) -> _TXMCGBFIMW {
        _TXMCGBFIMW { w: self }
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txbcgbfim(&mut self) -> _TXBCGBFIMW {
        _TXBCGBFIMW { w: self }
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txuflowerfim(&mut self) -> _TXUFLOWERFIMW {
        _TXUFLOWERFIMW { w: self }
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txscolgfim(&mut self) -> _TXSCOLGFIMW {
        _TXSCOLGFIMW { w: self }
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txmcolgfim(&mut self) -> _TXMCOLGFIMW {
        _TXMCOLGFIMW { w: self }
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txdeffim(&mut self) -> _TXDEFFIMW {
        _TXDEFFIMW { w: self }
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txlatcolfim(&mut self) -> _TXLATCOLFIMW {
        _TXLATCOLFIMW { w: self }
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txexcolfim(&mut self) -> _TXEXCOLFIMW {
        _TXEXCOLFIMW { w: self }
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txcarerfim(&mut self) -> _TXCARERFIMW {
        _TXCARERFIMW { w: self }
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline]
    pub fn txgoctim(&mut self) -> _TXGOCTIMW {
        _TXGOCTIMW { w: self }
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txgfrmim(&mut self) -> _TXGFRMIMW {
        _TXGFRMIMW { w: self }
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txexdeffim(&mut self) -> _TXEXDEFFIMW {
        _TXEXDEFFIMW { w: self }
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txpausfim(&mut self) -> _TXPAUSFIMW {
        _TXPAUSFIMW { w: self }
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txvlangfim(&mut self) -> _TXVLANGFIMW {
        _TXVLANGFIMW { w: self }
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline]
    pub fn txosizegfim(&mut self) -> _TXOSIZEGFIMW {
        _TXOSIZEGFIMW { w: self }
    }
}
