#[doc = "Register `SRS` reader"]
pub struct R(crate::R<SRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRS` writer"]
pub struct W(crate::W<SRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRS_SPEC>;
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
impl From<crate::W<SRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POSR` reader - Period/One match Service request selector"]
pub type POSR_R = crate::FieldReader<u8, POSR_A>;
#[doc = "Period/One match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POSR_A {
    #[doc = "0: Forward to CC8ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    VALUE4 = 3,
}
impl From<POSR_A> for u8 {
    #[inline(always)]
    fn from(variant: POSR_A) -> Self {
        variant as _
    }
}
impl POSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSR_A {
        match self.bits {
            0 => POSR_A::VALUE1,
            1 => POSR_A::VALUE2,
            2 => POSR_A::VALUE3,
            3 => POSR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POSR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == POSR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == POSR_A::VALUE4
    }
}
#[doc = "Field `POSR` writer - Period/One match Service request selector"]
pub type POSR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRS_SPEC, u8, POSR_A, 2, O>;
impl<'a, const O: u8> POSR_W<'a, O> {
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSR_A::VALUE1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSR_A::VALUE2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(POSR_A::VALUE3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(POSR_A::VALUE4)
    }
}
#[doc = "Field `CM1SR` reader - Channel 1 Compare match Service request selector"]
pub type CM1SR_R = crate::FieldReader<u8, CM1SR_A>;
#[doc = "Channel 1 Compare match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM1SR_A {
    #[doc = "0: Forward to CC8ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    VALUE4 = 3,
}
impl From<CM1SR_A> for u8 {
    #[inline(always)]
    fn from(variant: CM1SR_A) -> Self {
        variant as _
    }
}
impl CM1SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM1SR_A {
        match self.bits {
            0 => CM1SR_A::VALUE1,
            1 => CM1SR_A::VALUE2,
            2 => CM1SR_A::VALUE3,
            3 => CM1SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CM1SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CM1SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CM1SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CM1SR_A::VALUE4
    }
}
#[doc = "Field `CM1SR` writer - Channel 1 Compare match Service request selector"]
pub type CM1SR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRS_SPEC, u8, CM1SR_A, 2, O>;
impl<'a, const O: u8> CM1SR_W<'a, O> {
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CM1SR_A::VALUE1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CM1SR_A::VALUE2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CM1SR_A::VALUE3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CM1SR_A::VALUE4)
    }
}
#[doc = "Field `CM2SR` reader - Channel 2 Compare match Service request selector"]
pub type CM2SR_R = crate::FieldReader<u8, CM2SR_A>;
#[doc = "Channel 2 Compare match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM2SR_A {
    #[doc = "0: Forward to CC8ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    VALUE4 = 3,
}
impl From<CM2SR_A> for u8 {
    #[inline(always)]
    fn from(variant: CM2SR_A) -> Self {
        variant as _
    }
}
impl CM2SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM2SR_A {
        match self.bits {
            0 => CM2SR_A::VALUE1,
            1 => CM2SR_A::VALUE2,
            2 => CM2SR_A::VALUE3,
            3 => CM2SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CM2SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CM2SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CM2SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CM2SR_A::VALUE4
    }
}
#[doc = "Field `CM2SR` writer - Channel 2 Compare match Service request selector"]
pub type CM2SR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRS_SPEC, u8, CM2SR_A, 2, O>;
impl<'a, const O: u8> CM2SR_W<'a, O> {
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CM2SR_A::VALUE1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CM2SR_A::VALUE2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CM2SR_A::VALUE3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CM2SR_A::VALUE4)
    }
}
#[doc = "Field `E0SR` reader - Event 0 Service request selector"]
pub type E0SR_R = crate::FieldReader<u8, E0SR_A>;
#[doc = "Event 0 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E0SR_A {
    #[doc = "0: Forward to CCvySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    VALUE4 = 3,
}
impl From<E0SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E0SR_A) -> Self {
        variant as _
    }
}
impl E0SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E0SR_A {
        match self.bits {
            0 => E0SR_A::VALUE1,
            1 => E0SR_A::VALUE2,
            2 => E0SR_A::VALUE3,
            3 => E0SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E0SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E0SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E0SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E0SR_A::VALUE4
    }
}
#[doc = "Field `E0SR` writer - Event 0 Service request selector"]
pub type E0SR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRS_SPEC, u8, E0SR_A, 2, O>;
impl<'a, const O: u8> E0SR_W<'a, O> {
    #[doc = "Forward to CCvySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE4)
    }
}
#[doc = "Field `E1SR` reader - Event 1 Service request selector"]
pub type E1SR_R = crate::FieldReader<u8, E1SR_A>;
#[doc = "Event 1 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E1SR_A {
    #[doc = "0: Forward to CC8ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC8ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    VALUE4 = 3,
}
impl From<E1SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E1SR_A) -> Self {
        variant as _
    }
}
impl E1SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E1SR_A {
        match self.bits {
            0 => E1SR_A::VALUE1,
            1 => E1SR_A::VALUE2,
            2 => E1SR_A::VALUE3,
            3 => E1SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E1SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E1SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E1SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E1SR_A::VALUE4
    }
}
#[doc = "Field `E1SR` writer - Event 1 Service request selector"]
pub type E1SR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRS_SPEC, u8, E1SR_A, 2, O>;
impl<'a, const O: u8> E1SR_W<'a, O> {
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE1)
    }
    #[doc = "Forward to CC8ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE4)
    }
}
#[doc = "Field `E2SR` reader - Event 2 Service request selector"]
pub type E2SR_R = crate::FieldReader<u8, E2SR_A>;
#[doc = "Event 2 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E2SR_A {
    #[doc = "0: Forward to CC8ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CCvySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC8ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC8ySR3"]
    VALUE4 = 3,
}
impl From<E2SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E2SR_A) -> Self {
        variant as _
    }
}
impl E2SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E2SR_A {
        match self.bits {
            0 => E2SR_A::VALUE1,
            1 => E2SR_A::VALUE2,
            2 => E2SR_A::VALUE3,
            3 => E2SR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == E2SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == E2SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == E2SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == E2SR_A::VALUE4
    }
}
#[doc = "Field `E2SR` writer - Event 2 Service request selector"]
pub type E2SR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRS_SPEC, u8, E2SR_A, 2, O>;
impl<'a, const O: u8> E2SR_W<'a, O> {
    #[doc = "Forward to CC8ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE1)
    }
    #[doc = "Forward to CCvySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE2)
    }
    #[doc = "Forward to CC8ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE3)
    }
    #[doc = "Forward to CC8ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    pub fn posr(&self) -> POSR_R {
        POSR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 Compare match Service request selector"]
    #[inline(always)]
    pub fn cm1sr(&self) -> CM1SR_R {
        CM1SR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Channel 2 Compare match Service request selector"]
    #[inline(always)]
    pub fn cm2sr(&self) -> CM2SR_R {
        CM2SR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    pub fn e0sr(&self) -> E0SR_R {
        E0SR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    pub fn e1sr(&self) -> E1SR_R {
        E1SR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    pub fn e2sr(&self) -> E2SR_R {
        E2SR_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn posr(&mut self) -> POSR_W<0> {
        POSR_W::new(self)
    }
    #[doc = "Bits 2:3 - Channel 1 Compare match Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn cm1sr(&mut self) -> CM1SR_W<2> {
        CM1SR_W::new(self)
    }
    #[doc = "Bits 4:5 - Channel 2 Compare match Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn cm2sr(&mut self) -> CM2SR_W<4> {
        CM2SR_W::new(self)
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn e0sr(&mut self) -> E0SR_W<8> {
        E0SR_W::new(self)
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn e1sr(&mut self) -> E1SR_W<10> {
        E1SR_W::new(self)
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    #[must_use]
    pub fn e2sr(&mut self) -> E2SR_W<12> {
        E2SR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service Request Selector\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs](index.html) module"]
pub struct SRS_SPEC;
impl crate::RegisterSpec for SRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srs::R](R) reader structure"]
impl crate::Readable for SRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srs::W](W) writer structure"]
impl crate::Writable for SRS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRS to value 0"]
impl crate::Resettable for SRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
