#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBCFG {
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
#[doc = "Possible values of the field `DIVA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVAR {
    #[doc = "fADCI = fADC / 2"]
    VALUE1,
    #[doc = "fADCI = fADC / 2"]
    VALUE2,
    #[doc = "fADCI = fADC / 3"]
    VALUE3,
    #[doc = "fADCI = fADC / 32"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVAR::VALUE1 => 0,
            DIVAR::VALUE2 => 1,
            DIVAR::VALUE3 => 2,
            DIVAR::VALUE4 => 31,
            DIVAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVAR {
        match value {
            0 => DIVAR::VALUE1,
            1 => DIVAR::VALUE2,
            2 => DIVAR::VALUE3,
            31 => DIVAR::VALUE4,
            i => DIVAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVAR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DIVAR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DIVAR::VALUE4
    }
}
#[doc = "Possible values of the field `DCMSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMSBR {
    #[doc = "1 clock cycles for the MSB (standard)"]
    VALUE1,
    #[doc = "2 clock cycles for the MSB (fADCI > 20 MHz)"]
    VALUE2,
}
impl DCMSBR {
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
            DCMSBR::VALUE1 => false,
            DCMSBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCMSBR {
        match value {
            false => DCMSBR::VALUE1,
            true => DCMSBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DCMSBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DCMSBR::VALUE2
    }
}
#[doc = "Possible values of the field `DIVD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVDR {
    #[doc = "fADCD = fADC"]
    VALUE1,
    #[doc = "fADCD = fADC / 2"]
    VALUE2,
    #[doc = "fADCD = fADC / 3"]
    VALUE3,
    #[doc = "fADCD = fADC / 4"]
    VALUE4,
}
impl DIVDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVDR::VALUE1 => 0,
            DIVDR::VALUE2 => 1,
            DIVDR::VALUE3 => 2,
            DIVDR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVDR {
        match value {
            0 => DIVDR::VALUE1,
            1 => DIVDR::VALUE2,
            2 => DIVDR::VALUE3,
            3 => DIVDR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVDR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DIVDR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DIVDR::VALUE4
    }
}
#[doc = "Possible values of the field `DPCAL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPCAL0R {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    VALUE1,
    #[doc = "No post-calibration"]
    VALUE2,
}
impl DPCAL0R {
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
            DPCAL0R::VALUE1 => false,
            DPCAL0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPCAL0R {
        match value {
            false => DPCAL0R::VALUE1,
            true => DPCAL0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL0R::VALUE2
    }
}
#[doc = "Possible values of the field `DPCAL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPCAL1R {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    VALUE1,
    #[doc = "No post-calibration"]
    VALUE2,
}
impl DPCAL1R {
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
            DPCAL1R::VALUE1 => false,
            DPCAL1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPCAL1R {
        match value {
            false => DPCAL1R::VALUE1,
            true => DPCAL1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL1R::VALUE2
    }
}
#[doc = "Possible values of the field `DPCAL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPCAL2R {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    VALUE1,
    #[doc = "No post-calibration"]
    VALUE2,
}
impl DPCAL2R {
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
            DPCAL2R::VALUE1 => false,
            DPCAL2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPCAL2R {
        match value {
            false => DPCAL2R::VALUE1,
            true => DPCAL2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL2R::VALUE2
    }
}
#[doc = "Possible values of the field `DPCAL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPCAL3R {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    VALUE1,
    #[doc = "No post-calibration"]
    VALUE2,
}
impl DPCAL3R {
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
            DPCAL3R::VALUE1 => false,
            DPCAL3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPCAL3R {
        match value {
            false => DPCAL3R::VALUE1,
            true => DPCAL3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL3R::VALUE2
    }
}
#[doc = "Values that can be written to the field `DIVA`"]
pub enum DIVAW {
    #[doc = "fADCI = fADC / 2"]
    VALUE1,
    #[doc = "fADCI = fADC / 2"]
    VALUE2,
    #[doc = "fADCI = fADC / 3"]
    VALUE3,
    #[doc = "fADCI = fADC / 32"]
    VALUE4,
}
impl DIVAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVAW::VALUE1 => 0,
            DIVAW::VALUE2 => 1,
            DIVAW::VALUE3 => 2,
            DIVAW::VALUE4 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVAW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVAW::VALUE1)
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVAW::VALUE2)
    }
    #[doc = "fADCI = fADC / 3"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVAW::VALUE3)
    }
    #[doc = "fADCI = fADC / 32"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVAW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCMSB`"]
pub enum DCMSBW {
    #[doc = "1 clock cycles for the MSB (standard)"]
    VALUE1,
    #[doc = "2 clock cycles for the MSB (fADCI > 20 MHz)"]
    VALUE2,
}
impl DCMSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCMSBW::VALUE1 => false,
            DCMSBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCMSBW<'a> {
    w: &'a mut W,
}
impl<'a> _DCMSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCMSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 clock cycles for the MSB (standard)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCMSBW::VALUE1)
    }
    #[doc = "2 clock cycles for the MSB (fADCI > 20 MHz)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCMSBW::VALUE2)
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
#[doc = "Values that can be written to the field `DIVD`"]
pub enum DIVDW {
    #[doc = "fADCD = fADC"]
    VALUE1,
    #[doc = "fADCD = fADC / 2"]
    VALUE2,
    #[doc = "fADCD = fADC / 3"]
    VALUE3,
    #[doc = "fADCD = fADC / 4"]
    VALUE4,
}
impl DIVDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVDW::VALUE1 => 0,
            DIVDW::VALUE2 => 1,
            DIVDW::VALUE3 => 2,
            DIVDW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVDW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "fADCD = fADC"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVDW::VALUE1)
    }
    #[doc = "fADCD = fADC / 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVDW::VALUE2)
    }
    #[doc = "fADCD = fADC / 3"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVDW::VALUE3)
    }
    #[doc = "fADCD = fADC / 4"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVDW::VALUE4)
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
#[doc = "Values that can be written to the field `DIVWC`"]
pub enum DIVWCW {
    #[doc = "No write access to divider parameters"]
    VALUE1,
    #[doc = "Bitfields DIVA, DCMSB, DIVD can be written"]
    VALUE2,
}
impl DIVWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVWCW::VALUE1 => false,
            DIVWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVWCW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to divider parameters"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVWCW::VALUE1)
    }
    #[doc = "Bitfields DIVA, DCMSB, DIVD can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVWCW::VALUE2)
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
#[doc = "Values that can be written to the field `DPCAL0`"]
pub enum DPCAL0W {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    VALUE1,
    #[doc = "No post-calibration"]
    VALUE2,
}
impl DPCAL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPCAL0W::VALUE1 => false,
            DPCAL0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPCAL0W<'a> {
    w: &'a mut W,
}
impl<'a> _DPCAL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPCAL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL0W::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL0W::VALUE2)
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
#[doc = "Values that can be written to the field `DPCAL1`"]
pub enum DPCAL1W {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    VALUE1,
    #[doc = "No post-calibration"]
    VALUE2,
}
impl DPCAL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPCAL1W::VALUE1 => false,
            DPCAL1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPCAL1W<'a> {
    w: &'a mut W,
}
impl<'a> _DPCAL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPCAL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL1W::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL1W::VALUE2)
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
#[doc = "Values that can be written to the field `DPCAL2`"]
pub enum DPCAL2W {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    VALUE1,
    #[doc = "No post-calibration"]
    VALUE2,
}
impl DPCAL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPCAL2W::VALUE1 => false,
            DPCAL2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPCAL2W<'a> {
    w: &'a mut W,
}
impl<'a> _DPCAL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPCAL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL2W::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL2W::VALUE2)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DPCAL3`"]
pub enum DPCAL3W {
    #[doc = "Automatic post-calibration after each conversion of group x"]
    VALUE1,
    #[doc = "No post-calibration"]
    VALUE2,
}
impl DPCAL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPCAL3W::VALUE1 => false,
            DPCAL3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPCAL3W<'a> {
    w: &'a mut W,
}
impl<'a> _DPCAL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPCAL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPCAL3W::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPCAL3W::VALUE2)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SUCAL`"]
pub enum SUCALW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    VALUE2,
}
impl SUCALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUCALW::VALUE1 => false,
            SUCALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUCALW<'a> {
    w: &'a mut W,
}
impl<'a> _SUCALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUCALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUCALW::VALUE1)
    }
    #[doc = "Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUCALW::VALUE2)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline]
    pub fn diva(&self) -> DIVAR {
        DIVAR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline]
    pub fn dcmsb(&self) -> DCMSBR {
        DCMSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline]
    pub fn divd(&self) -> DIVDR {
        DIVDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline]
    pub fn dpcal0(&self) -> DPCAL0R {
        DPCAL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline]
    pub fn dpcal1(&self) -> DPCAL1R {
        DPCAL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline]
    pub fn dpcal2(&self) -> DPCAL2R {
        DPCAL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline]
    pub fn dpcal3(&self) -> DPCAL3R {
        DPCAL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline]
    pub fn diva(&mut self) -> _DIVAW {
        _DIVAW { w: self }
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline]
    pub fn dcmsb(&mut self) -> _DCMSBW {
        _DCMSBW { w: self }
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline]
    pub fn divd(&mut self) -> _DIVDW {
        _DIVDW { w: self }
    }
    #[doc = "Bit 15 - Write Control for Divider Parameters"]
    #[inline]
    pub fn divwc(&mut self) -> _DIVWCW {
        _DIVWCW { w: self }
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline]
    pub fn dpcal0(&mut self) -> _DPCAL0W {
        _DPCAL0W { w: self }
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline]
    pub fn dpcal1(&mut self) -> _DPCAL1W {
        _DPCAL1W { w: self }
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline]
    pub fn dpcal2(&mut self) -> _DPCAL2W {
        _DPCAL2W { w: self }
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline]
    pub fn dpcal3(&mut self) -> _DPCAL3W {
        _DPCAL3W { w: self }
    }
    #[doc = "Bit 31 - Start-Up Calibration"]
    #[inline]
    pub fn sucal(&mut self) -> _SUCALW {
        _SUCALW { w: self }
    }
}
