#[doc = "Register `DC_ACT_STAT` reader"]
pub type R = crate::R<DcActStatSpec>;
#[doc = "SYNC0 activation state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0AckState {
    #[doc = "0: First SYNC0 pulse is not pending"]
    Value1 = 0,
    #[doc = "1: First SYNC0 pulse is pending"]
    Value2 = 1,
}
impl From<S0AckState> for bool {
    #[inline(always)]
    fn from(variant: S0AckState) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0_ACK_STATE` reader - SYNC0 activation state"]
pub type S0AckStateR = crate::BitReader<S0AckState>;
impl S0AckStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0AckState {
        match self.bits {
            false => S0AckState::Value1,
            true => S0AckState::Value2,
        }
    }
    #[doc = "First SYNC0 pulse is not pending"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0AckState::Value1
    }
    #[doc = "First SYNC0 pulse is pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0AckState::Value2
    }
}
#[doc = "SYNC1 activation state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1AckState {
    #[doc = "0: First SYNC1 pulse is not pending"]
    Value1 = 0,
    #[doc = "1: First SYNC1 pulse is pending"]
    Value2 = 1,
}
impl From<S1AckState> for bool {
    #[inline(always)]
    fn from(variant: S1AckState) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1_ACK_STATE` reader - SYNC1 activation state"]
pub type S1AckStateR = crate::BitReader<S1AckState>;
impl S1AckStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1AckState {
        match self.bits {
            false => S1AckState::Value1,
            true => S1AckState::Value2,
        }
    }
    #[doc = "First SYNC1 pulse is not pending"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1AckState::Value1
    }
    #[doc = "First SYNC1 pulse is pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1AckState::Value2
    }
}
#[doc = "Start Time Cyclic Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STime {
    #[doc = "0: Start Time was within near future"]
    Value1 = 0,
    #[doc = "1: Start Time was out of near future (0x0981.6)"]
    Value2 = 1,
}
impl From<STime> for bool {
    #[inline(always)]
    fn from(variant: STime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_TIME` reader - Start Time Cyclic Operation"]
pub type STimeR = crate::BitReader<STime>;
impl STimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STime {
        match self.bits {
            false => STime::Value1,
            true => STime::Value2,
        }
    }
    #[doc = "Start Time was within near future"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STime::Value1
    }
    #[doc = "Start Time was out of near future (0x0981.6)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STime::Value2
    }
}
impl R {
    #[doc = "Bit 0 - SYNC0 activation state"]
    #[inline(always)]
    pub fn s0_ack_state(&self) -> S0AckStateR {
        S0AckStateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC1 activation state"]
    #[inline(always)]
    pub fn s1_ack_state(&self) -> S1AckStateR {
        S1AckStateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub fn s_time(&self) -> STimeR {
        STimeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Activation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_act_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcActStatSpec;
impl crate::RegisterSpec for DcActStatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_act_stat::R`](R) reader structure"]
impl crate::Readable for DcActStatSpec {}
#[doc = "`reset()` method sets DC_ACT_STAT to value 0"]
impl crate::Resettable for DcActStatSpec {
    const RESET_VALUE: u8 = 0;
}
