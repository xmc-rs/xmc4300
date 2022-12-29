#[doc = "Register `CHASS` reader"]
pub struct R(crate::R<CHASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHASS` writer"]
pub struct W(crate::W<CHASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CHASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASSCH0` reader - Assignment for Channel 0"]
pub type ASSCH0_R = crate::BitReader<ASSCH0_A>;
#[doc = "Assignment for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH0_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH0_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH0_A) -> Self {
        variant as u8 != 0
    }
}
impl ASSCH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH0_A {
        match self.bits {
            false => ASSCH0_A::VALUE1,
            true => ASSCH0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH0_A::VALUE2
    }
}
#[doc = "Field `ASSCH0` writer - Assignment for Channel 0"]
pub type ASSCH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHASS_SPEC, ASSCH0_A, O>;
impl<'a, const O: u8> ASSCH0_W<'a, O> {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH0_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH0_A::VALUE2)
    }
}
#[doc = "Field `ASSCH1` reader - Assignment for Channel 1"]
pub type ASSCH1_R = crate::BitReader<ASSCH1_A>;
#[doc = "Assignment for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH1_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH1_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH1_A) -> Self {
        variant as u8 != 0
    }
}
impl ASSCH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH1_A {
        match self.bits {
            false => ASSCH1_A::VALUE1,
            true => ASSCH1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH1_A::VALUE2
    }
}
#[doc = "Field `ASSCH1` writer - Assignment for Channel 1"]
pub type ASSCH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHASS_SPEC, ASSCH1_A, O>;
impl<'a, const O: u8> ASSCH1_W<'a, O> {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH1_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH1_A::VALUE2)
    }
}
#[doc = "Field `ASSCH2` reader - Assignment for Channel 2"]
pub type ASSCH2_R = crate::BitReader<ASSCH2_A>;
#[doc = "Assignment for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH2_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH2_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH2_A) -> Self {
        variant as u8 != 0
    }
}
impl ASSCH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH2_A {
        match self.bits {
            false => ASSCH2_A::VALUE1,
            true => ASSCH2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH2_A::VALUE2
    }
}
#[doc = "Field `ASSCH2` writer - Assignment for Channel 2"]
pub type ASSCH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHASS_SPEC, ASSCH2_A, O>;
impl<'a, const O: u8> ASSCH2_W<'a, O> {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH2_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH2_A::VALUE2)
    }
}
#[doc = "Field `ASSCH3` reader - Assignment for Channel 3"]
pub type ASSCH3_R = crate::BitReader<ASSCH3_A>;
#[doc = "Assignment for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH3_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH3_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH3_A) -> Self {
        variant as u8 != 0
    }
}
impl ASSCH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH3_A {
        match self.bits {
            false => ASSCH3_A::VALUE1,
            true => ASSCH3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH3_A::VALUE2
    }
}
#[doc = "Field `ASSCH3` writer - Assignment for Channel 3"]
pub type ASSCH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHASS_SPEC, ASSCH3_A, O>;
impl<'a, const O: u8> ASSCH3_W<'a, O> {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH3_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH3_A::VALUE2)
    }
}
#[doc = "Field `ASSCH4` reader - Assignment for Channel 4"]
pub type ASSCH4_R = crate::BitReader<ASSCH4_A>;
#[doc = "Assignment for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH4_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH4_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH4_A) -> Self {
        variant as u8 != 0
    }
}
impl ASSCH4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH4_A {
        match self.bits {
            false => ASSCH4_A::VALUE1,
            true => ASSCH4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH4_A::VALUE2
    }
}
#[doc = "Field `ASSCH4` writer - Assignment for Channel 4"]
pub type ASSCH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHASS_SPEC, ASSCH4_A, O>;
impl<'a, const O: u8> ASSCH4_W<'a, O> {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH4_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH4_A::VALUE2)
    }
}
#[doc = "Field `ASSCH5` reader - Assignment for Channel 5"]
pub type ASSCH5_R = crate::BitReader<ASSCH5_A>;
#[doc = "Assignment for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH5_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH5_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH5_A) -> Self {
        variant as u8 != 0
    }
}
impl ASSCH5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH5_A {
        match self.bits {
            false => ASSCH5_A::VALUE1,
            true => ASSCH5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH5_A::VALUE2
    }
}
#[doc = "Field `ASSCH5` writer - Assignment for Channel 5"]
pub type ASSCH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHASS_SPEC, ASSCH5_A, O>;
impl<'a, const O: u8> ASSCH5_W<'a, O> {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH5_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH5_A::VALUE2)
    }
}
#[doc = "Field `ASSCH6` reader - Assignment for Channel 6"]
pub type ASSCH6_R = crate::BitReader<ASSCH6_A>;
#[doc = "Assignment for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH6_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH6_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH6_A) -> Self {
        variant as u8 != 0
    }
}
impl ASSCH6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH6_A {
        match self.bits {
            false => ASSCH6_A::VALUE1,
            true => ASSCH6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH6_A::VALUE2
    }
}
#[doc = "Field `ASSCH6` writer - Assignment for Channel 6"]
pub type ASSCH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHASS_SPEC, ASSCH6_A, O>;
impl<'a, const O: u8> ASSCH6_W<'a, O> {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH6_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH6_A::VALUE2)
    }
}
#[doc = "Field `ASSCH7` reader - Assignment for Channel 7"]
pub type ASSCH7_R = crate::BitReader<ASSCH7_A>;
#[doc = "Assignment for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSCH7_A {
    #[doc = "0: Channel y can be a background channel converted with lowest priority"]
    VALUE1 = 0,
    #[doc = "1: Channel y is a priority channel within group x"]
    VALUE2 = 1,
}
impl From<ASSCH7_A> for bool {
    #[inline(always)]
    fn from(variant: ASSCH7_A) -> Self {
        variant as u8 != 0
    }
}
impl ASSCH7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSCH7_A {
        match self.bits {
            false => ASSCH7_A::VALUE1,
            true => ASSCH7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASSCH7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASSCH7_A::VALUE2
    }
}
#[doc = "Field `ASSCH7` writer - Assignment for Channel 7"]
pub type ASSCH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHASS_SPEC, ASSCH7_A, O>;
impl<'a, const O: u8> ASSCH7_W<'a, O> {
    #[doc = "Channel y can be a background channel converted with lowest priority"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSCH7_A::VALUE1)
    }
    #[doc = "Channel y is a priority channel within group x"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSCH7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    pub fn assch0(&self) -> ASSCH0_R {
        ASSCH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    pub fn assch1(&self) -> ASSCH1_R {
        ASSCH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    pub fn assch2(&self) -> ASSCH2_R {
        ASSCH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    pub fn assch3(&self) -> ASSCH3_R {
        ASSCH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    pub fn assch4(&self) -> ASSCH4_R {
        ASSCH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    pub fn assch5(&self) -> ASSCH5_R {
        ASSCH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    pub fn assch6(&self) -> ASSCH6_R {
        ASSCH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    pub fn assch7(&self) -> ASSCH7_R {
        ASSCH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assignment for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn assch0(&mut self) -> ASSCH0_W<0> {
        ASSCH0_W::new(self)
    }
    #[doc = "Bit 1 - Assignment for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn assch1(&mut self) -> ASSCH1_W<1> {
        ASSCH1_W::new(self)
    }
    #[doc = "Bit 2 - Assignment for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn assch2(&mut self) -> ASSCH2_W<2> {
        ASSCH2_W::new(self)
    }
    #[doc = "Bit 3 - Assignment for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn assch3(&mut self) -> ASSCH3_W<3> {
        ASSCH3_W::new(self)
    }
    #[doc = "Bit 4 - Assignment for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn assch4(&mut self) -> ASSCH4_W<4> {
        ASSCH4_W::new(self)
    }
    #[doc = "Bit 5 - Assignment for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn assch5(&mut self) -> ASSCH5_W<5> {
        ASSCH5_W::new(self)
    }
    #[doc = "Bit 6 - Assignment for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn assch6(&mut self) -> ASSCH6_W<6> {
        ASSCH6_W::new(self)
    }
    #[doc = "Bit 7 - Assignment for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn assch7(&mut self) -> ASSCH7_W<7> {
        ASSCH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Assignment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chass](index.html) module"]
pub struct CHASS_SPEC;
impl crate::RegisterSpec for CHASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chass::R](R) reader structure"]
impl crate::Readable for CHASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chass::W](W) writer structure"]
impl crate::Writable for CHASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHASS to value 0"]
impl crate::Resettable for CHASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
