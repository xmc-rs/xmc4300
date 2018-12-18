#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STC {
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
#[doc = "Possible values of the field `CSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSER {
    #[doc = "Cascaded shadow transfer disabled"]
    VALUE1,
    #[doc = "Cascaded shadow transfer enabled"]
    VALUE2,
}
impl CSER {
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
            CSER::VALUE1 => false,
            CSER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSER {
        match value {
            false => CSER::VALUE1,
            true => CSER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CSER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CSER::VALUE2
    }
}
#[doc = "Possible values of the field `STM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STMR {
    #[doc = "Shadow transfer is done in Period Match and One match."]
    VALUE1,
    #[doc = "Shadow transfer is done only in Period Match."]
    VALUE2,
    #[doc = "Shadow transfer is done only in One Match."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STMR::VALUE1 => 0,
            STMR::VALUE2 => 1,
            STMR::VALUE3 => 2,
            STMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STMR {
        match value {
            0 => STMR::VALUE1,
            1 => STMR::VALUE2,
            2 => STMR::VALUE3,
            i => STMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STMR::VALUE3
    }
}
#[doc = "Values that can be written to the field `CSE`"]
pub enum CSEW {
    #[doc = "Cascaded shadow transfer disabled"]
    VALUE1,
    #[doc = "Cascaded shadow transfer enabled"]
    VALUE2,
}
impl CSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSEW::VALUE1 => false,
            CSEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cascaded shadow transfer disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSEW::VALUE1)
    }
    #[doc = "Cascaded shadow transfer enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSEW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STM`"]
pub enum STMW {
    #[doc = "Shadow transfer is done in Period Match and One match."]
    VALUE1,
    #[doc = "Shadow transfer is done only in Period Match."]
    VALUE2,
    #[doc = "Shadow transfer is done only in One Match."]
    VALUE3,
}
impl STMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STMW::VALUE1 => 0,
            STMW::VALUE2 => 1,
            STMW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STMW<'a> {
    w: &'a mut W,
}
impl<'a> _STMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Shadow transfer is done in Period Match and One match."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STMW::VALUE1)
    }
    #[doc = "Shadow transfer is done only in Period Match."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STMW::VALUE2)
    }
    #[doc = "Shadow transfer is done only in One Match."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(STMW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline]
    pub fn cse(&self) -> CSER {
        CSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline]
    pub fn stm(&self) -> STMR {
        STMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Cascaded shadow transfer enable"]
    #[inline]
    pub fn cse(&mut self) -> _CSEW {
        _CSEW { w: self }
    }
    #[doc = "Bits 1:2 - Shadow transfer mode"]
    #[inline]
    pub fn stm(&mut self) -> _STMW {
        _STMW { w: self }
    }
}
