#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::POWER_CTRL {
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
pub struct HARDWARE_RESETR {
    bits: bool,
}
impl HARDWARE_RESETR {
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
#[doc = "Possible values of the field `SD_BUS_VOLTAGE_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SD_BUS_VOLTAGE_SELR {
    #[doc = "3.3V (Flattop.)"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SD_BUS_VOLTAGE_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SD_BUS_VOLTAGE_SELR::VALUE1 => 7,
            SD_BUS_VOLTAGE_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SD_BUS_VOLTAGE_SELR {
        match value {
            7 => SD_BUS_VOLTAGE_SELR::VALUE1,
            i => SD_BUS_VOLTAGE_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SD_BUS_VOLTAGE_SELR::VALUE1
    }
}
#[doc = "Possible values of the field `SD_BUS_POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SD_BUS_POWERR {
    #[doc = "Power off"]
    VALUE1,
    #[doc = "Power on"]
    VALUE2,
}
impl SD_BUS_POWERR {
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
            SD_BUS_POWERR::VALUE1 => false,
            SD_BUS_POWERR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SD_BUS_POWERR {
        match value {
            false => SD_BUS_POWERR::VALUE1,
            true => SD_BUS_POWERR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SD_BUS_POWERR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SD_BUS_POWERR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _HARDWARE_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _HARDWARE_RESETW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SD_BUS_VOLTAGE_SEL`"]
pub enum SD_BUS_VOLTAGE_SELW {
    #[doc = "3.3V (Flattop.)"]
    VALUE1,
}
impl SD_BUS_VOLTAGE_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SD_BUS_VOLTAGE_SELW::VALUE1 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SD_BUS_VOLTAGE_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SD_BUS_VOLTAGE_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SD_BUS_VOLTAGE_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "3.3V (Flattop.)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SD_BUS_VOLTAGE_SELW::VALUE1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SD_BUS_POWER`"]
pub enum SD_BUS_POWERW {
    #[doc = "Power off"]
    VALUE1,
    #[doc = "Power on"]
    VALUE2,
}
impl SD_BUS_POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SD_BUS_POWERW::VALUE1 => false,
            SD_BUS_POWERW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SD_BUS_POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _SD_BUS_POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SD_BUS_POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power off"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SD_BUS_POWERW::VALUE1)
    }
    #[doc = "Power on"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SD_BUS_POWERW::VALUE2)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 4 - Hardware reset"]
    #[inline]
    pub fn hardware_reset(&self) -> HARDWARE_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        HARDWARE_RESETR { bits }
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline]
    pub fn sd_bus_voltage_sel(&self) -> SD_BUS_VOLTAGE_SELR {
        SD_BUS_VOLTAGE_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline]
    pub fn sd_bus_power(&self) -> SD_BUS_POWERR {
        SD_BUS_POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Hardware reset"]
    #[inline]
    pub fn hardware_reset(&mut self) -> _HARDWARE_RESETW {
        _HARDWARE_RESETW { w: self }
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline]
    pub fn sd_bus_voltage_sel(&mut self) -> _SD_BUS_VOLTAGE_SELW {
        _SD_BUS_VOLTAGE_SELW { w: self }
    }
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline]
    pub fn sd_bus_power(&mut self) -> _SD_BUS_POWERW {
        _SD_BUS_POWERW { w: self }
    }
}
