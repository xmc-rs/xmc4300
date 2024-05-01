#[doc = "Register `INTS` reader"]
pub type R = crate::R<IntsSpec>;
#[doc = "Period Match while Counting Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmus {
    #[doc = "0: Period match while counting up not detected"]
    Value1 = 0,
    #[doc = "1: Period match while counting up detected"]
    Value2 = 1,
}
impl From<Pmus> for bool {
    #[inline(always)]
    fn from(variant: Pmus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMUS` reader - Period Match while Counting Up"]
pub type PmusR = crate::BitReader<Pmus>;
impl PmusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmus {
        match self.bits {
            false => Pmus::Value1,
            true => Pmus::Value2,
        }
    }
    #[doc = "Period match while counting up not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pmus::Value1
    }
    #[doc = "Period match while counting up detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pmus::Value2
    }
}
#[doc = "One Match while Counting Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Omds {
    #[doc = "0: One match while counting down not detected"]
    Value1 = 0,
    #[doc = "1: One match while counting down detected"]
    Value2 = 1,
}
impl From<Omds> for bool {
    #[inline(always)]
    fn from(variant: Omds) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OMDS` reader - One Match while Counting Down"]
pub type OmdsR = crate::BitReader<Omds>;
impl OmdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Omds {
        match self.bits {
            false => Omds::Value1,
            true => Omds::Value2,
        }
    }
    #[doc = "One match while counting down not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Omds::Value1
    }
    #[doc = "One match while counting down detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Omds::Value2
    }
}
#[doc = "Compare Match while Counting Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmus {
    #[doc = "0: Compare match while counting up not detected"]
    Value1 = 0,
    #[doc = "1: Compare match while counting up detected"]
    Value2 = 1,
}
impl From<Cmus> for bool {
    #[inline(always)]
    fn from(variant: Cmus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMUS` reader - Compare Match while Counting Up"]
pub type CmusR = crate::BitReader<Cmus>;
impl CmusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmus {
        match self.bits {
            false => Cmus::Value1,
            true => Cmus::Value2,
        }
    }
    #[doc = "Compare match while counting up not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cmus::Value1
    }
    #[doc = "Compare match while counting up detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cmus::Value2
    }
}
#[doc = "Compare Match while Counting Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmds {
    #[doc = "0: Compare match while counting down not detected"]
    Value1 = 0,
    #[doc = "1: Compare match while counting down detected"]
    Value2 = 1,
}
impl From<Cmds> for bool {
    #[inline(always)]
    fn from(variant: Cmds) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDS` reader - Compare Match while Counting Down"]
pub type CmdsR = crate::BitReader<Cmds>;
impl CmdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmds {
        match self.bits {
            false => Cmds::Value1,
            true => Cmds::Value2,
        }
    }
    #[doc = "Compare match while counting down not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cmds::Value1
    }
    #[doc = "Compare match while counting down detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cmds::Value2
    }
}
#[doc = "Event 0 Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E0as {
    #[doc = "0: Event 0 not detected"]
    Value1 = 0,
    #[doc = "1: Event 0 detected"]
    Value2 = 1,
}
impl From<E0as> for bool {
    #[inline(always)]
    fn from(variant: E0as) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E0AS` reader - Event 0 Detection Status"]
pub type E0asR = crate::BitReader<E0as>;
impl E0asR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E0as {
        match self.bits {
            false => E0as::Value1,
            true => E0as::Value2,
        }
    }
    #[doc = "Event 0 not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0as::Value1
    }
    #[doc = "Event 0 detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0as::Value2
    }
}
#[doc = "Event 1 Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E1as {
    #[doc = "0: Event 1 not detected"]
    Value1 = 0,
    #[doc = "1: Event 1 detected"]
    Value2 = 1,
}
impl From<E1as> for bool {
    #[inline(always)]
    fn from(variant: E1as) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E1AS` reader - Event 1 Detection Status"]
pub type E1asR = crate::BitReader<E1as>;
impl E1asR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E1as {
        match self.bits {
            false => E1as::Value1,
            true => E1as::Value2,
        }
    }
    #[doc = "Event 1 not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1as::Value1
    }
    #[doc = "Event 1 detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1as::Value2
    }
}
#[doc = "Event 2 Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E2as {
    #[doc = "0: Event 2 not detected"]
    Value1 = 0,
    #[doc = "1: Event 2 detected"]
    Value2 = 1,
}
impl From<E2as> for bool {
    #[inline(always)]
    fn from(variant: E2as) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E2AS` reader - Event 2 Detection Status"]
pub type E2asR = crate::BitReader<E2as>;
impl E2asR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E2as {
        match self.bits {
            false => E2as::Value1,
            true => E2as::Value2,
        }
    }
    #[doc = "Event 2 not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2as::Value1
    }
    #[doc = "Event 2 detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2as::Value2
    }
}
#[doc = "Field `TRPF` reader - Trap Flag Status"]
pub type TrpfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Period Match while Counting Up"]
    #[inline(always)]
    pub fn pmus(&self) -> PmusR {
        PmusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - One Match while Counting Down"]
    #[inline(always)]
    pub fn omds(&self) -> OmdsR {
        OmdsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Match while Counting Up"]
    #[inline(always)]
    pub fn cmus(&self) -> CmusR {
        CmusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Match while Counting Down"]
    #[inline(always)]
    pub fn cmds(&self) -> CmdsR {
        CmdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Event 0 Detection Status"]
    #[inline(always)]
    pub fn e0as(&self) -> E0asR {
        E0asR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event 1 Detection Status"]
    #[inline(always)]
    pub fn e1as(&self) -> E1asR {
        E1asR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event 2 Detection Status"]
    #[inline(always)]
    pub fn e2as(&self) -> E2asR {
        E2asR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Trap Flag Status"]
    #[inline(always)]
    pub fn trpf(&self) -> TrpfR {
        TrpfR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ints::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsSpec;
impl crate::RegisterSpec for IntsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints::R`](R) reader structure"]
impl crate::Readable for IntsSpec {}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for IntsSpec {
    const RESET_VALUE: u32 = 0;
}
