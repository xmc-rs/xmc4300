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
#[doc = "Period/One match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POSR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<POSR_A> for u8 {
    #[inline(always)]
    fn from(variant: POSR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POSR` reader - Period/One match Service request selector"]
pub struct POSR_R(crate::FieldReader<u8, POSR_A>);
impl POSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        POSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == POSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == POSR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == POSR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == POSR_A::VALUE4
    }
}
impl core::ops::Deref for POSR_R {
    type Target = crate::FieldReader<u8, POSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSR` writer - Period/One match Service request selector"]
pub struct POSR_W<'a> {
    w: &'a mut W,
}
impl<'a> POSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(POSR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(POSR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Compare match Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMSR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<CMSR_A> for u8 {
    #[inline(always)]
    fn from(variant: CMSR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMSR` reader - Compare match Service request selector"]
pub struct CMSR_R(crate::FieldReader<u8, CMSR_A>);
impl CMSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMSR_A {
        match self.bits {
            0 => CMSR_A::VALUE1,
            1 => CMSR_A::VALUE2,
            2 => CMSR_A::VALUE3,
            3 => CMSR_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMSR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CMSR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CMSR_A::VALUE4
    }
}
impl core::ops::Deref for CMSR_R {
    type Target = crate::FieldReader<u8, CMSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMSR` writer - Compare match Service request selector"]
pub struct CMSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMSR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMSR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMSR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMSR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMSR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Event 0 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum E0SR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<E0SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E0SR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `E0SR` reader - Event 0 Service request selector"]
pub struct E0SR_R(crate::FieldReader<u8, E0SR_A>);
impl E0SR_R {
    pub(crate) fn new(bits: u8) -> Self {
        E0SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == E0SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == E0SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == E0SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == E0SR_A::VALUE4
    }
}
impl core::ops::Deref for E0SR_R {
    type Target = crate::FieldReader<u8, E0SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E0SR` writer - Event 0 Service request selector"]
pub struct E0SR_W<'a> {
    w: &'a mut W,
}
impl<'a> E0SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E0SR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(E0SR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Event 1 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum E1SR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<E1SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E1SR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `E1SR` reader - Event 1 Service request selector"]
pub struct E1SR_R(crate::FieldReader<u8, E1SR_A>);
impl E1SR_R {
    pub(crate) fn new(bits: u8) -> Self {
        E1SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == E1SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == E1SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == E1SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == E1SR_A::VALUE4
    }
}
impl core::ops::Deref for E1SR_R {
    type Target = crate::FieldReader<u8, E1SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E1SR` writer - Event 1 Service request selector"]
pub struct E1SR_W<'a> {
    w: &'a mut W,
}
impl<'a> E1SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E1SR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(E1SR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Event 2 Service request selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum E2SR_A {
    #[doc = "0: Forward to CC4ySR0"]
    VALUE1 = 0,
    #[doc = "1: Forward to CC4ySR1"]
    VALUE2 = 1,
    #[doc = "2: Forward to CC4ySR2"]
    VALUE3 = 2,
    #[doc = "3: Forward to CC4ySR3"]
    VALUE4 = 3,
}
impl From<E2SR_A> for u8 {
    #[inline(always)]
    fn from(variant: E2SR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `E2SR` reader - Event 2 Service request selector"]
pub struct E2SR_R(crate::FieldReader<u8, E2SR_A>);
impl E2SR_R {
    pub(crate) fn new(bits: u8) -> Self {
        E2SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == E2SR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == E2SR_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == E2SR_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == E2SR_A::VALUE4
    }
}
impl core::ops::Deref for E2SR_R {
    type Target = crate::FieldReader<u8, E2SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E2SR` writer - Event 2 Service request selector"]
pub struct E2SR_W<'a> {
    w: &'a mut W,
}
impl<'a> E2SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E2SR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Forward to CC4ySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE1)
    }
    #[doc = "Forward to CC4ySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE2)
    }
    #[doc = "Forward to CC4ySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE3)
    }
    #[doc = "Forward to CC4ySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(E2SR_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    pub fn posr(&self) -> POSR_R {
        POSR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Compare match Service request selector"]
    #[inline(always)]
    pub fn cmsr(&self) -> CMSR_R {
        CMSR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    pub fn e0sr(&self) -> E0SR_R {
        E0SR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    pub fn e1sr(&self) -> E1SR_R {
        E1SR_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    pub fn e2sr(&self) -> E2SR_R {
        E2SR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Period/One match Service request selector"]
    #[inline(always)]
    pub fn posr(&mut self) -> POSR_W {
        POSR_W { w: self }
    }
    #[doc = "Bits 2:3 - Compare match Service request selector"]
    #[inline(always)]
    pub fn cmsr(&mut self) -> CMSR_W {
        CMSR_W { w: self }
    }
    #[doc = "Bits 8:9 - Event 0 Service request selector"]
    #[inline(always)]
    pub fn e0sr(&mut self) -> E0SR_W {
        E0SR_W { w: self }
    }
    #[doc = "Bits 10:11 - Event 1 Service request selector"]
    #[inline(always)]
    pub fn e1sr(&mut self) -> E1SR_W {
        E1SR_W { w: self }
    }
    #[doc = "Bits 12:13 - Event 2 Service request selector"]
    #[inline(always)]
    pub fn e2sr(&mut self) -> E2SR_W {
        E2SR_W { w: self }
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
}
#[doc = "`reset()` method sets SRS to value 0"]
impl crate::Resettable for SRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
