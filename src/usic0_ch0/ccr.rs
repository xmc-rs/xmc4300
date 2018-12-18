#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    VALUE1,
    #[doc = "The SSC (SPI) protocol is selected."]
    VALUE2,
    #[doc = "The ASC (SCI, UART) protocol is selected."]
    VALUE3,
    #[doc = "The IIS protocol is selected."]
    VALUE4,
    #[doc = "The IIC protocol is selected."]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::VALUE1 => 0,
            MODER::VALUE2 => 1,
            MODER::VALUE3 => 2,
            MODER::VALUE4 => 3,
            MODER::VALUE5 => 4,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::VALUE1,
            1 => MODER::VALUE2,
            2 => MODER::VALUE3,
            3 => MODER::VALUE4,
            4 => MODER::VALUE5,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MODER::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == MODER::VALUE5
    }
}
#[doc = "Possible values of the field `HPCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPCENR {
    #[doc = "The hardware port control is disabled."]
    VALUE1,
    #[doc = "The hardware port control is enabled for DX0 and DOUT0."]
    VALUE2,
    #[doc = "The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    VALUE3,
    #[doc = "The hardware port control is enabled for DX0, DX\\[5:3\\] and DOUT\\[3:0\\]."]
    VALUE4,
}
impl HPCENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPCENR::VALUE1 => 0,
            HPCENR::VALUE2 => 1,
            HPCENR::VALUE3 => 2,
            HPCENR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPCENR {
        match value {
            0 => HPCENR::VALUE1,
            1 => HPCENR::VALUE2,
            2 => HPCENR::VALUE3,
            3 => HPCENR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HPCENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HPCENR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HPCENR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == HPCENR::VALUE4
    }
}
#[doc = "Possible values of the field `PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMR {
    #[doc = "The parity generation is disabled."]
    VALUE1,
    #[doc = "Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    VALUE3,
    #[doc = "Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMR::VALUE1 => 0,
            PMR::VALUE3 => 2,
            PMR::VALUE4 => 3,
            PMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMR {
        match value {
            0 => PMR::VALUE1,
            2 => PMR::VALUE3,
            3 => PMR::VALUE4,
            i => PMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PMR::VALUE4
    }
}
#[doc = "Possible values of the field `RSIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIENR {
    #[doc = "The receiver start interrupt is disabled."]
    VALUE1,
    #[doc = "The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    VALUE2,
}
impl RSIENR {
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
            RSIENR::VALUE1 => false,
            RSIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSIENR {
        match value {
            false => RSIENR::VALUE1,
            true => RSIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSIENR::VALUE2
    }
}
#[doc = "Possible values of the field `DLIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLIENR {
    #[doc = "The data lost interrupt is disabled."]
    VALUE1,
    #[doc = "The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    VALUE2,
}
impl DLIENR {
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
            DLIENR::VALUE1 => false,
            DLIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLIENR {
        match value {
            false => DLIENR::VALUE1,
            true => DLIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DLIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DLIENR::VALUE2
    }
}
#[doc = "Possible values of the field `TSIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIENR {
    #[doc = "The transmit shift interrupt is disabled."]
    VALUE1,
    #[doc = "The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    VALUE2,
}
impl TSIENR {
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
            TSIENR::VALUE1 => false,
            TSIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIENR {
        match value {
            false => TSIENR::VALUE1,
            true => TSIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSIENR::VALUE2
    }
}
#[doc = "Possible values of the field `TBIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBIENR {
    #[doc = "The transmit buffer interrupt is disabled."]
    VALUE1,
    #[doc = "The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    VALUE2,
}
impl TBIENR {
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
            TBIENR::VALUE1 => false,
            TBIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBIENR {
        match value {
            false => TBIENR::VALUE1,
            true => TBIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TBIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TBIENR::VALUE2
    }
}
#[doc = "Possible values of the field `RIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIENR {
    #[doc = "The receive interrupt is disabled."]
    VALUE1,
    #[doc = "The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    VALUE2,
}
impl RIENR {
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
            RIENR::VALUE1 => false,
            RIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIENR {
        match value {
            false => RIENR::VALUE1,
            true => RIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RIENR::VALUE2
    }
}
#[doc = "Possible values of the field `AIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIENR {
    #[doc = "The alternative receive interrupt is disabled."]
    VALUE1,
    #[doc = "The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    VALUE2,
}
impl AIENR {
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
            AIENR::VALUE1 => false,
            AIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIENR {
        match value {
            false => AIENR::VALUE1,
            true => AIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AIENR::VALUE2
    }
}
#[doc = "Possible values of the field `BRGIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRGIENR {
    #[doc = "The baud rate generator interrupt is disabled."]
    VALUE1,
    #[doc = "The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    VALUE2,
}
impl BRGIENR {
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
            BRGIENR::VALUE1 => false,
            BRGIENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRGIENR {
        match value {
            false => BRGIENR::VALUE1,
            true => BRGIENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BRGIENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BRGIENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    VALUE1,
    #[doc = "The SSC (SPI) protocol is selected."]
    VALUE2,
    #[doc = "The ASC (SCI, UART) protocol is selected."]
    VALUE3,
    #[doc = "The IIS protocol is selected."]
    VALUE4,
    #[doc = "The IIC protocol is selected."]
    VALUE5,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::VALUE1 => 0,
            MODEW::VALUE2 => 1,
            MODEW::VALUE3 => 2,
            MODEW::VALUE4 => 3,
            MODEW::VALUE5 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The USIC channel is disabled. All protocol-related state machines are set to an idle state."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODEW::VALUE1)
    }
    #[doc = "The SSC (SPI) protocol is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODEW::VALUE2)
    }
    #[doc = "The ASC (SCI, UART) protocol is selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MODEW::VALUE3)
    }
    #[doc = "The IIS protocol is selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MODEW::VALUE4)
    }
    #[doc = "The IIC protocol is selected."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(MODEW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPCEN`"]
pub enum HPCENW {
    #[doc = "The hardware port control is disabled."]
    VALUE1,
    #[doc = "The hardware port control is enabled for DX0 and DOUT0."]
    VALUE2,
    #[doc = "The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    VALUE3,
    #[doc = "The hardware port control is enabled for DX0, DX\\[5:3\\] and DOUT\\[3:0\\]."]
    VALUE4,
}
impl HPCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPCENW::VALUE1 => 0,
            HPCENW::VALUE2 => 1,
            HPCENW::VALUE3 => 2,
            HPCENW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPCENW<'a> {
    w: &'a mut W,
}
impl<'a> _HPCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPCENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The hardware port control is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HPCENW::VALUE1)
    }
    #[doc = "The hardware port control is enabled for DX0 and DOUT0."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HPCENW::VALUE2)
    }
    #[doc = "The hardware port control is enabled for DX3, DX0 and DOUT\\[1:0\\]."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HPCENW::VALUE3)
    }
    #[doc = "The hardware port control is enabled for DX0, DX\\[5:3\\] and DOUT\\[3:0\\]."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(HPCENW::VALUE4)
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
#[doc = "Values that can be written to the field `PM`"]
pub enum PMW {
    #[doc = "The parity generation is disabled."]
    VALUE1,
    #[doc = "Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    VALUE3,
    #[doc = "Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    VALUE4,
}
impl PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMW::VALUE1 => 0,
            PMW::VALUE3 => 2,
            PMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The parity generation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PMW::VALUE1)
    }
    #[doc = "Even parity is selected (parity bit = 1 on odd number of 1s in data, parity bit = 0 on even number of 1s in data)."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PMW::VALUE3)
    }
    #[doc = "Odd parity is selected (parity bit = 0 on odd number of 1s in data, parity bit = 1 on even number of 1s in data)."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSIEN`"]
pub enum RSIENW {
    #[doc = "The receiver start interrupt is disabled."]
    VALUE1,
    #[doc = "The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    VALUE2,
}
impl RSIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSIENW::VALUE1 => false,
            RSIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSIENW<'a> {
    w: &'a mut W,
}
impl<'a> _RSIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receiver start interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSIENW::VALUE1)
    }
    #[doc = "The receiver start interrupt is enabled. In case of a receiver start event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSIENW::VALUE2)
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
#[doc = "Values that can be written to the field `DLIEN`"]
pub enum DLIENW {
    #[doc = "The data lost interrupt is disabled."]
    VALUE1,
    #[doc = "The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    VALUE2,
}
impl DLIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLIENW::VALUE1 => false,
            DLIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DLIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data lost interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLIENW::VALUE1)
    }
    #[doc = "The data lost interrupt is enabled. In case of a data lost event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLIENW::VALUE2)
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
#[doc = "Values that can be written to the field `TSIEN`"]
pub enum TSIENW {
    #[doc = "The transmit shift interrupt is disabled."]
    VALUE1,
    #[doc = "The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    VALUE2,
}
impl TSIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIENW::VALUE1 => false,
            TSIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmit shift interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSIENW::VALUE1)
    }
    #[doc = "The transmit shift interrupt is enabled. In case of a transmit shift interrupt event, the service request output SRx indicated by INPR.TSINP is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSIENW::VALUE2)
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
#[doc = "Values that can be written to the field `TBIEN`"]
pub enum TBIENW {
    #[doc = "The transmit buffer interrupt is disabled."]
    VALUE1,
    #[doc = "The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    VALUE2,
}
impl TBIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBIENW::VALUE1 => false,
            TBIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TBIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmit buffer interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBIENW::VALUE1)
    }
    #[doc = "The transmit buffer interrupt is enabled. In case of a transmit buffer event, the service request output SRx indicated by INPR.TBINP is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBIENW::VALUE2)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RIEN`"]
pub enum RIENW {
    #[doc = "The receive interrupt is disabled."]
    VALUE1,
    #[doc = "The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    VALUE2,
}
impl RIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIENW::VALUE1 => false,
            RIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIENW<'a> {
    w: &'a mut W,
}
impl<'a> _RIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receive interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RIENW::VALUE1)
    }
    #[doc = "The receive interrupt is enabled. In case of a receive event, the service request output SRx indicated by INPR.RINP is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RIENW::VALUE2)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AIEN`"]
pub enum AIENW {
    #[doc = "The alternative receive interrupt is disabled."]
    VALUE1,
    #[doc = "The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    VALUE2,
}
impl AIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIENW::VALUE1 => false,
            AIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIENW<'a> {
    w: &'a mut W,
}
impl<'a> _AIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The alternative receive interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIENW::VALUE1)
    }
    #[doc = "The alternative receive interrupt is enabled. In case of an alternative receive event, the service request output SRx indicated by INPR.AINP is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIENW::VALUE2)
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
#[doc = "Values that can be written to the field `BRGIEN`"]
pub enum BRGIENW {
    #[doc = "The baud rate generator interrupt is disabled."]
    VALUE1,
    #[doc = "The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    VALUE2,
}
impl BRGIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRGIENW::VALUE1 => false,
            BRGIENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRGIENW<'a> {
    w: &'a mut W,
}
impl<'a> _BRGIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRGIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The baud rate generator interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BRGIENW::VALUE1)
    }
    #[doc = "The baud rate generator interrupt is enabled. In case of a baud rate generator event, the service request output SRx indicated by INPR.PINP is activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BRGIENW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Operating Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Hardware Port Control Enable"]
    #[inline]
    pub fn hpcen(&self) -> HPCENR {
        HPCENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Parity Mode"]
    #[inline]
    pub fn pm(&self) -> PMR {
        PMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Receiver Start Interrupt Enable"]
    #[inline]
    pub fn rsien(&self) -> RSIENR {
        RSIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Data Lost Interrupt Enable"]
    #[inline]
    pub fn dlien(&self) -> DLIENR {
        DLIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Transmit Shift Interrupt Enable"]
    #[inline]
    pub fn tsien(&self) -> TSIENR {
        TSIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Transmit Buffer Interrupt Enable"]
    #[inline]
    pub fn tbien(&self) -> TBIENR {
        TBIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Receive Interrupt Enable"]
    #[inline]
    pub fn rien(&self) -> RIENR {
        RIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Alternative Receive Interrupt Enable"]
    #[inline]
    pub fn aien(&self) -> AIENR {
        AIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Baud Rate Generator Interrupt Enable"]
    #[inline]
    pub fn brgien(&self) -> BRGIENR {
        BRGIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3 - Operating Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 6:7 - Hardware Port Control Enable"]
    #[inline]
    pub fn hpcen(&mut self) -> _HPCENW {
        _HPCENW { w: self }
    }
    #[doc = "Bits 8:9 - Parity Mode"]
    #[inline]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 10 - Receiver Start Interrupt Enable"]
    #[inline]
    pub fn rsien(&mut self) -> _RSIENW {
        _RSIENW { w: self }
    }
    #[doc = "Bit 11 - Data Lost Interrupt Enable"]
    #[inline]
    pub fn dlien(&mut self) -> _DLIENW {
        _DLIENW { w: self }
    }
    #[doc = "Bit 12 - Transmit Shift Interrupt Enable"]
    #[inline]
    pub fn tsien(&mut self) -> _TSIENW {
        _TSIENW { w: self }
    }
    #[doc = "Bit 13 - Transmit Buffer Interrupt Enable"]
    #[inline]
    pub fn tbien(&mut self) -> _TBIENW {
        _TBIENW { w: self }
    }
    #[doc = "Bit 14 - Receive Interrupt Enable"]
    #[inline]
    pub fn rien(&mut self) -> _RIENW {
        _RIENW { w: self }
    }
    #[doc = "Bit 15 - Alternative Receive Interrupt Enable"]
    #[inline]
    pub fn aien(&mut self) -> _AIENW {
        _AIENW { w: self }
    }
    #[doc = "Bit 16 - Baud Rate Generator Interrupt Enable"]
    #[inline]
    pub fn brgien(&mut self) -> _BRGIENW {
        _BRGIENW { w: self }
    }
}
