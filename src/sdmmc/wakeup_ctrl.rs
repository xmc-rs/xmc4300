#[doc = "Register `WAKEUP_CTRL` reader"]
pub struct R(crate::R<WAKEUP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP_CTRL` writer"]
pub struct W(crate::W<WAKEUP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_CTRL_SPEC>;
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
impl From<crate::W<WAKEUP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Event Enable On SD Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_EVENT_EN_REM_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WAKEUP_EVENT_EN_REM_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_EVENT_EN_REM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_REM` reader - Wakeup Event Enable On SD Card Removal"]
pub struct WAKEUP_EVENT_EN_REM_R(crate::FieldReader<bool, WAKEUP_EVENT_EN_REM_A>);
impl WAKEUP_EVENT_EN_REM_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_EVENT_EN_REM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_EVENT_EN_REM_A {
        match self.bits {
            false => WAKEUP_EVENT_EN_REM_A::VALUE1,
            true => WAKEUP_EVENT_EN_REM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WAKEUP_EVENT_EN_REM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WAKEUP_EVENT_EN_REM_A::VALUE2
    }
}
impl core::ops::Deref for WAKEUP_EVENT_EN_REM_R {
    type Target = crate::FieldReader<bool, WAKEUP_EVENT_EN_REM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_REM` writer - Wakeup Event Enable On SD Card Removal"]
pub struct WAKEUP_EVENT_EN_REM_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_EVENT_EN_REM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_EVENT_EN_REM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_REM_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_REM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Wakeup Event Enable On SD Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_EVENT_EN_INS_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WAKEUP_EVENT_EN_INS_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_EVENT_EN_INS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INS` reader - Wakeup Event Enable On SD Card Insertion"]
pub struct WAKEUP_EVENT_EN_INS_R(crate::FieldReader<bool, WAKEUP_EVENT_EN_INS_A>);
impl WAKEUP_EVENT_EN_INS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_EVENT_EN_INS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_EVENT_EN_INS_A {
        match self.bits {
            false => WAKEUP_EVENT_EN_INS_A::VALUE1,
            true => WAKEUP_EVENT_EN_INS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WAKEUP_EVENT_EN_INS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WAKEUP_EVENT_EN_INS_A::VALUE2
    }
}
impl core::ops::Deref for WAKEUP_EVENT_EN_INS_R {
    type Target = crate::FieldReader<bool, WAKEUP_EVENT_EN_INS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INS` writer - Wakeup Event Enable On SD Card Insertion"]
pub struct WAKEUP_EVENT_EN_INS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_EVENT_EN_INS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_EVENT_EN_INS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INS_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INS_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Wakeup Event Enable On Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_EVENT_EN_INT_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WAKEUP_EVENT_EN_INT_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_EVENT_EN_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INT` reader - Wakeup Event Enable On Card Interrupt"]
pub struct WAKEUP_EVENT_EN_INT_R(crate::FieldReader<bool, WAKEUP_EVENT_EN_INT_A>);
impl WAKEUP_EVENT_EN_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_EVENT_EN_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_EVENT_EN_INT_A {
        match self.bits {
            false => WAKEUP_EVENT_EN_INT_A::VALUE1,
            true => WAKEUP_EVENT_EN_INT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WAKEUP_EVENT_EN_INT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WAKEUP_EVENT_EN_INT_A::VALUE2
    }
}
impl core::ops::Deref for WAKEUP_EVENT_EN_INT_R {
    type Target = crate::FieldReader<bool, WAKEUP_EVENT_EN_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INT` writer - Wakeup Event Enable On Card Interrupt"]
pub struct WAKEUP_EVENT_EN_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_EVENT_EN_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_EVENT_EN_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INT_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAKEUP_EVENT_EN_INT_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wakeup_event_en_rem(&self) -> WAKEUP_EVENT_EN_REM_R {
        WAKEUP_EVENT_EN_REM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wakeup_event_en_ins(&self) -> WAKEUP_EVENT_EN_INS_R {
        WAKEUP_EVENT_EN_INS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wakeup_event_en_int(&self) -> WAKEUP_EVENT_EN_INT_R {
        WAKEUP_EVENT_EN_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wakeup_event_en_rem(&mut self) -> WAKEUP_EVENT_EN_REM_W {
        WAKEUP_EVENT_EN_REM_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wakeup_event_en_ins(&mut self) -> WAKEUP_EVENT_EN_INS_W {
        WAKEUP_EVENT_EN_INS_W { w: self }
    }
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wakeup_event_en_int(&mut self) -> WAKEUP_EVENT_EN_INT_W {
        WAKEUP_EVENT_EN_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-up Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_ctrl](index.html) module"]
pub struct WAKEUP_CTRL_SPEC;
impl crate::RegisterSpec for WAKEUP_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wakeup_ctrl::R](R) reader structure"]
impl crate::Readable for WAKEUP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_ctrl::W](W) writer structure"]
impl crate::Writable for WAKEUP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUP_CTRL to value 0"]
impl crate::Resettable for WAKEUP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
