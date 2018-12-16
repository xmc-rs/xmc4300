#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::DC_ACT_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `S0_ACK_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0_ACK_STATER {
    #[doc = "First SYNC0 pulse is not pending"]
    VALUE1,
    #[doc = "First SYNC0 pulse is pending"]
    VALUE2,
}
impl S0_ACK_STATER {
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
            S0_ACK_STATER::VALUE1 => false,
            S0_ACK_STATER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0_ACK_STATER {
        match value {
            false => S0_ACK_STATER::VALUE1,
            true => S0_ACK_STATER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S0_ACK_STATER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S0_ACK_STATER::VALUE2
    }
}
#[doc = "Possible values of the field `S1_ACK_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1_ACK_STATER {
    #[doc = "First SYNC1 pulse is not pending"]
    VALUE1,
    #[doc = "First SYNC1 pulse is pending"]
    VALUE2,
}
impl S1_ACK_STATER {
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
            S1_ACK_STATER::VALUE1 => false,
            S1_ACK_STATER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1_ACK_STATER {
        match value {
            false => S1_ACK_STATER::VALUE1,
            true => S1_ACK_STATER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S1_ACK_STATER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S1_ACK_STATER::VALUE2
    }
}
#[doc = "Possible values of the field `S_TIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_TIMER {
    #[doc = "Start Time was within near future"]
    VALUE1,
    #[doc = "Start Time was out of near future (0x0981.6)"]
    VALUE2,
}
impl S_TIMER {
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
            S_TIMER::VALUE1 => false,
            S_TIMER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S_TIMER {
        match value {
            false => S_TIMER::VALUE1,
            true => S_TIMER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S_TIMER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S_TIMER::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - SYNC0 activation state"]
    #[inline]
    pub fn s0_ack_state(&self) -> S0_ACK_STATER {
        S0_ACK_STATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - SYNC1 activation state"]
    #[inline]
    pub fn s1_ack_state(&self) -> S1_ACK_STATER {
        S1_ACK_STATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Start Time Cyclic Operation"]
    #[inline]
    pub fn s_time(&self) -> S_TIMER {
        S_TIMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
