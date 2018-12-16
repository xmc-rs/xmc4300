#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TC {
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
#[doc = "Possible values of the field `TCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCMR {
    #[doc = "Edge aligned mode"]
    VALUE1,
    #[doc = "Center aligned mode"]
    VALUE2,
}
impl TCMR {
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
            TCMR::VALUE1 => false,
            TCMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCMR {
        match value {
            false => TCMR::VALUE1,
            true => TCMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TCMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TCMR::VALUE2
    }
}
#[doc = "Possible values of the field `TSSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSSMR {
    #[doc = "Single shot mode is disabled"]
    VALUE1,
    #[doc = "Single shot mode is enabled"]
    VALUE2,
}
impl TSSMR {
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
            TSSMR::VALUE1 => false,
            TSSMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSSMR {
        match value {
            false => TSSMR::VALUE1,
            true => TSSMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSSMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSSMR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CLSTR {
    bits: bool,
}
impl CLSTR {
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
#[doc = "Possible values of the field `CMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMODR {
    #[doc = "Compare Mode"]
    VALUE1,
    #[doc = "Capture Mode"]
    VALUE2,
}
impl CMODR {
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
            CMODR::VALUE1 => false,
            CMODR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMODR {
        match value {
            false => CMODR::VALUE1,
            true => CMODR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMODR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMODR::VALUE2
    }
}
#[doc = "Possible values of the field `ECM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECMR {
    #[doc = "Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    VALUE1,
    #[doc = "Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the ECRD.VPTR is cleared."]
    VALUE2,
}
impl ECMR {
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
            ECMR::VALUE1 => false,
            ECMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECMR {
        match value {
            false => ECMR::VALUE1,
            true => ECMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECMR::VALUE2
    }
}
#[doc = "Possible values of the field `CAPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPCR {
    #[doc = "Timer is never cleared on a capture event"]
    VALUE1,
    #[doc = "Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    VALUE2,
    #[doc = "Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    VALUE3,
    #[doc = "Timer is always cleared in a capture event."]
    VALUE4,
}
impl CAPCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAPCR::VALUE1 => 0,
            CAPCR::VALUE2 => 1,
            CAPCR::VALUE3 => 2,
            CAPCR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAPCR {
        match value {
            0 => CAPCR::VALUE1,
            1 => CAPCR::VALUE2,
            2 => CAPCR::VALUE3,
            3 => CAPCR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CAPCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CAPCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CAPCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CAPCR::VALUE4
    }
}
#[doc = "Possible values of the field `ENDM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDMR {
    #[doc = "Clears the timer run bit only (default stop)"]
    VALUE1,
    #[doc = "Clears the timer only (flush)"]
    VALUE2,
    #[doc = "Clears the timer and run bit (flush/stop)"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENDMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENDMR::VALUE1 => 0,
            ENDMR::VALUE2 => 1,
            ENDMR::VALUE3 => 2,
            ENDMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENDMR {
        match value {
            0 => ENDMR::VALUE1,
            1 => ENDMR::VALUE2,
            2 => ENDMR::VALUE3,
            i => ENDMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENDMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENDMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ENDMR::VALUE3
    }
}
#[doc = "Possible values of the field `STRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRMR {
    #[doc = "Sets run bit only (default start)"]
    VALUE1,
    #[doc = "Clears the timer and sets run bit (flush/start)"]
    VALUE2,
}
impl STRMR {
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
            STRMR::VALUE1 => false,
            STRMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STRMR {
        match value {
            false => STRMR::VALUE1,
            true => STRMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STRMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STRMR::VALUE2
    }
}
#[doc = "Possible values of the field `SCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCER {
    #[doc = "Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    VALUE1,
    #[doc = "Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. and CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    VALUE2,
}
impl SCER {
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
            SCER::VALUE1 => false,
            SCER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCER {
        match value {
            false => SCER::VALUE1,
            true => SCER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SCER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SCER::VALUE2
    }
}
#[doc = "Possible values of the field `CCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCSR {
    #[doc = "The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    VALUE1,
    #[doc = "The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    VALUE2,
}
impl CCSR {
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
            CCSR::VALUE1 => false,
            CCSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCSR {
        match value {
            false => CCSR::VALUE1,
            true => CCSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCSR::VALUE2
    }
}
#[doc = "Possible values of the field `DITHE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DITHER {
    #[doc = "Dither is disabled"]
    VALUE1,
    #[doc = "Dither is applied to the Period"]
    VALUE2,
    #[doc = "Dither is applied to the Compare"]
    VALUE3,
    #[doc = "Dither is applied to the Period and Compare"]
    VALUE4,
}
impl DITHER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DITHER::VALUE1 => 0,
            DITHER::VALUE2 => 1,
            DITHER::VALUE3 => 2,
            DITHER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DITHER {
        match value {
            0 => DITHER::VALUE1,
            1 => DITHER::VALUE2,
            2 => DITHER::VALUE3,
            3 => DITHER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DITHER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DITHER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DITHER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DITHER::VALUE4
    }
}
#[doc = "Possible values of the field `DIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIMR {
    #[doc = "Slice is using its own dither unit"]
    VALUE1,
    #[doc = "Slice is connected to the dither unit of slice 0."]
    VALUE2,
}
impl DIMR {
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
            DIMR::VALUE1 => false,
            DIMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIMR {
        match value {
            false => DIMR::VALUE1,
            true => DIMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIMR::VALUE2
    }
}
#[doc = "Possible values of the field `FPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPER {
    #[doc = "Floating prescaler mode is disabled"]
    VALUE1,
    #[doc = "Floating prescaler mode is enabled"]
    VALUE2,
}
impl FPER {
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
            FPER::VALUE1 => false,
            FPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPER {
        match value {
            false => FPER::VALUE1,
            true => FPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FPER::VALUE2
    }
}
#[doc = "Possible values of the field `TRAPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRAPER {
    #[doc = "TRAP functionality has no effect on the output"]
    VALUE1,
    #[doc = "TRAP functionality affects the output"]
    VALUE2,
}
impl TRAPER {
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
            TRAPER::VALUE1 => false,
            TRAPER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRAPER {
        match value {
            false => TRAPER::VALUE1,
            true => TRAPER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRAPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRAPER::VALUE2
    }
}
#[doc = "Possible values of the field `TRPSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRPSER {
    #[doc = "Exiting from TRAP state isn't synchronized with the PWM signal"]
    VALUE1,
    #[doc = "Exiting from TRAP state is synchronized with the PWM signal"]
    VALUE2,
}
impl TRPSER {
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
            TRPSER::VALUE1 => false,
            TRPSER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRPSER {
        match value {
            false => TRPSER::VALUE1,
            true => TRPSER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRPSER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRPSER::VALUE2
    }
}
#[doc = "Possible values of the field `TRPSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRPSWR {
    #[doc = "The slice exits the TRAP state automatically when the TRAP condition is not present"]
    VALUE1,
    #[doc = "The TRAP state can only be exited by a SW request."]
    VALUE2,
}
impl TRPSWR {
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
            TRPSWR::VALUE1 => false,
            TRPSWR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRPSWR {
        match value {
            false => TRPSWR::VALUE1,
            true => TRPSWR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRPSWR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRPSWR::VALUE2
    }
}
#[doc = "Possible values of the field `EMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMSR {
    #[doc = "External Modulation functionality is not synchronized with the PWM signal"]
    VALUE1,
    #[doc = "External Modulation functionality is synchronized with the PWM signal"]
    VALUE2,
}
impl EMSR {
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
            EMSR::VALUE1 => false,
            EMSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMSR {
        match value {
            false => EMSR::VALUE1,
            true => EMSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMSR::VALUE2
    }
}
#[doc = "Possible values of the field `EMT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMTR {
    #[doc = "External Modulation functionality is clearing the CC4yST bit."]
    VALUE1,
    #[doc = "External Modulation functionality is gating the outputs."]
    VALUE2,
}
impl EMTR {
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
            EMTR::VALUE1 => false,
            EMTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMTR {
        match value {
            false => EMTR::VALUE1,
            true => EMTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMTR::VALUE2
    }
}
#[doc = "Possible values of the field `MCME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCMER {
    #[doc = "Multi Channel Mode is disabled"]
    VALUE1,
    #[doc = "Multi Channel Mode is enabled"]
    VALUE2,
}
impl MCMER {
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
            MCMER::VALUE1 => false,
            MCMER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCMER {
        match value {
            false => MCMER::VALUE1,
            true => MCMER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCMER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCMER::VALUE2
    }
}
#[doc = "Values that can be written to the field `TCM`"]
pub enum TCMW {
    #[doc = "Edge aligned mode"]
    VALUE1,
    #[doc = "Center aligned mode"]
    VALUE2,
}
impl TCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCMW::VALUE1 => false,
            TCMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCMW<'a> {
    w: &'a mut W,
}
impl<'a> _TCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge aligned mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TCMW::VALUE1)
    }
    #[doc = "Center aligned mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TCMW::VALUE2)
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
#[doc = "Values that can be written to the field `TSSM`"]
pub enum TSSMW {
    #[doc = "Single shot mode is disabled"]
    VALUE1,
    #[doc = "Single shot mode is enabled"]
    VALUE2,
}
impl TSSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSSMW::VALUE1 => false,
            TSSMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSSMW<'a> {
    w: &'a mut W,
}
impl<'a> _TSSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSSMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single shot mode is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSSMW::VALUE1)
    }
    #[doc = "Single shot mode is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSSMW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _CLSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CLSTW<'a> {
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
#[doc = "Values that can be written to the field `ECM`"]
pub enum ECMW {
    #[doc = "Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    VALUE1,
    #[doc = "Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the ECRD.VPTR is cleared."]
    VALUE2,
}
impl ECMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECMW::VALUE1 => false,
            ECMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECMW<'a> {
    w: &'a mut W,
}
impl<'a> _ECMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Capture Mode. Clear of the Full Flag of each capture register is done by accessing the registers individually only."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECMW::VALUE1)
    }
    #[doc = "Extended Capture Mode. Clear of the Full Flag of each capture register is done not only by accessing the individual registers but also by accessing the ECRD register. When reading the ECRD register, only the capture register register full flag pointed by the ECRD.VPTR is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECMW::VALUE2)
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
#[doc = "Values that can be written to the field `CAPC`"]
pub enum CAPCW {
    #[doc = "Timer is never cleared on a capture event"]
    VALUE1,
    #[doc = "Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    VALUE2,
    #[doc = "Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    VALUE3,
    #[doc = "Timer is always cleared in a capture event."]
    VALUE4,
}
impl CAPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAPCW::VALUE1 => 0,
            CAPCW::VALUE2 => 1,
            CAPCW::VALUE3 => 2,
            CAPCW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPCW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer is never cleared on a capture event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CAPCW::VALUE1)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 2 and 3. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CAPCW::VALUE2)
    }
    #[doc = "Timer is cleared on a capture event into capture registers 0 and 1. (When SCE = 1#, Timer is always cleared in a capture event)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CAPCW::VALUE3)
    }
    #[doc = "Timer is always cleared in a capture event."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CAPCW::VALUE4)
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
#[doc = "Values that can be written to the field `ENDM`"]
pub enum ENDMW {
    #[doc = "Clears the timer run bit only (default stop)"]
    VALUE1,
    #[doc = "Clears the timer only (flush)"]
    VALUE2,
    #[doc = "Clears the timer and run bit (flush/stop)"]
    VALUE3,
}
impl ENDMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENDMW::VALUE1 => 0,
            ENDMW::VALUE2 => 1,
            ENDMW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDMW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clears the timer run bit only (default stop)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDMW::VALUE1)
    }
    #[doc = "Clears the timer only (flush)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENDMW::VALUE2)
    }
    #[doc = "Clears the timer and run bit (flush/stop)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENDMW::VALUE3)
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
#[doc = "Values that can be written to the field `STRM`"]
pub enum STRMW {
    #[doc = "Sets run bit only (default start)"]
    VALUE1,
    #[doc = "Clears the timer and sets run bit (flush/start)"]
    VALUE2,
}
impl STRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STRMW::VALUE1 => false,
            STRMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STRMW<'a> {
    w: &'a mut W,
}
impl<'a> _STRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sets run bit only (default start)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STRMW::VALUE1)
    }
    #[doc = "Clears the timer and sets run bit (flush/start)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STRMW::VALUE2)
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
#[doc = "Values that can be written to the field `SCE`"]
pub enum SCEW {
    #[doc = "Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    VALUE1,
    #[doc = "Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. and CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    VALUE2,
}
impl SCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCEW::VALUE1 => false,
            SCEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. registers control by CCycapt0 and capture into CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCEW::VALUE1)
    }
    #[doc = "Capture into CC4yC0VThis register contains the values associated with the Capture 0 field./CC4yC1VThis register contains the values associated with the Capture 1 field. and CC4yC3VThis register contains the values associated with the Capture 3 field./CC4yC2VThis register contains the values associated with the Capture 2 field. control by CCycapt1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCEW::VALUE2)
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
#[doc = "Values that can be written to the field `CCS`"]
pub enum CCSW {
    #[doc = "The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    VALUE1,
    #[doc = "The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    VALUE2,
}
impl CCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCSW::VALUE1 => false,
            CCSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The capture into a specific capture register is done with the rules linked with the full flags, described at ."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCSW::VALUE1)
    }
    #[doc = "The capture into the capture registers is always done regardless of the full flag status (even if the register has not been read back)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCSW::VALUE2)
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
#[doc = "Values that can be written to the field `DITHE`"]
pub enum DITHEW {
    #[doc = "Dither is disabled"]
    VALUE1,
    #[doc = "Dither is applied to the Period"]
    VALUE2,
    #[doc = "Dither is applied to the Compare"]
    VALUE3,
    #[doc = "Dither is applied to the Period and Compare"]
    VALUE4,
}
impl DITHEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DITHEW::VALUE1 => 0,
            DITHEW::VALUE2 => 1,
            DITHEW::VALUE3 => 2,
            DITHEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DITHEW<'a> {
    w: &'a mut W,
}
impl<'a> _DITHEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DITHEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Dither is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DITHEW::VALUE1)
    }
    #[doc = "Dither is applied to the Period"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DITHEW::VALUE2)
    }
    #[doc = "Dither is applied to the Compare"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DITHEW::VALUE3)
    }
    #[doc = "Dither is applied to the Period and Compare"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DITHEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIM`"]
pub enum DIMW {
    #[doc = "Slice is using its own dither unit"]
    VALUE1,
    #[doc = "Slice is connected to the dither unit of slice 0."]
    VALUE2,
}
impl DIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIMW::VALUE1 => false,
            DIMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIMW<'a> {
    w: &'a mut W,
}
impl<'a> _DIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slice is using its own dither unit"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIMW::VALUE1)
    }
    #[doc = "Slice is connected to the dither unit of slice 0."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIMW::VALUE2)
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
#[doc = "Values that can be written to the field `FPE`"]
pub enum FPEW {
    #[doc = "Floating prescaler mode is disabled"]
    VALUE1,
    #[doc = "Floating prescaler mode is enabled"]
    VALUE2,
}
impl FPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPEW::VALUE1 => false,
            FPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPEW<'a> {
    w: &'a mut W,
}
impl<'a> _FPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Floating prescaler mode is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FPEW::VALUE1)
    }
    #[doc = "Floating prescaler mode is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FPEW::VALUE2)
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
#[doc = "Values that can be written to the field `TRAPE`"]
pub enum TRAPEW {
    #[doc = "TRAP functionality has no effect on the output"]
    VALUE1,
    #[doc = "TRAP functionality affects the output"]
    VALUE2,
}
impl TRAPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRAPEW::VALUE1 => false,
            TRAPEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRAPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRAPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRAPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TRAP functionality has no effect on the output"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRAPEW::VALUE1)
    }
    #[doc = "TRAP functionality affects the output"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRAPEW::VALUE2)
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
#[doc = "Values that can be written to the field `TRPSE`"]
pub enum TRPSEW {
    #[doc = "Exiting from TRAP state isn't synchronized with the PWM signal"]
    VALUE1,
    #[doc = "Exiting from TRAP state is synchronized with the PWM signal"]
    VALUE2,
}
impl TRPSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRPSEW::VALUE1 => false,
            TRPSEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRPSEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRPSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRPSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exiting from TRAP state isn't synchronized with the PWM signal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRPSEW::VALUE1)
    }
    #[doc = "Exiting from TRAP state is synchronized with the PWM signal"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRPSEW::VALUE2)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRPSW`"]
pub enum TRPSWW {
    #[doc = "The slice exits the TRAP state automatically when the TRAP condition is not present"]
    VALUE1,
    #[doc = "The TRAP state can only be exited by a SW request."]
    VALUE2,
}
impl TRPSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRPSWW::VALUE1 => false,
            TRPSWW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRPSWW<'a> {
    w: &'a mut W,
}
impl<'a> _TRPSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRPSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The slice exits the TRAP state automatically when the TRAP condition is not present"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRPSWW::VALUE1)
    }
    #[doc = "The TRAP state can only be exited by a SW request."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRPSWW::VALUE2)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMS`"]
pub enum EMSW {
    #[doc = "External Modulation functionality is not synchronized with the PWM signal"]
    VALUE1,
    #[doc = "External Modulation functionality is synchronized with the PWM signal"]
    VALUE2,
}
impl EMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMSW::VALUE1 => false,
            EMSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External Modulation functionality is not synchronized with the PWM signal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMSW::VALUE1)
    }
    #[doc = "External Modulation functionality is synchronized with the PWM signal"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMSW::VALUE2)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMT`"]
pub enum EMTW {
    #[doc = "External Modulation functionality is clearing the CC4yST bit."]
    VALUE1,
    #[doc = "External Modulation functionality is gating the outputs."]
    VALUE2,
}
impl EMTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMTW::VALUE1 => false,
            EMTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External Modulation functionality is clearing the CC4yST bit."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMTW::VALUE1)
    }
    #[doc = "External Modulation functionality is gating the outputs."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMTW::VALUE2)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCME`"]
pub enum MCMEW {
    #[doc = "Multi Channel Mode is disabled"]
    VALUE1,
    #[doc = "Multi Channel Mode is enabled"]
    VALUE2,
}
impl MCMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCMEW::VALUE1 => false,
            MCMEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCMEW<'a> {
    w: &'a mut W,
}
impl<'a> _MCMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Multi Channel Mode is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCMEW::VALUE1)
    }
    #[doc = "Multi Channel Mode is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCMEW::VALUE2)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Timer Counting Mode"]
    #[inline]
    pub fn tcm(&self) -> TCMR {
        TCMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Timer Single Shot Mode"]
    #[inline]
    pub fn tssm(&self) -> TSSMR {
        TSSMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Shadow Transfer on Clear"]
    #[inline]
    pub fn clst(&self) -> CLSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLSTR { bits }
    }
    #[doc = "Bit 3 - Capture Compare Mode"]
    #[inline]
    pub fn cmod(&self) -> CMODR {
        CMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Extended Capture Mode"]
    #[inline]
    pub fn ecm(&self) -> ECMR {
        ECMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Clear on Capture Control"]
    #[inline]
    pub fn capc(&self) -> CAPCR {
        CAPCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Extended Stop Function Control"]
    #[inline]
    pub fn endm(&self) -> ENDMR {
        ENDMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Extended Start Function Control"]
    #[inline]
    pub fn strm(&self) -> STRMR {
        STRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Equal Capture Event enable"]
    #[inline]
    pub fn sce(&self) -> SCER {
        SCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Continuous Capture Enable"]
    #[inline]
    pub fn ccs(&self) -> CCSR {
        CCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - Dither Enable"]
    #[inline]
    pub fn dithe(&self) -> DITHER {
        DITHER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Dither input selector"]
    #[inline]
    pub fn dim(&self) -> DIMR {
        DIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Floating Prescaler enable"]
    #[inline]
    pub fn fpe(&self) -> FPER {
        FPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TRAP enable"]
    #[inline]
    pub fn trape(&self) -> TRAPER {
        TRAPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - TRAP Synchronization Enable"]
    #[inline]
    pub fn trpse(&self) -> TRPSER {
        TRPSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - TRAP State Clear Control"]
    #[inline]
    pub fn trpsw(&self) -> TRPSWR {
        TRPSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - External Modulation Synchronization"]
    #[inline]
    pub fn ems(&self) -> EMSR {
        EMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - External Modulation Type"]
    #[inline]
    pub fn emt(&self) -> EMTR {
        EMTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Multi Channel Mode Enable"]
    #[inline]
    pub fn mcme(&self) -> MCMER {
        MCMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Timer Counting Mode"]
    #[inline]
    pub fn tcm(&mut self) -> _TCMW {
        _TCMW { w: self }
    }
    #[doc = "Bit 1 - Timer Single Shot Mode"]
    #[inline]
    pub fn tssm(&mut self) -> _TSSMW {
        _TSSMW { w: self }
    }
    #[doc = "Bit 2 - Shadow Transfer on Clear"]
    #[inline]
    pub fn clst(&mut self) -> _CLSTW {
        _CLSTW { w: self }
    }
    #[doc = "Bit 4 - Extended Capture Mode"]
    #[inline]
    pub fn ecm(&mut self) -> _ECMW {
        _ECMW { w: self }
    }
    #[doc = "Bits 5:6 - Clear on Capture Control"]
    #[inline]
    pub fn capc(&mut self) -> _CAPCW {
        _CAPCW { w: self }
    }
    #[doc = "Bits 8:9 - Extended Stop Function Control"]
    #[inline]
    pub fn endm(&mut self) -> _ENDMW {
        _ENDMW { w: self }
    }
    #[doc = "Bit 10 - Extended Start Function Control"]
    #[inline]
    pub fn strm(&mut self) -> _STRMW {
        _STRMW { w: self }
    }
    #[doc = "Bit 11 - Equal Capture Event enable"]
    #[inline]
    pub fn sce(&mut self) -> _SCEW {
        _SCEW { w: self }
    }
    #[doc = "Bit 12 - Continuous Capture Enable"]
    #[inline]
    pub fn ccs(&mut self) -> _CCSW {
        _CCSW { w: self }
    }
    #[doc = "Bits 13:14 - Dither Enable"]
    #[inline]
    pub fn dithe(&mut self) -> _DITHEW {
        _DITHEW { w: self }
    }
    #[doc = "Bit 15 - Dither input selector"]
    #[inline]
    pub fn dim(&mut self) -> _DIMW {
        _DIMW { w: self }
    }
    #[doc = "Bit 16 - Floating Prescaler enable"]
    #[inline]
    pub fn fpe(&mut self) -> _FPEW {
        _FPEW { w: self }
    }
    #[doc = "Bit 17 - TRAP enable"]
    #[inline]
    pub fn trape(&mut self) -> _TRAPEW {
        _TRAPEW { w: self }
    }
    #[doc = "Bit 21 - TRAP Synchronization Enable"]
    #[inline]
    pub fn trpse(&mut self) -> _TRPSEW {
        _TRPSEW { w: self }
    }
    #[doc = "Bit 22 - TRAP State Clear Control"]
    #[inline]
    pub fn trpsw(&mut self) -> _TRPSWW {
        _TRPSWW { w: self }
    }
    #[doc = "Bit 23 - External Modulation Synchronization"]
    #[inline]
    pub fn ems(&mut self) -> _EMSW {
        _EMSW { w: self }
    }
    #[doc = "Bit 24 - External Modulation Type"]
    #[inline]
    pub fn emt(&mut self) -> _EMTW {
        _EMTW { w: self }
    }
    #[doc = "Bit 25 - Multi Channel Mode Enable"]
    #[inline]
    pub fn mcme(&mut self) -> _MCMEW {
        _MCMEW { w: self }
    }
}
