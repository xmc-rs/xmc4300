#[doc = "Register `DC_CYC_CONT` reader"]
pub type R = crate::R<DC_CYC_CONT_SPEC>;
#[doc = "Field `SYNC` reader - SYNC out unit control"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "SYNC out unit control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::VALUE1,
            true => SYNC_A::VALUE2,
        }
    }
    #[doc = "ECAT controlled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_A::VALUE1
    }
    #[doc = "PDI controlled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_A::VALUE2
    }
}
#[doc = "Field `LATCH_U0` reader - Latch In unit 0"]
pub type LATCH_U0_R = crate::BitReader<LATCH_U0_A>;
#[doc = "Latch In unit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LATCH_U0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LATCH_U0_A {
        match self.bits {
            false => LATCH_U0_A::VALUE1,
            true => LATCH_U0_A::VALUE2,
        }
    }
    #[doc = "ECAT controlled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LATCH_U0_A::VALUE1
    }
    #[doc = "PDI controlled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LATCH_U0_A::VALUE2
    }
}
#[doc = "Field `LATCH_U1` reader - Latch In unit 1"]
pub type LATCH_U1_R = crate::BitReader<LATCH_U1_A>;
#[doc = "Latch In unit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LATCH_U1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LATCH_U1_A {
        match self.bits {
            false => LATCH_U1_A::VALUE1,
            true => LATCH_U1_A::VALUE2,
        }
    }
    #[doc = "ECAT controlled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LATCH_U1_A::VALUE1
    }
    #[doc = "PDI controlled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LATCH_U1_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - SYNC out unit control"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Latch In unit 0"]
    #[inline(always)]
    pub fn latch_u0(&self) -> LATCH_U0_R {
        LATCH_U0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Latch In unit 1"]
    #[inline(always)]
    pub fn latch_u1(&self) -> LATCH_U1_R {
        LATCH_U1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Cyclic Unit Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_cyc_cont::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_CYC_CONT_SPEC;
impl crate::RegisterSpec for DC_CYC_CONT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_cyc_cont::R`](R) reader structure"]
impl crate::Readable for DC_CYC_CONT_SPEC {}
#[doc = "`reset()` method sets DC_CYC_CONT to value 0"]
impl crate::Resettable for DC_CYC_CONT_SPEC {
    const RESET_VALUE: u8 = 0;
}
