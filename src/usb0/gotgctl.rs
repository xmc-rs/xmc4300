#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GOTGCTL {
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
#[doc = "Possible values of the field `SesReqScs`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESREQSCSR {
    #[doc = "Session request failure"]
    VALUE1,
    #[doc = "Session request success"]
    VALUE2,
}
impl SESREQSCSR {
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
            SESREQSCSR::VALUE1 => false,
            SESREQSCSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SESREQSCSR {
        match value {
            false => SESREQSCSR::VALUE1,
            true => SESREQSCSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SESREQSCSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SESREQSCSR::VALUE2
    }
}
#[doc = "Possible values of the field `SesReq`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESREQR {
    #[doc = "No session request"]
    VALUE1,
    #[doc = "Session request"]
    VALUE2,
}
impl SESREQR {
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
            SESREQR::VALUE1 => false,
            SESREQR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SESREQR {
        match value {
            false => SESREQR::VALUE1,
            true => SESREQR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SESREQR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SESREQR::VALUE2
    }
}
#[doc = "Possible values of the field `VbvalidOvEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBVALIDOVENR {
    #[doc = "Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    VALUE1,
    #[doc = "Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    VALUE2,
}
impl VBVALIDOVENR {
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
            VBVALIDOVENR::VALUE1 => false,
            VBVALIDOVENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBVALIDOVENR {
        match value {
            false => VBVALIDOVENR::VALUE1,
            true => VBVALIDOVENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBVALIDOVENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VBVALIDOVENR::VALUE2
    }
}
#[doc = "Possible values of the field `VbvalidOvVal`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBVALIDOVVALR {
    #[doc = "vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    VALUE1,
    #[doc = "vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    VALUE2,
}
impl VBVALIDOVVALR {
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
            VBVALIDOVVALR::VALUE1 => false,
            VBVALIDOVVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBVALIDOVVALR {
        match value {
            false => VBVALIDOVVALR::VALUE1,
            true => VBVALIDOVVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBVALIDOVVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VBVALIDOVVALR::VALUE2
    }
}
#[doc = "Possible values of the field `AvalidOvEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVALIDOVENR {
    #[doc = "Override is disabled and Avalid signal from the PHY is used internally by the core."]
    VALUE1,
    #[doc = "Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    VALUE2,
}
impl AVALIDOVENR {
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
            AVALIDOVENR::VALUE1 => false,
            AVALIDOVENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVALIDOVENR {
        match value {
            false => AVALIDOVENR::VALUE1,
            true => AVALIDOVENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVALIDOVENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVALIDOVENR::VALUE2
    }
}
#[doc = "Possible values of the field `AvalidOvVal`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVALIDOVVALR {
    #[doc = "Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    VALUE1,
    #[doc = "Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    VALUE2,
}
impl AVALIDOVVALR {
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
            AVALIDOVVALR::VALUE1 => false,
            AVALIDOVVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVALIDOVVALR {
        match value {
            false => AVALIDOVVALR::VALUE1,
            true => AVALIDOVVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AVALIDOVVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AVALIDOVVALR::VALUE2
    }
}
#[doc = "Possible values of the field `BvalidOvEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BVALIDOVENR {
    #[doc = "Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    VALUE1,
    #[doc = "Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    VALUE2,
}
impl BVALIDOVENR {
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
            BVALIDOVENR::VALUE1 => false,
            BVALIDOVENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BVALIDOVENR {
        match value {
            false => BVALIDOVENR::VALUE1,
            true => BVALIDOVENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BVALIDOVENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BVALIDOVENR::VALUE2
    }
}
#[doc = "Possible values of the field `BvalidOvVal`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BVALIDOVVALR {
    #[doc = "Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    VALUE1,
    #[doc = "Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    VALUE2,
}
impl BVALIDOVVALR {
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
            BVALIDOVVALR::VALUE1 => false,
            BVALIDOVVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BVALIDOVVALR {
        match value {
            false => BVALIDOVVALR::VALUE1,
            true => BVALIDOVVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BVALIDOVVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BVALIDOVVALR::VALUE2
    }
}
#[doc = "Possible values of the field `HstNegScs`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSTNEGSCSR {
    #[doc = "Host negotiation failure"]
    VALUE1,
    #[doc = "Host negotiation success"]
    VALUE2,
}
impl HSTNEGSCSR {
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
            HSTNEGSCSR::VALUE1 => false,
            HSTNEGSCSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSTNEGSCSR {
        match value {
            false => HSTNEGSCSR::VALUE1,
            true => HSTNEGSCSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HSTNEGSCSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HSTNEGSCSR::VALUE2
    }
}
#[doc = "Possible values of the field `HNPReq`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HNPREQR {
    #[doc = "No HNP request"]
    VALUE1,
    #[doc = "HNP request"]
    VALUE2,
}
impl HNPREQR {
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
            HNPREQR::VALUE1 => false,
            HNPREQR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HNPREQR {
        match value {
            false => HNPREQR::VALUE1,
            true => HNPREQR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HNPREQR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HNPREQR::VALUE2
    }
}
#[doc = "Possible values of the field `HstSetHNPEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSTSETHNPENR {
    #[doc = "Host Set HNP is not enabled"]
    VALUE1,
    #[doc = "Host Set HNP is enabled"]
    VALUE2,
}
impl HSTSETHNPENR {
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
            HSTSETHNPENR::VALUE1 => false,
            HSTSETHNPENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSTSETHNPENR {
        match value {
            false => HSTSETHNPENR::VALUE1,
            true => HSTSETHNPENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HSTSETHNPENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HSTSETHNPENR::VALUE2
    }
}
#[doc = "Possible values of the field `DevHNPEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVHNPENR {
    #[doc = "HNP is not enabled in the application"]
    VALUE1,
    #[doc = "HNP is enabled in the application"]
    VALUE2,
}
impl DEVHNPENR {
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
            DEVHNPENR::VALUE1 => false,
            DEVHNPENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEVHNPENR {
        match value {
            false => DEVHNPENR::VALUE1,
            true => DEVHNPENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DEVHNPENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DEVHNPENR::VALUE2
    }
}
#[doc = "Possible values of the field `ConlDSts`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONLDSTSR {
    #[doc = "The USB core is in A-Device mode"]
    VALUE1,
    #[doc = "The USB core is in B-Device mode"]
    VALUE2,
}
impl CONLDSTSR {
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
            CONLDSTSR::VALUE1 => false,
            CONLDSTSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONLDSTSR {
        match value {
            false => CONLDSTSR::VALUE1,
            true => CONLDSTSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CONLDSTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CONLDSTSR::VALUE2
    }
}
#[doc = "Possible values of the field `DbncTime`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBNCTIMER {
    #[doc = "Long debounce time, used for physical connections (100 ms + 2.5 us)"]
    VALUE1,
    #[doc = "Short debounce time, used for soft connections (2.5 us)"]
    VALUE2,
}
impl DBNCTIMER {
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
            DBNCTIMER::VALUE1 => false,
            DBNCTIMER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBNCTIMER {
        match value {
            false => DBNCTIMER::VALUE1,
            true => DBNCTIMER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DBNCTIMER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DBNCTIMER::VALUE2
    }
}
#[doc = "Possible values of the field `ASesVId`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASESVIDR {
    #[doc = "A-session is not valid"]
    VALUE1,
    #[doc = "A-session is valid"]
    VALUE2,
}
impl ASESVIDR {
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
            ASESVIDR::VALUE1 => false,
            ASESVIDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASESVIDR {
        match value {
            false => ASESVIDR::VALUE1,
            true => ASESVIDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ASESVIDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ASESVIDR::VALUE2
    }
}
#[doc = "Possible values of the field `BSesVld`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSESVLDR {
    #[doc = "B-session is not valid."]
    VALUE1,
    #[doc = "B-session is valid."]
    VALUE2,
}
impl BSESVLDR {
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
            BSESVLDR::VALUE1 => false,
            BSESVLDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSESVLDR {
        match value {
            false => BSESVLDR::VALUE1,
            true => BSESVLDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BSESVLDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BSESVLDR::VALUE2
    }
}
#[doc = "Possible values of the field `OTGVer`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGVERR {
    #[doc = "OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP."]
    VALUE1,
    #[doc = "OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    VALUE2,
}
impl OTGVERR {
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
            OTGVERR::VALUE1 => false,
            OTGVERR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGVERR {
        match value {
            false => OTGVERR::VALUE1,
            true => OTGVERR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OTGVERR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OTGVERR::VALUE2
    }
}
#[doc = "Values that can be written to the field `SesReq`"]
pub enum SESREQW {
    #[doc = "No session request"]
    VALUE1,
    #[doc = "Session request"]
    VALUE2,
}
impl SESREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SESREQW::VALUE1 => false,
            SESREQW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SESREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SESREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SESREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No session request"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SESREQW::VALUE1)
    }
    #[doc = "Session request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SESREQW::VALUE2)
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
#[doc = "Values that can be written to the field `VbvalidOvEn`"]
pub enum VBVALIDOVENW {
    #[doc = "Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    VALUE1,
    #[doc = "Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    VALUE2,
}
impl VBVALIDOVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBVALIDOVENW::VALUE1 => false,
            VBVALIDOVENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBVALIDOVENW<'a> {
    w: &'a mut W,
}
impl<'a> _VBVALIDOVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBVALIDOVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBVALIDOVENW::VALUE1)
    }
    #[doc = "Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBVALIDOVENW::VALUE2)
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
#[doc = "Values that can be written to the field `VbvalidOvVal`"]
pub enum VBVALIDOVVALW {
    #[doc = "vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    VALUE1,
    #[doc = "vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    VALUE2,
}
impl VBVALIDOVVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBVALIDOVVALW::VALUE1 => false,
            VBVALIDOVVALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBVALIDOVVALW<'a> {
    w: &'a mut W,
}
impl<'a> _VBVALIDOVVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBVALIDOVVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBVALIDOVVALW::VALUE1)
    }
    #[doc = "vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBVALIDOVVALW::VALUE2)
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
#[doc = "Values that can be written to the field `AvalidOvEn`"]
pub enum AVALIDOVENW {
    #[doc = "Override is disabled and Avalid signal from the PHY is used internally by the core."]
    VALUE1,
    #[doc = "Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    VALUE2,
}
impl AVALIDOVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVALIDOVENW::VALUE1 => false,
            AVALIDOVENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVALIDOVENW<'a> {
    w: &'a mut W,
}
impl<'a> _AVALIDOVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVALIDOVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Override is disabled and Avalid signal from the PHY is used internally by the core."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AVALIDOVENW::VALUE1)
    }
    #[doc = "Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AVALIDOVENW::VALUE2)
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
#[doc = "Values that can be written to the field `AvalidOvVal`"]
pub enum AVALIDOVVALW {
    #[doc = "Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    VALUE1,
    #[doc = "Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    VALUE2,
}
impl AVALIDOVVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVALIDOVVALW::VALUE1 => false,
            AVALIDOVVALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVALIDOVVALW<'a> {
    w: &'a mut W,
}
impl<'a> _AVALIDOVVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVALIDOVVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AVALIDOVVALW::VALUE1)
    }
    #[doc = "Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AVALIDOVVALW::VALUE2)
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
#[doc = "Values that can be written to the field `BvalidOvEn`"]
pub enum BVALIDOVENW {
    #[doc = "Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    VALUE1,
    #[doc = "Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    VALUE2,
}
impl BVALIDOVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BVALIDOVENW::VALUE1 => false,
            BVALIDOVENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BVALIDOVENW<'a> {
    w: &'a mut W,
}
impl<'a> _BVALIDOVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BVALIDOVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BVALIDOVENW::VALUE1)
    }
    #[doc = "Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BVALIDOVENW::VALUE2)
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
#[doc = "Values that can be written to the field `BvalidOvVal`"]
pub enum BVALIDOVVALW {
    #[doc = "Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    VALUE1,
    #[doc = "Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    VALUE2,
}
impl BVALIDOVVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BVALIDOVVALW::VALUE1 => false,
            BVALIDOVVALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BVALIDOVVALW<'a> {
    w: &'a mut W,
}
impl<'a> _BVALIDOVVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BVALIDOVVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BVALIDOVVALW::VALUE1)
    }
    #[doc = "Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BVALIDOVVALW::VALUE2)
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
#[doc = "Values that can be written to the field `HNPReq`"]
pub enum HNPREQW {
    #[doc = "No HNP request"]
    VALUE1,
    #[doc = "HNP request"]
    VALUE2,
}
impl HNPREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HNPREQW::VALUE1 => false,
            HNPREQW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HNPREQW<'a> {
    w: &'a mut W,
}
impl<'a> _HNPREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HNPREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No HNP request"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HNPREQW::VALUE1)
    }
    #[doc = "HNP request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HNPREQW::VALUE2)
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
#[doc = "Values that can be written to the field `HstSetHNPEn`"]
pub enum HSTSETHNPENW {
    #[doc = "Host Set HNP is not enabled"]
    VALUE1,
    #[doc = "Host Set HNP is enabled"]
    VALUE2,
}
impl HSTSETHNPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSTSETHNPENW::VALUE1 => false,
            HSTSETHNPENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSTSETHNPENW<'a> {
    w: &'a mut W,
}
impl<'a> _HSTSETHNPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSTSETHNPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Host Set HNP is not enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HSTSETHNPENW::VALUE1)
    }
    #[doc = "Host Set HNP is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HSTSETHNPENW::VALUE2)
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
#[doc = "Values that can be written to the field `DevHNPEn`"]
pub enum DEVHNPENW {
    #[doc = "HNP is not enabled in the application"]
    VALUE1,
    #[doc = "HNP is enabled in the application"]
    VALUE2,
}
impl DEVHNPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEVHNPENW::VALUE1 => false,
            DEVHNPENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEVHNPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVHNPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEVHNPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HNP is not enabled in the application"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEVHNPENW::VALUE1)
    }
    #[doc = "HNP is enabled in the application"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DEVHNPENW::VALUE2)
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
#[doc = "Values that can be written to the field `OTGVer`"]
pub enum OTGVERW {
    #[doc = "OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP."]
    VALUE1,
    #[doc = "OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    VALUE2,
}
impl OTGVERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGVERW::VALUE1 => false,
            OTGVERW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGVERW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGVERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGVERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OTGVERW::VALUE1)
    }
    #[doc = "OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OTGVERW::VALUE2)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Session Request Success"]
    #[inline]
    pub fn ses_req_scs(&self) -> SESREQSCSR {
        SESREQSCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Session Request"]
    #[inline]
    pub fn ses_req(&self) -> SESREQR {
        SESREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline]
    pub fn vbvalid_ov_en(&self) -> VBVALIDOVENR {
        VBVALIDOVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline]
    pub fn vbvalid_ov_val(&self) -> VBVALIDOVVALR {
        VBVALIDOVVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline]
    pub fn avalid_ov_en(&self) -> AVALIDOVENR {
        AVALIDOVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid Override Value"]
    #[inline]
    pub fn avalid_ov_val(&self) -> AVALIDOVVALR {
        AVALIDOVVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline]
    pub fn bvalid_ov_en(&self) -> BVALIDOVENR {
        BVALIDOVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid Override Value"]
    #[inline]
    pub fn bvalid_ov_val(&self) -> BVALIDOVVALR {
        BVALIDOVVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Host Negotiation Success"]
    #[inline]
    pub fn hst_neg_scs(&self) -> HSTNEGSCSR {
        HSTNEGSCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline]
    pub fn hnpreq(&self) -> HNPREQR {
        HNPREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline]
    pub fn hst_set_hnpen(&self) -> HSTSETHNPENR {
        HSTSETHNPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline]
    pub fn dev_hnpen(&self) -> DEVHNPENR {
        DEVHNPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Connector ID Status"]
    #[inline]
    pub fn conl_dsts(&self) -> CONLDSTSR {
        CONLDSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Long/Short Debounce Time"]
    #[inline]
    pub fn dbnc_time(&self) -> DBNCTIMER {
        DBNCTIMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - A-Session Valid"]
    #[inline]
    pub fn ases_vid(&self) -> ASESVIDR {
        ASESVIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - B-Session Valid"]
    #[inline]
    pub fn bses_vld(&self) -> BSESVLDR {
        BSESVLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline]
    pub fn otgver(&self) -> OTGVERR {
        OTGVERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65536 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Session Request"]
    #[inline]
    pub fn ses_req(&mut self) -> _SESREQW {
        _SESREQW { w: self }
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline]
    pub fn vbvalid_ov_en(&mut self) -> _VBVALIDOVENW {
        _VBVALIDOVENW { w: self }
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline]
    pub fn vbvalid_ov_val(&mut self) -> _VBVALIDOVVALW {
        _VBVALIDOVVALW { w: self }
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline]
    pub fn avalid_ov_en(&mut self) -> _AVALIDOVENW {
        _AVALIDOVENW { w: self }
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid Override Value"]
    #[inline]
    pub fn avalid_ov_val(&mut self) -> _AVALIDOVVALW {
        _AVALIDOVVALW { w: self }
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline]
    pub fn bvalid_ov_en(&mut self) -> _BVALIDOVENW {
        _BVALIDOVENW { w: self }
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid Override Value"]
    #[inline]
    pub fn bvalid_ov_val(&mut self) -> _BVALIDOVVALW {
        _BVALIDOVVALW { w: self }
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline]
    pub fn hnpreq(&mut self) -> _HNPREQW {
        _HNPREQW { w: self }
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline]
    pub fn hst_set_hnpen(&mut self) -> _HSTSETHNPENW {
        _HSTSETHNPENW { w: self }
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline]
    pub fn dev_hnpen(&mut self) -> _DEVHNPENW {
        _DEVHNPENW { w: self }
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline]
    pub fn otgver(&mut self) -> _OTGVERW {
        _OTGVERW { w: self }
    }
}
