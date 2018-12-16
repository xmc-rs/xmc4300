#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMC {
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
#[doc = "Possible values of the field `STRTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRTSR {
    #[doc = "External Start Function deactivated"]
    VALUE1,
    #[doc = "External Start Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Start Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Start Function triggered by Event 2"]
    VALUE4,
}
impl STRTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STRTSR::VALUE1 => 0,
            STRTSR::VALUE2 => 1,
            STRTSR::VALUE3 => 2,
            STRTSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STRTSR {
        match value {
            0 => STRTSR::VALUE1,
            1 => STRTSR::VALUE2,
            2 => STRTSR::VALUE3,
            3 => STRTSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STRTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STRTSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STRTSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == STRTSR::VALUE4
    }
}
#[doc = "Possible values of the field `ENDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDSR {
    #[doc = "External Stop Function deactivated"]
    VALUE1,
    #[doc = "External Stop Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Stop Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Stop Function triggered by Event 2"]
    VALUE4,
}
impl ENDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENDSR::VALUE1 => 0,
            ENDSR::VALUE2 => 1,
            ENDSR::VALUE3 => 2,
            ENDSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENDSR {
        match value {
            0 => ENDSR::VALUE1,
            1 => ENDSR::VALUE2,
            2 => ENDSR::VALUE3,
            3 => ENDSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENDSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENDSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ENDSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ENDSR::VALUE4
    }
}
#[doc = "Possible values of the field `CAP0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0SR {
    #[doc = "External Capture 0 Function deactivated"]
    VALUE1,
    #[doc = "External Capture 0 Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Capture 0 Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Capture 0 Function triggered by Event 2"]
    VALUE4,
}
impl CAP0SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAP0SR::VALUE1 => 0,
            CAP0SR::VALUE2 => 1,
            CAP0SR::VALUE3 => 2,
            CAP0SR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAP0SR {
        match value {
            0 => CAP0SR::VALUE1,
            1 => CAP0SR::VALUE2,
            2 => CAP0SR::VALUE3,
            3 => CAP0SR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CAP0SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CAP0SR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CAP0SR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CAP0SR::VALUE4
    }
}
#[doc = "Possible values of the field `CAP1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1SR {
    #[doc = "External Capture 1 Function deactivated"]
    VALUE1,
    #[doc = "External Capture 1 Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Capture 1 Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Capture 1 Function triggered by Event 2"]
    VALUE4,
}
impl CAP1SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAP1SR::VALUE1 => 0,
            CAP1SR::VALUE2 => 1,
            CAP1SR::VALUE3 => 2,
            CAP1SR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAP1SR {
        match value {
            0 => CAP1SR::VALUE1,
            1 => CAP1SR::VALUE2,
            2 => CAP1SR::VALUE3,
            3 => CAP1SR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CAP1SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CAP1SR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CAP1SR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CAP1SR::VALUE4
    }
}
#[doc = "Possible values of the field `GATES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GATESR {
    #[doc = "External Gating Function deactivated"]
    VALUE1,
    #[doc = "External Gating Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Gating Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Gating Function triggered by Event 2"]
    VALUE4,
}
impl GATESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GATESR::VALUE1 => 0,
            GATESR::VALUE2 => 1,
            GATESR::VALUE3 => 2,
            GATESR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GATESR {
        match value {
            0 => GATESR::VALUE1,
            1 => GATESR::VALUE2,
            2 => GATESR::VALUE3,
            3 => GATESR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GATESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GATESR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GATESR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GATESR::VALUE4
    }
}
#[doc = "Possible values of the field `UDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDSR {
    #[doc = "External Up/Down Function deactivated"]
    VALUE1,
    #[doc = "External Up/Down Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Up/Down Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Up/Down Function triggered by Event 2"]
    VALUE4,
}
impl UDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UDSR::VALUE1 => 0,
            UDSR::VALUE2 => 1,
            UDSR::VALUE3 => 2,
            UDSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UDSR {
        match value {
            0 => UDSR::VALUE1,
            1 => UDSR::VALUE2,
            2 => UDSR::VALUE3,
            3 => UDSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == UDSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == UDSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == UDSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == UDSR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct LDSR {
    bits: u8,
}
impl LDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTSR {
    #[doc = "External Count Function deactivated"]
    VALUE1,
    #[doc = "External Count Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Count Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Count Function triggered by Event 2"]
    VALUE4,
}
impl CNTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CNTSR::VALUE1 => 0,
            CNTSR::VALUE2 => 1,
            CNTSR::VALUE3 => 2,
            CNTSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CNTSR {
        match value {
            0 => CNTSR::VALUE1,
            1 => CNTSR::VALUE2,
            2 => CNTSR::VALUE3,
            3 => CNTSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNTSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CNTSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CNTSR::VALUE4
    }
}
#[doc = "Possible values of the field `OFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFSR {
    #[doc = "Override functionality disabled"]
    VALUE1,
    #[doc = "Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    VALUE2,
}
impl OFSR {
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
            OFSR::VALUE1 => false,
            OFSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFSR {
        match value {
            false => OFSR::VALUE1,
            true => OFSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OFSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OFSR::VALUE2
    }
}
#[doc = "Possible values of the field `TS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSR {
    #[doc = "Trap function disabled"]
    VALUE1,
    #[doc = "TRAP function connected to Event 2"]
    VALUE2,
}
impl TSR {
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
            TSR::VALUE1 => false,
            TSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSR {
        match value {
            false => TSR::VALUE1,
            true => TSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct MOSR {
    bits: u8,
}
impl MOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCER {
    #[doc = "Timer concatenation is disabled"]
    VALUE1,
    #[doc = "Timer concatenation is enabled"]
    VALUE2,
}
impl TCER {
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
            TCER::VALUE1 => false,
            TCER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCER {
        match value {
            false => TCER::VALUE1,
            true => TCER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TCER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TCER::VALUE2
    }
}
#[doc = "Values that can be written to the field `STRTS`"]
pub enum STRTSW {
    #[doc = "External Start Function deactivated"]
    VALUE1,
    #[doc = "External Start Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Start Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Start Function triggered by Event 2"]
    VALUE4,
}
impl STRTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STRTSW::VALUE1 => 0,
            STRTSW::VALUE2 => 1,
            STRTSW::VALUE3 => 2,
            STRTSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STRTSW<'a> {
    w: &'a mut W,
}
impl<'a> _STRTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STRTSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External Start Function deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STRTSW::VALUE1)
    }
    #[doc = "External Start Function triggered by Event 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STRTSW::VALUE2)
    }
    #[doc = "External Start Function triggered by Event 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(STRTSW::VALUE3)
    }
    #[doc = "External Start Function triggered by Event 2"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(STRTSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENDS`"]
pub enum ENDSW {
    #[doc = "External Stop Function deactivated"]
    VALUE1,
    #[doc = "External Stop Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Stop Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Stop Function triggered by Event 2"]
    VALUE4,
}
impl ENDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENDSW::VALUE1 => 0,
            ENDSW::VALUE2 => 1,
            ENDSW::VALUE3 => 2,
            ENDSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDSW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External Stop Function deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDSW::VALUE1)
    }
    #[doc = "External Stop Function triggered by Event 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENDSW::VALUE2)
    }
    #[doc = "External Stop Function triggered by Event 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENDSW::VALUE3)
    }
    #[doc = "External Stop Function triggered by Event 2"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ENDSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAP0S`"]
pub enum CAP0SW {
    #[doc = "External Capture 0 Function deactivated"]
    VALUE1,
    #[doc = "External Capture 0 Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Capture 0 Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Capture 0 Function triggered by Event 2"]
    VALUE4,
}
impl CAP0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAP0SW::VALUE1 => 0,
            CAP0SW::VALUE2 => 1,
            CAP0SW::VALUE3 => 2,
            CAP0SW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP0SW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP0SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External Capture 0 Function deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CAP0SW::VALUE1)
    }
    #[doc = "External Capture 0 Function triggered by Event 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CAP0SW::VALUE2)
    }
    #[doc = "External Capture 0 Function triggered by Event 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CAP0SW::VALUE3)
    }
    #[doc = "External Capture 0 Function triggered by Event 2"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CAP0SW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAP1S`"]
pub enum CAP1SW {
    #[doc = "External Capture 1 Function deactivated"]
    VALUE1,
    #[doc = "External Capture 1 Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Capture 1 Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Capture 1 Function triggered by Event 2"]
    VALUE4,
}
impl CAP1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAP1SW::VALUE1 => 0,
            CAP1SW::VALUE2 => 1,
            CAP1SW::VALUE3 => 2,
            CAP1SW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP1SW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP1SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External Capture 1 Function deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CAP1SW::VALUE1)
    }
    #[doc = "External Capture 1 Function triggered by Event 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CAP1SW::VALUE2)
    }
    #[doc = "External Capture 1 Function triggered by Event 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CAP1SW::VALUE3)
    }
    #[doc = "External Capture 1 Function triggered by Event 2"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CAP1SW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GATES`"]
pub enum GATESW {
    #[doc = "External Gating Function deactivated"]
    VALUE1,
    #[doc = "External Gating Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Gating Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Gating Function triggered by Event 2"]
    VALUE4,
}
impl GATESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GATESW::VALUE1 => 0,
            GATESW::VALUE2 => 1,
            GATESW::VALUE3 => 2,
            GATESW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GATESW<'a> {
    w: &'a mut W,
}
impl<'a> _GATESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GATESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External Gating Function deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GATESW::VALUE1)
    }
    #[doc = "External Gating Function triggered by Event 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GATESW::VALUE2)
    }
    #[doc = "External Gating Function triggered by Event 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GATESW::VALUE3)
    }
    #[doc = "External Gating Function triggered by Event 2"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GATESW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UDS`"]
pub enum UDSW {
    #[doc = "External Up/Down Function deactivated"]
    VALUE1,
    #[doc = "External Up/Down Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Up/Down Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Up/Down Function triggered by Event 2"]
    VALUE4,
}
impl UDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UDSW::VALUE1 => 0,
            UDSW::VALUE2 => 1,
            UDSW::VALUE3 => 2,
            UDSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UDSW<'a> {
    w: &'a mut W,
}
impl<'a> _UDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UDSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External Up/Down Function deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(UDSW::VALUE1)
    }
    #[doc = "External Up/Down Function triggered by Event 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(UDSW::VALUE2)
    }
    #[doc = "External Up/Down Function triggered by Event 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(UDSW::VALUE3)
    }
    #[doc = "External Up/Down Function triggered by Event 2"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(UDSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LDSW<'a> {
    w: &'a mut W,
}
impl<'a> _LDSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNTS`"]
pub enum CNTSW {
    #[doc = "External Count Function deactivated"]
    VALUE1,
    #[doc = "External Count Function triggered by Event 0"]
    VALUE2,
    #[doc = "External Count Function triggered by Event 1"]
    VALUE3,
    #[doc = "External Count Function triggered by Event 2"]
    VALUE4,
}
impl CNTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CNTSW::VALUE1 => 0,
            CNTSW::VALUE2 => 1,
            CNTSW::VALUE3 => 2,
            CNTSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTSW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External Count Function deactivated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNTSW::VALUE1)
    }
    #[doc = "External Count Function triggered by Event 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNTSW::VALUE2)
    }
    #[doc = "External Count Function triggered by Event 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CNTSW::VALUE3)
    }
    #[doc = "External Count Function triggered by Event 2"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CNTSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OFS`"]
pub enum OFSW {
    #[doc = "Override functionality disabled"]
    VALUE1,
    #[doc = "Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    VALUE2,
}
impl OFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFSW::VALUE1 => false,
            OFSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFSW<'a> {
    w: &'a mut W,
}
impl<'a> _OFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Override functionality disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OFSW::VALUE1)
    }
    #[doc = "Status bit trigger override connected to Event 1; Status bit value override connected to Event 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OFSW::VALUE2)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TS`"]
pub enum TSW {
    #[doc = "Trap function disabled"]
    VALUE1,
    #[doc = "TRAP function connected to Event 2"]
    VALUE2,
}
impl TSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSW::VALUE1 => false,
            TSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSW::VALUE1)
    }
    #[doc = "TRAP function connected to Event 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSW::VALUE2)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MOSW<'a> {
    w: &'a mut W,
}
impl<'a> _MOSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCE`"]
pub enum TCEW {
    #[doc = "Timer concatenation is disabled"]
    VALUE1,
    #[doc = "Timer concatenation is enabled"]
    VALUE2,
}
impl TCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCEW::VALUE1 => false,
            TCEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer concatenation is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TCEW::VALUE1)
    }
    #[doc = "Timer concatenation is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TCEW::VALUE2)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:1 - External Start Functionality Selector"]
    #[inline]
    pub fn strts(&self) -> STRTSR {
        STRTSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - External Stop Functionality Selector"]
    #[inline]
    pub fn ends(&self) -> ENDSR {
        ENDSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - External Capture 0 Functionality Selector"]
    #[inline]
    pub fn cap0s(&self) -> CAP0SR {
        CAP0SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - External Capture 1 Functionality Selector"]
    #[inline]
    pub fn cap1s(&self) -> CAP1SR {
        CAP1SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - External Gate Functionality Selector"]
    #[inline]
    pub fn gates(&self) -> GATESR {
        GATESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - External Up/Down Functionality Selector"]
    #[inline]
    pub fn uds(&self) -> UDSR {
        UDSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - External Timer Load Functionality Selector"]
    #[inline]
    pub fn lds(&self) -> LDSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LDSR { bits }
    }
    #[doc = "Bits 14:15 - External Count Selector"]
    #[inline]
    pub fn cnts(&self) -> CNTSR {
        CNTSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Override Function Selector"]
    #[inline]
    pub fn ofs(&self) -> OFSR {
        OFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Trap Function Selector"]
    #[inline]
    pub fn ts(&self) -> TSR {
        TSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - External Modulation Functionality Selector"]
    #[inline]
    pub fn mos(&self) -> MOSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MOSR { bits }
    }
    #[doc = "Bit 20 - Timer Concatenation Enable"]
    #[inline]
    pub fn tce(&self) -> TCER {
        TCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:1 - External Start Functionality Selector"]
    #[inline]
    pub fn strts(&mut self) -> _STRTSW {
        _STRTSW { w: self }
    }
    #[doc = "Bits 2:3 - External Stop Functionality Selector"]
    #[inline]
    pub fn ends(&mut self) -> _ENDSW {
        _ENDSW { w: self }
    }
    #[doc = "Bits 4:5 - External Capture 0 Functionality Selector"]
    #[inline]
    pub fn cap0s(&mut self) -> _CAP0SW {
        _CAP0SW { w: self }
    }
    #[doc = "Bits 6:7 - External Capture 1 Functionality Selector"]
    #[inline]
    pub fn cap1s(&mut self) -> _CAP1SW {
        _CAP1SW { w: self }
    }
    #[doc = "Bits 8:9 - External Gate Functionality Selector"]
    #[inline]
    pub fn gates(&mut self) -> _GATESW {
        _GATESW { w: self }
    }
    #[doc = "Bits 10:11 - External Up/Down Functionality Selector"]
    #[inline]
    pub fn uds(&mut self) -> _UDSW {
        _UDSW { w: self }
    }
    #[doc = "Bits 12:13 - External Timer Load Functionality Selector"]
    #[inline]
    pub fn lds(&mut self) -> _LDSW {
        _LDSW { w: self }
    }
    #[doc = "Bits 14:15 - External Count Selector"]
    #[inline]
    pub fn cnts(&mut self) -> _CNTSW {
        _CNTSW { w: self }
    }
    #[doc = "Bit 16 - Override Function Selector"]
    #[inline]
    pub fn ofs(&mut self) -> _OFSW {
        _OFSW { w: self }
    }
    #[doc = "Bit 17 - Trap Function Selector"]
    #[inline]
    pub fn ts(&mut self) -> _TSW {
        _TSW { w: self }
    }
    #[doc = "Bits 18:19 - External Modulation Functionality Selector"]
    #[inline]
    pub fn mos(&mut self) -> _MOSW {
        _MOSW { w: self }
    }
    #[doc = "Bit 20 - Timer Concatenation Enable"]
    #[inline]
    pub fn tce(&mut self) -> _TCEW {
        _TCEW { w: self }
    }
}
