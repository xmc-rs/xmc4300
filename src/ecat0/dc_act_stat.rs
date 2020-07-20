#[doc = "Reader of register DC_ACT_STAT"]
pub type R = crate::R<u8, super::DC_ACT_STAT>;
#[doc = "SYNC0 activation state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0_ACK_STATE_A {
    #[doc = "0: First SYNC0 pulse is not pending"]
    VALUE1 = 0,
    #[doc = "1: First SYNC0 pulse is pending"]
    VALUE2 = 1,
}
impl From<S0_ACK_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: S0_ACK_STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S0_ACK_STATE`"]
pub type S0_ACK_STATE_R = crate::R<bool, S0_ACK_STATE_A>;
impl S0_ACK_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0_ACK_STATE_A {
        match self.bits {
            false => S0_ACK_STATE_A::VALUE1,
            true => S0_ACK_STATE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0_ACK_STATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0_ACK_STATE_A::VALUE2
    }
}
#[doc = "SYNC1 activation state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1_ACK_STATE_A {
    #[doc = "0: First SYNC1 pulse is not pending"]
    VALUE1 = 0,
    #[doc = "1: First SYNC1 pulse is pending"]
    VALUE2 = 1,
}
impl From<S1_ACK_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: S1_ACK_STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S1_ACK_STATE`"]
pub type S1_ACK_STATE_R = crate::R<bool, S1_ACK_STATE_A>;
impl S1_ACK_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1_ACK_STATE_A {
        match self.bits {
            false => S1_ACK_STATE_A::VALUE1,
            true => S1_ACK_STATE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1_ACK_STATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1_ACK_STATE_A::VALUE2
    }
}
#[doc = "Start Time Cyclic Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_TIME_A {
    #[doc = "0: Start Time was within near future"]
    VALUE1 = 0,
    #[doc = "1: Start Time was out of near future (0x0981.6)"]
    VALUE2 = 1,
}
impl From<S_TIME_A> for bool {
    #[inline(always)]
    fn from(variant: S_TIME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S_TIME`"]
pub type S_TIME_R = crate::R<bool, S_TIME_A>;
impl S_TIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_TIME_A {
        match self.bits {
            false => S_TIME_A::VALUE1,
            true => S_TIME_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S_TIME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S_TIME_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - SYNC0 activation state"]
    #[inline(always)]
    pub fn s0_ack_state(&self) -> S0_ACK_STATE_R {
        S0_ACK_STATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SYNC1 activation state"]
    #[inline(always)]
    pub fn s1_ack_state(&self) -> S1_ACK_STATE_R {
        S1_ACK_STATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub fn s_time(&self) -> S_TIME_R {
        S_TIME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
