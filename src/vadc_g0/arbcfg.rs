#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ARBCFG {
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
pub struct ANONCR {
    bits: u8,
}
impl ANONCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ARBRND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBRNDR {
    #[doc = "4 arbitration slots per round (tARB = 4 / fADCD)"]
    VALUE1,
    #[doc = "8 arbitration slots per round (tARB = 8 / fADCD)"]
    VALUE2,
    #[doc = "16 arbitration slots per round (tARB = 16 / fADCD)"]
    VALUE3,
    #[doc = "20 arbitration slots per round (tARB = 20 / fADCD)"]
    VALUE4,
}
impl ARBRNDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARBRNDR::VALUE1 => 0,
            ARBRNDR::VALUE2 => 1,
            ARBRNDR::VALUE3 => 2,
            ARBRNDR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARBRNDR {
        match value {
            0 => ARBRNDR::VALUE1,
            1 => ARBRNDR::VALUE2,
            2 => ARBRNDR::VALUE3,
            3 => ARBRNDR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARBRNDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ARBRNDR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ARBRNDR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ARBRNDR::VALUE4
    }
}
#[doc = "Possible values of the field `ARBM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBMR {
    #[doc = "The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    VALUE1,
    #[doc = "The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    VALUE2,
}
impl ARBMR {
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
            ARBMR::VALUE1 => false,
            ARBMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARBMR {
        match value {
            false => ARBMR::VALUE1,
            true => ARBMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARBMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ARBMR::VALUE2
    }
}
#[doc = "Possible values of the field `ANONS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANONSR {
    #[doc = "Analog converter off"]
    VALUE1,
    #[doc = "Normal operation (permanently on)"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ANONSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ANONSR::VALUE1 => 0,
            ANONSR::VALUE4 => 3,
            ANONSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ANONSR {
        match value {
            0 => ANONSR::VALUE1,
            3 => ANONSR::VALUE4,
            i => ANONSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ANONSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ANONSR::VALUE4
    }
}
#[doc = "Possible values of the field `CAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALR {
    #[doc = "Completed or not yet started"]
    VALUE1,
    #[doc = "Start-up calibration phase is active"]
    VALUE2,
}
impl CALR {
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
            CALR::VALUE1 => false,
            CALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALR {
        match value {
            false => CALR::VALUE1,
            true => CALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CALR::VALUE2
    }
}
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSYR {
    #[doc = "Not busy"]
    VALUE1,
    #[doc = "Converter is busy with a conversion"]
    VALUE2,
}
impl BUSYR {
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
            BUSYR::VALUE1 => false,
            BUSYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSYR {
        match value {
            false => BUSYR::VALUE1,
            true => BUSYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUSYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUSYR::VALUE2
    }
}
#[doc = "Possible values of the field `SAMPLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLER {
    #[doc = "Converting or idle"]
    VALUE1,
    #[doc = "Input signal is currently sampled"]
    VALUE2,
}
impl SAMPLER {
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
            SAMPLER::VALUE1 => false,
            SAMPLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAMPLER {
        match value {
            false => SAMPLER::VALUE1,
            true => SAMPLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SAMPLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SAMPLER::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _ANONCW<'a> {
    w: &'a mut W,
}
impl<'a> _ANONCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARBRND`"]
pub enum ARBRNDW {
    #[doc = "4 arbitration slots per round (tARB = 4 / fADCD)"]
    VALUE1,
    #[doc = "8 arbitration slots per round (tARB = 8 / fADCD)"]
    VALUE2,
    #[doc = "16 arbitration slots per round (tARB = 16 / fADCD)"]
    VALUE3,
    #[doc = "20 arbitration slots per round (tARB = 20 / fADCD)"]
    VALUE4,
}
impl ARBRNDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ARBRNDW::VALUE1 => 0,
            ARBRNDW::VALUE2 => 1,
            ARBRNDW::VALUE3 => 2,
            ARBRNDW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBRNDW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBRNDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBRNDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 arbitration slots per round (tARB = 4 / fADCD)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBRNDW::VALUE1)
    }
    #[doc = "8 arbitration slots per round (tARB = 8 / fADCD)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBRNDW::VALUE2)
    }
    #[doc = "16 arbitration slots per round (tARB = 16 / fADCD)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ARBRNDW::VALUE3)
    }
    #[doc = "20 arbitration slots per round (tARB = 20 / fADCD)"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ARBRNDW::VALUE4)
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
#[doc = "Values that can be written to the field `ARBM`"]
pub enum ARBMW {
    #[doc = "The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    VALUE1,
    #[doc = "The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    VALUE2,
}
impl ARBMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARBMW::VALUE1 => false,
            ARBMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBMW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBMW::VALUE1)
    }
    #[doc = "The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBMW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline]
    pub fn anonc(&self) -> ANONCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ANONCR { bits }
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline]
    pub fn arbrnd(&self) -> ARBRNDR {
        ARBRNDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline]
    pub fn arbm(&self) -> ARBMR {
        ARBMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - Analog Converter Control Status"]
    #[inline]
    pub fn anons(&self) -> ANONSR {
        ANONSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Start-Up Calibration Active Indication"]
    #[inline]
    pub fn cal(&self) -> CALR {
        CALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Converter Busy Flag"]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Sample Phase Flag"]
    #[inline]
    pub fn sample(&self) -> SAMPLER {
        SAMPLER::_from({
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
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline]
    pub fn anonc(&mut self) -> _ANONCW {
        _ANONCW { w: self }
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline]
    pub fn arbrnd(&mut self) -> _ARBRNDW {
        _ARBRNDW { w: self }
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline]
    pub fn arbm(&mut self) -> _ARBMW {
        _ARBMW { w: self }
    }
}
