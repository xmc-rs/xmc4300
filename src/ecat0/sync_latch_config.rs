#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::SYNC_LATCH_CONFIG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SYNC0_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC0_POLR {
    #[doc = "Push-Pull active low"]
    VALUE1,
    #[doc = "Open Drain (active low)"]
    VALUE2,
    #[doc = "Push-Pull active high"]
    VALUE3,
    #[doc = "Open Source (active high)"]
    VALUE4,
}
impl SYNC0_POLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNC0_POLR::VALUE1 => 0,
            SYNC0_POLR::VALUE2 => 1,
            SYNC0_POLR::VALUE3 => 2,
            SYNC0_POLR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNC0_POLR {
        match value {
            0 => SYNC0_POLR::VALUE1,
            1 => SYNC0_POLR::VALUE2,
            2 => SYNC0_POLR::VALUE3,
            3 => SYNC0_POLR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYNC0_POLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYNC0_POLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SYNC0_POLR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SYNC0_POLR::VALUE4
    }
}
#[doc = "Possible values of the field `SL0_CNF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SL0_CNFR {
    #[doc = "LATCH0 Input"]
    VALUE1,
    #[doc = "SYNC0 Output"]
    VALUE2,
}
impl SL0_CNFR {
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
            SL0_CNFR::VALUE1 => false,
            SL0_CNFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SL0_CNFR {
        match value {
            false => SL0_CNFR::VALUE1,
            true => SL0_CNFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SL0_CNFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SL0_CNFR::VALUE2
    }
}
#[doc = "Possible values of the field `S0_MAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0_MAPR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl S0_MAPR {
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
            S0_MAPR::VALUE1 => false,
            S0_MAPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0_MAPR {
        match value {
            false => S0_MAPR::VALUE1,
            true => S0_MAPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S0_MAPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S0_MAPR::VALUE2
    }
}
#[doc = "Possible values of the field `SYNC1_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC1_POLR {
    #[doc = "Push-Pull active low"]
    VALUE1,
    #[doc = "Open Drain (active low)"]
    VALUE2,
    #[doc = "Push-Pull active high"]
    VALUE3,
    #[doc = "Open Source (active high)"]
    VALUE4,
}
impl SYNC1_POLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNC1_POLR::VALUE1 => 0,
            SYNC1_POLR::VALUE2 => 1,
            SYNC1_POLR::VALUE3 => 2,
            SYNC1_POLR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNC1_POLR {
        match value {
            0 => SYNC1_POLR::VALUE1,
            1 => SYNC1_POLR::VALUE2,
            2 => SYNC1_POLR::VALUE3,
            3 => SYNC1_POLR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYNC1_POLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYNC1_POLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SYNC1_POLR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SYNC1_POLR::VALUE4
    }
}
#[doc = "Possible values of the field `SL1_CNF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SL1_CNFR {
    #[doc = "LATCH1 Input"]
    VALUE1,
    #[doc = "SYNC1 Output"]
    VALUE2,
}
impl SL1_CNFR {
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
            SL1_CNFR::VALUE1 => false,
            SL1_CNFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SL1_CNFR {
        match value {
            false => SL1_CNFR::VALUE1,
            true => SL1_CNFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SL1_CNFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SL1_CNFR::VALUE2
    }
}
#[doc = "Possible values of the field `S1_MAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1_MAPR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl S1_MAPR {
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
            S1_MAPR::VALUE1 => false,
            S1_MAPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1_MAPR {
        match value {
            false => S1_MAPR::VALUE1,
            true => S1_MAPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S1_MAPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S1_MAPR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - SYNC0 output driver/polarity"]
    #[inline]
    pub fn sync0_pol(&self) -> SYNC0_POLR {
        SYNC0_POLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 2 - SYNC0/LATCH0 configuration"]
    #[inline]
    pub fn sl0_cnf(&self) -> SL0_CNFR {
        SL0_CNFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - SYNC0 mapped to registerECAT0_AL_EVENT_REQ. ST_S0"]
    #[inline]
    pub fn s0_map(&self) -> S0_MAPR {
        S0_MAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 4:5 - SYNC1 output driver/polarity"]
    #[inline]
    pub fn sync1_pol(&self) -> SYNC1_POLR {
        SYNC1_POLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 6 - SYNC1/LATCH1 configuration"]
    #[inline]
    pub fn sl1_cnf(&self) -> SL1_CNFR {
        SL1_CNFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - SYNC1 mapped to registerECAT0_AL_EVENT_REQ. ST_S1"]
    #[inline]
    pub fn s1_map(&self) -> S1_MAPR {
        S1_MAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
