#[doc = "Reader of register SYNC_LATCH_CONFIG"]
pub type R = crate::R<u8, super::SYNC_LATCH_CONFIG>;
#[doc = "SYNC0 output driver/polarity\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SYNC0_POL`"]
pub type SYNC0_POL_R = crate::R<u8, SYNC0_POL_A>;
impl SYNC0_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC0_POL_A {
        match self.bits {
            0 => SYNC0_POL_A::VALUE1,
            1 => SYNC0_POL_A::VALUE2,
            2 => SYNC0_POL_A::VALUE3,
            3 => SYNC0_POL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC0_POL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC0_POL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SYNC0_POL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SYNC0_POL_A::VALUE4
    }
}
#[doc = "SYNC0/LATCH0 configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SL0_CNF`"]
pub type SL0_CNF_R = crate::R<bool, SL0_CNF_A>;
impl SL0_CNF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SL0_CNF_A {
        match self.bits {
            false => SL0_CNF_A::VALUE1,
            true => SL0_CNF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SL0_CNF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SL0_CNF_A::VALUE2
    }
}
#[doc = "SYNC0 mapped to registerECAT0_AL_EVENT_REQ. ST_S0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `S0_MAP`"]
pub type S0_MAP_R = crate::R<bool, S0_MAP_A>;
impl S0_MAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0_MAP_A {
        match self.bits {
            false => S0_MAP_A::VALUE1,
            true => S0_MAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0_MAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0_MAP_A::VALUE2
    }
}
#[doc = "SYNC1 output driver/polarity\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SYNC1_POL`"]
pub type SYNC1_POL_R = crate::R<u8, SYNC1_POL_A>;
impl SYNC1_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC1_POL_A {
        match self.bits {
            0 => SYNC1_POL_A::VALUE1,
            1 => SYNC1_POL_A::VALUE2,
            2 => SYNC1_POL_A::VALUE3,
            3 => SYNC1_POL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC1_POL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC1_POL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SYNC1_POL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SYNC1_POL_A::VALUE4
    }
}
#[doc = "SYNC1/LATCH1 configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SL1_CNF`"]
pub type SL1_CNF_R = crate::R<bool, SL1_CNF_A>;
impl SL1_CNF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SL1_CNF_A {
        match self.bits {
            false => SL1_CNF_A::VALUE1,
            true => SL1_CNF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SL1_CNF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SL1_CNF_A::VALUE2
    }
}
#[doc = "SYNC1 mapped to registerECAT0_AL_EVENT_REQ. ST_S1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `S1_MAP`"]
pub type S1_MAP_R = crate::R<bool, S1_MAP_A>;
impl S1_MAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1_MAP_A {
        match self.bits {
            false => S1_MAP_A::VALUE1,
            true => S1_MAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1_MAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1_MAP_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - SYNC0 output driver/polarity"]
    #[inline(always)]
    pub fn sync0_pol(&self) -> SYNC0_POL_R {
        SYNC0_POL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - SYNC0/LATCH0 configuration"]
    #[inline(always)]
    pub fn sl0_cnf(&self) -> SL0_CNF_R {
        SL0_CNF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SYNC0 mapped to registerECAT0_AL_EVENT_REQ. ST_S0"]
    #[inline(always)]
    pub fn s0_map(&self) -> S0_MAP_R {
        S0_MAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - SYNC1 output driver/polarity"]
    #[inline(always)]
    pub fn sync1_pol(&self) -> SYNC1_POL_R {
        SYNC1_POL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - SYNC1/LATCH1 configuration"]
    #[inline(always)]
    pub fn sl1_cnf(&self) -> SL1_CNF_R {
        SL1_CNF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SYNC1 mapped to registerECAT0_AL_EVENT_REQ. ST_S1"]
    #[inline(always)]
    pub fn s1_map(&self) -> S1_MAP_R {
        S1_MAP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
