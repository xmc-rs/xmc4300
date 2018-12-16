#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SW_RESET {
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
#[doc = "Possible values of the field `SW_RST_DAT_LINE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RST_DAT_LINER {
    #[doc = "Work"]
    VALUE1,
    #[doc = "Reset"]
    VALUE2,
}
impl SW_RST_DAT_LINER {
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
            SW_RST_DAT_LINER::VALUE1 => false,
            SW_RST_DAT_LINER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SW_RST_DAT_LINER {
        match value {
            false => SW_RST_DAT_LINER::VALUE1,
            true => SW_RST_DAT_LINER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SW_RST_DAT_LINER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SW_RST_DAT_LINER::VALUE2
    }
}
#[doc = "Possible values of the field `SW_RST_CMD_LINE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RST_CMD_LINER {
    #[doc = "Work"]
    VALUE1,
    #[doc = "Reset"]
    VALUE2,
}
impl SW_RST_CMD_LINER {
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
            SW_RST_CMD_LINER::VALUE1 => false,
            SW_RST_CMD_LINER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SW_RST_CMD_LINER {
        match value {
            false => SW_RST_CMD_LINER::VALUE1,
            true => SW_RST_CMD_LINER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SW_RST_CMD_LINER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SW_RST_CMD_LINER::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct SW_RST_ALLR {
    bits: bool,
}
impl SW_RST_ALLR {
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
#[doc = "Values that can be written to the field `SW_RST_DAT_LINE`"]
pub enum SW_RST_DAT_LINEW {
    #[doc = "Work"]
    VALUE1,
    #[doc = "Reset"]
    VALUE2,
}
impl SW_RST_DAT_LINEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SW_RST_DAT_LINEW::VALUE1 => false,
            SW_RST_DAT_LINEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SW_RST_DAT_LINEW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_RST_DAT_LINEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SW_RST_DAT_LINEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Work"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SW_RST_DAT_LINEW::VALUE1)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SW_RST_DAT_LINEW::VALUE2)
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
#[doc = "Values that can be written to the field `SW_RST_CMD_LINE`"]
pub enum SW_RST_CMD_LINEW {
    #[doc = "Work"]
    VALUE1,
    #[doc = "Reset"]
    VALUE2,
}
impl SW_RST_CMD_LINEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SW_RST_CMD_LINEW::VALUE1 => false,
            SW_RST_CMD_LINEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SW_RST_CMD_LINEW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_RST_CMD_LINEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SW_RST_CMD_LINEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Work"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SW_RST_CMD_LINEW::VALUE1)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SW_RST_CMD_LINEW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _SW_RST_ALLW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_RST_ALLW<'a> {
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
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline]
    pub fn sw_rst_dat_line(&self) -> SW_RST_DAT_LINER {
        SW_RST_DAT_LINER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline]
    pub fn sw_rst_cmd_line(&self) -> SW_RST_CMD_LINER {
        SW_RST_CMD_LINER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline]
    pub fn sw_rst_all(&self) -> SW_RST_ALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SW_RST_ALLR { bits }
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
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline]
    pub fn sw_rst_dat_line(&mut self) -> _SW_RST_DAT_LINEW {
        _SW_RST_DAT_LINEW { w: self }
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline]
    pub fn sw_rst_cmd_line(&mut self) -> _SW_RST_CMD_LINEW {
        _SW_RST_CMD_LINEW { w: self }
    }
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline]
    pub fn sw_rst_all(&mut self) -> _SW_RST_ALLW {
        _SW_RST_ALLW { w: self }
    }
}
