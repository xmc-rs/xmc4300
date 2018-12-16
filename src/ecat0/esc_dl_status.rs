#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::ESC_DL_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PDI_EEPROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI_EEPROMR {
    #[doc = "EEPROM not loaded, PDI not operational (no access to Process Data RAM)"]
    VALUE1,
    #[doc = "EEPROM loaded correctly, PDI operational (access to Process Data RAM)"]
    VALUE2,
}
impl PDI_EEPROMR {
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
            PDI_EEPROMR::VALUE1 => false,
            PDI_EEPROMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI_EEPROMR {
        match value {
            false => PDI_EEPROMR::VALUE1,
            true => PDI_EEPROMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDI_EEPROMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDI_EEPROMR::VALUE2
    }
}
#[doc = "Possible values of the field `PDI_WDT_S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI_WDT_SR {
    #[doc = "Watchdog expired"]
    VALUE1,
    #[doc = "Watchdog reloaded"]
    VALUE2,
}
impl PDI_WDT_SR {
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
            PDI_WDT_SR::VALUE1 => false,
            PDI_WDT_SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI_WDT_SR {
        match value {
            false => PDI_WDT_SR::VALUE1,
            true => PDI_WDT_SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PDI_WDT_SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PDI_WDT_SR::VALUE2
    }
}
#[doc = "Possible values of the field `ELD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELDR {
    #[doc = "Deactivated for all ports"]
    VALUE1,
    #[doc = "Activated for at least one port"]
    VALUE2,
}
impl ELDR {
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
            ELDR::VALUE1 => false,
            ELDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ELDR {
        match value {
            false => ELDR::VALUE1,
            true => ELDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ELDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ELDR::VALUE2
    }
}
#[doc = "Possible values of the field `LINK_P0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINK_P0R {
    #[doc = "No link"]
    VALUE1,
    #[doc = "Link detected"]
    VALUE2,
}
impl LINK_P0R {
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
            LINK_P0R::VALUE1 => false,
            LINK_P0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINK_P0R {
        match value {
            false => LINK_P0R::VALUE1,
            true => LINK_P0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P0R::VALUE2
    }
}
#[doc = "Possible values of the field `LINK_P1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINK_P1R {
    #[doc = "No link"]
    VALUE1,
    #[doc = "Link detected"]
    VALUE2,
}
impl LINK_P1R {
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
            LINK_P1R::VALUE1 => false,
            LINK_P1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINK_P1R {
        match value {
            false => LINK_P1R::VALUE1,
            true => LINK_P1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P1R::VALUE2
    }
}
#[doc = "Possible values of the field `LINK_P2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINK_P2R {
    #[doc = "No link"]
    VALUE1,
    #[doc = "Link detected"]
    VALUE2,
}
impl LINK_P2R {
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
            LINK_P2R::VALUE1 => false,
            LINK_P2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINK_P2R {
        match value {
            false => LINK_P2R::VALUE1,
            true => LINK_P2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P2R::VALUE2
    }
}
#[doc = "Possible values of the field `LINK_P3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINK_P3R {
    #[doc = "No link"]
    VALUE1,
    #[doc = "Link detected"]
    VALUE2,
}
impl LINK_P3R {
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
            LINK_P3R::VALUE1 => false,
            LINK_P3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINK_P3R {
        match value {
            false => LINK_P3R::VALUE1,
            true => LINK_P3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P3R::VALUE2
    }
}
#[doc = "Possible values of the field `LP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP0R {
    #[doc = "Open"]
    VALUE1,
    #[doc = "Closed"]
    VALUE2,
}
impl LP0R {
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
            LP0R::VALUE1 => false,
            LP0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LP0R {
        match value {
            false => LP0R::VALUE1,
            true => LP0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LP0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LP0R::VALUE2
    }
}
#[doc = "Possible values of the field `COM_P0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_P0R {
    #[doc = "No stable communication"]
    VALUE1,
    #[doc = "Communication established"]
    VALUE2,
}
impl COM_P0R {
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
            COM_P0R::VALUE1 => false,
            COM_P0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COM_P0R {
        match value {
            false => COM_P0R::VALUE1,
            true => COM_P0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COM_P0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COM_P0R::VALUE2
    }
}
#[doc = "Possible values of the field `LP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP1R {
    #[doc = "Open"]
    VALUE1,
    #[doc = "Closed"]
    VALUE2,
}
impl LP1R {
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
            LP1R::VALUE1 => false,
            LP1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LP1R {
        match value {
            false => LP1R::VALUE1,
            true => LP1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LP1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LP1R::VALUE2
    }
}
#[doc = "Possible values of the field `COM_P1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_P1R {
    #[doc = "No stable communication"]
    VALUE1,
    #[doc = "Communication established"]
    VALUE2,
}
impl COM_P1R {
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
            COM_P1R::VALUE1 => false,
            COM_P1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COM_P1R {
        match value {
            false => COM_P1R::VALUE1,
            true => COM_P1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COM_P1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COM_P1R::VALUE2
    }
}
#[doc = "Possible values of the field `LP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP2R {
    #[doc = "Open"]
    VALUE1,
    #[doc = "Closed"]
    VALUE2,
}
impl LP2R {
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
            LP2R::VALUE1 => false,
            LP2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LP2R {
        match value {
            false => LP2R::VALUE1,
            true => LP2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LP2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LP2R::VALUE2
    }
}
#[doc = "Possible values of the field `COM_P2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_P2R {
    #[doc = "No stable communication"]
    VALUE1,
    #[doc = "Communication established"]
    VALUE2,
}
impl COM_P2R {
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
            COM_P2R::VALUE1 => false,
            COM_P2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COM_P2R {
        match value {
            false => COM_P2R::VALUE1,
            true => COM_P2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COM_P2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COM_P2R::VALUE2
    }
}
#[doc = "Possible values of the field `LP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP3R {
    #[doc = "Open"]
    VALUE1,
    #[doc = "Closed"]
    VALUE2,
}
impl LP3R {
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
            LP3R::VALUE1 => false,
            LP3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LP3R {
        match value {
            false => LP3R::VALUE1,
            true => LP3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LP3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LP3R::VALUE2
    }
}
#[doc = "Possible values of the field `COM_P3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_P3R {
    #[doc = "No stable communication"]
    VALUE1,
    #[doc = "Communication established"]
    VALUE2,
}
impl COM_P3R {
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
            COM_P3R::VALUE1 => false,
            COM_P3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COM_P3R {
        match value {
            false => COM_P3R::VALUE1,
            true => COM_P3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COM_P3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COM_P3R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - PDI operational/EEPROM loaded correctly"]
    #[inline]
    pub fn pdi_eeprom(&self) -> PDI_EEPROMR {
        PDI_EEPROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - PDI Watchdog Status"]
    #[inline]
    pub fn pdi_wdt_s(&self) -> PDI_WDT_SR {
        PDI_WDT_SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Enhanced Link detection"]
    #[inline]
    pub fn eld(&self) -> ELDR {
        ELDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Physical link on Port 0"]
    #[inline]
    pub fn link_p0(&self) -> LINK_P0R {
        LINK_P0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Physical link on Port 1"]
    #[inline]
    pub fn link_p1(&self) -> LINK_P1R {
        LINK_P1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Physical link on Port 2"]
    #[inline]
    pub fn link_p2(&self) -> LINK_P2R {
        LINK_P2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Physical link on Port 3"]
    #[inline]
    pub fn link_p3(&self) -> LINK_P3R {
        LINK_P3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Loop Port 0"]
    #[inline]
    pub fn lp0(&self) -> LP0R {
        LP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Communication on Port 0"]
    #[inline]
    pub fn com_p0(&self) -> COM_P0R {
        COM_P0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Loop Port 1"]
    #[inline]
    pub fn lp1(&self) -> LP1R {
        LP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Communication on Port 1"]
    #[inline]
    pub fn com_p1(&self) -> COM_P1R {
        COM_P1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Loop Port 2"]
    #[inline]
    pub fn lp2(&self) -> LP2R {
        LP2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - Communication on Port 2"]
    #[inline]
    pub fn com_p2(&self) -> COM_P2R {
        COM_P2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - Loop Port 3"]
    #[inline]
    pub fn lp3(&self) -> LP3R {
        LP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Communication on Port 3"]
    #[inline]
    pub fn com_p3(&self) -> COM_P3R {
        COM_P3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
