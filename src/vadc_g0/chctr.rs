#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHCTR {
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
#[doc = "Possible values of the field `ICLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICLSELR {
    #[doc = "Use group-specific class 0"]
    VALUE1,
    #[doc = "Use group-specific class 1"]
    VALUE2,
    #[doc = "Use global class 0"]
    VALUE3,
    #[doc = "Use global class 1"]
    VALUE4,
}
impl ICLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICLSELR::VALUE1 => 0,
            ICLSELR::VALUE2 => 1,
            ICLSELR::VALUE3 => 2,
            ICLSELR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICLSELR {
        match value {
            0 => ICLSELR::VALUE1,
            1 => ICLSELR::VALUE2,
            2 => ICLSELR::VALUE3,
            3 => ICLSELR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ICLSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ICLSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ICLSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ICLSELR::VALUE4
    }
}
#[doc = "Possible values of the field `BNDSELL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNDSELLR {
    #[doc = "Use group-specific boundary 0"]
    VALUE1,
    #[doc = "Use group-specific boundary 1"]
    VALUE2,
    #[doc = "Use global boundary 0"]
    VALUE3,
    #[doc = "Use global boundary 1"]
    VALUE4,
}
impl BNDSELLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BNDSELLR::VALUE1 => 0,
            BNDSELLR::VALUE2 => 1,
            BNDSELLR::VALUE3 => 2,
            BNDSELLR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BNDSELLR {
        match value {
            0 => BNDSELLR::VALUE1,
            1 => BNDSELLR::VALUE2,
            2 => BNDSELLR::VALUE3,
            3 => BNDSELLR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BNDSELLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BNDSELLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BNDSELLR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BNDSELLR::VALUE4
    }
}
#[doc = "Possible values of the field `BNDSELU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNDSELUR {
    #[doc = "Use group-specific boundary 0"]
    VALUE1,
    #[doc = "Use group-specific boundary 1"]
    VALUE2,
    #[doc = "Use global boundary 0"]
    VALUE3,
    #[doc = "Use global boundary 1"]
    VALUE4,
}
impl BNDSELUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BNDSELUR::VALUE1 => 0,
            BNDSELUR::VALUE2 => 1,
            BNDSELUR::VALUE3 => 2,
            BNDSELUR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BNDSELUR {
        match value {
            0 => BNDSELUR::VALUE1,
            1 => BNDSELUR::VALUE2,
            2 => BNDSELUR::VALUE3,
            3 => BNDSELUR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BNDSELUR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BNDSELUR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BNDSELUR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BNDSELUR::VALUE4
    }
}
#[doc = "Possible values of the field `CHEVMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEVMODER {
    #[doc = "Never"]
    VALUE1,
    #[doc = "NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    VALUE2,
    #[doc = "NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    VALUE3,
    #[doc = "NCM: Always (ignore band) FCM: If result switches to either level"]
    VALUE4,
}
impl CHEVMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHEVMODER::VALUE1 => 0,
            CHEVMODER::VALUE2 => 1,
            CHEVMODER::VALUE3 => 2,
            CHEVMODER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHEVMODER {
        match value {
            0 => CHEVMODER::VALUE1,
            1 => CHEVMODER::VALUE2,
            2 => CHEVMODER::VALUE3,
            3 => CHEVMODER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHEVMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHEVMODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CHEVMODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CHEVMODER::VALUE4
    }
}
#[doc = "Possible values of the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCR {
    #[doc = "No synchroniz. request, standalone operation"]
    VALUE1,
    #[doc = "Request a synchronized conversion of this channel (only taken into account for a master)"]
    VALUE2,
}
impl SYNCR {
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
            SYNCR::VALUE1 => false,
            SYNCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCR {
        match value {
            false => SYNCR::VALUE1,
            true => SYNCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYNCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYNCR::VALUE2
    }
}
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Standard reference input VAREF"]
    VALUE1,
    #[doc = "Alternate reference input from CH0"]
    VALUE2,
}
impl REFSELR {
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
            REFSELR::VALUE1 => false,
            REFSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFSELR {
        match value {
            false => REFSELR::VALUE1,
            true => REFSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REFSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REFSELR::VALUE2
    }
}
#[doc = "Possible values of the field `RESREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESREGR {
    #[doc = "Store result in group result register GxRES0"]
    VALUE1,
    #[doc = "Store result in group result register GxRES15"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RESREGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESREGR::VALUE1 => 0,
            RESREGR::VALUE2 => 15,
            RESREGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESREGR {
        match value {
            0 => RESREGR::VALUE1,
            15 => RESREGR::VALUE2,
            i => RESREGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RESREGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RESREGR::VALUE2
    }
}
#[doc = "Possible values of the field `RESTBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESTBSR {
    #[doc = "Store results in the selected group result register"]
    VALUE1,
    #[doc = "Store results in the global result register"]
    VALUE2,
}
impl RESTBSR {
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
            RESTBSR::VALUE1 => false,
            RESTBSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESTBSR {
        match value {
            false => RESTBSR::VALUE1,
            true => RESTBSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RESTBSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RESTBSR::VALUE2
    }
}
#[doc = "Possible values of the field `RESPOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESPOSR {
    #[doc = "Store results left-aligned"]
    VALUE1,
    #[doc = "Store results right-aligned"]
    VALUE2,
}
impl RESPOSR {
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
            RESPOSR::VALUE1 => false,
            RESPOSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESPOSR {
        match value {
            false => RESPOSR::VALUE1,
            true => RESPOSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RESPOSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RESPOSR::VALUE2
    }
}
#[doc = "Possible values of the field `BWDCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWDCHR {
    #[doc = "Select VAGND"]
    VALUE1,
    #[doc = "Select VAREF"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BWDCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BWDCHR::VALUE1 => 0,
            BWDCHR::VALUE2 => 1,
            BWDCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BWDCHR {
        match value {
            0 => BWDCHR::VALUE1,
            1 => BWDCHR::VALUE2,
            i => BWDCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BWDCHR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BWDCHR::VALUE2
    }
}
#[doc = "Possible values of the field `BWDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWDENR {
    #[doc = "Normal operation"]
    VALUE1,
    #[doc = "Additional preparation phase is enabled"]
    VALUE2,
}
impl BWDENR {
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
            BWDENR::VALUE1 => false,
            BWDENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWDENR {
        match value {
            false => BWDENR::VALUE1,
            true => BWDENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BWDENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BWDENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `ICLSEL`"]
pub enum ICLSELW {
    #[doc = "Use group-specific class 0"]
    VALUE1,
    #[doc = "Use group-specific class 1"]
    VALUE2,
    #[doc = "Use global class 0"]
    VALUE3,
    #[doc = "Use global class 1"]
    VALUE4,
}
impl ICLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICLSELW::VALUE1 => 0,
            ICLSELW::VALUE2 => 1,
            ICLSELW::VALUE3 => 2,
            ICLSELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ICLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICLSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Use group-specific class 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ICLSELW::VALUE1)
    }
    #[doc = "Use group-specific class 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ICLSELW::VALUE2)
    }
    #[doc = "Use global class 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ICLSELW::VALUE3)
    }
    #[doc = "Use global class 1"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ICLSELW::VALUE4)
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
#[doc = "Values that can be written to the field `BNDSELL`"]
pub enum BNDSELLW {
    #[doc = "Use group-specific boundary 0"]
    VALUE1,
    #[doc = "Use group-specific boundary 1"]
    VALUE2,
    #[doc = "Use global boundary 0"]
    VALUE3,
    #[doc = "Use global boundary 1"]
    VALUE4,
}
impl BNDSELLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BNDSELLW::VALUE1 => 0,
            BNDSELLW::VALUE2 => 1,
            BNDSELLW::VALUE3 => 2,
            BNDSELLW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BNDSELLW<'a> {
    w: &'a mut W,
}
impl<'a> _BNDSELLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BNDSELLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Use group-specific boundary 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BNDSELLW::VALUE1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BNDSELLW::VALUE2)
    }
    #[doc = "Use global boundary 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BNDSELLW::VALUE3)
    }
    #[doc = "Use global boundary 1"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BNDSELLW::VALUE4)
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
#[doc = "Values that can be written to the field `BNDSELU`"]
pub enum BNDSELUW {
    #[doc = "Use group-specific boundary 0"]
    VALUE1,
    #[doc = "Use group-specific boundary 1"]
    VALUE2,
    #[doc = "Use global boundary 0"]
    VALUE3,
    #[doc = "Use global boundary 1"]
    VALUE4,
}
impl BNDSELUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BNDSELUW::VALUE1 => 0,
            BNDSELUW::VALUE2 => 1,
            BNDSELUW::VALUE3 => 2,
            BNDSELUW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BNDSELUW<'a> {
    w: &'a mut W,
}
impl<'a> _BNDSELUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BNDSELUW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Use group-specific boundary 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BNDSELUW::VALUE1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BNDSELUW::VALUE2)
    }
    #[doc = "Use global boundary 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BNDSELUW::VALUE3)
    }
    #[doc = "Use global boundary 1"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BNDSELUW::VALUE4)
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
#[doc = "Values that can be written to the field `CHEVMODE`"]
pub enum CHEVMODEW {
    #[doc = "Never"]
    VALUE1,
    #[doc = "NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    VALUE2,
    #[doc = "NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    VALUE3,
    #[doc = "NCM: Always (ignore band) FCM: If result switches to either level"]
    VALUE4,
}
impl CHEVMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHEVMODEW::VALUE1 => 0,
            CHEVMODEW::VALUE2 => 1,
            CHEVMODEW::VALUE3 => 2,
            CHEVMODEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHEVMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHEVMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHEVMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Never"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHEVMODEW::VALUE1)
    }
    #[doc = "NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHEVMODEW::VALUE2)
    }
    #[doc = "NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CHEVMODEW::VALUE3)
    }
    #[doc = "NCM: Always (ignore band) FCM: If result switches to either level"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CHEVMODEW::VALUE4)
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
#[doc = "Values that can be written to the field `SYNC`"]
pub enum SYNCW {
    #[doc = "No synchroniz. request, standalone operation"]
    VALUE1,
    #[doc = "Request a synchronized conversion of this channel (only taken into account for a master)"]
    VALUE2,
}
impl SYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCW::VALUE1 => false,
            SYNCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No synchroniz. request, standalone operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNCW::VALUE1)
    }
    #[doc = "Request a synchronized conversion of this channel (only taken into account for a master)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNCW::VALUE2)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Standard reference input VAREF"]
    VALUE1,
    #[doc = "Alternate reference input from CH0"]
    VALUE2,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFSELW::VALUE1 => false,
            REFSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard reference input VAREF"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFSELW::VALUE1)
    }
    #[doc = "Alternate reference input from CH0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REFSELW::VALUE2)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESREG`"]
pub enum RESREGW {
    #[doc = "Store result in group result register GxRES0"]
    VALUE1,
    #[doc = "Store result in group result register GxRES15"]
    VALUE2,
}
impl RESREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESREGW::VALUE1 => 0,
            RESREGW::VALUE2 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESREGW<'a> {
    w: &'a mut W,
}
impl<'a> _RESREGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESREGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Store result in group result register GxRES0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESREGW::VALUE1)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESREGW::VALUE2)
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
#[doc = "Values that can be written to the field `RESTBS`"]
pub enum RESTBSW {
    #[doc = "Store results in the selected group result register"]
    VALUE1,
    #[doc = "Store results in the global result register"]
    VALUE2,
}
impl RESTBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESTBSW::VALUE1 => false,
            RESTBSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESTBSW<'a> {
    w: &'a mut W,
}
impl<'a> _RESTBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESTBSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Store results in the selected group result register"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESTBSW::VALUE1)
    }
    #[doc = "Store results in the global result register"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESTBSW::VALUE2)
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
#[doc = "Values that can be written to the field `RESPOS`"]
pub enum RESPOSW {
    #[doc = "Store results left-aligned"]
    VALUE1,
    #[doc = "Store results right-aligned"]
    VALUE2,
}
impl RESPOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESPOSW::VALUE1 => false,
            RESPOSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESPOSW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESPOSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Store results left-aligned"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESPOSW::VALUE1)
    }
    #[doc = "Store results right-aligned"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESPOSW::VALUE2)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWDCH`"]
pub enum BWDCHW {
    #[doc = "Select VAGND"]
    VALUE1,
    #[doc = "Select VAREF"]
    VALUE2,
}
impl BWDCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BWDCHW::VALUE1 => 0,
            BWDCHW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWDCHW<'a> {
    w: &'a mut W,
}
impl<'a> _BWDCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWDCHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select VAGND"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWDCHW::VALUE1)
    }
    #[doc = "Select VAREF"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWDCHW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWDEN`"]
pub enum BWDENW {
    #[doc = "Normal operation"]
    VALUE1,
    #[doc = "Additional preparation phase is enabled"]
    VALUE2,
}
impl BWDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWDENW::VALUE1 => false,
            BWDENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWDENW<'a> {
    w: &'a mut W,
}
impl<'a> _BWDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWDENW::VALUE1)
    }
    #[doc = "Additional preparation phase is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWDENW::VALUE2)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline]
    pub fn iclsel(&self) -> ICLSELR {
        ICLSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline]
    pub fn bndsell(&self) -> BNDSELLR {
        BNDSELLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline]
    pub fn bndselu(&self) -> BNDSELUR {
        BNDSELUR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline]
    pub fn chevmode(&self) -> CHEVMODER {
        CHEVMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        SYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline]
    pub fn resreg(&self) -> RESREGR {
        RESREGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline]
    pub fn restbs(&self) -> RESTBSR {
        RESTBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline]
    pub fn respos(&self) -> RESPOSR {
        RESPOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline]
    pub fn bwdch(&self) -> BWDCHR {
        BWDCHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline]
    pub fn bwden(&self) -> BWDENR {
        BWDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline]
    pub fn iclsel(&mut self) -> _ICLSELW {
        _ICLSELW { w: self }
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline]
    pub fn bndsell(&mut self) -> _BNDSELLW {
        _BNDSELLW { w: self }
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline]
    pub fn bndselu(&mut self) -> _BNDSELUW {
        _BNDSELUW { w: self }
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline]
    pub fn chevmode(&mut self) -> _CHEVMODEW {
        _CHEVMODEW { w: self }
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline]
    pub fn resreg(&mut self) -> _RESREGW {
        _RESREGW { w: self }
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline]
    pub fn restbs(&mut self) -> _RESTBSW {
        _RESTBSW { w: self }
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline]
    pub fn respos(&mut self) -> _RESPOSW {
        _RESPOSW { w: self }
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline]
    pub fn bwdch(&mut self) -> _BWDCHW {
        _BWDCHW { w: self }
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline]
    pub fn bwden(&mut self) -> _BWDENW {
        _BWDENW { w: self }
    }
}
