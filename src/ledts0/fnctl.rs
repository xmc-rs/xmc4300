#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FNCTL {
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
#[doc = "Possible values of the field `PADT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADTR {
    #[doc = "TSIN0"]
    VALUE1,
    #[doc = "TSIN7"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PADTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PADTR::VALUE1 => 0,
            PADTR::VALUE2 => 7,
            PADTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PADTR {
        match value {
            0 => PADTR::VALUE1,
            7 => PADTR::VALUE2,
            i => PADTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PADTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PADTR::VALUE2
    }
}
#[doc = "Possible values of the field `PADTSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADTSWR {
    #[doc = "The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    VALUE1,
    #[doc = "Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    VALUE2,
}
impl PADTSWR {
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
            PADTSWR::VALUE1 => false,
            PADTSWR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PADTSWR {
        match value {
            false => PADTSWR::VALUE1,
            true => PADTSWR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PADTSWR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PADTSWR::VALUE2
    }
}
#[doc = "Possible values of the field `EPULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPULLR {
    #[doc = "HW over-rule to enable internal pull-up is active on TSIN\\[x\\] for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    VALUE1,
    #[doc = "Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    VALUE2,
}
impl EPULLR {
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
            EPULLR::VALUE1 => false,
            EPULLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPULLR {
        match value {
            false => EPULLR::VALUE1,
            true => EPULLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EPULLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EPULLR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct FNCOLR {
    bits: u8,
}
impl FNCOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ACCCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCCNTR {
    #[doc = "1 time"]
    VALUE1,
    #[doc = "2 times"]
    VALUE2,
    #[doc = "16 times"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACCCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACCCNTR::VALUE1 => 0,
            ACCCNTR::VALUE2 => 1,
            ACCCNTR::VALUE3 => 15,
            ACCCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACCCNTR {
        match value {
            0 => ACCCNTR::VALUE1,
            1 => ACCCNTR::VALUE2,
            15 => ACCCNTR::VALUE3,
            i => ACCCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACCCNTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACCCNTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ACCCNTR::VALUE3
    }
}
#[doc = "Possible values of the field `TSCCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCCMPR {
    #[doc = "Disable common compare for touch-sense"]
    VALUE1,
    #[doc = "Enable common compare for touch-sense"]
    VALUE2,
}
impl TSCCMPR {
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
            TSCCMPR::VALUE1 => false,
            TSCCMPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSCCMPR {
        match value {
            false => TSCCMPR::VALUE1,
            true => TSCCMPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSCCMPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSCCMPR::VALUE2
    }
}
#[doc = "Possible values of the field `TSOEXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOEXTR {
    #[doc = "Extend by 1 ledts_clk"]
    VALUE1,
    #[doc = "Extend by 4 ledts_clk"]
    VALUE2,
    #[doc = "Extend by 8 ledts_clk"]
    VALUE3,
    #[doc = "Extend by 16 ledts_clk"]
    VALUE4,
}
impl TSOEXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSOEXTR::VALUE1 => 0,
            TSOEXTR::VALUE2 => 1,
            TSOEXTR::VALUE3 => 2,
            TSOEXTR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSOEXTR {
        match value {
            0 => TSOEXTR::VALUE1,
            1 => TSOEXTR::VALUE2,
            2 => TSOEXTR::VALUE3,
            3 => TSOEXTR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSOEXTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSOEXTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TSOEXTR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TSOEXTR::VALUE4
    }
}
#[doc = "Possible values of the field `TSCTRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCTRRR {
    #[doc = "Disable TS-counter automatic reset"]
    VALUE1,
    #[doc = "Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    VALUE2,
}
impl TSCTRRR {
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
            TSCTRRR::VALUE1 => false,
            TSCTRRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSCTRRR {
        match value {
            false => TSCTRRR::VALUE1,
            true => TSCTRRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSCTRRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSCTRRR::VALUE2
    }
}
#[doc = "Possible values of the field `TSCTRSAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCTRSATR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    VALUE2,
}
impl TSCTRSATR {
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
            TSCTRSATR::VALUE1 => false,
            TSCTRSATR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSCTRSATR {
        match value {
            false => TSCTRSATR::VALUE1,
            true => TSCTRSATR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSCTRSATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSCTRSATR::VALUE2
    }
}
#[doc = "Possible values of the field `NR_TSIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NR_TSINR {
    #[doc = "1"]
    VALUE1,
    #[doc = "8"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NR_TSINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NR_TSINR::VALUE1 => 0,
            NR_TSINR::VALUE2 => 7,
            NR_TSINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NR_TSINR {
        match value {
            0 => NR_TSINR::VALUE1,
            7 => NR_TSINR::VALUE2,
            i => NR_TSINR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NR_TSINR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NR_TSINR::VALUE2
    }
}
#[doc = "Possible values of the field `COLLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLLEVR {
    #[doc = "Active low"]
    VALUE1,
    #[doc = "Active high"]
    VALUE2,
}
impl COLLEVR {
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
            COLLEVR::VALUE1 => false,
            COLLEVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COLLEVR {
        match value {
            false => COLLEVR::VALUE1,
            true => COLLEVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COLLEVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COLLEVR::VALUE2
    }
}
#[doc = "Possible values of the field `NR_LEDCOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NR_LEDCOLR {
    #[doc = "1 LED column"]
    VALUE1,
    #[doc = "2 LED columns"]
    VALUE2,
    #[doc = "3 LED columns"]
    VALUE3,
    #[doc = "4 LED columns"]
    VALUE4,
    #[doc = "5 LED columns"]
    VALUE5,
    #[doc = "6 LED columns"]
    VALUE6,
    #[doc = "7 LED columns"]
    VALUE7,
    #[doc = "8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    VALUE8,
}
impl NR_LEDCOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NR_LEDCOLR::VALUE1 => 0,
            NR_LEDCOLR::VALUE2 => 1,
            NR_LEDCOLR::VALUE3 => 2,
            NR_LEDCOLR::VALUE4 => 3,
            NR_LEDCOLR::VALUE5 => 4,
            NR_LEDCOLR::VALUE6 => 5,
            NR_LEDCOLR::VALUE7 => 6,
            NR_LEDCOLR::VALUE8 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NR_LEDCOLR {
        match value {
            0 => NR_LEDCOLR::VALUE1,
            1 => NR_LEDCOLR::VALUE2,
            2 => NR_LEDCOLR::VALUE3,
            3 => NR_LEDCOLR::VALUE4,
            4 => NR_LEDCOLR::VALUE5,
            5 => NR_LEDCOLR::VALUE6,
            6 => NR_LEDCOLR::VALUE7,
            7 => NR_LEDCOLR::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NR_LEDCOLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NR_LEDCOLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == NR_LEDCOLR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == NR_LEDCOLR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == NR_LEDCOLR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == NR_LEDCOLR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == NR_LEDCOLR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == NR_LEDCOLR::VALUE8
    }
}
#[doc = "Values that can be written to the field `PADT`"]
pub enum PADTW {
    #[doc = "TSIN0"]
    VALUE1,
    #[doc = "TSIN7"]
    VALUE2,
}
impl PADTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PADTW::VALUE1 => 0,
            PADTW::VALUE2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PADTW<'a> {
    w: &'a mut W,
}
impl<'a> _PADTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PADTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TSIN0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PADTW::VALUE1)
    }
    #[doc = "TSIN7"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PADTW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PADTSW`"]
pub enum PADTSWW {
    #[doc = "The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    VALUE1,
    #[doc = "Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    VALUE2,
}
impl PADTSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PADTSWW::VALUE1 => false,
            PADTSWW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PADTSWW<'a> {
    w: &'a mut W,
}
impl<'a> _PADTSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PADTSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hardware automatically enables the touch-sense inputs in sequence round-robin, starting from TSIN0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PADTSWW::VALUE1)
    }
    #[doc = "Disable hardware control for software control only. The touch-sense input is configured in bit PADT."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PADTSWW::VALUE2)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPULL`"]
pub enum EPULLW {
    #[doc = "HW over-rule to enable internal pull-up is active on TSIN\\[x\\] for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    VALUE1,
    #[doc = "Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    VALUE2,
}
impl EPULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPULLW::VALUE1 => false,
            EPULLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPULLW<'a> {
    w: &'a mut W,
}
impl<'a> _EPULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HW over-rule to enable internal pull-up is active on TSIN\\[x\\] for set duration in touch-sense time slice. With this setting, it is not specified to assign the COLA to any pin."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPULLW::VALUE1)
    }
    #[doc = "Enable external pull-up: Output 1 on pin COLA for whole duration of touch-sense time slice."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPULLW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACCCNT`"]
pub enum ACCCNTW {
    #[doc = "1 time"]
    VALUE1,
    #[doc = "2 times"]
    VALUE2,
    #[doc = "16 times"]
    VALUE3,
}
impl ACCCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACCCNTW::VALUE1 => 0,
            ACCCNTW::VALUE2 => 1,
            ACCCNTW::VALUE3 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACCCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACCCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 time"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACCCNTW::VALUE1)
    }
    #[doc = "2 times"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACCCNTW::VALUE2)
    }
    #[doc = "16 times"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ACCCNTW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSCCMP`"]
pub enum TSCCMPW {
    #[doc = "Disable common compare for touch-sense"]
    VALUE1,
    #[doc = "Enable common compare for touch-sense"]
    VALUE2,
}
impl TSCCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSCCMPW::VALUE1 => false,
            TSCCMPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSCCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCCMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSCCMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable common compare for touch-sense"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSCCMPW::VALUE1)
    }
    #[doc = "Enable common compare for touch-sense"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSCCMPW::VALUE2)
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
#[doc = "Values that can be written to the field `TSOEXT`"]
pub enum TSOEXTW {
    #[doc = "Extend by 1 ledts_clk"]
    VALUE1,
    #[doc = "Extend by 4 ledts_clk"]
    VALUE2,
    #[doc = "Extend by 8 ledts_clk"]
    VALUE3,
    #[doc = "Extend by 16 ledts_clk"]
    VALUE4,
}
impl TSOEXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSOEXTW::VALUE1 => 0,
            TSOEXTW::VALUE2 => 1,
            TSOEXTW::VALUE3 => 2,
            TSOEXTW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSOEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSOEXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSOEXTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Extend by 1 ledts_clk"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSOEXTW::VALUE1)
    }
    #[doc = "Extend by 4 ledts_clk"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSOEXTW::VALUE2)
    }
    #[doc = "Extend by 8 ledts_clk"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSOEXTW::VALUE3)
    }
    #[doc = "Extend by 16 ledts_clk"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSOEXTW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSCTRR`"]
pub enum TSCTRRW {
    #[doc = "Disable TS-counter automatic reset"]
    VALUE1,
    #[doc = "Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    VALUE2,
}
impl TSCTRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSCTRRW::VALUE1 => false,
            TSCTRRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSCTRRW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCTRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSCTRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable TS-counter automatic reset"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSCTRRW::VALUE1)
    }
    #[doc = "Enable TS-counter automatic reset to 00H on the first pad turn of a new TSIN\\[x\\]. Triggered on compare match in time slice."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSCTRRW::VALUE2)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSCTRSAT`"]
pub enum TSCTRSATW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    VALUE2,
}
impl TSCTRSATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSCTRSATW::VALUE1 => false,
            TSCTRSATW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSCTRSATW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCTRSATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSCTRSATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSCTRSATW::VALUE1)
    }
    #[doc = "Enable. TS-counter stops counting in the touch-sense time slice(s) of the same (extended) frame when it reaches FFH. Counter starts to count again on the first pad turn of a new TSIN\\[x\\], triggered on compare match."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSCTRSATW::VALUE2)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NR_TSIN`"]
pub enum NR_TSINW {
    #[doc = "1"]
    VALUE1,
    #[doc = "8"]
    VALUE2,
}
impl NR_TSINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NR_TSINW::VALUE1 => 0,
            NR_TSINW::VALUE2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NR_TSINW<'a> {
    w: &'a mut W,
}
impl<'a> _NR_TSINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NR_TSINW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NR_TSINW::VALUE1)
    }
    #[doc = "8"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NR_TSINW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COLLEV`"]
pub enum COLLEVW {
    #[doc = "Active low"]
    VALUE1,
    #[doc = "Active high"]
    VALUE2,
}
impl COLLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COLLEVW::VALUE1 => false,
            COLLEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COLLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _COLLEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COLLEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(COLLEVW::VALUE1)
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(COLLEVW::VALUE2)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NR_LEDCOL`"]
pub enum NR_LEDCOLW {
    #[doc = "1 LED column"]
    VALUE1,
    #[doc = "2 LED columns"]
    VALUE2,
    #[doc = "3 LED columns"]
    VALUE3,
    #[doc = "4 LED columns"]
    VALUE4,
    #[doc = "5 LED columns"]
    VALUE5,
    #[doc = "6 LED columns"]
    VALUE6,
    #[doc = "7 LED columns"]
    VALUE7,
    #[doc = "8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    VALUE8,
}
impl NR_LEDCOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NR_LEDCOLW::VALUE1 => 0,
            NR_LEDCOLW::VALUE2 => 1,
            NR_LEDCOLW::VALUE3 => 2,
            NR_LEDCOLW::VALUE4 => 3,
            NR_LEDCOLW::VALUE5 => 4,
            NR_LEDCOLW::VALUE6 => 5,
            NR_LEDCOLW::VALUE7 => 6,
            NR_LEDCOLW::VALUE8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NR_LEDCOLW<'a> {
    w: &'a mut W,
}
impl<'a> _NR_LEDCOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NR_LEDCOLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 LED column"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NR_LEDCOLW::VALUE1)
    }
    #[doc = "2 LED columns"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NR_LEDCOLW::VALUE2)
    }
    #[doc = "3 LED columns"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(NR_LEDCOLW::VALUE3)
    }
    #[doc = "4 LED columns"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(NR_LEDCOLW::VALUE4)
    }
    #[doc = "5 LED columns"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(NR_LEDCOLW::VALUE5)
    }
    #[doc = "6 LED columns"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(NR_LEDCOLW::VALUE6)
    }
    #[doc = "7 LED columns"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(NR_LEDCOLW::VALUE7)
    }
    #[doc = "8 LED columns (max. LED columns = 7 if bit TS_EN = 1)"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(NR_LEDCOLW::VALUE8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline]
    pub fn padt(&self) -> PADTR {
        PADTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline]
    pub fn padtsw(&self) -> PADTSWR {
        PADTSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline]
    pub fn epull(&self) -> EPULLR {
        EPULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - Previous Active Function/LED Column Status"]
    #[inline]
    pub fn fncol(&self) -> FNCOLR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FNCOLR { bits }
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline]
    pub fn acccnt(&self) -> ACCCNTR {
        ACCCNTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline]
    pub fn tsccmp(&self) -> TSCCMPR {
        TSCCMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline]
    pub fn tsoext(&self) -> TSOEXTR {
        TSOEXTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline]
    pub fn tsctrr(&self) -> TSCTRRR {
        TSCTRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline]
    pub fn tsctrsat(&self) -> TSCTRSATR {
        TSCTRSATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline]
    pub fn nr_tsin(&self) -> NR_TSINR {
        NR_TSINR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline]
    pub fn collev(&self) -> COLLEVR {
        COLLEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline]
    pub fn nr_ledcol(&self) -> NR_LEDCOLR {
        NR_LEDCOLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:2 - Touch-Sense TSIN Pad Turn"]
    #[inline]
    pub fn padt(&mut self) -> _PADTW {
        _PADTW { w: self }
    }
    #[doc = "Bit 3 - Software Control for Touch-Sense Pad Turn"]
    #[inline]
    pub fn padtsw(&mut self) -> _PADTSWW {
        _PADTSWW { w: self }
    }
    #[doc = "Bit 4 - Enable External Pull-up Configuration on Pin COLA"]
    #[inline]
    pub fn epull(&mut self) -> _EPULLW {
        _EPULLW { w: self }
    }
    #[doc = "Bits 16:19 - Accumulate Count on Touch-Sense Input"]
    #[inline]
    pub fn acccnt(&mut self) -> _ACCCNTW {
        _ACCCNTW { w: self }
    }
    #[doc = "Bit 20 - Common Compare Enable for Touch-Sense"]
    #[inline]
    pub fn tsccmp(&mut self) -> _TSCCMPW {
        _TSCCMPW { w: self }
    }
    #[doc = "Bits 21:22 - Extension for Touch-Sense Output for Pin-Low-Level"]
    #[inline]
    pub fn tsoext(&mut self) -> _TSOEXTW {
        _TSOEXTW { w: self }
    }
    #[doc = "Bit 23 - TS-Counter Auto Reset"]
    #[inline]
    pub fn tsctrr(&mut self) -> _TSCTRRW {
        _TSCTRRW { w: self }
    }
    #[doc = "Bit 24 - Saturation of TS-Counter"]
    #[inline]
    pub fn tsctrsat(&mut self) -> _TSCTRSATW {
        _TSCTRSATW { w: self }
    }
    #[doc = "Bits 25:27 - Number of Touch-Sense Input"]
    #[inline]
    pub fn nr_tsin(&mut self) -> _NR_TSINW {
        _NR_TSINW { w: self }
    }
    #[doc = "Bit 28 - Active Level of LED Column"]
    #[inline]
    pub fn collev(&mut self) -> _COLLEVW {
        _COLLEVW { w: self }
    }
    #[doc = "Bits 29:31 - Number of LED Columns"]
    #[inline]
    pub fn nr_ledcol(&mut self) -> _NR_LEDCOLW {
        _NR_LEDCOLW { w: self }
    }
}
