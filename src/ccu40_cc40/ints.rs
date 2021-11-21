#[doc = "Register `INTS` reader"]
pub struct R(crate::R<INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTS_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `PMUS` reader - Period Match while Counting Up"]
pub struct PMUS_R(crate::FieldReader<bool, PMUS_A>);
impl PMUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMUS_R(crate::FieldReader::new(bits))
    }
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
        **self == PMUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PMUS_A::VALUE2
    }
}
impl core::ops::Deref for PMUS_R {
    type Target = crate::FieldReader<bool, PMUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `OMDS` reader - One Match while Counting Down"]
pub struct OMDS_R(crate::FieldReader<bool, OMDS_A>);
impl OMDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OMDS_R(crate::FieldReader::new(bits))
    }
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
        **self == OMDS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == OMDS_A::VALUE2
    }
}
impl core::ops::Deref for OMDS_R {
    type Target = crate::FieldReader<bool, OMDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Compare Match while Counting Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct CMUS_R(crate::FieldReader<bool, CMUS_A>);
impl CMUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMUS_A {
        match self.bits {
            false => CMUS_A::VALUE1,
            true => CMUS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMUS_A::VALUE2
    }
}
impl core::ops::Deref for CMUS_R {
    type Target = crate::FieldReader<bool, CMUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Compare Match while Counting Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct CMDS_R(crate::FieldReader<bool, CMDS_A>);
impl CMDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDS_A {
        match self.bits {
            false => CMDS_A::VALUE1,
            true => CMDS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMDS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMDS_A::VALUE2
    }
}
impl core::ops::Deref for CMDS_R {
    type Target = crate::FieldReader<bool, CMDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `E0AS` reader - Event 0 Detection Status"]
pub struct E0AS_R(crate::FieldReader<bool, E0AS_A>);
impl E0AS_R {
    pub(crate) fn new(bits: bool) -> Self {
        E0AS_R(crate::FieldReader::new(bits))
    }
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
        **self == E0AS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == E0AS_A::VALUE2
    }
}
impl core::ops::Deref for E0AS_R {
    type Target = crate::FieldReader<bool, E0AS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `E1AS` reader - Event 1 Detection Status"]
pub struct E1AS_R(crate::FieldReader<bool, E1AS_A>);
impl E1AS_R {
    pub(crate) fn new(bits: bool) -> Self {
        E1AS_R(crate::FieldReader::new(bits))
    }
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
        **self == E1AS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == E1AS_A::VALUE2
    }
}
impl core::ops::Deref for E1AS_R {
    type Target = crate::FieldReader<bool, E1AS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `E2AS` reader - Event 2 Detection Status"]
pub struct E2AS_R(crate::FieldReader<bool, E2AS_A>);
impl E2AS_R {
    pub(crate) fn new(bits: bool) -> Self {
        E2AS_R(crate::FieldReader::new(bits))
    }
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
        **self == E2AS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == E2AS_A::VALUE2
    }
}
impl core::ops::Deref for E2AS_R {
    type Target = crate::FieldReader<bool, E2AS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRPF` reader - Trap Flag Status"]
pub struct TRPF_R(crate::FieldReader<bool, bool>);
impl TRPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[doc = "Bit 2 - Compare Match while Counting Up"]
    #[inline(always)]
    pub fn cmus(&self) -> CMUS_R {
        CMUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare Match while Counting Down"]
    #[inline(always)]
    pub fn cmds(&self) -> CMDS_R {
        CMDS_R::new(((self.bits >> 3) & 0x01) != 0)
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
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ints](index.html) module"]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ints::R](R) reader structure"]
impl crate::Readable for INTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
