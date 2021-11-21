#[doc = "Register `SW_RESET` reader"]
pub struct R(crate::R<SW_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_RESET` writer"]
pub struct W(crate::W<SW_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_RESET_SPEC>;
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
impl From<crate::W<SW_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software Reset for DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RST_DAT_LINE_A {
    #[doc = "0: Work"]
    VALUE1 = 0,
    #[doc = "1: Reset"]
    VALUE2 = 1,
}
impl From<SW_RST_DAT_LINE_A> for bool {
    #[inline(always)]
    fn from(variant: SW_RST_DAT_LINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RST_DAT_LINE` reader - Software Reset for DAT Line"]
pub struct SW_RST_DAT_LINE_R(crate::FieldReader<bool, SW_RST_DAT_LINE_A>);
impl SW_RST_DAT_LINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_RST_DAT_LINE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_RST_DAT_LINE_A {
        match self.bits {
            false => SW_RST_DAT_LINE_A::VALUE1,
            true => SW_RST_DAT_LINE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SW_RST_DAT_LINE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SW_RST_DAT_LINE_A::VALUE2
    }
}
impl core::ops::Deref for SW_RST_DAT_LINE_R {
    type Target = crate::FieldReader<bool, SW_RST_DAT_LINE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_RST_DAT_LINE` writer - Software Reset for DAT Line"]
pub struct SW_RST_DAT_LINE_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_DAT_LINE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_RST_DAT_LINE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SW_RST_DAT_LINE_A::VALUE1)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SW_RST_DAT_LINE_A::VALUE2)
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
#[doc = "Software Reset for CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RST_CMD_LINE_A {
    #[doc = "0: Work"]
    VALUE1 = 0,
    #[doc = "1: Reset"]
    VALUE2 = 1,
}
impl From<SW_RST_CMD_LINE_A> for bool {
    #[inline(always)]
    fn from(variant: SW_RST_CMD_LINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RST_CMD_LINE` reader - Software Reset for CMD Line"]
pub struct SW_RST_CMD_LINE_R(crate::FieldReader<bool, SW_RST_CMD_LINE_A>);
impl SW_RST_CMD_LINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_RST_CMD_LINE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_RST_CMD_LINE_A {
        match self.bits {
            false => SW_RST_CMD_LINE_A::VALUE1,
            true => SW_RST_CMD_LINE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SW_RST_CMD_LINE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SW_RST_CMD_LINE_A::VALUE2
    }
}
impl core::ops::Deref for SW_RST_CMD_LINE_R {
    type Target = crate::FieldReader<bool, SW_RST_CMD_LINE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_RST_CMD_LINE` writer - Software Reset for CMD Line"]
pub struct SW_RST_CMD_LINE_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_CMD_LINE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_RST_CMD_LINE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SW_RST_CMD_LINE_A::VALUE1)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SW_RST_CMD_LINE_A::VALUE2)
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
#[doc = "Field `SW_RST_ALL` reader - Software Reset for All"]
pub struct SW_RST_ALL_R(crate::FieldReader<bool, bool>);
impl SW_RST_ALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_RST_ALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_RST_ALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_RST_ALL` writer - Software Reset for All"]
pub struct SW_RST_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_ALL_W<'a> {
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
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sw_rst_dat_line(&self) -> SW_RST_DAT_LINE_R {
        SW_RST_DAT_LINE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sw_rst_cmd_line(&self) -> SW_RST_CMD_LINE_R {
        SW_RST_CMD_LINE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SW_RST_ALL_R {
        SW_RST_ALL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sw_rst_dat_line(&mut self) -> SW_RST_DAT_LINE_W {
        SW_RST_DAT_LINE_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sw_rst_cmd_line(&mut self) -> SW_RST_CMD_LINE_W {
        SW_RST_CMD_LINE_W { w: self }
    }
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    pub fn sw_rst_all(&mut self) -> SW_RST_ALL_W {
        SW_RST_ALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_reset](index.html) module"]
pub struct SW_RESET_SPEC;
impl crate::RegisterSpec for SW_RESET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sw_reset::R](R) reader structure"]
impl crate::Readable for SW_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_reset::W](W) writer structure"]
impl crate::Writable for SW_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_RESET to value 0"]
impl crate::Resettable for SW_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
