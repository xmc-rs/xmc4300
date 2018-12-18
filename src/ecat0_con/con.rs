#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CON {
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
#[doc = "Possible values of the field `ECATRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECATRSTENR {
    #[doc = "Reset request by EtherCAT Master disabled"]
    VALUE1,
    #[doc = "Reset request by EtherCAT Master enabled"]
    VALUE2,
}
impl ECATRSTENR {
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
            ECATRSTENR::VALUE1 => false,
            ECATRSTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECATRSTENR {
        match value {
            false => ECATRSTENR::VALUE1,
            true => ECATRSTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECATRSTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECATRSTENR::VALUE2
    }
}
#[doc = "Possible values of the field `LATCHIN0SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATCHIN0SELR {
    #[doc = "Data input LATCHIN0A is selected"]
    VALUE1,
    #[doc = "Data input LATCHIN0B is selected"]
    VALUE2,
    #[doc = "Data input LATCHIN0C is selected"]
    VALUE3,
    #[doc = "Data input LATCHIN0D is selected"]
    VALUE4,
}
impl LATCHIN0SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LATCHIN0SELR::VALUE1 => 0,
            LATCHIN0SELR::VALUE2 => 1,
            LATCHIN0SELR::VALUE3 => 2,
            LATCHIN0SELR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LATCHIN0SELR {
        match value {
            0 => LATCHIN0SELR::VALUE1,
            1 => LATCHIN0SELR::VALUE2,
            2 => LATCHIN0SELR::VALUE3,
            3 => LATCHIN0SELR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LATCHIN0SELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LATCHIN0SELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LATCHIN0SELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LATCHIN0SELR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct LATCHIN0R {
    bits: bool,
}
impl LATCHIN0R {
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
#[doc = "Possible values of the field `LATCHIN1SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATCHIN1SELR {
    #[doc = "Data input LATCHIN1A is selected"]
    VALUE1,
    #[doc = "Data input LATCHIN1B is selected"]
    VALUE2,
    #[doc = "Data input LATCHIN1C is selected"]
    VALUE3,
    #[doc = "Data input LATCHIN1D is selected"]
    VALUE4,
}
impl LATCHIN1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LATCHIN1SELR::VALUE1 => 0,
            LATCHIN1SELR::VALUE2 => 1,
            LATCHIN1SELR::VALUE3 => 2,
            LATCHIN1SELR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LATCHIN1SELR {
        match value {
            0 => LATCHIN1SELR::VALUE1,
            1 => LATCHIN1SELR::VALUE2,
            2 => LATCHIN1SELR::VALUE3,
            3 => LATCHIN1SELR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LATCHIN1SELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LATCHIN1SELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LATCHIN1SELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LATCHIN1SELR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct LATCHIN1R {
    bits: bool,
}
impl LATCHIN1R {
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
pub struct PHYOFFSETR {
    bits: u8,
}
impl PHYOFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIOR {
    #[doc = "Data input MDIA is selected"]
    VALUE1,
    #[doc = "Data input MDIB is selected"]
    VALUE2,
    #[doc = "Data input MDIC is selected"]
    VALUE3,
    #[doc = "Data input MDID is selected"]
    VALUE4,
}
impl MDIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MDIOR::VALUE1 => 0,
            MDIOR::VALUE2 => 1,
            MDIOR::VALUE3 => 2,
            MDIOR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MDIOR {
        match value {
            0 => MDIOR::VALUE1,
            1 => MDIOR::VALUE2,
            2 => MDIOR::VALUE3,
            3 => MDIOR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MDIOR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MDIOR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MDIOR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MDIOR::VALUE4
    }
}
#[doc = "Values that can be written to the field `ECATRSTEN`"]
pub enum ECATRSTENW {
    #[doc = "Reset request by EtherCAT Master disabled"]
    VALUE1,
    #[doc = "Reset request by EtherCAT Master enabled"]
    VALUE2,
}
impl ECATRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECATRSTENW::VALUE1 => false,
            ECATRSTENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECATRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ECATRSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECATRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset request by EtherCAT Master disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECATRSTENW::VALUE1)
    }
    #[doc = "Reset request by EtherCAT Master enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECATRSTENW::VALUE2)
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
#[doc = "Values that can be written to the field `LATCHIN0SEL`"]
pub enum LATCHIN0SELW {
    #[doc = "Data input LATCHIN0A is selected"]
    VALUE1,
    #[doc = "Data input LATCHIN0B is selected"]
    VALUE2,
    #[doc = "Data input LATCHIN0C is selected"]
    VALUE3,
    #[doc = "Data input LATCHIN0D is selected"]
    VALUE4,
}
impl LATCHIN0SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LATCHIN0SELW::VALUE1 => 0,
            LATCHIN0SELW::VALUE2 => 1,
            LATCHIN0SELW::VALUE3 => 2,
            LATCHIN0SELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LATCHIN0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LATCHIN0SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LATCHIN0SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input LATCHIN0A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LATCHIN0SELW::VALUE1)
    }
    #[doc = "Data input LATCHIN0B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LATCHIN0SELW::VALUE2)
    }
    #[doc = "Data input LATCHIN0C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LATCHIN0SELW::VALUE3)
    }
    #[doc = "Data input LATCHIN0D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(LATCHIN0SELW::VALUE4)
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
#[doc = "Values that can be written to the field `LATCHIN1SEL`"]
pub enum LATCHIN1SELW {
    #[doc = "Data input LATCHIN1A is selected"]
    VALUE1,
    #[doc = "Data input LATCHIN1B is selected"]
    VALUE2,
    #[doc = "Data input LATCHIN1C is selected"]
    VALUE3,
    #[doc = "Data input LATCHIN1D is selected"]
    VALUE4,
}
impl LATCHIN1SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LATCHIN1SELW::VALUE1 => 0,
            LATCHIN1SELW::VALUE2 => 1,
            LATCHIN1SELW::VALUE3 => 2,
            LATCHIN1SELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LATCHIN1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LATCHIN1SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LATCHIN1SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input LATCHIN1A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LATCHIN1SELW::VALUE1)
    }
    #[doc = "Data input LATCHIN1B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LATCHIN1SELW::VALUE2)
    }
    #[doc = "Data input LATCHIN1C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LATCHIN1SELW::VALUE3)
    }
    #[doc = "Data input LATCHIN1D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(LATCHIN1SELW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHYOFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PHYOFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDIO`"]
pub enum MDIOW {
    #[doc = "Data input MDIA is selected"]
    VALUE1,
    #[doc = "Data input MDIB is selected"]
    VALUE2,
    #[doc = "Data input MDIC is selected"]
    VALUE3,
    #[doc = "Data input MDID is selected"]
    VALUE4,
}
impl MDIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MDIOW::VALUE1 => 0,
            MDIOW::VALUE2 => 1,
            MDIOW::VALUE3 => 2,
            MDIOW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _MDIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDIOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input MDIA is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MDIOW::VALUE1)
    }
    #[doc = "Data input MDIB is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MDIOW::VALUE2)
    }
    #[doc = "Data input MDIC is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MDIOW::VALUE3)
    }
    #[doc = "Data input MDID is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MDIOW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
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
    #[doc = "Bit 0 - Enable EtherCAT Reset Request"]
    #[inline]
    pub fn ecatrsten(&self) -> ECATRSTENR {
        ECATRSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - LATCHIN0 Input Select"]
    #[inline]
    pub fn latchin0sel(&self) -> LATCHIN0SELR {
        LATCHIN0SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - EtherCAT LATCH_IN0 Input Signal"]
    #[inline]
    pub fn latchin0(&self) -> LATCHIN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LATCHIN0R { bits }
    }
    #[doc = "Bits 12:13 - LATCHIN1 Input Select"]
    #[inline]
    pub fn latchin1sel(&self) -> LATCHIN1SELR {
        LATCHIN1SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - EtherCAT LATCH_IN1 Input Signal"]
    #[inline]
    pub fn latchin1(&self) -> LATCHIN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LATCHIN1R { bits }
    }
    #[doc = "Bits 16:20 - Ethernet PHY Address Offset"]
    #[inline]
    pub fn phyoffset(&self) -> PHYOFFSETR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHYOFFSETR { bits }
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline]
    pub fn mdio(&self) -> MDIOR {
        MDIOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
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
    #[doc = "Bit 0 - Enable EtherCAT Reset Request"]
    #[inline]
    pub fn ecatrsten(&mut self) -> _ECATRSTENW {
        _ECATRSTENW { w: self }
    }
    #[doc = "Bits 8:9 - LATCHIN0 Input Select"]
    #[inline]
    pub fn latchin0sel(&mut self) -> _LATCHIN0SELW {
        _LATCHIN0SELW { w: self }
    }
    #[doc = "Bits 12:13 - LATCHIN1 Input Select"]
    #[inline]
    pub fn latchin1sel(&mut self) -> _LATCHIN1SELW {
        _LATCHIN1SELW { w: self }
    }
    #[doc = "Bits 16:20 - Ethernet PHY Address Offset"]
    #[inline]
    pub fn phyoffset(&mut self) -> _PHYOFFSETW {
        _PHYOFFSETW { w: self }
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline]
    pub fn mdio(&mut self) -> _MDIOW {
        _MDIOW { w: self }
    }
}
