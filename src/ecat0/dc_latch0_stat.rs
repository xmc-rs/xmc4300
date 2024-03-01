#[doc = "Register `DC_LATCH0_STAT` reader"]
pub type R = crate::R<DcLatch0StatSpec>;
#[doc = "Event Latch0 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvL0Pos {
    #[doc = "0: Positive edge not detected or continuous mode"]
    Value1 = 0,
    #[doc = "1: Positive edge detected in single event mode only"]
    Value2 = 1,
}
impl From<EvL0Pos> for bool {
    #[inline(always)]
    fn from(variant: EvL0Pos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV_L0_POS` reader - Event Latch0 positive edge"]
pub type EvL0PosR = crate::BitReader<EvL0Pos>;
impl EvL0PosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EvL0Pos {
        match self.bits {
            false => EvL0Pos::Value1,
            true => EvL0Pos::Value2,
        }
    }
    #[doc = "Positive edge not detected or continuous mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EvL0Pos::Value1
    }
    #[doc = "Positive edge detected in single event mode only"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EvL0Pos::Value2
    }
}
#[doc = "Event Latch0 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvL0Neg {
    #[doc = "0: Negative edge not detected or continuous mode"]
    Value1 = 0,
    #[doc = "1: Negative edge detected in single event mode only"]
    Value2 = 1,
}
impl From<EvL0Neg> for bool {
    #[inline(always)]
    fn from(variant: EvL0Neg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV_L0_NEG` reader - Event Latch0 negative edge"]
pub type EvL0NegR = crate::BitReader<EvL0Neg>;
impl EvL0NegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EvL0Neg {
        match self.bits {
            false => EvL0Neg::Value1,
            true => EvL0Neg::Value2,
        }
    }
    #[doc = "Negative edge not detected or continuous mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EvL0Neg::Value1
    }
    #[doc = "Negative edge detected in single event mode only"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EvL0Neg::Value2
    }
}
#[doc = "Field `L0_PIN` reader - Latch0 pin state"]
pub type L0PinR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Event Latch0 positive edge"]
    #[inline(always)]
    pub fn ev_l0_pos(&self) -> EvL0PosR {
        EvL0PosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Latch0 negative edge"]
    #[inline(always)]
    pub fn ev_l0_neg(&self) -> EvL0NegR {
        EvL0NegR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Latch0 pin state"]
    #[inline(always)]
    pub fn l0_pin(&self) -> L0PinR {
        L0PinR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Latch0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcLatch0StatSpec;
impl crate::RegisterSpec for DcLatch0StatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_latch0_stat::R`](R) reader structure"]
impl crate::Readable for DcLatch0StatSpec {}
#[doc = "`reset()` method sets DC_LATCH0_STAT to value 0"]
impl crate::Resettable for DcLatch0StatSpec {
    const RESET_VALUE: u8 = 0;
}
