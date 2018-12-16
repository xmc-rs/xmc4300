#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPCHK {
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
#[doc = r" Value of the field"]
pub struct PASER {
    bits: bool,
}
impl PASER {
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
#[doc = "Possible values of the field `PACS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACSR {
    #[doc = "CC80"]
    VALUE1,
    #[doc = "CC81"]
    VALUE2,
    #[doc = "CC82"]
    VALUE3,
    #[doc = "CC83"]
    VALUE4,
}
impl PACSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PACSR::VALUE1 => 0,
            PACSR::VALUE2 => 1,
            PACSR::VALUE3 => 2,
            PACSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PACSR {
        match value {
            0 => PACSR::VALUE1,
            1 => PACSR::VALUE2,
            2 => PACSR::VALUE3,
            3 => PACSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PACSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PACSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PACSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PACSR::VALUE4
    }
}
#[doc = "Possible values of the field `PISEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PISELR {
    #[doc = "CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    VALUE1,
    #[doc = "CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    VALUE2,
    #[doc = "CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    VALUE3,
    #[doc = "CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    VALUE4,
}
impl PISELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PISELR::VALUE1 => 0,
            PISELR::VALUE2 => 1,
            PISELR::VALUE3 => 2,
            PISELR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PISELR {
        match value {
            0 => PISELR::VALUE1,
            1 => PISELR::VALUE2,
            2 => PISELR::VALUE3,
            3 => PISELR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PISELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PISELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PISELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PISELR::VALUE4
    }
}
#[doc = "Possible values of the field `PCDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCDSR {
    #[doc = "CCU8x.IGBTA"]
    VALUE1,
    #[doc = "CCU8x.IGBTB"]
    VALUE2,
    #[doc = "CCU8x.IGBTC"]
    VALUE3,
    #[doc = "CCU8x.IGBTD"]
    VALUE4,
}
impl PCDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCDSR::VALUE1 => 0,
            PCDSR::VALUE2 => 1,
            PCDSR::VALUE3 => 2,
            PCDSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCDSR {
        match value {
            0 => PCDSR::VALUE1,
            1 => PCDSR::VALUE2,
            2 => PCDSR::VALUE3,
            3 => PCDSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PCDSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PCDSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PCDSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PCDSR::VALUE4
    }
}
#[doc = "Possible values of the field `PCTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCTSR {
    #[doc = "Even parity enabled"]
    VALUE1,
    #[doc = "Odd parity enabled"]
    VALUE2,
}
impl PCTSR {
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
            PCTSR::VALUE1 => false,
            PCTSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCTSR {
        match value {
            false => PCTSR::VALUE1,
            true => PCTSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PCTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PCTSR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PCSTR {
    bits: bool,
}
impl PCSTR {
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
#[doc = r" Value of the field"]
pub struct PCSEL0R {
    bits: u8,
}
impl PCSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PCSEL1R {
    bits: u8,
}
impl PCSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PCSEL2R {
    bits: u8,
}
impl PCSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PCSEL3R {
    bits: u8,
}
impl PCSEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PASEW<'a> {
    w: &'a mut W,
}
impl<'a> _PASEW<'a> {
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
#[doc = "Values that can be written to the field `PACS`"]
pub enum PACSW {
    #[doc = "CC80"]
    VALUE1,
    #[doc = "CC81"]
    VALUE2,
    #[doc = "CC82"]
    VALUE3,
    #[doc = "CC83"]
    VALUE4,
}
impl PACSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PACSW::VALUE1 => 0,
            PACSW::VALUE2 => 1,
            PACSW::VALUE3 => 2,
            PACSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PACSW<'a> {
    w: &'a mut W,
}
impl<'a> _PACSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PACSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC80"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PACSW::VALUE1)
    }
    #[doc = "CC81"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PACSW::VALUE2)
    }
    #[doc = "CC82"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PACSW::VALUE3)
    }
    #[doc = "CC83"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PACSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PISEL`"]
pub enum PISELW {
    #[doc = "CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    VALUE1,
    #[doc = "CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    VALUE2,
    #[doc = "CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    VALUE3,
    #[doc = "CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    VALUE4,
}
impl PISELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PISELW::VALUE1 => 0,
            PISELW::VALUE2 => 1,
            PISELW::VALUE3 => 2,
            PISELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PISELW<'a> {
    w: &'a mut W,
}
impl<'a> _PISELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PISELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PISELW::VALUE1)
    }
    #[doc = "CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PISELW::VALUE2)
    }
    #[doc = "CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PISELW::VALUE3)
    }
    #[doc = "CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PISELW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCDS`"]
pub enum PCDSW {
    #[doc = "CCU8x.IGBTA"]
    VALUE1,
    #[doc = "CCU8x.IGBTB"]
    VALUE2,
    #[doc = "CCU8x.IGBTC"]
    VALUE3,
    #[doc = "CCU8x.IGBTD"]
    VALUE4,
}
impl PCDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCDSW::VALUE1 => 0,
            PCDSW::VALUE2 => 1,
            PCDSW::VALUE3 => 2,
            PCDSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCDSW<'a> {
    w: &'a mut W,
}
impl<'a> _PCDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCDSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCU8x.IGBTA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCDSW::VALUE1)
    }
    #[doc = "CCU8x.IGBTB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCDSW::VALUE2)
    }
    #[doc = "CCU8x.IGBTC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PCDSW::VALUE3)
    }
    #[doc = "CCU8x.IGBTD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PCDSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCTS`"]
pub enum PCTSW {
    #[doc = "Even parity enabled"]
    VALUE1,
    #[doc = "Odd parity enabled"]
    VALUE2,
}
impl PCTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCTSW::VALUE1 => false,
            PCTSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCTSW<'a> {
    w: &'a mut W,
}
impl<'a> _PCTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even parity enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCTSW::VALUE1)
    }
    #[doc = "Odd parity enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCTSW::VALUE2)
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
pub struct _PCSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCSEL0W<'a> {
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
#[doc = r" Proxy"]
pub struct _PCSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCSEL1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCSEL2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _PCSEL3W<'a> {
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
    #[doc = "Bit 0 - Parity Checker Automatic start/stop"]
    #[inline]
    pub fn pase(&self) -> PASER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PASER { bits }
    }
    #[doc = "Bits 1:2 - Parity Checker Automatic start/stop selector"]
    #[inline]
    pub fn pacs(&self) -> PACSR {
        PACSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Driver Input signal selector"]
    #[inline]
    pub fn pisel(&self) -> PISELR {
        PISELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Parity Checker Delay Input Selector"]
    #[inline]
    pub fn pcds(&self) -> PCDSR {
        PCDSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Parity Checker type selector"]
    #[inline]
    pub fn pcts(&self) -> PCTSR {
        PCTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Parity Checker XOR status"]
    #[inline]
    pub fn pcst(&self) -> PCSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCSTR { bits }
    }
    #[doc = "Bits 16:19 - Parity Checker Slice 0 output selection"]
    #[inline]
    pub fn pcsel0(&self) -> PCSEL0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCSEL0R { bits }
    }
    #[doc = "Bits 20:23 - Parity Checker Slice 1 output selection"]
    #[inline]
    pub fn pcsel1(&self) -> PCSEL1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCSEL1R { bits }
    }
    #[doc = "Bits 24:27 - Parity Checker Slice 2 output selection"]
    #[inline]
    pub fn pcsel2(&self) -> PCSEL2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCSEL2R { bits }
    }
    #[doc = "Bits 28:31 - Parity Checker Slice 3 output selection"]
    #[inline]
    pub fn pcsel3(&self) -> PCSEL3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCSEL3R { bits }
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
    #[doc = "Bit 0 - Parity Checker Automatic start/stop"]
    #[inline]
    pub fn pase(&mut self) -> _PASEW {
        _PASEW { w: self }
    }
    #[doc = "Bits 1:2 - Parity Checker Automatic start/stop selector"]
    #[inline]
    pub fn pacs(&mut self) -> _PACSW {
        _PACSW { w: self }
    }
    #[doc = "Bits 3:4 - Driver Input signal selector"]
    #[inline]
    pub fn pisel(&mut self) -> _PISELW {
        _PISELW { w: self }
    }
    #[doc = "Bits 5:6 - Parity Checker Delay Input Selector"]
    #[inline]
    pub fn pcds(&mut self) -> _PCDSW {
        _PCDSW { w: self }
    }
    #[doc = "Bit 7 - Parity Checker type selector"]
    #[inline]
    pub fn pcts(&mut self) -> _PCTSW {
        _PCTSW { w: self }
    }
    #[doc = "Bits 16:19 - Parity Checker Slice 0 output selection"]
    #[inline]
    pub fn pcsel0(&mut self) -> _PCSEL0W {
        _PCSEL0W { w: self }
    }
    #[doc = "Bits 20:23 - Parity Checker Slice 1 output selection"]
    #[inline]
    pub fn pcsel1(&mut self) -> _PCSEL1W {
        _PCSEL1W { w: self }
    }
    #[doc = "Bits 24:27 - Parity Checker Slice 2 output selection"]
    #[inline]
    pub fn pcsel2(&mut self) -> _PCSEL2W {
        _PCSEL2W { w: self }
    }
    #[doc = "Bits 28:31 - Parity Checker Slice 3 output selection"]
    #[inline]
    pub fn pcsel3(&mut self) -> _PCSEL3W {
        _PCSEL3W { w: self }
    }
}
