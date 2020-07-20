#[doc = "Reader of register DC_LATCH1_STAT"]
pub type R = crate::R<u8, super::DC_LATCH1_STAT>;
#[doc = "Event Latch1 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `EV_L1_POS`"]
pub type EV_L1_POS_R = crate::R<bool, EV_L1_POS_A>;
impl EV_L1_POS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV_L1_POS_A {
        match self.bits {
            false => EV_L1_POS_A::VALUE1,
            true => EV_L1_POS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV_L1_POS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV_L1_POS_A::VALUE2
    }
}
#[doc = "Event Latch1 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `EV_L1_NEG`"]
pub type EV_L1_NEG_R = crate::R<bool, EV_L1_NEG_A>;
impl EV_L1_NEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV_L1_NEG_A {
        match self.bits {
            false => EV_L1_NEG_A::VALUE1,
            true => EV_L1_NEG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EV_L1_NEG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EV_L1_NEG_A::VALUE2
    }
}
#[doc = "Reader of field `L1_PIN`"]
pub type L1_PIN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Event Latch1 positive edge"]
    #[inline(always)]
    pub fn ev_l1_pos(&self) -> EV_L1_POS_R {
        EV_L1_POS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Event Latch1 negative edge"]
    #[inline(always)]
    pub fn ev_l1_neg(&self) -> EV_L1_NEG_R {
        EV_L1_NEG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Latch1 pin state"]
    #[inline(always)]
    pub fn l1_pin(&self) -> L1_PIN_R {
        L1_PIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
