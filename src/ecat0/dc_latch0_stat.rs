#[doc = "Register `DC_LATCH0_STAT` reader"]
pub type R = crate::R<DC_LATCH0_STAT_SPEC>;
#[doc = "Field `EV_L0_POS` reader - Event Latch0 positive edge"]
pub type EV_L0_POS_R = crate::BitReader<EV_L0_POS_A>;
#[doc = "Event Latch0 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EV_L0_POS_A {
    #[doc = "0: Positive edge not detected or continuous mode"]
    VALUE1 = 0,
    #[doc = "1: Positive edge detected in single event mode only"]
    VALUE2 = 1,
}
impl From<EV_L0_POS_A> for bool {
    #[inline(always)]
    fn from(variant: EV_L0_POS_A) -> Self {
        variant as u8 != 0
    }
}
impl EV_L0_POS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV_L0_POS_A {
        match self.bits {
            false => EV_L0_POS_A::VALUE1,
            true => EV_L0_POS_A::VALUE2,
        }
    }
    #[doc = "Positive edge not detected or continuous mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV_L0_POS_A::VALUE1
    }
    #[doc = "Positive edge detected in single event mode only"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV_L0_POS_A::VALUE2
    }
}
#[doc = "Field `EV_L0_NEG` reader - Event Latch0 negative edge"]
pub type EV_L0_NEG_R = crate::BitReader<EV_L0_NEG_A>;
#[doc = "Event Latch0 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EV_L0_NEG_A {
    #[doc = "0: Negative edge not detected or continuous mode"]
    VALUE1 = 0,
    #[doc = "1: Negative edge detected in single event mode only"]
    VALUE2 = 1,
}
impl From<EV_L0_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: EV_L0_NEG_A) -> Self {
        variant as u8 != 0
    }
}
impl EV_L0_NEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV_L0_NEG_A {
        match self.bits {
            false => EV_L0_NEG_A::VALUE1,
            true => EV_L0_NEG_A::VALUE2,
        }
    }
    #[doc = "Negative edge not detected or continuous mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV_L0_NEG_A::VALUE1
    }
    #[doc = "Negative edge detected in single event mode only"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV_L0_NEG_A::VALUE2
    }
}
#[doc = "Field `L0_PIN` reader - Latch0 pin state"]
pub type L0_PIN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Event Latch0 positive edge"]
    #[inline(always)]
    pub fn ev_l0_pos(&self) -> EV_L0_POS_R {
        EV_L0_POS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Latch0 negative edge"]
    #[inline(always)]
    pub fn ev_l0_neg(&self) -> EV_L0_NEG_R {
        EV_L0_NEG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Latch0 pin state"]
    #[inline(always)]
    pub fn l0_pin(&self) -> L0_PIN_R {
        L0_PIN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Latch0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_LATCH0_STAT_SPEC;
impl crate::RegisterSpec for DC_LATCH0_STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_latch0_stat::R`](R) reader structure"]
impl crate::Readable for DC_LATCH0_STAT_SPEC {}
#[doc = "`reset()` method sets DC_LATCH0_STAT to value 0"]
impl crate::Resettable for DC_LATCH0_STAT_SPEC {
    const RESET_VALUE: u8 = 0;
}
