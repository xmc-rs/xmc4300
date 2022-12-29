#[doc = "Register `ASPND` reader"]
pub struct R(crate::R<ASPND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASPND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASPND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASPND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASPND` writer"]
pub struct W(crate::W<ASPND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASPND_SPEC>;
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
impl From<crate::W<ASPND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASPND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHPND0` reader - Channels Pending"]
pub type CHPND0_R = crate::BitReader<CHPND0_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND0_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND0_A {
        match self.bits {
            false => CHPND0_A::VALUE1,
            true => CHPND0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND0_A::VALUE2
    }
}
#[doc = "Field `CHPND0` writer - Channels Pending"]
pub type CHPND0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASPND_SPEC, CHPND0_A, O>;
impl<'a, const O: u8> CHPND0_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND0_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND0_A::VALUE2)
    }
}
#[doc = "Field `CHPND1` reader - Channels Pending"]
pub type CHPND1_R = crate::BitReader<CHPND1_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND1_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND1_A {
        match self.bits {
            false => CHPND1_A::VALUE1,
            true => CHPND1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND1_A::VALUE2
    }
}
#[doc = "Field `CHPND1` writer - Channels Pending"]
pub type CHPND1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASPND_SPEC, CHPND1_A, O>;
impl<'a, const O: u8> CHPND1_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND1_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND1_A::VALUE2)
    }
}
#[doc = "Field `CHPND2` reader - Channels Pending"]
pub type CHPND2_R = crate::BitReader<CHPND2_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND2_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND2_A {
        match self.bits {
            false => CHPND2_A::VALUE1,
            true => CHPND2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND2_A::VALUE2
    }
}
#[doc = "Field `CHPND2` writer - Channels Pending"]
pub type CHPND2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASPND_SPEC, CHPND2_A, O>;
impl<'a, const O: u8> CHPND2_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND2_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND2_A::VALUE2)
    }
}
#[doc = "Field `CHPND3` reader - Channels Pending"]
pub type CHPND3_R = crate::BitReader<CHPND3_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND3_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND3_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND3_A {
        match self.bits {
            false => CHPND3_A::VALUE1,
            true => CHPND3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND3_A::VALUE2
    }
}
#[doc = "Field `CHPND3` writer - Channels Pending"]
pub type CHPND3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASPND_SPEC, CHPND3_A, O>;
impl<'a, const O: u8> CHPND3_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND3_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND3_A::VALUE2)
    }
}
#[doc = "Field `CHPND4` reader - Channels Pending"]
pub type CHPND4_R = crate::BitReader<CHPND4_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND4_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND4_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND4_A {
        match self.bits {
            false => CHPND4_A::VALUE1,
            true => CHPND4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND4_A::VALUE2
    }
}
#[doc = "Field `CHPND4` writer - Channels Pending"]
pub type CHPND4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASPND_SPEC, CHPND4_A, O>;
impl<'a, const O: u8> CHPND4_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND4_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND4_A::VALUE2)
    }
}
#[doc = "Field `CHPND5` reader - Channels Pending"]
pub type CHPND5_R = crate::BitReader<CHPND5_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND5_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND5_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND5_A {
        match self.bits {
            false => CHPND5_A::VALUE1,
            true => CHPND5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND5_A::VALUE2
    }
}
#[doc = "Field `CHPND5` writer - Channels Pending"]
pub type CHPND5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASPND_SPEC, CHPND5_A, O>;
impl<'a, const O: u8> CHPND5_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND5_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND5_A::VALUE2)
    }
}
#[doc = "Field `CHPND6` reader - Channels Pending"]
pub type CHPND6_R = crate::BitReader<CHPND6_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND6_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND6_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND6_A {
        match self.bits {
            false => CHPND6_A::VALUE1,
            true => CHPND6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND6_A::VALUE2
    }
}
#[doc = "Field `CHPND6` writer - Channels Pending"]
pub type CHPND6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASPND_SPEC, CHPND6_A, O>;
impl<'a, const O: u8> CHPND6_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND6_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND6_A::VALUE2)
    }
}
#[doc = "Field `CHPND7` reader - Channels Pending"]
pub type CHPND7_R = crate::BitReader<CHPND7_A>;
#[doc = "Channels Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPND7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPND7_A> for bool {
    #[inline(always)]
    fn from(variant: CHPND7_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPND7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPND7_A {
        match self.bits {
            false => CHPND7_A::VALUE1,
            true => CHPND7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPND7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPND7_A::VALUE2
    }
}
#[doc = "Field `CHPND7` writer - Channels Pending"]
pub type CHPND7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASPND_SPEC, CHPND7_A, O>;
impl<'a, const O: u8> CHPND7_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPND7_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPND7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd0(&self) -> CHPND0_R {
        CHPND0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd1(&self) -> CHPND1_R {
        CHPND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd2(&self) -> CHPND2_R {
        CHPND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd3(&self) -> CHPND3_R {
        CHPND3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd4(&self) -> CHPND4_R {
        CHPND4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd5(&self) -> CHPND5_R {
        CHPND5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd6(&self) -> CHPND6_R {
        CHPND6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline(always)]
    pub fn chpnd7(&self) -> CHPND7_R {
        CHPND7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd0(&mut self) -> CHPND0_W<0> {
        CHPND0_W::new(self)
    }
    #[doc = "Bit 1 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd1(&mut self) -> CHPND1_W<1> {
        CHPND1_W::new(self)
    }
    #[doc = "Bit 2 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd2(&mut self) -> CHPND2_W<2> {
        CHPND2_W::new(self)
    }
    #[doc = "Bit 3 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd3(&mut self) -> CHPND3_W<3> {
        CHPND3_W::new(self)
    }
    #[doc = "Bit 4 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd4(&mut self) -> CHPND4_W<4> {
        CHPND4_W::new(self)
    }
    #[doc = "Bit 5 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd5(&mut self) -> CHPND5_W<5> {
        CHPND5_W::new(self)
    }
    #[doc = "Bit 6 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd6(&mut self) -> CHPND6_W<6> {
        CHPND6_W::new(self)
    }
    #[doc = "Bit 7 - Channels Pending"]
    #[inline(always)]
    #[must_use]
    pub fn chpnd7(&mut self) -> CHPND7_W<7> {
        CHPND7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Autoscan Source Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aspnd](index.html) module"]
pub struct ASPND_SPEC;
impl crate::RegisterSpec for ASPND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aspnd::R](R) reader structure"]
impl crate::Readable for ASPND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aspnd::W](W) writer structure"]
impl crate::Writable for ASPND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASPND to value 0"]
impl crate::Resettable for ASPND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
