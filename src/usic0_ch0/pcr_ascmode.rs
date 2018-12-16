#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCR_ASCMODE {
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
#[doc = "Possible values of the field `SMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMDR {
    #[doc = "Only one sample is taken per bit time. The current input value is sampled."]
    VALUE1,
    #[doc = "Three samples are taken per bit time and a majority decision is made."]
    VALUE2,
}
impl SMDR {
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
            SMDR::VALUE1 => false,
            SMDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMDR {
        match value {
            false => SMDR::VALUE1,
            true => SMDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMDR::VALUE2
    }
}
#[doc = "Possible values of the field `STPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPBR {
    #[doc = "The number of stop bits is 1."]
    VALUE1,
    #[doc = "The number of stop bits is 2."]
    VALUE2,
}
impl STPBR {
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
            STPBR::VALUE1 => false,
            STPBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPBR {
        match value {
            false => STPBR::VALUE1,
            true => STPBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STPBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STPBR::VALUE2
    }
}
#[doc = "Possible values of the field `IDM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDMR {
    #[doc = "The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    VALUE1,
    #[doc = "The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    VALUE2,
}
impl IDMR {
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
            IDMR::VALUE1 => false,
            IDMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDMR {
        match value {
            false => IDMR::VALUE1,
            true => IDMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IDMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IDMR::VALUE2
    }
}
#[doc = "Possible values of the field `SBIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBIENR {
    #[doc = "The interrupt generation is disabled."]
    VALUE1,
    #[doc = "The interrupt generation is enabled."]
    VALUE2,
}
impl SBIENR {
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
            SBIENR::VALUE1 => false,
            SBIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBIENR {
        match value {
            false => SBIENR::VALUE1,
            true => SBIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SBIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SBIENR::VALUE2
    }
}
#[doc = "Possible values of the field `CDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDENR {
    #[doc = "The collision detection is disabled."]
    VALUE1,
    #[doc = "If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    VALUE2,
}
impl CDENR {
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
            CDENR::VALUE1 => false,
            CDENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDENR {
        match value {
            false => CDENR::VALUE1,
            true => CDENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CDENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CDENR::VALUE2
    }
}
#[doc = "Possible values of the field `RNIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNIENR {
    #[doc = "The interrupt generation is disabled."]
    VALUE1,
    #[doc = "The interrupt generation is enabled."]
    VALUE2,
}
impl RNIENR {
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
            RNIENR::VALUE1 => false,
            RNIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RNIENR {
        match value {
            false => RNIENR::VALUE1,
            true => RNIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RNIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RNIENR::VALUE2
    }
}
#[doc = "Possible values of the field `FEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIENR {
    #[doc = "The interrupt generation is disabled."]
    VALUE1,
    #[doc = "The interrupt generation is enabled."]
    VALUE2,
}
impl FEIENR {
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
            FEIENR::VALUE1 => false,
            FEIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEIENR {
        match value {
            false => FEIENR::VALUE1,
            true => FEIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FEIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FEIENR::VALUE2
    }
}
#[doc = "Possible values of the field `FFIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFIENR {
    #[doc = "The interrupt generation is disabled."]
    VALUE1,
    #[doc = "The interrupt generation is enabled."]
    VALUE2,
}
impl FFIENR {
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
            FFIENR::VALUE1 => false,
            FFIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FFIENR {
        match value {
            false => FFIENR::VALUE1,
            true => FFIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FFIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FFIENR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct SPR {
    bits: u8,
}
impl SPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLR {
    #[doc = "The pulse length is equal to the bit length (no shortened 0)."]
    VALUE1,
    #[doc = "The pulse length of a 0 bit is 2 time quanta."]
    VALUE2,
    #[doc = "The pulse length of a 0 bit is 3 time quanta."]
    VALUE3,
    #[doc = "The pulse length of a 0 bit is 8 time quanta."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLR::VALUE1 => 0,
            PLR::VALUE2 => 1,
            PLR::VALUE3 => 2,
            PLR::VALUE4 => 7,
            PLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLR {
        match value {
            0 => PLR::VALUE1,
            1 => PLR::VALUE2,
            2 => PLR::VALUE3,
            7 => PLR::VALUE4,
            i => PLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PLR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PLR::VALUE4
    }
}
#[doc = "Possible values of the field `RSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTENR {
    #[doc = "Flag PSR\\[9\\] is not modified depending on the receiver status."]
    VALUE1,
    #[doc = "Flag PSR\\[9\\] is set during the complete reception of a frame."]
    VALUE2,
}
impl RSTENR {
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
            RSTENR::VALUE1 => false,
            RSTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTENR {
        match value {
            false => RSTENR::VALUE1,
            true => RSTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSTENR::VALUE2
    }
}
#[doc = "Possible values of the field `TSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTENR {
    #[doc = "Flag PSR\\[9\\] is not modified depending on the transmitter status."]
    VALUE1,
    #[doc = "Flag PSR\\[9\\] is set during the complete transmission of a frame."]
    VALUE2,
}
impl TSTENR {
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
            TSTENR::VALUE1 => false,
            TSTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSTENR {
        match value {
            false => TSTENR::VALUE1,
            true => TSTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSTENR::VALUE2
    }
}
#[doc = "Possible values of the field `MCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKR {
    #[doc = "The MCLK generation is disabled and the MCLK signal is 0."]
    VALUE1,
    #[doc = "The MCLK generation is enabled."]
    VALUE2,
}
impl MCLKR {
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
            MCLKR::VALUE1 => false,
            MCLKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCLKR {
        match value {
            false => MCLKR::VALUE1,
            true => MCLKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCLKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCLKR::VALUE2
    }
}
#[doc = "Values that can be written to the field `SMD`"]
pub enum SMDW {
    #[doc = "Only one sample is taken per bit time. The current input value is sampled."]
    VALUE1,
    #[doc = "Three samples are taken per bit time and a majority decision is made."]
    VALUE2,
}
impl SMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMDW::VALUE1 => false,
            SMDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMDW<'a> {
    w: &'a mut W,
}
impl<'a> _SMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only one sample is taken per bit time. The current input value is sampled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMDW::VALUE1)
    }
    #[doc = "Three samples are taken per bit time and a majority decision is made."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMDW::VALUE2)
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
#[doc = "Values that can be written to the field `STPB`"]
pub enum STPBW {
    #[doc = "The number of stop bits is 1."]
    VALUE1,
    #[doc = "The number of stop bits is 2."]
    VALUE2,
}
impl STPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPBW::VALUE1 => false,
            STPBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPBW<'a> {
    w: &'a mut W,
}
impl<'a> _STPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The number of stop bits is 1."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STPBW::VALUE1)
    }
    #[doc = "The number of stop bits is 2."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STPBW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDM`"]
pub enum IDMW {
    #[doc = "The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    VALUE1,
    #[doc = "The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    VALUE2,
}
impl IDMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDMW::VALUE1 => false,
            IDMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDMW<'a> {
    w: &'a mut W,
}
impl<'a> _IDMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The bus idle detection is switched off and bits PSR.TXIDLE and PSR.RXIDLE are set automatically to enable data transfers without checking the inputs before."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDMW::VALUE1)
    }
    #[doc = "The bus is considered as idle after a number of consecutive passive bit times defined by SCTR.FLE plus 2 (in the case without parity bit) or plus 3 (in the case with parity bit)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDMW::VALUE2)
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
#[doc = "Values that can be written to the field `SBIEN`"]
pub enum SBIENW {
    #[doc = "The interrupt generation is disabled."]
    VALUE1,
    #[doc = "The interrupt generation is enabled."]
    VALUE2,
}
impl SBIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBIENW::VALUE1 => false,
            SBIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SBIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SBIENW::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SBIENW::VALUE2)
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
#[doc = "Values that can be written to the field `CDEN`"]
pub enum CDENW {
    #[doc = "The collision detection is disabled."]
    VALUE1,
    #[doc = "If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    VALUE2,
}
impl CDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDENW::VALUE1 => false,
            CDENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDENW<'a> {
    w: &'a mut W,
}
impl<'a> _CDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The collision detection is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDENW::VALUE1)
    }
    #[doc = "If a collision is detected, the transmitter stops its data transmission, outputs a 1, sets bit PSR.COL and generates a protocol interrupt. In order to allow data transmission again, PSR.COL has to be cleared by software."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDENW::VALUE2)
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
#[doc = "Values that can be written to the field `RNIEN`"]
pub enum RNIENW {
    #[doc = "The interrupt generation is disabled."]
    VALUE1,
    #[doc = "The interrupt generation is enabled."]
    VALUE2,
}
impl RNIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RNIENW::VALUE1 => false,
            RNIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RNIENW<'a> {
    w: &'a mut W,
}
impl<'a> _RNIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RNIENW::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RNIENW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FEIEN`"]
pub enum FEIENW {
    #[doc = "The interrupt generation is disabled."]
    VALUE1,
    #[doc = "The interrupt generation is enabled."]
    VALUE2,
}
impl FEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEIENW::VALUE1 => false,
            FEIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _FEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FEIENW::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FEIENW::VALUE2)
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
#[doc = "Values that can be written to the field `FFIEN`"]
pub enum FFIENW {
    #[doc = "The interrupt generation is disabled."]
    VALUE1,
    #[doc = "The interrupt generation is enabled."]
    VALUE2,
}
impl FFIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FFIENW::VALUE1 => false,
            FFIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FFIENW<'a> {
    w: &'a mut W,
}
impl<'a> _FFIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FFIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FFIENW::VALUE1)
    }
    #[doc = "The interrupt generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FFIENW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _SPW<'a> {
    w: &'a mut W,
}
impl<'a> _SPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PL`"]
pub enum PLW {
    #[doc = "The pulse length is equal to the bit length (no shortened 0)."]
    VALUE1,
    #[doc = "The pulse length of a 0 bit is 2 time quanta."]
    VALUE2,
    #[doc = "The pulse length of a 0 bit is 3 time quanta."]
    VALUE3,
    #[doc = "The pulse length of a 0 bit is 8 time quanta."]
    VALUE4,
}
impl PLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLW::VALUE1 => 0,
            PLW::VALUE2 => 1,
            PLW::VALUE3 => 2,
            PLW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLW<'a> {
    w: &'a mut W,
}
impl<'a> _PLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The pulse length is equal to the bit length (no shortened 0)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLW::VALUE1)
    }
    #[doc = "The pulse length of a 0 bit is 2 time quanta."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLW::VALUE2)
    }
    #[doc = "The pulse length of a 0 bit is 3 time quanta."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLW::VALUE3)
    }
    #[doc = "The pulse length of a 0 bit is 8 time quanta."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PLW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTEN`"]
pub enum RSTENW {
    #[doc = "Flag PSR\\[9\\] is not modified depending on the receiver status."]
    VALUE1,
    #[doc = "Flag PSR\\[9\\] is set during the complete reception of a frame."]
    VALUE2,
}
impl RSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTENW::VALUE1 => false,
            RSTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flag PSR\\[9\\] is not modified depending on the receiver status."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSTENW::VALUE1)
    }
    #[doc = "Flag PSR\\[9\\] is set during the complete reception of a frame."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSTENW::VALUE2)
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
#[doc = "Values that can be written to the field `TSTEN`"]
pub enum TSTENW {
    #[doc = "Flag PSR\\[9\\] is not modified depending on the transmitter status."]
    VALUE1,
    #[doc = "Flag PSR\\[9\\] is set during the complete transmission of a frame."]
    VALUE2,
}
impl TSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSTENW::VALUE1 => false,
            TSTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flag PSR\\[9\\] is not modified depending on the transmitter status."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSTENW::VALUE1)
    }
    #[doc = "Flag PSR\\[9\\] is set during the complete transmission of a frame."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSTENW::VALUE2)
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
#[doc = "Values that can be written to the field `MCLK`"]
pub enum MCLKW {
    #[doc = "The MCLK generation is disabled and the MCLK signal is 0."]
    VALUE1,
    #[doc = "The MCLK generation is enabled."]
    VALUE2,
}
impl MCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCLKW::VALUE1 => false,
            MCLKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _MCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The MCLK generation is disabled and the MCLK signal is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLKW::VALUE1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLKW::VALUE2)
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
    #[doc = "Bit 0 - Sample Mode"]
    #[inline]
    pub fn smd(&self) -> SMDR {
        SMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Stop Bits"]
    #[inline]
    pub fn stpb(&self) -> STPBR {
        STPBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Idle Detection Mode"]
    #[inline]
    pub fn idm(&self) -> IDMR {
        IDMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Synchronization Break Interrupt Enable"]
    #[inline]
    pub fn sbien(&self) -> SBIENR {
        SBIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Collision Detection Enable"]
    #[inline]
    pub fn cden(&self) -> CDENR {
        CDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Receiver Noise Detection Interrupt Enable"]
    #[inline]
    pub fn rnien(&self) -> RNIENR {
        RNIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Format Error Interrupt Enable"]
    #[inline]
    pub fn feien(&self) -> FEIENR {
        FEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Frame Finished Interrupt Enable"]
    #[inline]
    pub fn ffien(&self) -> FFIENR {
        FFIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - Sample Point"]
    #[inline]
    pub fn sp(&self) -> SPR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPR { bits }
    }
    #[doc = "Bits 13:15 - Pulse Length"]
    #[inline]
    pub fn pl(&self) -> PLR {
        PLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Receiver Status Enable"]
    #[inline]
    pub fn rsten(&self) -> RSTENR {
        RSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Transmitter Status Enable"]
    #[inline]
    pub fn tsten(&self) -> TSTENR {
        TSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline]
    pub fn mclk(&self) -> MCLKR {
        MCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Sample Mode"]
    #[inline]
    pub fn smd(&mut self) -> _SMDW {
        _SMDW { w: self }
    }
    #[doc = "Bit 1 - Stop Bits"]
    #[inline]
    pub fn stpb(&mut self) -> _STPBW {
        _STPBW { w: self }
    }
    #[doc = "Bit 2 - Idle Detection Mode"]
    #[inline]
    pub fn idm(&mut self) -> _IDMW {
        _IDMW { w: self }
    }
    #[doc = "Bit 3 - Synchronization Break Interrupt Enable"]
    #[inline]
    pub fn sbien(&mut self) -> _SBIENW {
        _SBIENW { w: self }
    }
    #[doc = "Bit 4 - Collision Detection Enable"]
    #[inline]
    pub fn cden(&mut self) -> _CDENW {
        _CDENW { w: self }
    }
    #[doc = "Bit 5 - Receiver Noise Detection Interrupt Enable"]
    #[inline]
    pub fn rnien(&mut self) -> _RNIENW {
        _RNIENW { w: self }
    }
    #[doc = "Bit 6 - Format Error Interrupt Enable"]
    #[inline]
    pub fn feien(&mut self) -> _FEIENW {
        _FEIENW { w: self }
    }
    #[doc = "Bit 7 - Frame Finished Interrupt Enable"]
    #[inline]
    pub fn ffien(&mut self) -> _FFIENW {
        _FFIENW { w: self }
    }
    #[doc = "Bits 8:12 - Sample Point"]
    #[inline]
    pub fn sp(&mut self) -> _SPW {
        _SPW { w: self }
    }
    #[doc = "Bits 13:15 - Pulse Length"]
    #[inline]
    pub fn pl(&mut self) -> _PLW {
        _PLW { w: self }
    }
    #[doc = "Bit 16 - Receiver Status Enable"]
    #[inline]
    pub fn rsten(&mut self) -> _RSTENW {
        _RSTENW { w: self }
    }
    #[doc = "Bit 17 - Transmitter Status Enable"]
    #[inline]
    pub fn tsten(&mut self) -> _TSTENW {
        _TSTENW { w: self }
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline]
    pub fn mclk(&mut self) -> _MCLKW {
        _MCLKW { w: self }
    }
}
