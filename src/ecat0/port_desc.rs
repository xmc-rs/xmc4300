#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::PORT_DESC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `Port0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT0R {
    #[doc = "Not implemented"]
    VALUE1,
    #[doc = "Not configured (SII EEPROM)"]
    VALUE2,
    #[doc = "EBUS"]
    VALUE3,
    #[doc = "MII / RMII / RGMII"]
    VALUE4,
}
impl PORT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORT0R::VALUE1 => 0,
            PORT0R::VALUE2 => 1,
            PORT0R::VALUE3 => 2,
            PORT0R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORT0R {
        match value {
            0 => PORT0R::VALUE1,
            1 => PORT0R::VALUE2,
            2 => PORT0R::VALUE3,
            3 => PORT0R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PORT0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PORT0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PORT0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PORT0R::VALUE4
    }
}
#[doc = "Possible values of the field `Port1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT1R {
    #[doc = "Not implemented"]
    VALUE1,
    #[doc = "Not configured (SII EEPROM)"]
    VALUE2,
    #[doc = "EBUS"]
    VALUE3,
    #[doc = "MII / RMII / RGMII"]
    VALUE4,
}
impl PORT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORT1R::VALUE1 => 0,
            PORT1R::VALUE2 => 1,
            PORT1R::VALUE3 => 2,
            PORT1R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORT1R {
        match value {
            0 => PORT1R::VALUE1,
            1 => PORT1R::VALUE2,
            2 => PORT1R::VALUE3,
            3 => PORT1R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PORT1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PORT1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PORT1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PORT1R::VALUE4
    }
}
#[doc = "Possible values of the field `Port2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT2R {
    #[doc = "Not implemented"]
    VALUE1,
    #[doc = "Not configured (SII EEPROM)"]
    VALUE2,
    #[doc = "EBUS"]
    VALUE3,
    #[doc = "MII / RMII / RGMII"]
    VALUE4,
}
impl PORT2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORT2R::VALUE1 => 0,
            PORT2R::VALUE2 => 1,
            PORT2R::VALUE3 => 2,
            PORT2R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORT2R {
        match value {
            0 => PORT2R::VALUE1,
            1 => PORT2R::VALUE2,
            2 => PORT2R::VALUE3,
            3 => PORT2R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PORT2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PORT2R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PORT2R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PORT2R::VALUE4
    }
}
#[doc = "Possible values of the field `Port3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT3R {
    #[doc = "Not implemented"]
    VALUE1,
    #[doc = "Not configured (SII EEPROM)"]
    VALUE2,
    #[doc = "EBUS"]
    VALUE3,
    #[doc = "MII / RMII / RGMII"]
    VALUE4,
}
impl PORT3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORT3R::VALUE1 => 0,
            PORT3R::VALUE2 => 1,
            PORT3R::VALUE3 => 2,
            PORT3R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORT3R {
        match value {
            0 => PORT3R::VALUE1,
            1 => PORT3R::VALUE2,
            2 => PORT3R::VALUE3,
            3 => PORT3R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PORT3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PORT3R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PORT3R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PORT3R::VALUE4
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Port Configuration"]
    #[inline]
    pub fn port0(&self) -> PORT0R {
        PORT0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Port Configuration"]
    #[inline]
    pub fn port1(&self) -> PORT1R {
        PORT1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Port Configuration"]
    #[inline]
    pub fn port2(&self) -> PORT2R {
        PORT2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Port Configuration"]
    #[inline]
    pub fn port3(&self) -> PORT3R {
        PORT3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
