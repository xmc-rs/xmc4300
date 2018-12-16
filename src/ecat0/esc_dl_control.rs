#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ESC_DL_CONTROL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `FR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRR {
    #[doc = "EtherCAT frames are processed, Non-EtherCAT frames are forwarded without processing"]
    VALUE1,
    #[doc = "EtherCAT frames are processed, Non- EtherCAT frames are destroyed"]
    VALUE2,
}
impl FRR {
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
            FRR::VALUE1 => false,
            FRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRR {
        match value {
            false => FRR::VALUE1,
            true => FRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FRR::VALUE2
    }
}
#[doc = "Possible values of the field `TEMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPR {
    #[doc = "permanent use"]
    VALUE1,
    #[doc = "use for about 1 second, then revert to previous settings"]
    VALUE2,
}
impl TEMPR {
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
            TEMPR::VALUE1 => false,
            TEMPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEMPR {
        match value {
            false => TEMPR::VALUE1,
            true => TEMPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TEMPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TEMPR::VALUE2
    }
}
#[doc = "Possible values of the field `LP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP0R {
    #[doc = "Auto"]
    VALUE1,
    #[doc = "Auto Close"]
    VALUE2,
    #[doc = "Open"]
    VALUE3,
    #[doc = "Closed"]
    VALUE4,
}
impl LP0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LP0R::VALUE1 => 0,
            LP0R::VALUE2 => 1,
            LP0R::VALUE3 => 2,
            LP0R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LP0R {
        match value {
            0 => LP0R::VALUE1,
            1 => LP0R::VALUE2,
            2 => LP0R::VALUE3,
            3 => LP0R::VALUE4,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LP0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LP0R::VALUE4
    }
}
#[doc = "Possible values of the field `LP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP1R {
    #[doc = "Auto"]
    VALUE1,
    #[doc = "Auto Close"]
    VALUE2,
    #[doc = "Open"]
    VALUE3,
    #[doc = "Closed"]
    VALUE4,
}
impl LP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LP1R::VALUE1 => 0,
            LP1R::VALUE2 => 1,
            LP1R::VALUE3 => 2,
            LP1R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LP1R {
        match value {
            0 => LP1R::VALUE1,
            1 => LP1R::VALUE2,
            2 => LP1R::VALUE3,
            3 => LP1R::VALUE4,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LP1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LP1R::VALUE4
    }
}
#[doc = "Possible values of the field `LP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP2R {
    #[doc = "Auto"]
    VALUE1,
    #[doc = "Auto Close"]
    VALUE2,
    #[doc = "Open"]
    VALUE3,
    #[doc = "Closed"]
    VALUE4,
}
impl LP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LP2R::VALUE1 => 0,
            LP2R::VALUE2 => 1,
            LP2R::VALUE3 => 2,
            LP2R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LP2R {
        match value {
            0 => LP2R::VALUE1,
            1 => LP2R::VALUE2,
            2 => LP2R::VALUE3,
            3 => LP2R::VALUE4,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LP2R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LP2R::VALUE4
    }
}
#[doc = "Possible values of the field `LP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP3R {
    #[doc = "Auto"]
    VALUE1,
    #[doc = "Auto Close"]
    VALUE2,
    #[doc = "Open"]
    VALUE3,
    #[doc = "Closed"]
    VALUE4,
}
impl LP3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LP3R::VALUE1 => 0,
            LP3R::VALUE2 => 1,
            LP3R::VALUE3 => 2,
            LP3R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LP3R {
        match value {
            0 => LP3R::VALUE1,
            1 => LP3R::VALUE2,
            2 => LP3R::VALUE3,
            3 => LP3R::VALUE4,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LP3R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LP3R::VALUE4
    }
}
#[doc = "Possible values of the field `RX_FIFO_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_SIZER {
    #[doc = "-40 ns (-80 ns)"]
    VALUE1,
    #[doc = "-40 ns (-80 ns)"]
    VALUE2,
    #[doc = "-40 ns"]
    VALUE3,
    #[doc = "-40 ns"]
    VALUE4,
    #[doc = "no change"]
    VALUE5,
    #[doc = "no change"]
    VALUE6,
    #[doc = "no change"]
    VALUE7,
    #[doc = "default"]
    VALUE8,
}
impl RX_FIFO_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_FIFO_SIZER::VALUE1 => 0,
            RX_FIFO_SIZER::VALUE2 => 1,
            RX_FIFO_SIZER::VALUE3 => 2,
            RX_FIFO_SIZER::VALUE4 => 3,
            RX_FIFO_SIZER::VALUE5 => 4,
            RX_FIFO_SIZER::VALUE6 => 5,
            RX_FIFO_SIZER::VALUE7 => 6,
            RX_FIFO_SIZER::VALUE8 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_FIFO_SIZER {
        match value {
            0 => RX_FIFO_SIZER::VALUE1,
            1 => RX_FIFO_SIZER::VALUE2,
            2 => RX_FIFO_SIZER::VALUE3,
            3 => RX_FIFO_SIZER::VALUE4,
            4 => RX_FIFO_SIZER::VALUE5,
            5 => RX_FIFO_SIZER::VALUE6,
            6 => RX_FIFO_SIZER::VALUE7,
            7 => RX_FIFO_SIZER::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RX_FIFO_SIZER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RX_FIFO_SIZER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RX_FIFO_SIZER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RX_FIFO_SIZER::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == RX_FIFO_SIZER::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == RX_FIFO_SIZER::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == RX_FIFO_SIZER::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == RX_FIFO_SIZER::VALUE8
    }
}
#[doc = "Possible values of the field `LJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LJR {
    #[doc = "Normal jitter"]
    VALUE1,
    #[doc = "Reduced jitter"]
    VALUE2,
}
impl LJR {
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
            LJR::VALUE1 => false,
            LJR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LJR {
        match value {
            false => LJR::VALUE1,
            true => LJR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LJR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LJR::VALUE2
    }
}
#[doc = "Possible values of the field `RLD_ST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLD_STR {
    #[doc = "Default (~660 ms)"]
    VALUE1,
    #[doc = "Reduced (~80 us)"]
    VALUE2,
}
impl RLD_STR {
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
            RLD_STR::VALUE1 => false,
            RLD_STR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RLD_STR {
        match value {
            false => RLD_STR::VALUE1,
            true => RLD_STR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RLD_STR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RLD_STR::VALUE2
    }
}
#[doc = "Possible values of the field `S_ALIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_ALIASR {
    #[doc = "Ignore Station Alias"]
    VALUE1,
    #[doc = "Alias can be used for all configured address command types (FPRD,FPWR,...)"]
    VALUE2,
}
impl S_ALIASR {
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
            S_ALIASR::VALUE1 => false,
            S_ALIASR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S_ALIASR {
        match value {
            false => S_ALIASR::VALUE1,
            true => S_ALIASR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S_ALIASR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S_ALIASR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Forwarding rule"]
    #[inline]
    pub fn fr(&self) -> FRR {
        FRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Temporary use of settings in LP1-LP3"]
    #[inline]
    pub fn temp(&self) -> TEMPR {
        TEMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Loop Port 0"]
    #[inline]
    pub fn lp0(&self) -> LP0R {
        LP0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Loop Port 1"]
    #[inline]
    pub fn lp1(&self) -> LP1R {
        LP1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Loop Port 2"]
    #[inline]
    pub fn lp2(&self) -> LP2R {
        LP2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Loop Port 3"]
    #[inline]
    pub fn lp3(&self) -> LP3R {
        LP3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - RX FIFO Size"]
    #[inline]
    pub fn rx_fifo_size(&self) -> RX_FIFO_SIZER {
        RX_FIFO_SIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - EBUS Low Jitter"]
    #[inline]
    pub fn lj(&self) -> LJR {
        LJR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - EBUS remote link down signaling time"]
    #[inline]
    pub fn rld_st(&self) -> RLD_STR {
        RLD_STR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Station alias"]
    #[inline]
    pub fn s_alias(&self) -> S_ALIASR {
        S_ALIASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
