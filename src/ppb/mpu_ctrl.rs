#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MPU_CTRL {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "MPU disabled"]
    VALUE1,
    #[doc = "MPU enabled."]
    VALUE2,
}
impl ENABLER {
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
            ENABLER::VALUE1 => false,
            ENABLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::VALUE1,
            true => ENABLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENABLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENABLER::VALUE2
    }
}
#[doc = "Possible values of the field `HFNMIENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFNMIENAR {
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    VALUE1,
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    VALUE2,
}
impl HFNMIENAR {
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
            HFNMIENAR::VALUE1 => false,
            HFNMIENAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HFNMIENAR {
        match value {
            false => HFNMIENAR::VALUE1,
            true => HFNMIENAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HFNMIENAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HFNMIENAR::VALUE2
    }
}
#[doc = "Possible values of the field `PRIVDEFENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIVDEFENAR {
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    VALUE1,
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    VALUE2,
}
impl PRIVDEFENAR {
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
            PRIVDEFENAR::VALUE1 => false,
            PRIVDEFENAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRIVDEFENAR {
        match value {
            false => PRIVDEFENAR::VALUE1,
            true => PRIVDEFENAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRIVDEFENAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRIVDEFENAR::VALUE2
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "MPU disabled"]
    VALUE1,
    #[doc = "MPU enabled."]
    VALUE2,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::VALUE1 => false,
            ENABLEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MPU disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENABLEW::VALUE1)
    }
    #[doc = "MPU enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENABLEW::VALUE2)
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
#[doc = "Values that can be written to the field `HFNMIENA`"]
pub enum HFNMIENAW {
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    VALUE1,
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    VALUE2,
}
impl HFNMIENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HFNMIENAW::VALUE1 => false,
            HFNMIENAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFNMIENAW<'a> {
    w: &'a mut W,
}
impl<'a> _HFNMIENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFNMIENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HFNMIENAW::VALUE1)
    }
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HFNMIENAW::VALUE2)
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
#[doc = "Values that can be written to the field `PRIVDEFENA`"]
pub enum PRIVDEFENAW {
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    VALUE1,
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    VALUE2,
}
impl PRIVDEFENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRIVDEFENAW::VALUE1 => false,
            PRIVDEFENAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRIVDEFENAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRIVDEFENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRIVDEFENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIVDEFENAW::VALUE1)
    }
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIVDEFENAW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable MPU"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline]
    pub fn hfnmiena(&self) -> HFNMIENAR {
        HFNMIENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline]
    pub fn privdefena(&self) -> PRIVDEFENAR {
        PRIVDEFENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Enable MPU"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline]
    pub fn hfnmiena(&mut self) -> _HFNMIENAW {
        _HFNMIENAW { w: self }
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline]
    pub fn privdefena(&mut self) -> _PRIVDEFENAW {
        _PRIVDEFENAW { w: self }
    }
}
