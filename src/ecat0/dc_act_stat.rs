#[doc = "Register `DC_ACT_STAT` reader"]
pub struct R(crate::R<DC_ACT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_ACT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_ACT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_ACT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `S0_ACK_STATE` reader - SYNC0 activation state"]
pub struct S0_ACK_STATE_R(crate::FieldReader<bool, S0_ACK_STATE_A>);
impl S0_ACK_STATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        S0_ACK_STATE_R(crate::FieldReader::new(bits))
    }
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
        **self == S0_ACK_STATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S0_ACK_STATE_A::VALUE2
    }
}
impl core::ops::Deref for S0_ACK_STATE_R {
    type Target = crate::FieldReader<bool, S0_ACK_STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `S1_ACK_STATE` reader - SYNC1 activation state"]
pub struct S1_ACK_STATE_R(crate::FieldReader<bool, S1_ACK_STATE_A>);
impl S1_ACK_STATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        S1_ACK_STATE_R(crate::FieldReader::new(bits))
    }
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
        **self == S1_ACK_STATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S1_ACK_STATE_A::VALUE2
    }
}
impl core::ops::Deref for S1_ACK_STATE_R {
    type Target = crate::FieldReader<bool, S1_ACK_STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `S_TIME` reader - Start Time Cyclic Operation"]
pub struct S_TIME_R(crate::FieldReader<bool, S_TIME_A>);
impl S_TIME_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_TIME_R(crate::FieldReader::new(bits))
    }
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
        **self == S_TIME_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S_TIME_A::VALUE2
    }
}
impl core::ops::Deref for S_TIME_R {
    type Target = crate::FieldReader<bool, S_TIME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Activation Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_act_stat](index.html) module"]
pub struct DC_ACT_STAT_SPEC;
impl crate::RegisterSpec for DC_ACT_STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dc_act_stat::R](R) reader structure"]
impl crate::Readable for DC_ACT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_ACT_STAT to value 0"]
impl crate::Resettable for DC_ACT_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
