#[doc = "Register `PSL` reader"]
pub struct R(crate::R<PSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSL` writer"]
pub struct W(crate::W<PSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSL_SPEC>;
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
impl From<crate::W<PSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Output Passive Level for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL11_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL11_A> for bool {
    #[inline(always)]
    fn from(variant: PSL11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL11` reader - Output Passive Level for CCU8x.OUTy0"]
pub struct PSL11_R(crate::FieldReader<bool, PSL11_A>);
impl PSL11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSL11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL11_A {
        match self.bits {
            false => PSL11_A::VALUE1,
            true => PSL11_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PSL11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSL11_A::VALUE2
    }
}
impl core::ops::Deref for PSL11_R {
    type Target = crate::FieldReader<bool, PSL11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSL11` writer - Output Passive Level for CCU8x.OUTy0"]
pub struct PSL11_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL11_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL11_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Output Passive Level for CCU8x.OUTy1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL12_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL12_A> for bool {
    #[inline(always)]
    fn from(variant: PSL12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL12` reader - Output Passive Level for CCU8x.OUTy1"]
pub struct PSL12_R(crate::FieldReader<bool, PSL12_A>);
impl PSL12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSL12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL12_A {
        match self.bits {
            false => PSL12_A::VALUE1,
            true => PSL12_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PSL12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSL12_A::VALUE2
    }
}
impl core::ops::Deref for PSL12_R {
    type Target = crate::FieldReader<bool, PSL12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSL12` writer - Output Passive Level for CCU8x.OUTy1"]
pub struct PSL12_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL12_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL12_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Output Passive Level for CCU8x.OUTy2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL21_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL21_A> for bool {
    #[inline(always)]
    fn from(variant: PSL21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL21` reader - Output Passive Level for CCU8x.OUTy2"]
pub struct PSL21_R(crate::FieldReader<bool, PSL21_A>);
impl PSL21_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSL21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL21_A {
        match self.bits {
            false => PSL21_A::VALUE1,
            true => PSL21_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PSL21_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSL21_A::VALUE2
    }
}
impl core::ops::Deref for PSL21_R {
    type Target = crate::FieldReader<bool, PSL21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSL21` writer - Output Passive Level for CCU8x.OUTy2"]
pub struct PSL21_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL21_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL21_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Output Passive Level for CCU8x.OUTy3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL22_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL22_A> for bool {
    #[inline(always)]
    fn from(variant: PSL22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL22` reader - Output Passive Level for CCU8x.OUTy3"]
pub struct PSL22_R(crate::FieldReader<bool, PSL22_A>);
impl PSL22_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSL22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL22_A {
        match self.bits {
            false => PSL22_A::VALUE1,
            true => PSL22_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PSL22_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSL22_A::VALUE2
    }
}
impl core::ops::Deref for PSL22_R {
    type Target = crate::FieldReader<bool, PSL22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSL22` writer - Output Passive Level for CCU8x.OUTy3"]
pub struct PSL22_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL22_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL22_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn psl11(&self) -> PSL11_R {
        PSL11_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn psl12(&self) -> PSL12_R {
        PSL12_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn psl21(&self) -> PSL21_R {
        PSL21_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn psl22(&self) -> PSL22_R {
        PSL22_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn psl11(&mut self) -> PSL11_W {
        PSL11_W { w: self }
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn psl12(&mut self) -> PSL12_W {
        PSL12_W { w: self }
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn psl21(&mut self) -> PSL21_W {
        PSL21_W { w: self }
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn psl22(&mut self) -> PSL22_W {
        PSL22_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Passive Level Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psl](index.html) module"]
pub struct PSL_SPEC;
impl crate::RegisterSpec for PSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psl::R](R) reader structure"]
impl crate::Readable for PSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psl::W](W) writer structure"]
impl crate::Writable for PSL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSL to value 0"]
impl crate::Resettable for PSL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
