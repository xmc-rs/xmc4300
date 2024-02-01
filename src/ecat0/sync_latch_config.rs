#[doc = "Register `SYNC_LATCH_CONFIG` reader"]
pub type R = crate::R<SYNC_LATCH_CONFIG_SPEC>;
#[doc = "Field `SYNC0_POL` reader - SYNC0 output driver/polarity"]
pub type SYNC0_POL_R = crate::FieldReader<SYNC0_POL_A>;
#[doc = "SYNC0 output driver/polarity\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNC0_POL_A {
    #[doc = "0: Push-Pull active low"]
    VALUE1 = 0,
    #[doc = "1: Open Drain (active low)"]
    VALUE2 = 1,
    #[doc = "2: Push-Pull active high"]
    VALUE3 = 2,
    #[doc = "3: Open Source (active high)"]
    VALUE4 = 3,
}
impl From<SYNC0_POL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC0_POL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNC0_POL_A {
    type Ux = u8;
}
impl SYNC0_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNC0_POL_A {
        match self.bits {
            0 => SYNC0_POL_A::VALUE1,
            1 => SYNC0_POL_A::VALUE2,
            2 => SYNC0_POL_A::VALUE3,
            3 => SYNC0_POL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Push-Pull active low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC0_POL_A::VALUE1
    }
    #[doc = "Open Drain (active low)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC0_POL_A::VALUE2
    }
    #[doc = "Push-Pull active high"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SYNC0_POL_A::VALUE3
    }
    #[doc = "Open Source (active high)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SYNC0_POL_A::VALUE4
    }
}
#[doc = "Field `SL0_CNF` reader - SYNC0/LATCH0 configuration"]
pub type SL0_CNF_R = crate::BitReader<SL0_CNF_A>;
#[doc = "SYNC0/LATCH0 configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SL0_CNF_A {
    #[doc = "0: LATCH0 Input"]
    VALUE1 = 0,
    #[doc = "1: SYNC0 Output"]
    VALUE2 = 1,
}
impl From<SL0_CNF_A> for bool {
    #[inline(always)]
    fn from(variant: SL0_CNF_A) -> Self {
        variant as u8 != 0
    }
}
impl SL0_CNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SL0_CNF_A {
        match self.bits {
            false => SL0_CNF_A::VALUE1,
            true => SL0_CNF_A::VALUE2,
        }
    }
    #[doc = "LATCH0 Input"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SL0_CNF_A::VALUE1
    }
    #[doc = "SYNC0 Output"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SL0_CNF_A::VALUE2
    }
}
#[doc = "Field `S0_MAP` reader - SYNC0 mapped to registerECAT0_AL_EVENT_REQ. ST_S0"]
pub type S0_MAP_R = crate::BitReader<S0_MAP_A>;
#[doc = "SYNC0 mapped to registerECAT0_AL_EVENT_REQ. ST_S0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0_MAP_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<S0_MAP_A> for bool {
    #[inline(always)]
    fn from(variant: S0_MAP_A) -> Self {
        variant as u8 != 0
    }
}
impl S0_MAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0_MAP_A {
        match self.bits {
            false => S0_MAP_A::VALUE1,
            true => S0_MAP_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0_MAP_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0_MAP_A::VALUE2
    }
}
#[doc = "Field `SYNC1_POL` reader - SYNC1 output driver/polarity"]
pub type SYNC1_POL_R = crate::FieldReader<SYNC1_POL_A>;
#[doc = "SYNC1 output driver/polarity\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNC1_POL_A {
    #[doc = "0: Push-Pull active low"]
    VALUE1 = 0,
    #[doc = "1: Open Drain (active low)"]
    VALUE2 = 1,
    #[doc = "2: Push-Pull active high"]
    VALUE3 = 2,
    #[doc = "3: Open Source (active high)"]
    VALUE4 = 3,
}
impl From<SYNC1_POL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC1_POL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNC1_POL_A {
    type Ux = u8;
}
impl SYNC1_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNC1_POL_A {
        match self.bits {
            0 => SYNC1_POL_A::VALUE1,
            1 => SYNC1_POL_A::VALUE2,
            2 => SYNC1_POL_A::VALUE3,
            3 => SYNC1_POL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Push-Pull active low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC1_POL_A::VALUE1
    }
    #[doc = "Open Drain (active low)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC1_POL_A::VALUE2
    }
    #[doc = "Push-Pull active high"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SYNC1_POL_A::VALUE3
    }
    #[doc = "Open Source (active high)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SYNC1_POL_A::VALUE4
    }
}
#[doc = "Field `SL1_CNF` reader - SYNC1/LATCH1 configuration"]
pub type SL1_CNF_R = crate::BitReader<SL1_CNF_A>;
#[doc = "SYNC1/LATCH1 configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SL1_CNF_A {
    #[doc = "0: LATCH1 Input"]
    VALUE1 = 0,
    #[doc = "1: SYNC1 Output"]
    VALUE2 = 1,
}
impl From<SL1_CNF_A> for bool {
    #[inline(always)]
    fn from(variant: SL1_CNF_A) -> Self {
        variant as u8 != 0
    }
}
impl SL1_CNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SL1_CNF_A {
        match self.bits {
            false => SL1_CNF_A::VALUE1,
            true => SL1_CNF_A::VALUE2,
        }
    }
    #[doc = "LATCH1 Input"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SL1_CNF_A::VALUE1
    }
    #[doc = "SYNC1 Output"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SL1_CNF_A::VALUE2
    }
}
#[doc = "Field `S1_MAP` reader - SYNC1 mapped to registerECAT0_AL_EVENT_REQ. ST_S1"]
pub type S1_MAP_R = crate::BitReader<S1_MAP_A>;
#[doc = "SYNC1 mapped to registerECAT0_AL_EVENT_REQ. ST_S1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1_MAP_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<S1_MAP_A> for bool {
    #[inline(always)]
    fn from(variant: S1_MAP_A) -> Self {
        variant as u8 != 0
    }
}
impl S1_MAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1_MAP_A {
        match self.bits {
            false => S1_MAP_A::VALUE1,
            true => S1_MAP_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1_MAP_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1_MAP_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - SYNC0 output driver/polarity"]
    #[inline(always)]
    pub fn sync0_pol(&self) -> SYNC0_POL_R {
        SYNC0_POL_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - SYNC0/LATCH0 configuration"]
    #[inline(always)]
    pub fn sl0_cnf(&self) -> SL0_CNF_R {
        SL0_CNF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SYNC0 mapped to registerECAT0_AL_EVENT_REQ. ST_S0"]
    #[inline(always)]
    pub fn s0_map(&self) -> S0_MAP_R {
        S0_MAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SYNC1 output driver/polarity"]
    #[inline(always)]
    pub fn sync1_pol(&self) -> SYNC1_POL_R {
        SYNC1_POL_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - SYNC1/LATCH1 configuration"]
    #[inline(always)]
    pub fn sl1_cnf(&self) -> SL1_CNF_R {
        SL1_CNF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYNC1 mapped to registerECAT0_AL_EVENT_REQ. ST_S1"]
    #[inline(always)]
    pub fn s1_map(&self) -> S1_MAP_R {
        S1_MAP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Sync/Latch\\[1:0\\]
PDI Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_latch_config::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_LATCH_CONFIG_SPEC;
impl crate::RegisterSpec for SYNC_LATCH_CONFIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sync_latch_config::R`](R) reader structure"]
impl crate::Readable for SYNC_LATCH_CONFIG_SPEC {}
#[doc = "`reset()` method sets SYNC_LATCH_CONFIG to value 0xee"]
impl crate::Resettable for SYNC_LATCH_CONFIG_SPEC {
    const RESET_VALUE: u8 = 0xee;
}
