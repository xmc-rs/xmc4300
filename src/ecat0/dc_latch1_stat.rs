#[doc = "Register `DC_LATCH1_STAT` reader"]
pub type R = crate::R<DC_LATCH1_STAT_SPEC>;
#[doc = "Event Latch1 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EV_L1_POS_A {
    #[doc = "0: Positive edge not detected or continuous mode"]
    VALUE1 = 0,
    #[doc = "1: Positive edge detected in single event mode only"]
    VALUE2 = 1,
}
impl From<EV_L1_POS_A> for bool {
    #[inline(always)]
    fn from(variant: EV_L1_POS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV_L1_POS` reader - Event Latch1 positive edge"]
pub type EV_L1_POS_R = crate::BitReader<EV_L1_POS_A>;
impl EV_L1_POS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV_L1_POS_A {
        match self.bits {
            false => EV_L1_POS_A::VALUE1,
            true => EV_L1_POS_A::VALUE2,
        }
    }
    #[doc = "Positive edge not detected or continuous mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV_L1_POS_A::VALUE1
    }
    #[doc = "Positive edge detected in single event mode only"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV_L1_POS_A::VALUE2
    }
}
#[doc = "Event Latch1 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EV_L1_NEG_A {
    #[doc = "0: Negative edge not detected or continuous mode"]
    VALUE1 = 0,
    #[doc = "1: Negative edge detected in single event mode only"]
    VALUE2 = 1,
}
impl From<EV_L1_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: EV_L1_NEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV_L1_NEG` reader - Event Latch1 negative edge"]
pub type EV_L1_NEG_R = crate::BitReader<EV_L1_NEG_A>;
impl EV_L1_NEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EV_L1_NEG_A {
        match self.bits {
            false => EV_L1_NEG_A::VALUE1,
            true => EV_L1_NEG_A::VALUE2,
        }
    }
    #[doc = "Negative edge not detected or continuous mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV_L1_NEG_A::VALUE1
    }
    #[doc = "Negative edge detected in single event mode only"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV_L1_NEG_A::VALUE2
    }
}
#[doc = "Field `L1_PIN` reader - Latch1 pin state"]
pub type L1_PIN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Event Latch1 positive edge"]
    #[inline(always)]
    pub fn ev_l1_pos(&self) -> EV_L1_POS_R {
        EV_L1_POS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Latch1 negative edge"]
    #[inline(always)]
    pub fn ev_l1_neg(&self) -> EV_L1_NEG_R {
        EV_L1_NEG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Latch1 pin state"]
    #[inline(always)]
    pub fn l1_pin(&self) -> L1_PIN_R {
        L1_PIN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Latch1 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_LATCH1_STAT_SPEC;
impl crate::RegisterSpec for DC_LATCH1_STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_latch1_stat::R`](R) reader structure"]
impl crate::Readable for DC_LATCH1_STAT_SPEC {}
#[doc = "`reset()` method sets DC_LATCH1_STAT to value 0"]
impl crate::Resettable for DC_LATCH1_STAT_SPEC {
    const RESET_VALUE: u8 = 0;
}
