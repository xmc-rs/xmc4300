#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::AL_STATUS {
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
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "Init State"]
    VALUE1,
    #[doc = "Pre-Operational State"]
    VALUE2,
    #[doc = "Bootstrap State"]
    VALUE3,
    #[doc = "Safe-Operational State"]
    VALUE4,
    #[doc = "Operational State"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::VALUE1 => 1,
            STATER::VALUE2 => 2,
            STATER::VALUE3 => 3,
            STATER::VALUE4 => 4,
            STATER::VALUE5 => 8,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            1 => STATER::VALUE1,
            2 => STATER::VALUE2,
            3 => STATER::VALUE3,
            4 => STATER::VALUE4,
            8 => STATER::VALUE5,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STATER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STATER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STATER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == STATER::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == STATER::VALUE5
    }
}
#[doc = "Possible values of the field `ERRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIR {
    #[doc = "Device is in State as requested or Flag cleared by command"]
    VALUE1,
    #[doc = "Device has not entered requested State or changed State as result of a local action"]
    VALUE2,
}
impl ERRIR {
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
            ERRIR::VALUE1 => false,
            ERRIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRIR {
        match value {
            false => ERRIR::VALUE1,
            true => ERRIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERRIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERRIR::VALUE2
    }
}
#[doc = "Possible values of the field `DID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIDR {
    #[doc = "Device Identification not valid"]
    VALUE1,
    #[doc = "Device Identification loaded"]
    VALUE2,
}
impl DIDR {
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
            DIDR::VALUE1 => false,
            DIDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIDR {
        match value {
            false => DIDR::VALUE1,
            true => DIDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIDR::VALUE2
    }
}
#[doc = "Values that can be written to the field `STATE`"]
pub enum STATEW {
    #[doc = "Init State"]
    VALUE1,
    #[doc = "Pre-Operational State"]
    VALUE2,
    #[doc = "Bootstrap State"]
    VALUE3,
    #[doc = "Safe-Operational State"]
    VALUE4,
    #[doc = "Operational State"]
    VALUE5,
}
impl STATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STATEW::VALUE1 => 1,
            STATEW::VALUE2 => 2,
            STATEW::VALUE3 => 3,
            STATEW::VALUE4 => 4,
            STATEW::VALUE5 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _STATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Init State"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STATEW::VALUE1)
    }
    #[doc = "Pre-Operational State"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STATEW::VALUE2)
    }
    #[doc = "Bootstrap State"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(STATEW::VALUE3)
    }
    #[doc = "Safe-Operational State"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(STATEW::VALUE4)
    }
    #[doc = "Operational State"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(STATEW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERRI`"]
pub enum ERRIW {
    #[doc = "Device is in State as requested or Flag cleared by command"]
    VALUE1,
    #[doc = "Device has not entered requested State or changed State as result of a local action"]
    VALUE2,
}
impl ERRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRIW::VALUE1 => false,
            ERRIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRIW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Device is in State as requested or Flag cleared by command"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERRIW::VALUE1)
    }
    #[doc = "Device has not entered requested State or changed State as result of a local action"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERRIW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DID`"]
pub enum DIDW {
    #[doc = "Device Identification not valid"]
    VALUE1,
    #[doc = "Device Identification loaded"]
    VALUE2,
}
impl DIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIDW::VALUE1 => false,
            DIDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIDW<'a> {
    w: &'a mut W,
}
impl<'a> _DIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Device Identification not valid"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIDW::VALUE1)
    }
    #[doc = "Device Identification loaded"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIDW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:3 - Actual State of the Device State Machine"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 4 - Error Ind"]
    #[inline]
    pub fn erri(&self) -> ERRIR {
        ERRIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline]
    pub fn did(&self) -> DIDR {
        DIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Actual State of the Device State Machine"]
    #[inline]
    pub fn state(&mut self) -> _STATEW {
        _STATEW { w: self }
    }
    #[doc = "Bit 4 - Error Ind"]
    #[inline]
    pub fn erri(&mut self) -> _ERRIW {
        _ERRIW { w: self }
    }
    #[doc = "Bit 5 - Device Identification"]
    #[inline]
    pub fn did(&mut self) -> _DIDW {
        _DIDW { w: self }
    }
}
