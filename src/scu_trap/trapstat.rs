#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRAPSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SOSCWDGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCWDGTR {
    #[doc = "No pending trap request"]
    CONST_0,
    #[doc = "Pending trap request"]
    CONST_1,
}
impl SOSCWDGTR {
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
            SOSCWDGTR::CONST_0 => false,
            SOSCWDGTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOSCWDGTR {
        match value {
            false => SOSCWDGTR::CONST_0,
            true => SOSCWDGTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SOSCWDGTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SOSCWDGTR::CONST_1
    }
}
#[doc = "Possible values of the field `SVCOLCKT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCOLCKTR {
    #[doc = "No pending trap request"]
    CONST_0,
    #[doc = "Pending trap request"]
    CONST_1,
}
impl SVCOLCKTR {
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
            SVCOLCKTR::CONST_0 => false,
            SVCOLCKTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVCOLCKTR {
        match value {
            false => SVCOLCKTR::CONST_0,
            true => SVCOLCKTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SVCOLCKTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SVCOLCKTR::CONST_1
    }
}
#[doc = "Possible values of the field `UVCOLCKT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVCOLCKTR {
    #[doc = "No pending trap request"]
    CONST_0,
    #[doc = "Pending trap request"]
    CONST_1,
}
impl UVCOLCKTR {
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
            UVCOLCKTR::CONST_0 => false,
            UVCOLCKTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UVCOLCKTR {
        match value {
            false => UVCOLCKTR::CONST_0,
            true => UVCOLCKTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == UVCOLCKTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == UVCOLCKTR::CONST_1
    }
}
#[doc = "Possible values of the field `PET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETR {
    #[doc = "No pending trap request"]
    CONST_0,
    #[doc = "Pending trap request"]
    CONST_1,
}
impl PETR {
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
            PETR::CONST_0 => false,
            PETR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETR {
        match value {
            false => PETR::CONST_0,
            true => PETR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PETR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PETR::CONST_1
    }
}
#[doc = "Possible values of the field `BRWNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRWNTR {
    #[doc = "No pending trap request"]
    CONST_0,
    #[doc = "Pending trap request"]
    CONST_1,
}
impl BRWNTR {
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
            BRWNTR::CONST_0 => false,
            BRWNTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRWNTR {
        match value {
            false => BRWNTR::CONST_0,
            true => BRWNTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == BRWNTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == BRWNTR::CONST_1
    }
}
#[doc = "Possible values of the field `ULPWDGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGTR {
    #[doc = "No pending trap request"]
    CONST_0,
    #[doc = "Pending trap request"]
    CONST_1,
}
impl ULPWDGTR {
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
            ULPWDGTR::CONST_0 => false,
            ULPWDGTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULPWDGTR {
        match value {
            false => ULPWDGTR::CONST_0,
            true => ULPWDGTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDGTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDGTR::CONST_1
    }
}
#[doc = "Possible values of the field `BWERR0T`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR0TR {
    #[doc = "No pending trap request"]
    CONST_0,
    #[doc = "Pending trap request"]
    CONST_1,
}
impl BWERR0TR {
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
            BWERR0TR::CONST_0 => false,
            BWERR0TR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWERR0TR {
        match value {
            false => BWERR0TR::CONST_0,
            true => BWERR0TR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR0TR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR0TR::CONST_1
    }
}
#[doc = "Possible values of the field `BWERR1T`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR1TR {
    #[doc = "No pending trap request"]
    CONST_0,
    #[doc = "Pending trap request"]
    CONST_1,
}
impl BWERR1TR {
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
            BWERR1TR::CONST_0 => false,
            BWERR1TR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWERR1TR {
        match value {
            false => BWERR1TR::CONST_0,
            true => BWERR1TR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR1TR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR1TR::CONST_1
    }
}
#[doc = "Possible values of the field `ECAT0RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RSTR {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Pending trap request"]
    CONST_1,
}
impl ECAT0RSTR {
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
            ECAT0RSTR::CONST_0 => false,
            ECAT0RSTR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECAT0RSTR {
        match value {
            false => ECAT0RSTR::CONST_0,
            true => ECAT0RSTR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ECAT0RSTR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ECAT0RSTR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Status"]
    #[inline]
    pub fn soscwdgt(&self) -> SOSCWDGTR {
        SOSCWDGTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - System VCO Lock Trap Status"]
    #[inline]
    pub fn svcolckt(&self) -> SVCOLCKTR {
        SVCOLCKTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Status"]
    #[inline]
    pub fn uvcolckt(&self) -> UVCOLCKTR {
        UVCOLCKTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Parity Error Trap Status"]
    #[inline]
    pub fn pet(&self) -> PETR {
        PETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Brown Out Trap Status"]
    #[inline]
    pub fn brwnt(&self) -> BRWNTR {
        BRWNTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Status"]
    #[inline]
    pub fn ulpwdgt(&self) -> ULPWDGTR {
        ULPWDGTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Status"]
    #[inline]
    pub fn bwerr0t(&self) -> BWERR0TR {
        BWERR0TR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Status"]
    #[inline]
    pub fn bwerr1t(&self) -> BWERR1TR {
        BWERR1TR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Status"]
    #[inline]
    pub fn ecat0rst(&self) -> ECAT0RSTR {
        ECAT0RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
