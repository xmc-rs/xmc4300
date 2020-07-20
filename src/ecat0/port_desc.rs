#[doc = "Reader of register PORT_DESC"]
pub type R = crate::R<u8, super::PORT_DESC>;
#[doc = "Port Configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT0_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT0_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `Port0`"]
pub type PORT0_R = crate::R<u8, PORT0_A>;
impl PORT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT0_A {
        match self.bits {
            0 => PORT0_A::VALUE1,
            1 => PORT0_A::VALUE2,
            2 => PORT0_A::VALUE3,
            3 => PORT0_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PORT0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PORT0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PORT0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PORT0_A::VALUE4
    }
}
#[doc = "Port Configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT1_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT1_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `Port1`"]
pub type PORT1_R = crate::R<u8, PORT1_A>;
impl PORT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT1_A {
        match self.bits {
            0 => PORT1_A::VALUE1,
            1 => PORT1_A::VALUE2,
            2 => PORT1_A::VALUE3,
            3 => PORT1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PORT1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PORT1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PORT1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PORT1_A::VALUE4
    }
}
#[doc = "Port Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT2_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT2_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `Port2`"]
pub type PORT2_R = crate::R<u8, PORT2_A>;
impl PORT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT2_A {
        match self.bits {
            0 => PORT2_A::VALUE1,
            1 => PORT2_A::VALUE2,
            2 => PORT2_A::VALUE3,
            3 => PORT2_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PORT2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PORT2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PORT2_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PORT2_A::VALUE4
    }
}
#[doc = "Port Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT3_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT3_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `Port3`"]
pub type PORT3_R = crate::R<u8, PORT3_A>;
impl PORT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT3_A {
        match self.bits {
            0 => PORT3_A::VALUE1,
            1 => PORT3_A::VALUE2,
            2 => PORT3_A::VALUE3,
            3 => PORT3_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PORT3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PORT3_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PORT3_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PORT3_A::VALUE4
    }
}
impl R {
    #[doc = "Bits 0:1 - Port Configuration"]
    #[inline(always)]
    pub fn port0(&self) -> PORT0_R {
        PORT0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port Configuration"]
    #[inline(always)]
    pub fn port1(&self) -> PORT1_R {
        PORT1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port Configuration"]
    #[inline(always)]
    pub fn port2(&self) -> PORT2_R {
        PORT2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port Configuration"]
    #[inline(always)]
    pub fn port3(&self) -> PORT3_R {
        PORT3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
