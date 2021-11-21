#[doc = "Register `PANCTR` reader"]
pub struct R(crate::R<PANCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PANCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PANCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PANCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PANCTR` writer"]
pub struct W(crate::W<PANCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PANCTR_SPEC>;
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
impl From<crate::W<PANCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PANCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PANCMD` reader - Panel Command"]
pub struct PANCMD_R(crate::FieldReader<u8, u8>);
impl PANCMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        PANCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PANCMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PANCMD` writer - Panel Command"]
pub struct PANCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PANCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Panel Busy Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Panel has finished command and is ready to accept a new command."]
    VALUE1 = 0,
    #[doc = "1: Panel operation is in progress."]
    VALUE2 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Panel Busy Flag"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUSY_A::VALUE2
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Result Busy Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBUSY_A {
    #[doc = "0: No update of PANAR1 and PANAR2 is scheduled by the list controller."]
    VALUE1 = 0,
    #[doc = "1: A list command is running (BUSY = 1) that will write results to PANAR1 and PANAR2, but the results are not yet available."]
    VALUE2 = 1,
}
impl From<RBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: RBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBUSY` reader - Result Busy Flag"]
pub struct RBUSY_R(crate::FieldReader<bool, RBUSY_A>);
impl RBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBUSY_A {
        match self.bits {
            false => RBUSY_A::VALUE1,
            true => RBUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RBUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RBUSY_A::VALUE2
    }
}
impl core::ops::Deref for RBUSY_R {
    type Target = crate::FieldReader<bool, RBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PANAR1` reader - Panel Argument 1"]
pub struct PANAR1_R(crate::FieldReader<u8, u8>);
impl PANAR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PANAR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PANAR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PANAR1` writer - Panel Argument 1"]
pub struct PANAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PANAR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PANAR2` reader - Panel Argument 2"]
pub struct PANAR2_R(crate::FieldReader<u8, u8>);
impl PANAR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PANAR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PANAR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PANAR2` writer - Panel Argument 2"]
pub struct PANAR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PANAR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Panel Command"]
    #[inline(always)]
    pub fn pancmd(&self) -> PANCMD_R {
        PANCMD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Panel Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Result Busy Flag"]
    #[inline(always)]
    pub fn rbusy(&self) -> RBUSY_R {
        RBUSY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Panel Argument 1"]
    #[inline(always)]
    pub fn panar1(&self) -> PANAR1_R {
        PANAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Panel Argument 2"]
    #[inline(always)]
    pub fn panar2(&self) -> PANAR2_R {
        PANAR2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Panel Command"]
    #[inline(always)]
    pub fn pancmd(&mut self) -> PANCMD_W {
        PANCMD_W { w: self }
    }
    #[doc = "Bits 16:23 - Panel Argument 1"]
    #[inline(always)]
    pub fn panar1(&mut self) -> PANAR1_W {
        PANAR1_W { w: self }
    }
    #[doc = "Bits 24:31 - Panel Argument 2"]
    #[inline(always)]
    pub fn panar2(&mut self) -> PANAR2_W {
        PANAR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Panel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [panctr](index.html) module"]
pub struct PANCTR_SPEC;
impl crate::RegisterSpec for PANCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [panctr::R](R) reader structure"]
impl crate::Readable for PANCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [panctr::W](W) writer structure"]
impl crate::Writable for PANCTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PANCTR to value 0x0301"]
impl crate::Resettable for PANCTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0301
    }
}
