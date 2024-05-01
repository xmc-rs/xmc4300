#[doc = "Register `DC_LATCH1_STAT` reader"]
pub type R = crate::R<DcLatch1StatSpec>;
#[doc = "Event Latch1 positive edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvL1Pos {
    #[doc = "0: Positive edge not detected or continuous mode"]
    Value1 = 0,
    #[doc = "1: Positive edge detected in single event mode only"]
    Value2 = 1,
}
impl From<EvL1Pos> for bool {
    #[inline(always)]
    fn from(variant: EvL1Pos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV_L1_POS` reader - Event Latch1 positive edge"]
pub type EvL1PosR = crate::BitReader<EvL1Pos>;
impl EvL1PosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EvL1Pos {
        match self.bits {
            false => EvL1Pos::Value1,
            true => EvL1Pos::Value2,
        }
    }
    #[doc = "Positive edge not detected or continuous mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EvL1Pos::Value1
    }
    #[doc = "Positive edge detected in single event mode only"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EvL1Pos::Value2
    }
}
#[doc = "Event Latch1 negative edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvL1Neg {
    #[doc = "0: Negative edge not detected or continuous mode"]
    Value1 = 0,
    #[doc = "1: Negative edge detected in single event mode only"]
    Value2 = 1,
}
impl From<EvL1Neg> for bool {
    #[inline(always)]
    fn from(variant: EvL1Neg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV_L1_NEG` reader - Event Latch1 negative edge"]
pub type EvL1NegR = crate::BitReader<EvL1Neg>;
impl EvL1NegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EvL1Neg {
        match self.bits {
            false => EvL1Neg::Value1,
            true => EvL1Neg::Value2,
        }
    }
    #[doc = "Negative edge not detected or continuous mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EvL1Neg::Value1
    }
    #[doc = "Negative edge detected in single event mode only"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EvL1Neg::Value2
    }
}
#[doc = "Field `L1_PIN` reader - Latch1 pin state"]
pub type L1PinR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Event Latch1 positive edge"]
    #[inline(always)]
    pub fn ev_l1_pos(&self) -> EvL1PosR {
        EvL1PosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Latch1 negative edge"]
    #[inline(always)]
    pub fn ev_l1_neg(&self) -> EvL1NegR {
        EvL1NegR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Latch1 pin state"]
    #[inline(always)]
    pub fn l1_pin(&self) -> L1PinR {
        L1PinR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Latch1 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcLatch1StatSpec;
impl crate::RegisterSpec for DcLatch1StatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_latch1_stat::R`](R) reader structure"]
impl crate::Readable for DcLatch1StatSpec {}
#[doc = "`reset()` method sets DC_LATCH1_STAT to value 0"]
impl crate::Resettable for DcLatch1StatSpec {
    const RESET_VALUE: u8 = 0;
}
