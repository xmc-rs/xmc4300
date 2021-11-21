#[doc = "Register `TCST` reader"]
pub struct R(crate::R<TCST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Timer Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRB_A {
    #[doc = "0: Timer is stopped"]
    VALUE1 = 0,
    #[doc = "1: Timer is running"]
    VALUE2 = 1,
}
impl From<TRB_A> for bool {
    #[inline(always)]
    fn from(variant: TRB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRB` reader - Timer Run Bit"]
pub struct TRB_R(crate::FieldReader<bool, TRB_A>);
impl TRB_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRB_A {
        match self.bits {
            false => TRB_A::VALUE1,
            true => TRB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TRB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TRB_A::VALUE2
    }
}
impl core::ops::Deref for TRB_R {
    type Target = crate::FieldReader<bool, TRB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timer Counting Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDIR_A {
    #[doc = "0: Timer is counting up"]
    VALUE1 = 0,
    #[doc = "1: Timer is counting down"]
    VALUE2 = 1,
}
impl From<CDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDIR` reader - Timer Counting Direction"]
pub struct CDIR_R(crate::FieldReader<bool, CDIR_A>);
impl CDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDIR_A {
        match self.bits {
            false => CDIR_A::VALUE1,
            true => CDIR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CDIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CDIR_A::VALUE2
    }
}
impl core::ops::Deref for CDIR_R {
    type Target = crate::FieldReader<bool, CDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Dead Time Counter 1 Run bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTR1_A {
    #[doc = "0: Dead Time counter is idle"]
    VALUE1 = 0,
    #[doc = "1: Dead Time counter is running"]
    VALUE2 = 1,
}
impl From<DTR1_A> for bool {
    #[inline(always)]
    fn from(variant: DTR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTR1` reader - Dead Time Counter 1 Run bit"]
pub struct DTR1_R(crate::FieldReader<bool, DTR1_A>);
impl DTR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTR1_A {
        match self.bits {
            false => DTR1_A::VALUE1,
            true => DTR1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DTR1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DTR1_A::VALUE2
    }
}
impl core::ops::Deref for DTR1_R {
    type Target = crate::FieldReader<bool, DTR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Dead Time Counter 2 Run bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTR2_A {
    #[doc = "0: Dead Time counter is idle"]
    VALUE1 = 0,
    #[doc = "1: Dead Time counter is running"]
    VALUE2 = 1,
}
impl From<DTR2_A> for bool {
    #[inline(always)]
    fn from(variant: DTR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTR2` reader - Dead Time Counter 2 Run bit"]
pub struct DTR2_R(crate::FieldReader<bool, DTR2_A>);
impl DTR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTR2_A {
        match self.bits {
            false => DTR2_A::VALUE1,
            true => DTR2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DTR2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DTR2_A::VALUE2
    }
}
impl core::ops::Deref for DTR2_R {
    type Target = crate::FieldReader<bool, DTR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Timer Run Bit"]
    #[inline(always)]
    pub fn trb(&self) -> TRB_R {
        TRB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Counting Direction"]
    #[inline(always)]
    pub fn cdir(&self) -> CDIR_R {
        CDIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dead Time Counter 1 Run bit"]
    #[inline(always)]
    pub fn dtr1(&self) -> DTR1_R {
        DTR1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Dead Time Counter 2 Run bit"]
    #[inline(always)]
    pub fn dtr2(&self) -> DTR2_R {
        DTR2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "Slice Timer Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcst](index.html) module"]
pub struct TCST_SPEC;
impl crate::RegisterSpec for TCST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcst::R](R) reader structure"]
impl crate::Readable for TCST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCST to value 0"]
impl crate::Resettable for TCST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
