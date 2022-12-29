#[doc = "Register `BRSPND[%s]` reader"]
pub struct R(crate::R<BRSPND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRSPND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRSPND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRSPND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRSPND[%s]` writer"]
pub struct W(crate::W<BRSPND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRSPND_SPEC>;
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
impl From<crate::W<BRSPND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRSPND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHPNDG0` reader - Channels Pending Group x"]
pub type CHPNDG0_R = crate::BitReader<CHPNDG0_A>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG0_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPNDG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG0_A {
        match self.bits {
            false => CHPNDG0_A::VALUE1,
            true => CHPNDG0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG0_A::VALUE2
    }
}
#[doc = "Field `CHPNDG0` writer - Channels Pending Group x"]
pub type CHPNDG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRSPND_SPEC, CHPNDG0_A, O>;
impl<'a, const O: u8> CHPNDG0_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG0_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG0_A::VALUE2)
    }
}
#[doc = "Field `CHPNDG1` reader - Channels Pending Group x"]
pub type CHPNDG1_R = crate::BitReader<CHPNDG1_A>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG1_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPNDG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG1_A {
        match self.bits {
            false => CHPNDG1_A::VALUE1,
            true => CHPNDG1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG1_A::VALUE2
    }
}
#[doc = "Field `CHPNDG1` writer - Channels Pending Group x"]
pub type CHPNDG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRSPND_SPEC, CHPNDG1_A, O>;
impl<'a, const O: u8> CHPNDG1_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG1_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG1_A::VALUE2)
    }
}
#[doc = "Field `CHPNDG2` reader - Channels Pending Group x"]
pub type CHPNDG2_R = crate::BitReader<CHPNDG2_A>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG2_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPNDG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG2_A {
        match self.bits {
            false => CHPNDG2_A::VALUE1,
            true => CHPNDG2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG2_A::VALUE2
    }
}
#[doc = "Field `CHPNDG2` writer - Channels Pending Group x"]
pub type CHPNDG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRSPND_SPEC, CHPNDG2_A, O>;
impl<'a, const O: u8> CHPNDG2_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG2_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG2_A::VALUE2)
    }
}
#[doc = "Field `CHPNDG3` reader - Channels Pending Group x"]
pub type CHPNDG3_R = crate::BitReader<CHPNDG3_A>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG3_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG3_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPNDG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG3_A {
        match self.bits {
            false => CHPNDG3_A::VALUE1,
            true => CHPNDG3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG3_A::VALUE2
    }
}
#[doc = "Field `CHPNDG3` writer - Channels Pending Group x"]
pub type CHPNDG3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRSPND_SPEC, CHPNDG3_A, O>;
impl<'a, const O: u8> CHPNDG3_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG3_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG3_A::VALUE2)
    }
}
#[doc = "Field `CHPNDG4` reader - Channels Pending Group x"]
pub type CHPNDG4_R = crate::BitReader<CHPNDG4_A>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG4_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG4_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPNDG4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG4_A {
        match self.bits {
            false => CHPNDG4_A::VALUE1,
            true => CHPNDG4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG4_A::VALUE2
    }
}
#[doc = "Field `CHPNDG4` writer - Channels Pending Group x"]
pub type CHPNDG4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRSPND_SPEC, CHPNDG4_A, O>;
impl<'a, const O: u8> CHPNDG4_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG4_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG4_A::VALUE2)
    }
}
#[doc = "Field `CHPNDG5` reader - Channels Pending Group x"]
pub type CHPNDG5_R = crate::BitReader<CHPNDG5_A>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG5_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG5_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPNDG5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG5_A {
        match self.bits {
            false => CHPNDG5_A::VALUE1,
            true => CHPNDG5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG5_A::VALUE2
    }
}
#[doc = "Field `CHPNDG5` writer - Channels Pending Group x"]
pub type CHPNDG5_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRSPND_SPEC, CHPNDG5_A, O>;
impl<'a, const O: u8> CHPNDG5_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG5_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG5_A::VALUE2)
    }
}
#[doc = "Field `CHPNDG6` reader - Channels Pending Group x"]
pub type CHPNDG6_R = crate::BitReader<CHPNDG6_A>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG6_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG6_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPNDG6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG6_A {
        match self.bits {
            false => CHPNDG6_A::VALUE1,
            true => CHPNDG6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG6_A::VALUE2
    }
}
#[doc = "Field `CHPNDG6` writer - Channels Pending Group x"]
pub type CHPNDG6_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRSPND_SPEC, CHPNDG6_A, O>;
impl<'a, const O: u8> CHPNDG6_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG6_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG6_A::VALUE2)
    }
}
#[doc = "Field `CHPNDG7` reader - Channels Pending Group x"]
pub type CHPNDG7_R = crate::BitReader<CHPNDG7_A>;
#[doc = "Channels Pending Group x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHPNDG7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: Request conversion of this channel"]
    VALUE2 = 1,
}
impl From<CHPNDG7_A> for bool {
    #[inline(always)]
    fn from(variant: CHPNDG7_A) -> Self {
        variant as u8 != 0
    }
}
impl CHPNDG7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPNDG7_A {
        match self.bits {
            false => CHPNDG7_A::VALUE1,
            true => CHPNDG7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHPNDG7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHPNDG7_A::VALUE2
    }
}
#[doc = "Field `CHPNDG7` writer - Channels Pending Group x"]
pub type CHPNDG7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRSPND_SPEC, CHPNDG7_A, O>;
impl<'a, const O: u8> CHPNDG7_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHPNDG7_A::VALUE1)
    }
    #[doc = "Request conversion of this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHPNDG7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg0(&self) -> CHPNDG0_R {
        CHPNDG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg1(&self) -> CHPNDG1_R {
        CHPNDG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg2(&self) -> CHPNDG2_R {
        CHPNDG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg3(&self) -> CHPNDG3_R {
        CHPNDG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg4(&self) -> CHPNDG4_R {
        CHPNDG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg5(&self) -> CHPNDG5_R {
        CHPNDG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg6(&self) -> CHPNDG6_R {
        CHPNDG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline(always)]
    pub fn chpndg7(&self) -> CHPNDG7_R {
        CHPNDG7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg0(&mut self) -> CHPNDG0_W<0> {
        CHPNDG0_W::new(self)
    }
    #[doc = "Bit 1 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg1(&mut self) -> CHPNDG1_W<1> {
        CHPNDG1_W::new(self)
    }
    #[doc = "Bit 2 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg2(&mut self) -> CHPNDG2_W<2> {
        CHPNDG2_W::new(self)
    }
    #[doc = "Bit 3 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg3(&mut self) -> CHPNDG3_W<3> {
        CHPNDG3_W::new(self)
    }
    #[doc = "Bit 4 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg4(&mut self) -> CHPNDG4_W<4> {
        CHPNDG4_W::new(self)
    }
    #[doc = "Bit 5 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg5(&mut self) -> CHPNDG5_W<5> {
        CHPNDG5_W::new(self)
    }
    #[doc = "Bit 6 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg6(&mut self) -> CHPNDG6_W<6> {
        CHPNDG6_W::new(self)
    }
    #[doc = "Bit 7 - Channels Pending Group x"]
    #[inline(always)]
    #[must_use]
    pub fn chpndg7(&mut self) -> CHPNDG7_W<7> {
        CHPNDG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Request Source Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brspnd](index.html) module"]
pub struct BRSPND_SPEC;
impl crate::RegisterSpec for BRSPND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brspnd::R](R) reader structure"]
impl crate::Readable for BRSPND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brspnd::W](W) writer structure"]
impl crate::Writable for BRSPND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRSPND[%s]
to value 0"]
impl crate::Resettable for BRSPND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
