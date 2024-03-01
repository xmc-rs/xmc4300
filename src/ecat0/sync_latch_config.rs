#[doc = "Register `SYNC_LATCH_CONFIG` reader"]
pub type R = crate::R<SyncLatchConfigSpec>;
#[doc = "SYNC0 output driver/polarity\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sync0Pol {
    #[doc = "0: Push-Pull active low"]
    Value1 = 0,
    #[doc = "1: Open Drain (active low)"]
    Value2 = 1,
    #[doc = "2: Push-Pull active high"]
    Value3 = 2,
    #[doc = "3: Open Source (active high)"]
    Value4 = 3,
}
impl From<Sync0Pol> for u8 {
    #[inline(always)]
    fn from(variant: Sync0Pol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sync0Pol {
    type Ux = u8;
}
#[doc = "Field `SYNC0_POL` reader - SYNC0 output driver/polarity"]
pub type Sync0PolR = crate::FieldReader<Sync0Pol>;
impl Sync0PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync0Pol {
        match self.bits {
            0 => Sync0Pol::Value1,
            1 => Sync0Pol::Value2,
            2 => Sync0Pol::Value3,
            3 => Sync0Pol::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Push-Pull active low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sync0Pol::Value1
    }
    #[doc = "Open Drain (active low)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sync0Pol::Value2
    }
    #[doc = "Push-Pull active high"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sync0Pol::Value3
    }
    #[doc = "Open Source (active high)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Sync0Pol::Value4
    }
}
#[doc = "SYNC0/LATCH0 configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sl0Cnf {
    #[doc = "0: LATCH0 Input"]
    Value1 = 0,
    #[doc = "1: SYNC0 Output"]
    Value2 = 1,
}
impl From<Sl0Cnf> for bool {
    #[inline(always)]
    fn from(variant: Sl0Cnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SL0_CNF` reader - SYNC0/LATCH0 configuration"]
pub type Sl0CnfR = crate::BitReader<Sl0Cnf>;
impl Sl0CnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sl0Cnf {
        match self.bits {
            false => Sl0Cnf::Value1,
            true => Sl0Cnf::Value2,
        }
    }
    #[doc = "LATCH0 Input"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sl0Cnf::Value1
    }
    #[doc = "SYNC0 Output"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sl0Cnf::Value2
    }
}
#[doc = "SYNC0 mapped to registerECAT0_AL_EVENT_REQ. ST_S0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0Map {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<S0Map> for bool {
    #[inline(always)]
    fn from(variant: S0Map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0_MAP` reader - SYNC0 mapped to registerECAT0_AL_EVENT_REQ. ST_S0"]
pub type S0MapR = crate::BitReader<S0Map>;
impl S0MapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0Map {
        match self.bits {
            false => S0Map::Value1,
            true => S0Map::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0Map::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0Map::Value2
    }
}
#[doc = "SYNC1 output driver/polarity\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sync1Pol {
    #[doc = "0: Push-Pull active low"]
    Value1 = 0,
    #[doc = "1: Open Drain (active low)"]
    Value2 = 1,
    #[doc = "2: Push-Pull active high"]
    Value3 = 2,
    #[doc = "3: Open Source (active high)"]
    Value4 = 3,
}
impl From<Sync1Pol> for u8 {
    #[inline(always)]
    fn from(variant: Sync1Pol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sync1Pol {
    type Ux = u8;
}
#[doc = "Field `SYNC1_POL` reader - SYNC1 output driver/polarity"]
pub type Sync1PolR = crate::FieldReader<Sync1Pol>;
impl Sync1PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync1Pol {
        match self.bits {
            0 => Sync1Pol::Value1,
            1 => Sync1Pol::Value2,
            2 => Sync1Pol::Value3,
            3 => Sync1Pol::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Push-Pull active low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sync1Pol::Value1
    }
    #[doc = "Open Drain (active low)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sync1Pol::Value2
    }
    #[doc = "Push-Pull active high"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sync1Pol::Value3
    }
    #[doc = "Open Source (active high)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Sync1Pol::Value4
    }
}
#[doc = "SYNC1/LATCH1 configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sl1Cnf {
    #[doc = "0: LATCH1 Input"]
    Value1 = 0,
    #[doc = "1: SYNC1 Output"]
    Value2 = 1,
}
impl From<Sl1Cnf> for bool {
    #[inline(always)]
    fn from(variant: Sl1Cnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SL1_CNF` reader - SYNC1/LATCH1 configuration"]
pub type Sl1CnfR = crate::BitReader<Sl1Cnf>;
impl Sl1CnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sl1Cnf {
        match self.bits {
            false => Sl1Cnf::Value1,
            true => Sl1Cnf::Value2,
        }
    }
    #[doc = "LATCH1 Input"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sl1Cnf::Value1
    }
    #[doc = "SYNC1 Output"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sl1Cnf::Value2
    }
}
#[doc = "SYNC1 mapped to registerECAT0_AL_EVENT_REQ. ST_S1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1Map {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<S1Map> for bool {
    #[inline(always)]
    fn from(variant: S1Map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1_MAP` reader - SYNC1 mapped to registerECAT0_AL_EVENT_REQ. ST_S1"]
pub type S1MapR = crate::BitReader<S1Map>;
impl S1MapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1Map {
        match self.bits {
            false => S1Map::Value1,
            true => S1Map::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1Map::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1Map::Value2
    }
}
impl R {
    #[doc = "Bits 0:1 - SYNC0 output driver/polarity"]
    #[inline(always)]
    pub fn sync0_pol(&self) -> Sync0PolR {
        Sync0PolR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - SYNC0/LATCH0 configuration"]
    #[inline(always)]
    pub fn sl0_cnf(&self) -> Sl0CnfR {
        Sl0CnfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SYNC0 mapped to registerECAT0_AL_EVENT_REQ. ST_S0"]
    #[inline(always)]
    pub fn s0_map(&self) -> S0MapR {
        S0MapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SYNC1 output driver/polarity"]
    #[inline(always)]
    pub fn sync1_pol(&self) -> Sync1PolR {
        Sync1PolR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - SYNC1/LATCH1 configuration"]
    #[inline(always)]
    pub fn sl1_cnf(&self) -> Sl1CnfR {
        Sl1CnfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYNC1 mapped to registerECAT0_AL_EVENT_REQ. ST_S1"]
    #[inline(always)]
    pub fn s1_map(&self) -> S1MapR {
        S1MapR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Sync/Latch\\[1:0\\]
PDI Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_latch_config::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncLatchConfigSpec;
impl crate::RegisterSpec for SyncLatchConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sync_latch_config::R`](R) reader structure"]
impl crate::Readable for SyncLatchConfigSpec {}
#[doc = "`reset()` method sets SYNC_LATCH_CONFIG to value 0xee"]
impl crate::Resettable for SyncLatchConfigSpec {
    const RESET_VALUE: u8 = 0xee;
}
