#[doc = "Register `DC_ACT` reader"]
pub struct R(crate::R<DC_ACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_ACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_ACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_ACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_ACT` writer"]
pub struct W(crate::W<DC_ACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_ACT_SPEC>;
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
impl From<crate::W<DC_ACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_ACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sync Out Unit activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_OUT_A {
    #[doc = "0: Deactivated"]
    VALUE1 = 0,
    #[doc = "1: Activated"]
    VALUE2 = 1,
}
impl From<SYNC_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_OUT` reader - Sync Out Unit activation"]
pub struct SYNC_OUT_R(crate::FieldReader<bool, SYNC_OUT_A>);
impl SYNC_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_OUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_OUT_A {
        match self.bits {
            false => SYNC_OUT_A::VALUE1,
            true => SYNC_OUT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SYNC_OUT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SYNC_OUT_A::VALUE2
    }
}
impl core::ops::Deref for SYNC_OUT_R {
    type Target = crate::FieldReader<bool, SYNC_OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_OUT` writer - Sync Out Unit activation"]
pub struct SYNC_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_OUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_OUT_A::VALUE1)
    }
    #[doc = "Activated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_OUT_A::VALUE2)
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
#[doc = "SYNC0 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_0_A {
    #[doc = "0: Deactivated"]
    VALUE1 = 0,
    #[doc = "1: SYNC0 pulse is generated"]
    VALUE2 = 1,
}
impl From<SYNC_0_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_0` reader - SYNC0 generation"]
pub struct SYNC_0_R(crate::FieldReader<bool, SYNC_0_A>);
impl SYNC_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_0_A {
        match self.bits {
            false => SYNC_0_A::VALUE1,
            true => SYNC_0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SYNC_0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SYNC_0_A::VALUE2
    }
}
impl core::ops::Deref for SYNC_0_R {
    type Target = crate::FieldReader<bool, SYNC_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_0` writer - SYNC0 generation"]
pub struct SYNC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_0_A::VALUE1)
    }
    #[doc = "SYNC0 pulse is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_0_A::VALUE2)
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
#[doc = "SYNC1 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_1_A {
    #[doc = "0: Deactivated"]
    VALUE1 = 0,
    #[doc = "1: SYNC1 pulse is generated"]
    VALUE2 = 1,
}
impl From<SYNC_1_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_1` reader - SYNC1 generation"]
pub struct SYNC_1_R(crate::FieldReader<bool, SYNC_1_A>);
impl SYNC_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_1_A {
        match self.bits {
            false => SYNC_1_A::VALUE1,
            true => SYNC_1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SYNC_1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SYNC_1_A::VALUE2
    }
}
impl core::ops::Deref for SYNC_1_R {
    type Target = crate::FieldReader<bool, SYNC_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_1` writer - SYNC1 generation"]
pub struct SYNC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_1_A::VALUE1)
    }
    #[doc = "SYNC1 pulse is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_1_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline(always)]
    pub fn sync_out(&self) -> SYNC_OUT_R {
        SYNC_OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline(always)]
    pub fn sync_0(&self) -> SYNC_0_R {
        SYNC_0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline(always)]
    pub fn sync_1(&self) -> SYNC_1_R {
        SYNC_1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline(always)]
    pub fn sync_out(&mut self) -> SYNC_OUT_W {
        SYNC_OUT_W { w: self }
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline(always)]
    pub fn sync_0(&mut self) -> SYNC_0_W {
        SYNC_0_W { w: self }
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline(always)]
    pub fn sync_1(&mut self) -> SYNC_1_W {
        SYNC_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Activation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_act](index.html) module"]
pub struct DC_ACT_SPEC;
impl crate::RegisterSpec for DC_ACT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dc_act::R](R) reader structure"]
impl crate::Readable for DC_ACT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_act::W](W) writer structure"]
impl crate::Writable for DC_ACT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DC_ACT to value 0"]
impl crate::Resettable for DC_ACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
