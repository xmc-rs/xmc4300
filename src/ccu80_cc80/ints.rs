#[doc = "Reader of register INTS"]
pub type R = crate::R<u32, super::INTS>;
#[doc = "Period Match while Counting Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `PMUS`"]
pub type PMUS_R = crate::R<bool, PMUS_A>;
impl PMUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUS_A {
        match self.bits {
            false => PMUS_A::VALUE1,
            true => PMUS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PMUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PMUS_A::VALUE2
    }
}
#[doc = "One Match while Counting Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `OMDS`"]
pub type OMDS_R = crate::R<bool, OMDS_A>;
impl OMDS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OMDS_A {
        match self.bits {
            false => OMDS_A::VALUE1,
            true => OMDS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OMDS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OMDS_A::VALUE2
    }
}
#[doc = "Channel 1 Compare Match while Counting Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMU1S_A {
    #[doc = "0: Compare match while counting up not detected"]
    VALUE1 = 0,
    #[doc = "1: Compare match while counting up detected"]
    VALUE2 = 1,
}
impl From<CMU1S_A> for bool {
    #[inline(always)]
    fn from(variant: CMU1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMU1S`"]
pub type CMU1S_R = crate::R<bool, CMU1S_A>;
impl CMU1S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMU1S_A {
        match self.bits {
            false => CMU1S_A::VALUE1,
            true => CMU1S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMU1S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMU1S_A::VALUE2
    }
}
#[doc = "Channel 1 Compare Match while Counting Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD1S_A {
    #[doc = "0: Compare match while counting down not detected"]
    VALUE1 = 0,
    #[doc = "1: Compare match while counting down detected"]
    VALUE2 = 1,
}
impl From<CMD1S_A> for bool {
    #[inline(always)]
    fn from(variant: CMD1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMD1S`"]
pub type CMD1S_R = crate::R<bool, CMD1S_A>;
impl CMD1S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD1S_A {
        match self.bits {
            false => CMD1S_A::VALUE1,
            true => CMD1S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD1S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD1S_A::VALUE2
    }
}
#[doc = "Channel 2 Compare Match while Counting Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMU2S_A {
    #[doc = "0: Compare match while counting up not detected"]
    VALUE1 = 0,
    #[doc = "1: Compare match while counting up detected"]
    VALUE2 = 1,
}
impl From<CMU2S_A> for bool {
    #[inline(always)]
    fn from(variant: CMU2S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMU2S`"]
pub type CMU2S_R = crate::R<bool, CMU2S_A>;
impl CMU2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMU2S_A {
        match self.bits {
            false => CMU2S_A::VALUE1,
            true => CMU2S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMU2S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMU2S_A::VALUE2
    }
}
#[doc = "Channel 2 Compare Match while Counting Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD2S_A {
    #[doc = "0: Compare match while counting down not detected"]
    VALUE1 = 0,
    #[doc = "1: Compare match while counting down detected"]
    VALUE2 = 1,
}
impl From<CMD2S_A> for bool {
    #[inline(always)]
    fn from(variant: CMD2S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMD2S`"]
pub type CMD2S_R = crate::R<bool, CMD2S_A>;
impl CMD2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD2S_A {
        match self.bits {
            false => CMD2S_A::VALUE1,
            true => CMD2S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD2S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD2S_A::VALUE2
    }
}
#[doc = "Event 0 Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `E0AS`"]
pub type E0AS_R = crate::R<bool, E0AS_A>;
impl E0AS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E0AS_A {
        match self.bits {
            false => E0AS_A::VALUE1,
            true => E0AS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0AS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0AS_A::VALUE2
    }
}
#[doc = "Event 1 Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `E1AS`"]
pub type E1AS_R = crate::R<bool, E1AS_A>;
impl E1AS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E1AS_A {
        match self.bits {
            false => E1AS_A::VALUE1,
            true => E1AS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1AS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1AS_A::VALUE2
    }
}
#[doc = "Event 2 Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `E2AS`"]
pub type E2AS_R = crate::R<bool, E2AS_A>;
impl E2AS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E2AS_A {
        match self.bits {
            false => E2AS_A::VALUE1,
            true => E2AS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2AS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2AS_A::VALUE2
    }
}
#[doc = "Reader of field `TRPF`"]
pub type TRPF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Period Match while Counting Up"]
    #[inline(always)]
    pub fn pmus(&self) -> PMUS_R {
        PMUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - One Match while Counting Down"]
    #[inline(always)]
    pub fn omds(&self) -> OMDS_R {
        OMDS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Compare Match while Counting Up"]
    #[inline(always)]
    pub fn cmu1s(&self) -> CMU1S_R {
        CMU1S_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Compare Match while Counting Down"]
    #[inline(always)]
    pub fn cmd1s(&self) -> CMD1S_R {
        CMD1S_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Compare Match while Counting Up"]
    #[inline(always)]
    pub fn cmu2s(&self) -> CMU2S_R {
        CMU2S_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Compare Match while Counting Down"]
    #[inline(always)]
    pub fn cmd2s(&self) -> CMD2S_R {
        CMD2S_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Event 0 Detection Status"]
    #[inline(always)]
    pub fn e0as(&self) -> E0AS_R {
        E0AS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Event 1 Detection Status"]
    #[inline(always)]
    pub fn e1as(&self) -> E1AS_R {
        E1AS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Event 2 Detection Status"]
    #[inline(always)]
    pub fn e2as(&self) -> E2AS_R {
        E2AS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Trap Flag Status"]
    #[inline(always)]
    pub fn trpf(&self) -> TRPF_R {
        TRPF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
