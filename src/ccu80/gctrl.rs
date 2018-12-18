#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GCTRL {
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
#[doc = "Possible values of the field `PRBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRBCR {
    #[doc = "SW only"]
    VALUE1,
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC80 is cleared."]
    VALUE2,
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC81 is cleared."]
    VALUE3,
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC82 is cleared."]
    VALUE4,
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC83 is cleared."]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRBCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRBCR::VALUE1 => 0,
            PRBCR::VALUE2 => 1,
            PRBCR::VALUE3 => 2,
            PRBCR::VALUE4 => 3,
            PRBCR::VALUE5 => 4,
            PRBCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRBCR {
        match value {
            0 => PRBCR::VALUE1,
            1 => PRBCR::VALUE2,
            2 => PRBCR::VALUE3,
            3 => PRBCR::VALUE4,
            4 => PRBCR::VALUE5,
            i => PRBCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRBCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRBCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PRBCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PRBCR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PRBCR::VALUE5
    }
}
#[doc = "Possible values of the field `PCIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCISR {
    #[doc = "Module clock"]
    VALUE1,
    #[doc = "CCU8x.ECLKA"]
    VALUE2,
    #[doc = "CCU8x.ECLKB"]
    VALUE3,
    #[doc = "CCU8x.ECLKC"]
    VALUE4,
}
impl PCISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCISR::VALUE1 => 0,
            PCISR::VALUE2 => 1,
            PCISR::VALUE3 => 2,
            PCISR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCISR {
        match value {
            0 => PCISR::VALUE1,
            1 => PCISR::VALUE2,
            2 => PCISR::VALUE3,
            3 => PCISR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PCISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PCISR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PCISR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PCISR::VALUE4
    }
}
#[doc = "Possible values of the field `SUSCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSCFGR {
    #[doc = "Suspend request ignored. The module never enters in suspend"]
    VALUE1,
    #[doc = "Stops all the running slices immediately. Safe stop is not applied."]
    VALUE2,
    #[doc = "Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    VALUE3,
    #[doc = "Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    VALUE4,
}
impl SUSCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUSCFGR::VALUE1 => 0,
            SUSCFGR::VALUE2 => 1,
            SUSCFGR::VALUE3 => 2,
            SUSCFGR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUSCFGR {
        match value {
            0 => SUSCFGR::VALUE1,
            1 => SUSCFGR::VALUE2,
            2 => SUSCFGR::VALUE3,
            3 => SUSCFGR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SUSCFGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SUSCFGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SUSCFGR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SUSCFGR::VALUE4
    }
}
#[doc = "Possible values of the field `MSE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE0R {
    #[doc = "Shadow transfer can only be requested by SW"]
    VALUE1,
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    VALUE2,
}
impl MSE0R {
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
            MSE0R::VALUE1 => false,
            MSE0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSE0R {
        match value {
            false => MSE0R::VALUE1,
            true => MSE0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSE0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSE0R::VALUE2
    }
}
#[doc = "Possible values of the field `MSE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE1R {
    #[doc = "Shadow transfer can only be requested by SW"]
    VALUE1,
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    VALUE2,
}
impl MSE1R {
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
            MSE1R::VALUE1 => false,
            MSE1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSE1R {
        match value {
            false => MSE1R::VALUE1,
            true => MSE1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSE1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSE1R::VALUE2
    }
}
#[doc = "Possible values of the field `MSE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE2R {
    #[doc = "Shadow transfer can only be requested by SW"]
    VALUE1,
    #[doc = "Shadow transfer can be requested via SW and via the CCU8xMCSS input."]
    VALUE2,
}
impl MSE2R {
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
            MSE2R::VALUE1 => false,
            MSE2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSE2R {
        match value {
            false => MSE2R::VALUE1,
            true => MSE2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSE2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSE2R::VALUE2
    }
}
#[doc = "Possible values of the field `MSE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE3R {
    #[doc = "Shadow transfer can only be requested by SW"]
    VALUE1,
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    VALUE2,
}
impl MSE3R {
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
            MSE3R::VALUE1 => false,
            MSE3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSE3R {
        match value {
            false => MSE3R::VALUE1,
            true => MSE3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSE3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSE3R::VALUE2
    }
}
#[doc = "Possible values of the field `MSDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSDER {
    #[doc = "Only the shadow transfer for period and compare values is requested"]
    VALUE1,
    #[doc = "Shadow transfer for the compare, period and prescaler compare values is requested"]
    VALUE2,
    #[doc = "Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MSDER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSDER::VALUE1 => 0,
            MSDER::VALUE2 => 1,
            MSDER::VALUE4 => 3,
            MSDER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSDER {
        match value {
            0 => MSDER::VALUE1,
            1 => MSDER::VALUE2,
            3 => MSDER::VALUE4,
            i => MSDER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSDER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSDER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MSDER::VALUE4
    }
}
#[doc = "Values that can be written to the field `PRBC`"]
pub enum PRBCW {
    #[doc = "SW only"]
    VALUE1,
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC80 is cleared."]
    VALUE2,
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC81 is cleared."]
    VALUE3,
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC82 is cleared."]
    VALUE4,
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC83 is cleared."]
    VALUE5,
}
impl PRBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRBCW::VALUE1 => 0,
            PRBCW::VALUE2 => 1,
            PRBCW::VALUE3 => 2,
            PRBCW::VALUE4 => 3,
            PRBCW::VALUE5 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRBCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRBCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRBCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SW only"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRBCW::VALUE1)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC80 is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRBCW::VALUE2)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC81 is cleared."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PRBCW::VALUE3)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC82 is cleared."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PRBCW::VALUE4)
    }
    #[doc = "GSTATThe register contains the status of the prescaler and each timer slice (idle mode or running)..PRB and prescaler registers are cleared when the Run Bit of CC83 is cleared."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PRBCW::VALUE5)
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
#[doc = "Values that can be written to the field `PCIS`"]
pub enum PCISW {
    #[doc = "Module clock"]
    VALUE1,
    #[doc = "CCU8x.ECLKA"]
    VALUE2,
    #[doc = "CCU8x.ECLKB"]
    VALUE3,
    #[doc = "CCU8x.ECLKC"]
    VALUE4,
}
impl PCISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCISW::VALUE1 => 0,
            PCISW::VALUE2 => 1,
            PCISW::VALUE3 => 2,
            PCISW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCISW<'a> {
    w: &'a mut W,
}
impl<'a> _PCISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Module clock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCISW::VALUE1)
    }
    #[doc = "CCU8x.ECLKA"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCISW::VALUE2)
    }
    #[doc = "CCU8x.ECLKB"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PCISW::VALUE3)
    }
    #[doc = "CCU8x.ECLKC"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PCISW::VALUE4)
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
#[doc = "Values that can be written to the field `SUSCFG`"]
pub enum SUSCFGW {
    #[doc = "Suspend request ignored. The module never enters in suspend"]
    VALUE1,
    #[doc = "Stops all the running slices immediately. Safe stop is not applied."]
    VALUE2,
    #[doc = "Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    VALUE3,
    #[doc = "Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    VALUE4,
}
impl SUSCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SUSCFGW::VALUE1 => 0,
            SUSCFGW::VALUE2 => 1,
            SUSCFGW::VALUE3 => 2,
            SUSCFGW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Suspend request ignored. The module never enters in suspend"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE1)
    }
    #[doc = "Stops all the running slices immediately. Safe stop is not applied."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE2)
    }
    #[doc = "Stops the block immediately and clamps all the outputs to PASSIVE state. Safe stop is applied."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE3)
    }
    #[doc = "Waits for the roll over of each slice to stop and clamp the slices outputs. Safe stop is applied."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE4)
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
#[doc = "Values that can be written to the field `MSE0`"]
pub enum MSE0W {
    #[doc = "Shadow transfer can only be requested by SW"]
    VALUE1,
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    VALUE2,
}
impl MSE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSE0W::VALUE1 => false,
            MSE0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSE0W<'a> {
    w: &'a mut W,
}
impl<'a> _MSE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE0W::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE0W::VALUE2)
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
#[doc = "Values that can be written to the field `MSE1`"]
pub enum MSE1W {
    #[doc = "Shadow transfer can only be requested by SW"]
    VALUE1,
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    VALUE2,
}
impl MSE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSE1W::VALUE1 => false,
            MSE1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSE1W<'a> {
    w: &'a mut W,
}
impl<'a> _MSE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE1W::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE1W::VALUE2)
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
#[doc = "Values that can be written to the field `MSE2`"]
pub enum MSE2W {
    #[doc = "Shadow transfer can only be requested by SW"]
    VALUE1,
    #[doc = "Shadow transfer can be requested via SW and via the CCU8xMCSS input."]
    VALUE2,
}
impl MSE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSE2W::VALUE1 => false,
            MSE2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSE2W<'a> {
    w: &'a mut W,
}
impl<'a> _MSE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE2W::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU8xMCSS input."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE2W::VALUE2)
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
#[doc = "Values that can be written to the field `MSE3`"]
pub enum MSE3W {
    #[doc = "Shadow transfer can only be requested by SW"]
    VALUE1,
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    VALUE2,
}
impl MSE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSE3W::VALUE1 => false,
            MSE3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSE3W<'a> {
    w: &'a mut W,
}
impl<'a> _MSE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shadow transfer can only be requested by SW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSE3W::VALUE1)
    }
    #[doc = "Shadow transfer can be requested via SW and via the CCU8x.MCSS input."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSE3W::VALUE2)
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
#[doc = "Values that can be written to the field `MSDE`"]
pub enum MSDEW {
    #[doc = "Only the shadow transfer for period and compare values is requested"]
    VALUE1,
    #[doc = "Shadow transfer for the compare, period and prescaler compare values is requested"]
    VALUE2,
    #[doc = "Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    VALUE4,
}
impl MSDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSDEW::VALUE1 => 0,
            MSDEW::VALUE2 => 1,
            MSDEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSDEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSDEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Only the shadow transfer for period and compare values is requested"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSDEW::VALUE1)
    }
    #[doc = "Shadow transfer for the compare, period and prescaler compare values is requested"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSDEW::VALUE2)
    }
    #[doc = "Shadow transfer for the compare, period, prescaler and dither compare values is requested"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSDEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline]
    pub fn prbc(&self) -> PRBCR {
        PRBCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline]
    pub fn pcis(&self) -> PCISR {
        PCISR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline]
    pub fn suscfg(&self) -> SUSCFGR {
        SUSCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline]
    pub fn mse0(&self) -> MSE0R {
        MSE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline]
    pub fn mse1(&self) -> MSE1R {
        MSE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline]
    pub fn mse2(&self) -> MSE2R {
        MSE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline]
    pub fn mse3(&self) -> MSE3R {
        MSE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline]
    pub fn msde(&self) -> MSDER {
        MSDER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:2 - Prescaler Clear Configuration"]
    #[inline]
    pub fn prbc(&mut self) -> _PRBCW {
        _PRBCW { w: self }
    }
    #[doc = "Bits 4:5 - Prescaler Input Clock Selection"]
    #[inline]
    pub fn pcis(&mut self) -> _PCISW {
        _PCISW { w: self }
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline]
    pub fn suscfg(&mut self) -> _SUSCFGW {
        _SUSCFGW { w: self }
    }
    #[doc = "Bit 10 - Slice 0 Multi Channel shadow transfer enable"]
    #[inline]
    pub fn mse0(&mut self) -> _MSE0W {
        _MSE0W { w: self }
    }
    #[doc = "Bit 11 - Slice 1 Multi Channel shadow transfer enable"]
    #[inline]
    pub fn mse1(&mut self) -> _MSE1W {
        _MSE1W { w: self }
    }
    #[doc = "Bit 12 - Slice 2 Multi Channel shadow transfer enable"]
    #[inline]
    pub fn mse2(&mut self) -> _MSE2W {
        _MSE2W { w: self }
    }
    #[doc = "Bit 13 - Slice 3 Multi Channel shadow transfer enable"]
    #[inline]
    pub fn mse3(&mut self) -> _MSE3W {
        _MSE3W { w: self }
    }
    #[doc = "Bits 14:15 - Multi Channel shadow transfer request configuration"]
    #[inline]
    pub fn msde(&mut self) -> _MSDEW {
        _MSDEW { w: self }
    }
}
