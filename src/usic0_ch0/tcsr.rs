#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCSR {
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
#[doc = "Possible values of the field `WLEMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLEMDR {
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    VALUE1,
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    VALUE2,
}
impl WLEMDR {
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
            WLEMDR::VALUE1 => false,
            WLEMDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WLEMDR {
        match value {
            false => WLEMDR::VALUE1,
            true => WLEMDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WLEMDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WLEMDR::VALUE2
    }
}
#[doc = "Possible values of the field `SELMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELMDR {
    #[doc = "The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    VALUE1,
    #[doc = "The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    VALUE2,
}
impl SELMDR {
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
            SELMDR::VALUE1 => false,
            SELMDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELMDR {
        match value {
            false => SELMDR::VALUE1,
            true => SELMDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELMDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELMDR::VALUE2
    }
}
#[doc = "Possible values of the field `FLEMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEMDR {
    #[doc = "The automatic update of FLE is disabled."]
    VALUE1,
    #[doc = "The automatic update of FLE is enabled."]
    VALUE2,
}
impl FLEMDR {
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
            FLEMDR::VALUE1 => false,
            FLEMDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEMDR {
        match value {
            false => FLEMDR::VALUE1,
            true => FLEMDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FLEMDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FLEMDR::VALUE2
    }
}
#[doc = "Possible values of the field `WAMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAMDR {
    #[doc = "The automatic update of bit WA is disabled."]
    VALUE1,
    #[doc = "The automatic update of bit WA is enabled."]
    VALUE2,
}
impl WAMDR {
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
            WAMDR::VALUE1 => false,
            WAMDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAMDR {
        match value {
            false => WAMDR::VALUE1,
            true => WAMDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAMDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAMDR::VALUE2
    }
}
#[doc = "Possible values of the field `HPCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPCMDR {
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    VALUE1,
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    VALUE2,
}
impl HPCMDR {
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
            HPCMDR::VALUE1 => false,
            HPCMDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPCMDR {
        match value {
            false => HPCMDR::VALUE1,
            true => HPCMDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HPCMDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HPCMDR::VALUE2
    }
}
#[doc = "Possible values of the field `SOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFR {
    #[doc = "The data word in TBUF is not considered as first word of a frame."]
    VALUE1,
    #[doc = "The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    VALUE2,
}
impl SOFR {
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
            SOFR::VALUE1 => false,
            SOFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFR {
        match value {
            false => SOFR::VALUE1,
            true => SOFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SOFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SOFR::VALUE2
    }
}
#[doc = "Possible values of the field `EOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOFR {
    #[doc = "The data word in TBUF is not considered as last word of an SSC frame."]
    VALUE1,
    #[doc = "The data word in TBUF is considered as last word of an SSC frame."]
    VALUE2,
}
impl EOFR {
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
            EOFR::VALUE1 => false,
            EOFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOFR {
        match value {
            false => EOFR::VALUE1,
            true => EOFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EOFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EOFR::VALUE2
    }
}
#[doc = "Possible values of the field `TDV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDVR {
    #[doc = "The data word in TBUF is not valid for transmission."]
    VALUE1,
    #[doc = "The data word in TBUF is valid for transmission and a transmission start is possible. New data should not be written to a TBUFx input location while TDV = 1."]
    VALUE2,
}
impl TDVR {
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
            TDVR::VALUE1 => false,
            TDVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDVR {
        match value {
            false => TDVR::VALUE1,
            true => TDVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TDVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TDVR::VALUE2
    }
}
#[doc = "Possible values of the field `TDSSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDSSMR {
    #[doc = "The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    VALUE1,
    #[doc = "The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    VALUE2,
}
impl TDSSMR {
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
            TDSSMR::VALUE1 => false,
            TDSSMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDSSMR {
        match value {
            false => TDSSMR::VALUE1,
            true => TDSSMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TDSSMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TDSSMR::VALUE2
    }
}
#[doc = "Possible values of the field `TDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDENR {
    #[doc = "A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    VALUE1,
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1."]
    VALUE2,
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    VALUE3,
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    VALUE4,
}
impl TDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TDENR::VALUE1 => 0,
            TDENR::VALUE2 => 1,
            TDENR::VALUE3 => 2,
            TDENR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TDENR {
        match value {
            0 => TDENR::VALUE1,
            1 => TDENR::VALUE2,
            2 => TDENR::VALUE3,
            3 => TDENR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TDENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TDENR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TDENR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TDENR::VALUE4
    }
}
#[doc = "Possible values of the field `TDVTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDVTRR {
    #[doc = "Bit TCSR.TE is permanently set."]
    VALUE1,
    #[doc = "Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    VALUE2,
}
impl TDVTRR {
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
            TDVTRR::VALUE1 => false,
            TDVTRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDVTRR {
        match value {
            false => TDVTRR::VALUE1,
            true => TDVTRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TDVTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TDVTRR::VALUE2
    }
}
#[doc = "Possible values of the field `WA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAR {
    #[doc = "The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    VALUE1,
    #[doc = "The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    VALUE2,
}
impl WAR {
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
            WAR::VALUE1 => false,
            WAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAR {
        match value {
            false => WAR::VALUE1,
            true => WAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAR::VALUE2
    }
}
#[doc = "Possible values of the field `TSOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOFR {
    #[doc = "The latest data word transmission has not been started for the first word of a data frame."]
    VALUE1,
    #[doc = "The latest data word transmission has been started for the first word of a data frame."]
    VALUE2,
}
impl TSOFR {
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
            TSOFR::VALUE1 => false,
            TSOFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSOFR {
        match value {
            false => TSOFR::VALUE1,
            true => TSOFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSOFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSOFR::VALUE2
    }
}
#[doc = "Possible values of the field `TV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TVR {
    #[doc = "The latest start of a data word transmission has taken place while no valid data was available. As a result, the transmission of a data words with passive level (SCTR.PDL) has been started."]
    VALUE1,
    #[doc = "The latest start of a data word transmission has taken place with valid data from TBUF."]
    VALUE2,
}
impl TVR {
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
            TVR::VALUE1 => false,
            TVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TVR {
        match value {
            false => TVR::VALUE1,
            true => TVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TVR::VALUE2
    }
}
#[doc = "Possible values of the field `TVC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TVCR {
    #[doc = "Since TVC has been set, at least one data buffer underflow condition has occurred."]
    VALUE1,
    #[doc = "Since TVC has been set, no data buffer underflow condition has occurred."]
    VALUE2,
}
impl TVCR {
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
            TVCR::VALUE1 => false,
            TVCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TVCR {
        match value {
            false => TVCR::VALUE1,
            true => TVCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TVCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TVCR::VALUE2
    }
}
#[doc = "Possible values of the field `TE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TER {
    #[doc = "The trigger event has not yet been detected. A transmission of the data word in TBUF can not be started."]
    VALUE1,
    #[doc = "The trigger event has been detected (or the trigger mechanism is switched off) and a transmission of the data word in TBUF can not be started."]
    VALUE2,
}
impl TER {
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
            TER::VALUE1 => false,
            TER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TER {
        match value {
            false => TER::VALUE1,
            true => TER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TER::VALUE2
    }
}
#[doc = "Values that can be written to the field `WLEMD`"]
pub enum WLEMDW {
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    VALUE1,
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    VALUE2,
}
impl WLEMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WLEMDW::VALUE1 => false,
            WLEMDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WLEMDW<'a> {
    w: &'a mut W,
}
impl<'a> _WLEMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WLEMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WLEMDW::VALUE1)
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WLEMDW::VALUE2)
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
#[doc = "Values that can be written to the field `SELMD`"]
pub enum SELMDW {
    #[doc = "The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    VALUE1,
    #[doc = "The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    VALUE2,
}
impl SELMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELMDW::VALUE1 => false,
            SELMDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELMDW<'a> {
    w: &'a mut W,
}
impl<'a> _SELMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELMDW::VALUE1)
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELMDW::VALUE2)
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
#[doc = "Values that can be written to the field `FLEMD`"]
pub enum FLEMDW {
    #[doc = "The automatic update of FLE is disabled."]
    VALUE1,
    #[doc = "The automatic update of FLE is enabled."]
    VALUE2,
}
impl FLEMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEMDW::VALUE1 => false,
            FLEMDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEMDW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The automatic update of FLE is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLEMDW::VALUE1)
    }
    #[doc = "The automatic update of FLE is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLEMDW::VALUE2)
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
#[doc = "Values that can be written to the field `WAMD`"]
pub enum WAMDW {
    #[doc = "The automatic update of bit WA is disabled."]
    VALUE1,
    #[doc = "The automatic update of bit WA is enabled."]
    VALUE2,
}
impl WAMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAMDW::VALUE1 => false,
            WAMDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAMDW<'a> {
    w: &'a mut W,
}
impl<'a> _WAMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The automatic update of bit WA is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAMDW::VALUE1)
    }
    #[doc = "The automatic update of bit WA is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAMDW::VALUE2)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPCMD`"]
pub enum HPCMDW {
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    VALUE1,
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    VALUE2,
}
impl HPCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPCMDW::VALUE1 => false,
            HPCMDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _HPCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPCMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HPCMDW::VALUE1)
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HPCMDW::VALUE2)
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
#[doc = "Values that can be written to the field `SOF`"]
pub enum SOFW {
    #[doc = "The data word in TBUF is not considered as first word of a frame."]
    VALUE1,
    #[doc = "The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    VALUE2,
}
impl SOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFW::VALUE1 => false,
            SOFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data word in TBUF is not considered as first word of a frame."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SOFW::VALUE1)
    }
    #[doc = "The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SOFW::VALUE2)
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
#[doc = "Values that can be written to the field `EOF`"]
pub enum EOFW {
    #[doc = "The data word in TBUF is not considered as last word of an SSC frame."]
    VALUE1,
    #[doc = "The data word in TBUF is considered as last word of an SSC frame."]
    VALUE2,
}
impl EOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOFW::VALUE1 => false,
            EOFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOFW<'a> {
    w: &'a mut W,
}
impl<'a> _EOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data word in TBUF is not considered as last word of an SSC frame."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EOFW::VALUE1)
    }
    #[doc = "The data word in TBUF is considered as last word of an SSC frame."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EOFW::VALUE2)
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
#[doc = "Values that can be written to the field `TDSSM`"]
pub enum TDSSMW {
    #[doc = "The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    VALUE1,
    #[doc = "The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    VALUE2,
}
impl TDSSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDSSMW::VALUE1 => false,
            TDSSMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDSSMW<'a> {
    w: &'a mut W,
}
impl<'a> _TDSSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDSSMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TDSSMW::VALUE1)
    }
    #[doc = "The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TDSSMW::VALUE2)
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
#[doc = "Values that can be written to the field `TDEN`"]
pub enum TDENW {
    #[doc = "A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    VALUE1,
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1."]
    VALUE2,
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    VALUE3,
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    VALUE4,
}
impl TDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TDENW::VALUE1 => 0,
            TDENW::VALUE2 => 1,
            TDENW::VALUE3 => 2,
            TDENW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDENW<'a> {
    w: &'a mut W,
}
impl<'a> _TDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TDENW::VALUE1)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TDENW::VALUE2)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TDENW::VALUE3)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TDENW::VALUE4)
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
#[doc = "Values that can be written to the field `TDVTR`"]
pub enum TDVTRW {
    #[doc = "Bit TCSR.TE is permanently set."]
    VALUE1,
    #[doc = "Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    VALUE2,
}
impl TDVTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDVTRW::VALUE1 => false,
            TDVTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDVTRW<'a> {
    w: &'a mut W,
}
impl<'a> _TDVTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDVTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit TCSR.TE is permanently set."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TDVTRW::VALUE1)
    }
    #[doc = "Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TDVTRW::VALUE2)
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
#[doc = "Values that can be written to the field `WA`"]
pub enum WAW {
    #[doc = "The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    VALUE1,
    #[doc = "The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    VALUE2,
}
impl WAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAW::VALUE1 => false,
            WAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAW<'a> {
    w: &'a mut W,
}
impl<'a> _WAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAW::VALUE1)
    }
    #[doc = "The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - WLE Mode"]
    #[inline]
    pub fn wlemd(&self) -> WLEMDR {
        WLEMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Select Mode"]
    #[inline]
    pub fn selmd(&self) -> SELMDR {
        SELMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FLE Mode"]
    #[inline]
    pub fn flemd(&self) -> FLEMDR {
        FLEMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - WA Mode"]
    #[inline]
    pub fn wamd(&self) -> WAMDR {
        WAMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Hardware Port Control Mode"]
    #[inline]
    pub fn hpcmd(&self) -> HPCMDR {
        HPCMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Start Of Frame"]
    #[inline]
    pub fn sof(&self) -> SOFR {
        SOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - End Of Frame"]
    #[inline]
    pub fn eof(&self) -> EOFR {
        EOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transmit Data Valid"]
    #[inline]
    pub fn tdv(&self) -> TDVR {
        TDVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - TBUF Data Single Shot Mode"]
    #[inline]
    pub fn tdssm(&self) -> TDSSMR {
        TDSSMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - TBUF Data Enable"]
    #[inline]
    pub fn tden(&self) -> TDENR {
        TDENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - TBUF Data Valid Trigger"]
    #[inline]
    pub fn tdvtr(&self) -> TDVTRR {
        TDVTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Word Address"]
    #[inline]
    pub fn wa(&self) -> WAR {
        WAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Transmitted Start Of Frame"]
    #[inline]
    pub fn tsof(&self) -> TSOFR {
        TSOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Transmission Valid"]
    #[inline]
    pub fn tv(&self) -> TVR {
        TVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Transmission Valid Cumulated"]
    #[inline]
    pub fn tvc(&self) -> TVCR {
        TVCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Trigger Event"]
    #[inline]
    pub fn te(&self) -> TER {
        TER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - WLE Mode"]
    #[inline]
    pub fn wlemd(&mut self) -> _WLEMDW {
        _WLEMDW { w: self }
    }
    #[doc = "Bit 1 - Select Mode"]
    #[inline]
    pub fn selmd(&mut self) -> _SELMDW {
        _SELMDW { w: self }
    }
    #[doc = "Bit 2 - FLE Mode"]
    #[inline]
    pub fn flemd(&mut self) -> _FLEMDW {
        _FLEMDW { w: self }
    }
    #[doc = "Bit 3 - WA Mode"]
    #[inline]
    pub fn wamd(&mut self) -> _WAMDW {
        _WAMDW { w: self }
    }
    #[doc = "Bit 4 - Hardware Port Control Mode"]
    #[inline]
    pub fn hpcmd(&mut self) -> _HPCMDW {
        _HPCMDW { w: self }
    }
    #[doc = "Bit 5 - Start Of Frame"]
    #[inline]
    pub fn sof(&mut self) -> _SOFW {
        _SOFW { w: self }
    }
    #[doc = "Bit 6 - End Of Frame"]
    #[inline]
    pub fn eof(&mut self) -> _EOFW {
        _EOFW { w: self }
    }
    #[doc = "Bit 8 - TBUF Data Single Shot Mode"]
    #[inline]
    pub fn tdssm(&mut self) -> _TDSSMW {
        _TDSSMW { w: self }
    }
    #[doc = "Bits 10:11 - TBUF Data Enable"]
    #[inline]
    pub fn tden(&mut self) -> _TDENW {
        _TDENW { w: self }
    }
    #[doc = "Bit 12 - TBUF Data Valid Trigger"]
    #[inline]
    pub fn tdvtr(&mut self) -> _TDVTRW {
        _TDVTRW { w: self }
    }
    #[doc = "Bit 13 - Word Address"]
    #[inline]
    pub fn wa(&mut self) -> _WAW {
        _WAW { w: self }
    }
}
