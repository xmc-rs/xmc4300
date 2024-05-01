#[doc = "Register `INTS` reader"]
pub type R = crate::R<INTS_SPEC>;
#[doc = "Period Match while Counting Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUS_A {
    #[doc = "0: Period match while counting up not detected"]
    VALUE1 = 0,
    #[doc = "1: Period match while counting up detected"]
    VALUE2 = 1,
}
impl From<PMUS_A> for bool {
    #[inline(always)]
    fn from(variant: PMUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMUS` reader - Period Match while Counting Up"]
pub type PMUS_R = crate::BitReader<PMUS_A>;
impl PMUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMUS_A {
        match self.bits {
            false => PMUS_A::VALUE1,
            true => PMUS_A::VALUE2,
        }
    }
    #[doc = "Period match while counting up not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PMUS_A::VALUE1
    }
    #[doc = "Period match while counting up detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PMUS_A::VALUE2
    }
}
#[doc = "One Match while Counting Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OMDS_A {
    #[doc = "0: One match while counting down not detected"]
    VALUE1 = 0,
    #[doc = "1: One match while counting down detected"]
    VALUE2 = 1,
}
impl From<OMDS_A> for bool {
    #[inline(always)]
    fn from(variant: OMDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OMDS` reader - One Match while Counting Down"]
pub type OMDS_R = crate::BitReader<OMDS_A>;
impl OMDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OMDS_A {
        match self.bits {
            false => OMDS_A::VALUE1,
            true => OMDS_A::VALUE2,
        }
    }
    #[doc = "One match while counting down not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OMDS_A::VALUE1
    }
    #[doc = "One match while counting down detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OMDS_A::VALUE2
    }
}
#[doc = "Compare Match while Counting Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMUS_A {
    #[doc = "0: Compare match while counting up not detected"]
    VALUE1 = 0,
    #[doc = "1: Compare match while counting up detected"]
    VALUE2 = 1,
}
impl From<CMUS_A> for bool {
    #[inline(always)]
    fn from(variant: CMUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMUS` reader - Compare Match while Counting Up"]
pub type CMUS_R = crate::BitReader<CMUS_A>;
impl CMUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMUS_A {
        match self.bits {
            false => CMUS_A::VALUE1,
            true => CMUS_A::VALUE2,
        }
    }
    #[doc = "Compare match while counting up not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMUS_A::VALUE1
    }
    #[doc = "Compare match while counting up detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMUS_A::VALUE2
    }
}
#[doc = "Compare Match while Counting Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDS_A {
    #[doc = "0: Compare match while counting down not detected"]
    VALUE1 = 0,
    #[doc = "1: Compare match while counting down detected"]
    VALUE2 = 1,
}
impl From<CMDS_A> for bool {
    #[inline(always)]
    fn from(variant: CMDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDS` reader - Compare Match while Counting Down"]
pub type CMDS_R = crate::BitReader<CMDS_A>;
impl CMDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDS_A {
        match self.bits {
            false => CMDS_A::VALUE1,
            true => CMDS_A::VALUE2,
        }
    }
    #[doc = "Compare match while counting down not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMDS_A::VALUE1
    }
    #[doc = "Compare match while counting down detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMDS_A::VALUE2
    }
}
#[doc = "Event 0 Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E0AS_A {
    #[doc = "0: Event 0 not detected"]
    VALUE1 = 0,
    #[doc = "1: Event 0 detected"]
    VALUE2 = 1,
}
impl From<E0AS_A> for bool {
    #[inline(always)]
    fn from(variant: E0AS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E0AS` reader - Event 0 Detection Status"]
pub type E0AS_R = crate::BitReader<E0AS_A>;
impl E0AS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E0AS_A {
        match self.bits {
            false => E0AS_A::VALUE1,
            true => E0AS_A::VALUE2,
        }
    }
    #[doc = "Event 0 not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0AS_A::VALUE1
    }
    #[doc = "Event 0 detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0AS_A::VALUE2
    }
}
#[doc = "Event 1 Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E1AS_A {
    #[doc = "0: Event 1 not detected"]
    VALUE1 = 0,
    #[doc = "1: Event 1 detected"]
    VALUE2 = 1,
}
impl From<E1AS_A> for bool {
    #[inline(always)]
    fn from(variant: E1AS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E1AS` reader - Event 1 Detection Status"]
pub type E1AS_R = crate::BitReader<E1AS_A>;
impl E1AS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E1AS_A {
        match self.bits {
            false => E1AS_A::VALUE1,
            true => E1AS_A::VALUE2,
        }
    }
    #[doc = "Event 1 not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1AS_A::VALUE1
    }
    #[doc = "Event 1 detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1AS_A::VALUE2
    }
}
#[doc = "Event 2 Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E2AS_A {
    #[doc = "0: Event 2 not detected"]
    VALUE1 = 0,
    #[doc = "1: Event 2 detected"]
    VALUE2 = 1,
}
impl From<E2AS_A> for bool {
    #[inline(always)]
    fn from(variant: E2AS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E2AS` reader - Event 2 Detection Status"]
pub type E2AS_R = crate::BitReader<E2AS_A>;
impl E2AS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E2AS_A {
        match self.bits {
            false => E2AS_A::VALUE1,
            true => E2AS_A::VALUE2,
        }
    }
    #[doc = "Event 2 not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2AS_A::VALUE1
    }
    #[doc = "Event 2 detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2AS_A::VALUE2
    }
}
#[doc = "Field `TRPF` reader - Trap Flag Status"]
pub type TRPF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Period Match while Counting Up"]
    #[inline(always)]
    pub fn pmus(&self) -> PMUS_R {
        PMUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - One Match while Counting Down"]
    #[inline(always)]
    pub fn omds(&self) -> OMDS_R {
        OMDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Match while Counting Up"]
    #[inline(always)]
    pub fn cmus(&self) -> CMUS_R {
        CMUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Match while Counting Down"]
    #[inline(always)]
    pub fn cmds(&self) -> CMDS_R {
        CMDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Event 0 Detection Status"]
    #[inline(always)]
    pub fn e0as(&self) -> E0AS_R {
        E0AS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event 1 Detection Status"]
    #[inline(always)]
    pub fn e1as(&self) -> E1AS_R {
        E1AS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event 2 Detection Status"]
    #[inline(always)]
    pub fn e2as(&self) -> E2AS_R {
        E2AS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Trap Flag Status"]
    #[inline(always)]
    pub fn trpf(&self) -> TRPF_R {
        TRPF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ints::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints::R`](R) reader structure"]
impl crate::Readable for INTS_SPEC {}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
