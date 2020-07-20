#[doc = "Reader of register DC_CYC_CONT"]
pub type R = crate::R<u8, super::DC_CYC_CONT>;
#[doc = "SYNC out unit control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: ECAT controlled"]
    VALUE1 = 0,
    #[doc = "1: PDI controlled"]
    VALUE2 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, SYNC_A>;
impl SYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::VALUE1,
            true => SYNC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_A::VALUE2
    }
}
#[doc = "Latch In unit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATCH_U0_A {
    #[doc = "0: ECAT controlled"]
    VALUE1 = 0,
    #[doc = "1: PDI controlled"]
    VALUE2 = 1,
}
impl From<LATCH_U0_A> for bool {
    #[inline(always)]
    fn from(variant: LATCH_U0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LATCH_U0`"]
pub type LATCH_U0_R = crate::R<bool, LATCH_U0_A>;
impl LATCH_U0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATCH_U0_A {
        match self.bits {
            false => LATCH_U0_A::VALUE1,
            true => LATCH_U0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LATCH_U0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LATCH_U0_A::VALUE2
    }
}
#[doc = "Latch In unit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATCH_U1_A {
    #[doc = "0: ECAT controlled"]
    VALUE1 = 0,
    #[doc = "1: PDI controlled"]
    VALUE2 = 1,
}
impl From<LATCH_U1_A> for bool {
    #[inline(always)]
    fn from(variant: LATCH_U1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LATCH_U1`"]
pub type LATCH_U1_R = crate::R<bool, LATCH_U1_A>;
impl LATCH_U1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATCH_U1_A {
        match self.bits {
            false => LATCH_U1_A::VALUE1,
            true => LATCH_U1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LATCH_U1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LATCH_U1_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - SYNC out unit control"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Latch In unit 0"]
    #[inline(always)]
    pub fn latch_u0(&self) -> LATCH_U0_R {
        LATCH_U0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Latch In unit 1"]
    #[inline(always)]
    pub fn latch_u1(&self) -> LATCH_U1_R {
        LATCH_U1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
