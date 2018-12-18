#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::BLOCK_GAP_CTRL {
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
pub struct INT_AT_BLOCK_GAPR {
    bits: bool,
}
impl INT_AT_BLOCK_GAPR {
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
#[doc = "Possible values of the field `READ_WAIT_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_WAIT_CTRLR {
    #[doc = "Disable Read Wait Control"]
    VALUE1,
    #[doc = "Enable Read Wait Control"]
    VALUE2,
}
impl READ_WAIT_CTRLR {
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
            READ_WAIT_CTRLR::VALUE1 => false,
            READ_WAIT_CTRLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READ_WAIT_CTRLR {
        match value {
            false => READ_WAIT_CTRLR::VALUE1,
            true => READ_WAIT_CTRLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == READ_WAIT_CTRLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == READ_WAIT_CTRLR::VALUE2
    }
}
#[doc = "Possible values of the field `CONTINUE_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTINUE_REQR {
    #[doc = "Ignored"]
    VALUE1,
    #[doc = "Restart"]
    VALUE2,
}
impl CONTINUE_REQR {
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
            CONTINUE_REQR::VALUE1 => false,
            CONTINUE_REQR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTINUE_REQR {
        match value {
            false => CONTINUE_REQR::VALUE1,
            true => CONTINUE_REQR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CONTINUE_REQR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CONTINUE_REQR::VALUE2
    }
}
#[doc = "Possible values of the field `STOP_AT_BLOCK_GAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_AT_BLOCK_GAPR {
    #[doc = "Transfer"]
    VALUE1,
    #[doc = "Stop"]
    VALUE2,
}
impl STOP_AT_BLOCK_GAPR {
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
            STOP_AT_BLOCK_GAPR::VALUE1 => false,
            STOP_AT_BLOCK_GAPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOP_AT_BLOCK_GAPR {
        match value {
            false => STOP_AT_BLOCK_GAPR::VALUE1,
            true => STOP_AT_BLOCK_GAPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STOP_AT_BLOCK_GAPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STOP_AT_BLOCK_GAPR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _INT_AT_BLOCK_GAPW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_AT_BLOCK_GAPW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `READ_WAIT_CTRL`"]
pub enum READ_WAIT_CTRLW {
    #[doc = "Disable Read Wait Control"]
    VALUE1,
    #[doc = "Enable Read Wait Control"]
    VALUE2,
}
impl READ_WAIT_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READ_WAIT_CTRLW::VALUE1 => false,
            READ_WAIT_CTRLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READ_WAIT_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_WAIT_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READ_WAIT_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Read Wait Control"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(READ_WAIT_CTRLW::VALUE1)
    }
    #[doc = "Enable Read Wait Control"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(READ_WAIT_CTRLW::VALUE2)
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
#[doc = "Values that can be written to the field `CONTINUE_REQ`"]
pub enum CONTINUE_REQW {
    #[doc = "Ignored"]
    VALUE1,
    #[doc = "Restart"]
    VALUE2,
}
impl CONTINUE_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTINUE_REQW::VALUE1 => false,
            CONTINUE_REQW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTINUE_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTINUE_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTINUE_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignored"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CONTINUE_REQW::VALUE1)
    }
    #[doc = "Restart"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CONTINUE_REQW::VALUE2)
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
#[doc = "Values that can be written to the field `STOP_AT_BLOCK_GAP`"]
pub enum STOP_AT_BLOCK_GAPW {
    #[doc = "Transfer"]
    VALUE1,
    #[doc = "Stop"]
    VALUE2,
}
impl STOP_AT_BLOCK_GAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOP_AT_BLOCK_GAPW::VALUE1 => false,
            STOP_AT_BLOCK_GAPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOP_AT_BLOCK_GAPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_AT_BLOCK_GAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOP_AT_BLOCK_GAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STOP_AT_BLOCK_GAPW::VALUE1)
    }
    #[doc = "Stop"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STOP_AT_BLOCK_GAPW::VALUE2)
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
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline]
    pub fn int_at_block_gap(&self) -> INT_AT_BLOCK_GAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        INT_AT_BLOCK_GAPR { bits }
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline]
    pub fn read_wait_ctrl(&self) -> READ_WAIT_CTRLR {
        READ_WAIT_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline]
    pub fn continue_req(&self) -> CONTINUE_REQR {
        CONTINUE_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline]
    pub fn stop_at_block_gap(&self) -> STOP_AT_BLOCK_GAPR {
        STOP_AT_BLOCK_GAPR::_from({
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
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline]
    pub fn int_at_block_gap(&mut self) -> _INT_AT_BLOCK_GAPW {
        _INT_AT_BLOCK_GAPW { w: self }
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline]
    pub fn read_wait_ctrl(&mut self) -> _READ_WAIT_CTRLW {
        _READ_WAIT_CTRLW { w: self }
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline]
    pub fn continue_req(&mut self) -> _CONTINUE_REQW {
        _CONTINUE_REQW { w: self }
    }
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline]
    pub fn stop_at_block_gap(&mut self) -> _STOP_AT_BLOCK_GAPW {
        _STOP_AT_BLOCK_GAPW { w: self }
    }
}
