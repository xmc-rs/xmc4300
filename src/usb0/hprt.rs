#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPRT {
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
#[doc = "Possible values of the field `PrtConnSts`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTCONNSTSR {
    #[doc = "No device is attached to the port."]
    VALUE1,
    #[doc = "A device is attached to the port."]
    VALUE2,
}
impl PRTCONNSTSR {
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
            PRTCONNSTSR::VALUE1 => false,
            PRTCONNSTSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRTCONNSTSR {
        match value {
            false => PRTCONNSTSR::VALUE1,
            true => PRTCONNSTSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRTCONNSTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRTCONNSTSR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PRTCONNDETR {
    bits: bool,
}
impl PRTCONNDETR {
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
#[doc = "Possible values of the field `PrtEna`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTENAR {
    #[doc = "Port disabled"]
    VALUE1,
    #[doc = "Port enabled"]
    VALUE2,
}
impl PRTENAR {
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
            PRTENAR::VALUE1 => false,
            PRTENAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRTENAR {
        match value {
            false => PRTENAR::VALUE1,
            true => PRTENAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRTENAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRTENAR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PRTENCHNGR {
    bits: bool,
}
impl PRTENCHNGR {
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
#[doc = "Possible values of the field `PrtOvrCurrAct`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTOVRCURRACTR {
    #[doc = "No overcurrent condition"]
    VALUE1,
    #[doc = "Overcurrent condition"]
    VALUE2,
}
impl PRTOVRCURRACTR {
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
            PRTOVRCURRACTR::VALUE1 => false,
            PRTOVRCURRACTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRTOVRCURRACTR {
        match value {
            false => PRTOVRCURRACTR::VALUE1,
            true => PRTOVRCURRACTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRTOVRCURRACTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRTOVRCURRACTR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PRTOVRCURRCHNGR {
    bits: bool,
}
impl PRTOVRCURRCHNGR {
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
#[doc = "Possible values of the field `PrtRes`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTRESR {
    #[doc = "No resume driven"]
    VALUE1,
    #[doc = "Resume driven"]
    VALUE2,
}
impl PRTRESR {
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
            PRTRESR::VALUE1 => false,
            PRTRESR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRTRESR {
        match value {
            false => PRTRESR::VALUE1,
            true => PRTRESR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRTRESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRTRESR::VALUE2
    }
}
#[doc = "Possible values of the field `PrtSusp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTSUSPR {
    #[doc = "Port not in Suspend mode"]
    VALUE1,
    #[doc = "Port in Suspend mode"]
    VALUE2,
}
impl PRTSUSPR {
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
            PRTSUSPR::VALUE1 => false,
            PRTSUSPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRTSUSPR {
        match value {
            false => PRTSUSPR::VALUE1,
            true => PRTSUSPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRTSUSPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRTSUSPR::VALUE2
    }
}
#[doc = "Possible values of the field `PrtRst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTRSTR {
    #[doc = "Port not in reset"]
    VALUE1,
    #[doc = "Port in reset"]
    VALUE2,
}
impl PRTRSTR {
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
            PRTRSTR::VALUE1 => false,
            PRTRSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRTRSTR {
        match value {
            false => PRTRSTR::VALUE1,
            true => PRTRSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRTRSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRTRSTR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PRTLNSTSR {
    bits: u8,
}
impl PRTLNSTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PrtPwr`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTPWRR {
    #[doc = "Power off"]
    VALUE1,
    #[doc = "Power on"]
    VALUE2,
}
impl PRTPWRR {
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
            PRTPWRR::VALUE1 => false,
            PRTPWRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRTPWRR {
        match value {
            false => PRTPWRR::VALUE1,
            true => PRTPWRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRTPWRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRTPWRR::VALUE2
    }
}
#[doc = "Possible values of the field `PrtSpd`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTSPDR {
    #[doc = "Full speed"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRTSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRTSPDR::VALUE1 => 1,
            PRTSPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRTSPDR {
        match value {
            1 => PRTSPDR::VALUE1,
            i => PRTSPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRTSPDR::VALUE1
    }
}
#[doc = r" Proxy"]
pub struct _PRTCONNDETW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTCONNDETW<'a> {
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
#[doc = "Values that can be written to the field `PrtEna`"]
pub enum PRTENAW {
    #[doc = "Port disabled"]
    VALUE1,
    #[doc = "Port enabled"]
    VALUE2,
}
impl PRTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRTENAW::VALUE1 => false,
            PRTENAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTENAW::VALUE1)
    }
    #[doc = "Port enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTENAW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRTENCHNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTENCHNGW<'a> {
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
pub struct _PRTOVRCURRCHNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTOVRCURRCHNGW<'a> {
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
#[doc = "Values that can be written to the field `PrtRes`"]
pub enum PRTRESW {
    #[doc = "No resume driven"]
    VALUE1,
    #[doc = "Resume driven"]
    VALUE2,
}
impl PRTRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRTRESW::VALUE1 => false,
            PRTRESW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRTRESW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRTRESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No resume driven"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTRESW::VALUE1)
    }
    #[doc = "Resume driven"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTRESW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PrtSusp`"]
pub enum PRTSUSPW {
    #[doc = "Port not in Suspend mode"]
    VALUE1,
    #[doc = "Port in Suspend mode"]
    VALUE2,
}
impl PRTSUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRTSUSPW::VALUE1 => false,
            PRTSUSPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRTSUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTSUSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRTSUSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port not in Suspend mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTSUSPW::VALUE1)
    }
    #[doc = "Port in Suspend mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTSUSPW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PrtRst`"]
pub enum PRTRSTW {
    #[doc = "Port not in reset"]
    VALUE1,
    #[doc = "Port in reset"]
    VALUE2,
}
impl PRTRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRTRSTW::VALUE1 => false,
            PRTRSTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRTRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port not in reset"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTRSTW::VALUE1)
    }
    #[doc = "Port in reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTRSTW::VALUE2)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PrtPwr`"]
pub enum PRTPWRW {
    #[doc = "Power off"]
    VALUE1,
    #[doc = "Power on"]
    VALUE2,
}
impl PRTPWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRTPWRW::VALUE1 => false,
            PRTPWRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRTPWRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTPWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRTPWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power off"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTPWRW::VALUE1)
    }
    #[doc = "Power on"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTPWRW::VALUE2)
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline]
    pub fn prt_conn_sts(&self) -> PRTCONNSTSR {
        PRTCONNSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline]
    pub fn prt_conn_det(&self) -> PRTCONNDETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTCONNDETR { bits }
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline]
    pub fn prt_ena(&self) -> PRTENAR {
        PRTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline]
    pub fn prt_en_chng(&self) -> PRTENCHNGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTENCHNGR { bits }
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline]
    pub fn prt_ovr_curr_act(&self) -> PRTOVRCURRACTR {
        PRTOVRCURRACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline]
    pub fn prt_ovr_curr_chng(&self) -> PRTOVRCURRCHNGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTOVRCURRCHNGR { bits }
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline]
    pub fn prt_res(&self) -> PRTRESR {
        PRTRESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline]
    pub fn prt_susp(&self) -> PRTSUSPR {
        PRTSUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline]
    pub fn prt_rst(&self) -> PRTRSTR {
        PRTRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline]
    pub fn prt_ln_sts(&self) -> PRTLNSTSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRTLNSTSR { bits }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline]
    pub fn prt_pwr(&self) -> PRTPWRR {
        PRTPWRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline]
    pub fn prt_spd(&self) -> PRTSPDR {
        PRTSPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline]
    pub fn prt_conn_det(&mut self) -> _PRTCONNDETW {
        _PRTCONNDETW { w: self }
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline]
    pub fn prt_ena(&mut self) -> _PRTENAW {
        _PRTENAW { w: self }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline]
    pub fn prt_en_chng(&mut self) -> _PRTENCHNGW {
        _PRTENCHNGW { w: self }
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline]
    pub fn prt_ovr_curr_chng(&mut self) -> _PRTOVRCURRCHNGW {
        _PRTOVRCURRCHNGW { w: self }
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline]
    pub fn prt_res(&mut self) -> _PRTRESW {
        _PRTRESW { w: self }
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline]
    pub fn prt_susp(&mut self) -> _PRTSUSPW {
        _PRTSUSPW { w: self }
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline]
    pub fn prt_rst(&mut self) -> _PRTRSTW {
        _PRTRSTW { w: self }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline]
    pub fn prt_pwr(&mut self) -> _PRTPWRW {
        _PRTPWRW { w: self }
    }
}
