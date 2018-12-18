#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DX3CR {
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
#[doc = "Possible values of the field `DSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSELR {
    #[doc = "The data input DXnA is selected."]
    VALUE1,
    #[doc = "The data input DXnB is selected."]
    VALUE2,
    #[doc = "The data input DXnC is selected."]
    VALUE3,
    #[doc = "The data input DXnD is selected."]
    VALUE4,
    #[doc = "The data input DXnE is selected."]
    VALUE5,
    #[doc = "The data input DXnF is selected."]
    VALUE6,
    #[doc = "The data input DXnG is selected."]
    VALUE7,
    #[doc = "The data input is always 1."]
    VALUE8,
}
impl DSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSELR::VALUE1 => 0,
            DSELR::VALUE2 => 1,
            DSELR::VALUE3 => 2,
            DSELR::VALUE4 => 3,
            DSELR::VALUE5 => 4,
            DSELR::VALUE6 => 5,
            DSELR::VALUE7 => 6,
            DSELR::VALUE8 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSELR {
        match value {
            0 => DSELR::VALUE1,
            1 => DSELR::VALUE2,
            2 => DSELR::VALUE3,
            3 => DSELR::VALUE4,
            4 => DSELR::VALUE5,
            5 => DSELR::VALUE6,
            6 => DSELR::VALUE7,
            7 => DSELR::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DSELR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == DSELR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == DSELR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == DSELR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == DSELR::VALUE8
    }
}
#[doc = "Possible values of the field `INSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSWR {
    #[doc = "The input of the data shift unit is controlled by the protocol pre-processor."]
    VALUE1,
    #[doc = "The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    VALUE2,
}
impl INSWR {
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
            INSWR::VALUE1 => false,
            INSWR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INSWR {
        match value {
            false => INSWR::VALUE1,
            true => INSWR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INSWR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INSWR::VALUE2
    }
}
#[doc = "Possible values of the field `DFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFENR {
    #[doc = "The input signal is not digitally filtered."]
    VALUE1,
    #[doc = "The input signal is digitally filtered."]
    VALUE2,
}
impl DFENR {
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
            DFENR::VALUE1 => false,
            DFENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFENR {
        match value {
            false => DFENR::VALUE1,
            true => DFENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DFENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DFENR::VALUE2
    }
}
#[doc = "Possible values of the field `DSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSENR {
    #[doc = "The un-synchronized signal can be taken as input for the data shift unit."]
    VALUE1,
    #[doc = "The synchronized signal can be taken as input for the data shift unit."]
    VALUE2,
}
impl DSENR {
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
            DSENR::VALUE1 => false,
            DSENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSENR {
        match value {
            false => DSENR::VALUE1,
            true => DSENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DSENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DSENR::VALUE2
    }
}
#[doc = "Possible values of the field `DPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPOLR {
    #[doc = "The input signal is not inverted."]
    VALUE1,
    #[doc = "The input signal is inverted."]
    VALUE2,
}
impl DPOLR {
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
            DPOLR::VALUE1 => false,
            DPOLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPOLR {
        match value {
            false => DPOLR::VALUE1,
            true => DPOLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DPOLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DPOLR::VALUE2
    }
}
#[doc = "Possible values of the field `SFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFSELR {
    #[doc = "The sampling frequency is fPB."]
    VALUE1,
    #[doc = "The sampling frequency is fFD."]
    VALUE2,
}
impl SFSELR {
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
            SFSELR::VALUE1 => false,
            SFSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SFSELR {
        match value {
            false => SFSELR::VALUE1,
            true => SFSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SFSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SFSELR::VALUE2
    }
}
#[doc = "Possible values of the field `CM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMR {
    #[doc = "The trigger activation is disabled."]
    VALUE1,
    #[doc = "A rising edge activates DXnT."]
    VALUE2,
    #[doc = "A falling edge activates DXnT."]
    VALUE3,
    #[doc = "Both edges activate DXnT."]
    VALUE4,
}
impl CMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMR::VALUE1 => 0,
            CMR::VALUE2 => 1,
            CMR::VALUE3 => 2,
            CMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMR {
        match value {
            0 => CMR::VALUE1,
            1 => CMR::VALUE2,
            2 => CMR::VALUE3,
            3 => CMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CMR::VALUE4
    }
}
#[doc = "Possible values of the field `DXS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DXSR {
    #[doc = "The current value of DXnS is 0."]
    VALUE1,
    #[doc = "The current value of DXnS is 1."]
    VALUE2,
}
impl DXSR {
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
            DXSR::VALUE1 => false,
            DXSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DXSR {
        match value {
            false => DXSR::VALUE1,
            true => DXSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DXSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DXSR::VALUE2
    }
}
#[doc = "Values that can be written to the field `DSEL`"]
pub enum DSELW {
    #[doc = "The data input DXnA is selected."]
    VALUE1,
    #[doc = "The data input DXnB is selected."]
    VALUE2,
    #[doc = "The data input DXnC is selected."]
    VALUE3,
    #[doc = "The data input DXnD is selected."]
    VALUE4,
    #[doc = "The data input DXnE is selected."]
    VALUE5,
    #[doc = "The data input DXnF is selected."]
    VALUE6,
    #[doc = "The data input DXnG is selected."]
    VALUE7,
    #[doc = "The data input is always 1."]
    VALUE8,
}
impl DSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSELW::VALUE1 => 0,
            DSELW::VALUE2 => 1,
            DSELW::VALUE3 => 2,
            DSELW::VALUE4 => 3,
            DSELW::VALUE5 => 4,
            DSELW::VALUE6 => 5,
            DSELW::VALUE7 => 6,
            DSELW::VALUE8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The data input DXnA is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSELW::VALUE1)
    }
    #[doc = "The data input DXnB is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSELW::VALUE2)
    }
    #[doc = "The data input DXnC is selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DSELW::VALUE3)
    }
    #[doc = "The data input DXnD is selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DSELW::VALUE4)
    }
    #[doc = "The data input DXnE is selected."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(DSELW::VALUE5)
    }
    #[doc = "The data input DXnF is selected."]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(DSELW::VALUE6)
    }
    #[doc = "The data input DXnG is selected."]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(DSELW::VALUE7)
    }
    #[doc = "The data input is always 1."]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(DSELW::VALUE8)
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
#[doc = "Values that can be written to the field `INSW`"]
pub enum INSWW {
    #[doc = "The input of the data shift unit is controlled by the protocol pre-processor."]
    VALUE1,
    #[doc = "The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    VALUE2,
}
impl INSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INSWW::VALUE1 => false,
            INSWW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INSWW<'a> {
    w: &'a mut W,
}
impl<'a> _INSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The input of the data shift unit is controlled by the protocol pre-processor."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INSWW::VALUE1)
    }
    #[doc = "The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INSWW::VALUE2)
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
#[doc = "Values that can be written to the field `DFEN`"]
pub enum DFENW {
    #[doc = "The input signal is not digitally filtered."]
    VALUE1,
    #[doc = "The input signal is digitally filtered."]
    VALUE2,
}
impl DFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFENW::VALUE1 => false,
            DFENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFENW<'a> {
    w: &'a mut W,
}
impl<'a> _DFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The input signal is not digitally filtered."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DFENW::VALUE1)
    }
    #[doc = "The input signal is digitally filtered."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DFENW::VALUE2)
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
#[doc = "Values that can be written to the field `DSEN`"]
pub enum DSENW {
    #[doc = "The un-synchronized signal can be taken as input for the data shift unit."]
    VALUE1,
    #[doc = "The synchronized signal can be taken as input for the data shift unit."]
    VALUE2,
}
impl DSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSENW::VALUE1 => false,
            DSENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSENW<'a> {
    w: &'a mut W,
}
impl<'a> _DSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The un-synchronized signal can be taken as input for the data shift unit."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSENW::VALUE1)
    }
    #[doc = "The synchronized signal can be taken as input for the data shift unit."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSENW::VALUE2)
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
#[doc = "Values that can be written to the field `DPOL`"]
pub enum DPOLW {
    #[doc = "The input signal is not inverted."]
    VALUE1,
    #[doc = "The input signal is inverted."]
    VALUE2,
}
impl DPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPOLW::VALUE1 => false,
            DPOLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _DPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The input signal is not inverted."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DPOLW::VALUE1)
    }
    #[doc = "The input signal is inverted."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DPOLW::VALUE2)
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
#[doc = "Values that can be written to the field `SFSEL`"]
pub enum SFSELW {
    #[doc = "The sampling frequency is fPB."]
    VALUE1,
    #[doc = "The sampling frequency is fFD."]
    VALUE2,
}
impl SFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SFSELW::VALUE1 => false,
            SFSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SFSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The sampling frequency is fPB."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SFSELW::VALUE1)
    }
    #[doc = "The sampling frequency is fFD."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SFSELW::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CM`"]
pub enum CMW {
    #[doc = "The trigger activation is disabled."]
    VALUE1,
    #[doc = "A rising edge activates DXnT."]
    VALUE2,
    #[doc = "A falling edge activates DXnT."]
    VALUE3,
    #[doc = "Both edges activate DXnT."]
    VALUE4,
}
impl CMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMW::VALUE1 => 0,
            CMW::VALUE2 => 1,
            CMW::VALUE3 => 2,
            CMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMW<'a> {
    w: &'a mut W,
}
impl<'a> _CMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The trigger activation is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMW::VALUE1)
    }
    #[doc = "A rising edge activates DXnT."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMW::VALUE2)
    }
    #[doc = "A falling edge activates DXnT."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMW::VALUE3)
    }
    #[doc = "Both edges activate DXnT."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMW::VALUE4)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Data Selection for Input Signal"]
    #[inline]
    pub fn dsel(&self) -> DSELR {
        DSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Input Switch"]
    #[inline]
    pub fn insw(&self) -> INSWR {
        INSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline]
    pub fn dfen(&self) -> DFENR {
        DFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Data Synchronization Enable"]
    #[inline]
    pub fn dsen(&self) -> DSENR {
        DSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Data Polarity for DXn"]
    #[inline]
    pub fn dpol(&self) -> DPOLR {
        DPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Sampling Frequency Selection"]
    #[inline]
    pub fn sfsel(&self) -> SFSELR {
        SFSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Combination Mode"]
    #[inline]
    pub fn cm(&self) -> CMR {
        CMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Synchronized Data Value"]
    #[inline]
    pub fn dxs(&self) -> DXSR {
        DXSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:2 - Data Selection for Input Signal"]
    #[inline]
    pub fn dsel(&mut self) -> _DSELW {
        _DSELW { w: self }
    }
    #[doc = "Bit 4 - Input Switch"]
    #[inline]
    pub fn insw(&mut self) -> _INSWW {
        _INSWW { w: self }
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline]
    pub fn dfen(&mut self) -> _DFENW {
        _DFENW { w: self }
    }
    #[doc = "Bit 6 - Data Synchronization Enable"]
    #[inline]
    pub fn dsen(&mut self) -> _DSENW {
        _DSENW { w: self }
    }
    #[doc = "Bit 8 - Data Polarity for DXn"]
    #[inline]
    pub fn dpol(&mut self) -> _DPOLW {
        _DPOLW { w: self }
    }
    #[doc = "Bit 9 - Sampling Frequency Selection"]
    #[inline]
    pub fn sfsel(&mut self) -> _SFSELW {
        _SFSELW { w: self }
    }
    #[doc = "Bits 10:11 - Combination Mode"]
    #[inline]
    pub fn cm(&mut self) -> _CMW {
        _CMW { w: self }
    }
}
