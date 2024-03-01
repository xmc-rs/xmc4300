#[doc = "Register `DC_CYC_CONT` reader"]
pub type R = crate::R<DcCycContSpec>;
#[doc = "SYNC out unit control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync {
    #[doc = "0: ECAT controlled"]
    Value1 = 0,
    #[doc = "1: PDI controlled"]
    Value2 = 1,
}
impl From<Sync> for bool {
    #[inline(always)]
    fn from(variant: Sync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - SYNC out unit control"]
pub type SyncR = crate::BitReader<Sync>;
impl SyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync {
        match self.bits {
            false => Sync::Value1,
            true => Sync::Value2,
        }
    }
    #[doc = "ECAT controlled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sync::Value1
    }
    #[doc = "PDI controlled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sync::Value2
    }
}
#[doc = "Latch In unit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatchU0 {
    #[doc = "0: ECAT controlled"]
    Value1 = 0,
    #[doc = "1: PDI controlled"]
    Value2 = 1,
}
impl From<LatchU0> for bool {
    #[inline(always)]
    fn from(variant: LatchU0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LATCH_U0` reader - Latch In unit 0"]
pub type LatchU0R = crate::BitReader<LatchU0>;
impl LatchU0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LatchU0 {
        match self.bits {
            false => LatchU0::Value1,
            true => LatchU0::Value2,
        }
    }
    #[doc = "ECAT controlled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LatchU0::Value1
    }
    #[doc = "PDI controlled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LatchU0::Value2
    }
}
#[doc = "Latch In unit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatchU1 {
    #[doc = "0: ECAT controlled"]
    Value1 = 0,
    #[doc = "1: PDI controlled"]
    Value2 = 1,
}
impl From<LatchU1> for bool {
    #[inline(always)]
    fn from(variant: LatchU1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LATCH_U1` reader - Latch In unit 1"]
pub type LatchU1R = crate::BitReader<LatchU1>;
impl LatchU1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LatchU1 {
        match self.bits {
            false => LatchU1::Value1,
            true => LatchU1::Value2,
        }
    }
    #[doc = "ECAT controlled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LatchU1::Value1
    }
    #[doc = "PDI controlled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LatchU1::Value2
    }
}
impl R {
    #[doc = "Bit 0 - SYNC out unit control"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Latch In unit 0"]
    #[inline(always)]
    pub fn latch_u0(&self) -> LatchU0R {
        LatchU0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Latch In unit 1"]
    #[inline(always)]
    pub fn latch_u1(&self) -> LatchU1R {
        LatchU1R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Cyclic Unit Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_cyc_cont::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcCycContSpec;
impl crate::RegisterSpec for DcCycContSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_cyc_cont::R`](R) reader structure"]
impl crate::Readable for DcCycContSpec {}
#[doc = "`reset()` method sets DC_CYC_CONT to value 0"]
impl crate::Resettable for DcCycContSpec {
    const RESET_VALUE: u8 = 0;
}
