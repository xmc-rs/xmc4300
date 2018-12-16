#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DAC0CFG1 {
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
#[doc = "Possible values of the field `SCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCALER {
    #[doc = "no shift = multiplication/division by 1"]
    VALUE1,
    #[doc = "shift by 1 = multiplication/division by 2"]
    VALUE2,
    #[doc = "shift by 2 = multiplication/division by 4"]
    VALUE3,
    #[doc = "shift left by 3 = multiplication/division by 8"]
    VALUE4,
    #[doc = "shift left by 4 = multiplication/division by 16"]
    VALUE5,
    #[doc = "shift left by 5 = multiplication/division by 32"]
    VALUE6,
    #[doc = "shift left by 6 = multiplication/division by 64"]
    VALUE7,
    #[doc = "shift left by 7 = multiplication/division by 128"]
    VALUE8,
}
impl SCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCALER::VALUE1 => 0,
            SCALER::VALUE2 => 1,
            SCALER::VALUE3 => 2,
            SCALER::VALUE4 => 3,
            SCALER::VALUE5 => 4,
            SCALER::VALUE6 => 5,
            SCALER::VALUE7 => 6,
            SCALER::VALUE8 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCALER {
        match value {
            0 => SCALER::VALUE1,
            1 => SCALER::VALUE2,
            2 => SCALER::VALUE3,
            3 => SCALER::VALUE4,
            4 => SCALER::VALUE5,
            5 => SCALER::VALUE6,
            6 => SCALER::VALUE7,
            7 => SCALER::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SCALER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SCALER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SCALER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SCALER::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == SCALER::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == SCALER::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == SCALER::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == SCALER::VALUE8
    }
}
#[doc = "Possible values of the field `MULDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULDIVR {
    #[doc = "downscale = division (shift SCALE positions to the right)"]
    VALUE1,
    #[doc = "upscale = multiplication (shift SCALE positions to the left)"]
    VALUE2,
}
impl MULDIVR {
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
            MULDIVR::VALUE1 => false,
            MULDIVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MULDIVR {
        match value {
            false => MULDIVR::VALUE1,
            true => MULDIVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MULDIVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MULDIVR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct OFFSR {
    bits: u8,
}
impl OFFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIGSELR {
    bits: u8,
}
impl TRIGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DATMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMODR {
    #[doc = "independent data handling - process data from DATA0 register (bits 11:0) to DAC0 and data from DATA1 register (bits 11:0) to DAC1"]
    VALUE1,
    #[doc = "simultaneous data handling - process data from DAC01 register to both DACs (bits 11:0 to DAC0 and bits 23:12 to DAC1)."]
    VALUE2,
}
impl DATMODR {
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
            DATMODR::VALUE1 => false,
            DATMODR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATMODR {
        match value {
            false => DATMODR::VALUE1,
            true => DATMODR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATMODR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATMODR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct SWTRIGR {
    bits: bool,
}
impl SWTRIGR {
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
#[doc = "Possible values of the field `TRIGMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGMODR {
    #[doc = "internal Trigger (integer divided clock - see FREQ parameter)"]
    VALUE1,
    #[doc = "external Trigger (preselected trigger by TRIGSEL parameter)"]
    VALUE2,
    #[doc = "software Trigger (see SWTRIG parameter)"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGMODR::VALUE1 => 0,
            TRIGMODR::VALUE2 => 1,
            TRIGMODR::VALUE3 => 2,
            TRIGMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGMODR {
        match value {
            0 => TRIGMODR::VALUE1,
            1 => TRIGMODR::VALUE2,
            2 => TRIGMODR::VALUE3,
            i => TRIGMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRIGMODR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRIGMODR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TRIGMODR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct ANACFGR {
    bits: u8,
}
impl ANACFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ANAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANAENR {
    #[doc = "DAC0 is set to standby (analog output only)"]
    VALUE1,
    #[doc = "enable DAC0 (analog output only)"]
    VALUE2,
}
impl ANAENR {
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
            ANAENR::VALUE1 => false,
            ANAENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANAENR {
        match value {
            false => ANAENR::VALUE1,
            true => ANAENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ANAENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ANAENR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct REFCFGLR {
    bits: u8,
}
impl REFCFGLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SCALE`"]
pub enum SCALEW {
    #[doc = "no shift = multiplication/division by 1"]
    VALUE1,
    #[doc = "shift by 1 = multiplication/division by 2"]
    VALUE2,
    #[doc = "shift by 2 = multiplication/division by 4"]
    VALUE3,
    #[doc = "shift left by 3 = multiplication/division by 8"]
    VALUE4,
    #[doc = "shift left by 4 = multiplication/division by 16"]
    VALUE5,
    #[doc = "shift left by 5 = multiplication/division by 32"]
    VALUE6,
    #[doc = "shift left by 6 = multiplication/division by 64"]
    VALUE7,
    #[doc = "shift left by 7 = multiplication/division by 128"]
    VALUE8,
}
impl SCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCALEW::VALUE1 => 0,
            SCALEW::VALUE2 => 1,
            SCALEW::VALUE3 => 2,
            SCALEW::VALUE4 => 3,
            SCALEW::VALUE5 => 4,
            SCALEW::VALUE6 => 5,
            SCALEW::VALUE7 => 6,
            SCALEW::VALUE8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _SCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCALEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no shift = multiplication/division by 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCALEW::VALUE1)
    }
    #[doc = "shift by 1 = multiplication/division by 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCALEW::VALUE2)
    }
    #[doc = "shift by 2 = multiplication/division by 4"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCALEW::VALUE3)
    }
    #[doc = "shift left by 3 = multiplication/division by 8"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCALEW::VALUE4)
    }
    #[doc = "shift left by 4 = multiplication/division by 16"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(SCALEW::VALUE5)
    }
    #[doc = "shift left by 5 = multiplication/division by 32"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(SCALEW::VALUE6)
    }
    #[doc = "shift left by 6 = multiplication/division by 64"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(SCALEW::VALUE7)
    }
    #[doc = "shift left by 7 = multiplication/division by 128"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(SCALEW::VALUE8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MULDIV`"]
pub enum MULDIVW {
    #[doc = "downscale = division (shift SCALE positions to the right)"]
    VALUE1,
    #[doc = "upscale = multiplication (shift SCALE positions to the left)"]
    VALUE2,
}
impl MULDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MULDIVW::VALUE1 => false,
            MULDIVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MULDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _MULDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MULDIVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "downscale = division (shift SCALE positions to the right)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MULDIVW::VALUE1)
    }
    #[doc = "upscale = multiplication (shift SCALE positions to the left)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MULDIVW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _OFFSW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATMOD`"]
pub enum DATMODW {
    #[doc = "independent data handling - process data from DATA0 register (bits 11:0) to DAC0 and data from DATA1 register (bits 11:0) to DAC1"]
    VALUE1,
    #[doc = "simultaneous data handling - process data from DAC01 register to both DACs (bits 11:0 to DAC0 and bits 23:12 to DAC1)."]
    VALUE2,
}
impl DATMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATMODW::VALUE1 => false,
            DATMODW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATMODW<'a> {
    w: &'a mut W,
}
impl<'a> _DATMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "independent data handling - process data from DATA0 register (bits 11:0) to DAC0 and data from DATA1 register (bits 11:0) to DAC1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATMODW::VALUE1)
    }
    #[doc = "simultaneous data handling - process data from DAC01 register to both DACs (bits 11:0 to DAC0 and bits 23:12 to DAC1)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATMODW::VALUE2)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIGW<'a> {
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
#[doc = "Values that can be written to the field `TRIGMOD`"]
pub enum TRIGMODW {
    #[doc = "internal Trigger (integer divided clock - see FREQ parameter)"]
    VALUE1,
    #[doc = "external Trigger (preselected trigger by TRIGSEL parameter)"]
    VALUE2,
    #[doc = "software Trigger (see SWTRIG parameter)"]
    VALUE3,
}
impl TRIGMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGMODW::VALUE1 => 0,
            TRIGMODW::VALUE2 => 1,
            TRIGMODW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGMODW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "internal Trigger (integer divided clock - see FREQ parameter)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIGMODW::VALUE1)
    }
    #[doc = "external Trigger (preselected trigger by TRIGSEL parameter)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIGMODW::VALUE2)
    }
    #[doc = "software Trigger (see SWTRIG parameter)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRIGMODW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ANACFGW<'a> {
    w: &'a mut W,
}
impl<'a> _ANACFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ANAEN`"]
pub enum ANAENW {
    #[doc = "DAC0 is set to standby (analog output only)"]
    VALUE1,
    #[doc = "enable DAC0 (analog output only)"]
    VALUE2,
}
impl ANAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANAENW::VALUE1 => false,
            ANAENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANAENW<'a> {
    w: &'a mut W,
}
impl<'a> _ANAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC0 is set to standby (analog output only)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ANAENW::VALUE1)
    }
    #[doc = "enable DAC0 (analog output only)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ANAENW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _REFCFGLW<'a> {
    w: &'a mut W,
}
impl<'a> _REFCFGLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
    #[inline]
    pub fn scale(&self) -> SCALER {
        SCALER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC0 input data values"]
    #[inline]
    pub fn muldiv(&self) -> MULDIVR {
        MULDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline]
    pub fn offs(&self) -> OFFSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OFFSR { bits }
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC0"]
    #[inline]
    pub fn trigsel(&self) -> TRIGSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIGSELR { bits }
    }
    #[doc = "Bit 15 - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
    #[inline]
    pub fn datmod(&self) -> DATMODR {
        DATMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline]
    pub fn swtrig(&self) -> SWTRIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWTRIGR { bits }
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 0"]
    #[inline]
    pub fn trigmod(&self) -> TRIGMODR {
        TRIGMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:23 - DAC0 analog configuration/calibration parameters"]
    #[inline]
    pub fn anacfg(&self) -> ANACFGR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ANACFGR { bits }
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 0"]
    #[inline]
    pub fn anaen(&self) -> ANAENR {
        ANAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - Lower 4 band-gap configuration/calibration parameters"]
    #[inline]
    pub fn refcfgl(&self) -> REFCFGLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REFCFGLR { bits }
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
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
    #[inline]
    pub fn scale(&mut self) -> _SCALEW {
        _SCALEW { w: self }
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC0 input data values"]
    #[inline]
    pub fn muldiv(&mut self) -> _MULDIVW {
        _MULDIVW { w: self }
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline]
    pub fn offs(&mut self) -> _OFFSW {
        _OFFSW { w: self }
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC0"]
    #[inline]
    pub fn trigsel(&mut self) -> _TRIGSELW {
        _TRIGSELW { w: self }
    }
    #[doc = "Bit 15 - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
    #[inline]
    pub fn datmod(&mut self) -> _DATMODW {
        _DATMODW { w: self }
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline]
    pub fn swtrig(&mut self) -> _SWTRIGW {
        _SWTRIGW { w: self }
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 0"]
    #[inline]
    pub fn trigmod(&mut self) -> _TRIGMODW {
        _TRIGMODW { w: self }
    }
    #[doc = "Bits 19:23 - DAC0 analog configuration/calibration parameters"]
    #[inline]
    pub fn anacfg(&mut self) -> _ANACFGW {
        _ANACFGW { w: self }
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 0"]
    #[inline]
    pub fn anaen(&mut self) -> _ANAENW {
        _ANAENW { w: self }
    }
    #[doc = "Bits 28:31 - Lower 4 band-gap configuration/calibration parameters"]
    #[inline]
    pub fn refcfgl(&mut self) -> _REFCFGLW {
        _REFCFGLW { w: self }
    }
}
