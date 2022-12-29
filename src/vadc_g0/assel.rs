#[doc = "Register `ASSEL` reader"]
pub struct R(crate::R<ASSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASSEL` writer"]
pub struct W(crate::W<ASSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASSEL_SPEC>;
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
impl From<crate::W<ASSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL0` reader - Channel Selection"]
pub type CHSEL0_R = crate::BitReader<CHSEL0_A>;
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL0_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL0_A {
        match self.bits {
            false => CHSEL0_A::VALUE1,
            true => CHSEL0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL0_A::VALUE2
    }
}
#[doc = "Field `CHSEL0` writer - Channel Selection"]
pub type CHSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASSEL_SPEC, CHSEL0_A, O>;
impl<'a, const O: u8> CHSEL0_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL0_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL0_A::VALUE2)
    }
}
#[doc = "Field `CHSEL1` reader - Channel Selection"]
pub type CHSEL1_R = crate::BitReader<CHSEL1_A>;
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL1_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL1_A {
        match self.bits {
            false => CHSEL1_A::VALUE1,
            true => CHSEL1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL1_A::VALUE2
    }
}
#[doc = "Field `CHSEL1` writer - Channel Selection"]
pub type CHSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASSEL_SPEC, CHSEL1_A, O>;
impl<'a, const O: u8> CHSEL1_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL1_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL1_A::VALUE2)
    }
}
#[doc = "Field `CHSEL2` reader - Channel Selection"]
pub type CHSEL2_R = crate::BitReader<CHSEL2_A>;
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL2_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL2_A {
        match self.bits {
            false => CHSEL2_A::VALUE1,
            true => CHSEL2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL2_A::VALUE2
    }
}
#[doc = "Field `CHSEL2` writer - Channel Selection"]
pub type CHSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASSEL_SPEC, CHSEL2_A, O>;
impl<'a, const O: u8> CHSEL2_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL2_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL2_A::VALUE2)
    }
}
#[doc = "Field `CHSEL3` reader - Channel Selection"]
pub type CHSEL3_R = crate::BitReader<CHSEL3_A>;
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL3_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL3_A {
        match self.bits {
            false => CHSEL3_A::VALUE1,
            true => CHSEL3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL3_A::VALUE2
    }
}
#[doc = "Field `CHSEL3` writer - Channel Selection"]
pub type CHSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASSEL_SPEC, CHSEL3_A, O>;
impl<'a, const O: u8> CHSEL3_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL3_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL3_A::VALUE2)
    }
}
#[doc = "Field `CHSEL4` reader - Channel Selection"]
pub type CHSEL4_R = crate::BitReader<CHSEL4_A>;
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL4_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL4_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL4_A {
        match self.bits {
            false => CHSEL4_A::VALUE1,
            true => CHSEL4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL4_A::VALUE2
    }
}
#[doc = "Field `CHSEL4` writer - Channel Selection"]
pub type CHSEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASSEL_SPEC, CHSEL4_A, O>;
impl<'a, const O: u8> CHSEL4_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL4_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL4_A::VALUE2)
    }
}
#[doc = "Field `CHSEL5` reader - Channel Selection"]
pub type CHSEL5_R = crate::BitReader<CHSEL5_A>;
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL5_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL5_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL5_A {
        match self.bits {
            false => CHSEL5_A::VALUE1,
            true => CHSEL5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL5_A::VALUE2
    }
}
#[doc = "Field `CHSEL5` writer - Channel Selection"]
pub type CHSEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASSEL_SPEC, CHSEL5_A, O>;
impl<'a, const O: u8> CHSEL5_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL5_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL5_A::VALUE2)
    }
}
#[doc = "Field `CHSEL6` reader - Channel Selection"]
pub type CHSEL6_R = crate::BitReader<CHSEL6_A>;
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL6_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL6_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL6_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL6_A {
        match self.bits {
            false => CHSEL6_A::VALUE1,
            true => CHSEL6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL6_A::VALUE2
    }
}
#[doc = "Field `CHSEL6` writer - Channel Selection"]
pub type CHSEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASSEL_SPEC, CHSEL6_A, O>;
impl<'a, const O: u8> CHSEL6_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL6_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL6_A::VALUE2)
    }
}
#[doc = "Field `CHSEL7` reader - Channel Selection"]
pub type CHSEL7_R = crate::BitReader<CHSEL7_A>;
#[doc = "Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL7_A {
    #[doc = "0: Ignore this channel"]
    VALUE1 = 0,
    #[doc = "1: This channel is part of the scan sequence"]
    VALUE2 = 1,
}
impl From<CHSEL7_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL7_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL7_A {
        match self.bits {
            false => CHSEL7_A::VALUE1,
            true => CHSEL7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHSEL7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHSEL7_A::VALUE2
    }
}
#[doc = "Field `CHSEL7` writer - Channel Selection"]
pub type CHSEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASSEL_SPEC, CHSEL7_A, O>;
impl<'a, const O: u8> CHSEL7_W<'a, O> {
    #[doc = "Ignore this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHSEL7_A::VALUE1)
    }
    #[doc = "This channel is part of the scan sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHSEL7_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Selection"]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Selection"]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Selection"]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Selection"]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Selection"]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Selection"]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Selection"]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Selection"]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> CHSEL0_W<0> {
        CHSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> CHSEL1_W<1> {
        CHSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> CHSEL2_W<2> {
        CHSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> CHSEL3_W<3> {
        CHSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> CHSEL4_W<4> {
        CHSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> CHSEL5_W<5> {
        CHSEL5_W::new(self)
    }
    #[doc = "Bit 6 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> CHSEL6_W<6> {
        CHSEL6_W::new(self)
    }
    #[doc = "Bit 7 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> CHSEL7_W<7> {
        CHSEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Autoscan Source Channel Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assel](index.html) module"]
pub struct ASSEL_SPEC;
impl crate::RegisterSpec for ASSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [assel::R](R) reader structure"]
impl crate::Readable for ASSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [assel::W](W) writer structure"]
impl crate::Writable for ASSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASSEL to value 0"]
impl crate::Resettable for ASSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
