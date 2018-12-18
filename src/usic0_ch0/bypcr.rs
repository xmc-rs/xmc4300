#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BYPCR {
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
pub struct BWLER {
    bits: u8,
}
impl BWLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BDSSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDSSMR {
    #[doc = "The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    VALUE1,
    #[doc = "The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    VALUE2,
}
impl BDSSMR {
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
            BDSSMR::VALUE1 => false,
            BDSSMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BDSSMR {
        match value {
            false => BDSSMR::VALUE1,
            true => BDSSMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BDSSMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BDSSMR::VALUE2
    }
}
#[doc = "Possible values of the field `BDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDENR {
    #[doc = "The transfer of bypass data is disabled."]
    VALUE1,
    #[doc = "The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    VALUE2,
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    VALUE3,
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    VALUE4,
}
impl BDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BDENR::VALUE1 => 0,
            BDENR::VALUE2 => 1,
            BDENR::VALUE3 => 2,
            BDENR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BDENR {
        match value {
            0 => BDENR::VALUE1,
            1 => BDENR::VALUE2,
            2 => BDENR::VALUE3,
            3 => BDENR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BDENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BDENR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BDENR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BDENR::VALUE4
    }
}
#[doc = "Possible values of the field `BDVTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDVTRR {
    #[doc = "Bit BDV is not influenced by DX2T."]
    VALUE1,
    #[doc = "Bit BDV is set if DX2T is active."]
    VALUE2,
}
impl BDVTRR {
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
            BDVTRR::VALUE1 => false,
            BDVTRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BDVTRR {
        match value {
            false => BDVTRR::VALUE1,
            true => BDVTRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BDVTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BDVTRR::VALUE2
    }
}
#[doc = "Possible values of the field `BPRIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPRIOR {
    #[doc = "The transmit FIFO data has a higher priority than the bypass data."]
    VALUE1,
    #[doc = "The bypass data has a higher priority than the transmit FIFO data."]
    VALUE2,
}
impl BPRIOR {
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
            BPRIOR::VALUE1 => false,
            BPRIOR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BPRIOR {
        match value {
            false => BPRIOR::VALUE1,
            true => BPRIOR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BPRIOR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BPRIOR::VALUE2
    }
}
#[doc = "Possible values of the field `BDV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDVR {
    #[doc = "The bypass data is not valid."]
    VALUE1,
    #[doc = "The bypass data is valid."]
    VALUE2,
}
impl BDVR {
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
            BDVR::VALUE1 => false,
            BDVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BDVR {
        match value {
            false => BDVR::VALUE1,
            true => BDVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BDVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BDVR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct BSELOR {
    bits: u8,
}
impl BSELOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BHPCR {
    bits: u8,
}
impl BHPCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BWLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BWLEW<'a> {
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
#[doc = "Values that can be written to the field `BDSSM`"]
pub enum BDSSMW {
    #[doc = "The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    VALUE1,
    #[doc = "The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    VALUE2,
}
impl BDSSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BDSSMW::VALUE1 => false,
            BDSSMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BDSSMW<'a> {
    w: &'a mut W,
}
impl<'a> _BDSSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BDSSMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BDSSMW::VALUE1)
    }
    #[doc = "The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BDSSMW::VALUE2)
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
#[doc = "Values that can be written to the field `BDEN`"]
pub enum BDENW {
    #[doc = "The transfer of bypass data is disabled."]
    VALUE1,
    #[doc = "The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    VALUE2,
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    VALUE3,
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    VALUE4,
}
impl BDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BDENW::VALUE1 => 0,
            BDENW::VALUE2 => 1,
            BDENW::VALUE3 => 2,
            BDENW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BDENW<'a> {
    w: &'a mut W,
}
impl<'a> _BDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BDENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The transfer of bypass data is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BDENW::VALUE1)
    }
    #[doc = "The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BDENW::VALUE2)
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BDENW::VALUE3)
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BDENW::VALUE4)
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
#[doc = "Values that can be written to the field `BDVTR`"]
pub enum BDVTRW {
    #[doc = "Bit BDV is not influenced by DX2T."]
    VALUE1,
    #[doc = "Bit BDV is set if DX2T is active."]
    VALUE2,
}
impl BDVTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BDVTRW::VALUE1 => false,
            BDVTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BDVTRW<'a> {
    w: &'a mut W,
}
impl<'a> _BDVTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BDVTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit BDV is not influenced by DX2T."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BDVTRW::VALUE1)
    }
    #[doc = "Bit BDV is set if DX2T is active."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BDVTRW::VALUE2)
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
#[doc = "Values that can be written to the field `BPRIO`"]
pub enum BPRIOW {
    #[doc = "The transmit FIFO data has a higher priority than the bypass data."]
    VALUE1,
    #[doc = "The bypass data has a higher priority than the transmit FIFO data."]
    VALUE2,
}
impl BPRIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BPRIOW::VALUE1 => false,
            BPRIOW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BPRIOW<'a> {
    w: &'a mut W,
}
impl<'a> _BPRIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BPRIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmit FIFO data has a higher priority than the bypass data."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPRIOW::VALUE1)
    }
    #[doc = "The bypass data has a higher priority than the transmit FIFO data."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPRIOW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _BSELOW<'a> {
    w: &'a mut W,
}
impl<'a> _BSELOW<'a> {
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
#[doc = r" Proxy"]
pub struct _BHPCW<'a> {
    w: &'a mut W,
}
impl<'a> _BHPCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:3 - Bypass Word Length"]
    #[inline]
    pub fn bwle(&self) -> BWLER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BWLER { bits }
    }
    #[doc = "Bit 8 - Bypass Data Single Shot Mode"]
    #[inline]
    pub fn bdssm(&self) -> BDSSMR {
        BDSSMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Bypass Data Enable"]
    #[inline]
    pub fn bden(&self) -> BDENR {
        BDENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Bypass Data Valid Trigger"]
    #[inline]
    pub fn bdvtr(&self) -> BDVTRR {
        BDVTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Bypass Priority"]
    #[inline]
    pub fn bprio(&self) -> BPRIOR {
        BPRIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Bypass Data Valid"]
    #[inline]
    pub fn bdv(&self) -> BDVR {
        BDVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Bypass Select Outputs"]
    #[inline]
    pub fn bselo(&self) -> BSELOR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BSELOR { bits }
    }
    #[doc = "Bits 21:23 - Bypass Hardware Port Control"]
    #[inline]
    pub fn bhpc(&self) -> BHPCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BHPCR { bits }
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
    #[doc = "Bits 0:3 - Bypass Word Length"]
    #[inline]
    pub fn bwle(&mut self) -> _BWLEW {
        _BWLEW { w: self }
    }
    #[doc = "Bit 8 - Bypass Data Single Shot Mode"]
    #[inline]
    pub fn bdssm(&mut self) -> _BDSSMW {
        _BDSSMW { w: self }
    }
    #[doc = "Bits 10:11 - Bypass Data Enable"]
    #[inline]
    pub fn bden(&mut self) -> _BDENW {
        _BDENW { w: self }
    }
    #[doc = "Bit 12 - Bypass Data Valid Trigger"]
    #[inline]
    pub fn bdvtr(&mut self) -> _BDVTRW {
        _BDVTRW { w: self }
    }
    #[doc = "Bit 13 - Bypass Priority"]
    #[inline]
    pub fn bprio(&mut self) -> _BPRIOW {
        _BPRIOW { w: self }
    }
    #[doc = "Bits 16:20 - Bypass Select Outputs"]
    #[inline]
    pub fn bselo(&mut self) -> _BSELOW {
        _BSELOW { w: self }
    }
    #[doc = "Bits 21:23 - Bypass Hardware Port Control"]
    #[inline]
    pub fn bhpc(&mut self) -> _BHPCW {
        _BHPCW { w: self }
    }
}
