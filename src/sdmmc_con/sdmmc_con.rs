#[doc = "Register `SDMMC_CON` reader"]
pub struct R(crate::R<SDMMC_CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_CON` writer"]
pub struct W(crate::W<SDMMC_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_CON_SPEC>;
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
impl From<crate::W<SDMMC_CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SDMMC Write Protection Input Multiplexer Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSEL_A {
    #[doc = "0: P1.1 input pin selected"]
    VALUE1 = 0,
    #[doc = "1: Software bit WPVAL is selected"]
    VALUE2 = 1,
}
impl From<WPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPSEL` reader - SDMMC Write Protection Input Multiplexer Control"]
pub struct WPSEL_R(crate::FieldReader<bool, WPSEL_A>);
impl WPSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPSEL_A {
        match self.bits {
            false => WPSEL_A::VALUE1,
            true => WPSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WPSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WPSEL_A::VALUE2
    }
}
impl core::ops::Deref for WPSEL_R {
    type Target = crate::FieldReader<bool, WPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPSEL` writer - SDMMC Write Protection Input Multiplexer Control"]
pub struct WPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "P1.1 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPSEL_A::VALUE1)
    }
    #[doc = "Software bit WPVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPSEL_A::VALUE2)
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
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSVAL_A {
    #[doc = "0: No write protection"]
    VALUE1 = 0,
    #[doc = "1: Write protection active"]
    VALUE2 = 1,
}
impl From<WPSVAL_A> for bool {
    #[inline(always)]
    fn from(variant: WPSVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPSVAL` reader - SDMMC Write Protect Software Control"]
pub struct WPSVAL_R(crate::FieldReader<bool, WPSVAL_A>);
impl WPSVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPSVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPSVAL_A {
        match self.bits {
            false => WPSVAL_A::VALUE1,
            true => WPSVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WPSVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WPSVAL_A::VALUE2
    }
}
impl core::ops::Deref for WPSVAL_R {
    type Target = crate::FieldReader<bool, WPSVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPSVAL` writer - SDMMC Write Protect Software Control"]
pub struct WPSVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> WPSVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPSVAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPSVAL_A::VALUE1)
    }
    #[doc = "Write protection active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPSVAL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "SDMMC Card Detection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSEL_A {
    #[doc = "0: P1.10 input pin selected"]
    VALUE1 = 0,
    #[doc = "1: Software bit CDSVAL is selected"]
    VALUE2 = 1,
}
impl From<CDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDSEL` reader - SDMMC Card Detection Control"]
pub struct CDSEL_R(crate::FieldReader<bool, CDSEL_A>);
impl CDSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSEL_A {
        match self.bits {
            false => CDSEL_A::VALUE1,
            true => CDSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CDSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CDSEL_A::VALUE2
    }
}
impl core::ops::Deref for CDSEL_R {
    type Target = crate::FieldReader<bool, CDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDSEL` writer - SDMMC Card Detection Control"]
pub struct CDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "P1.10 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE1)
    }
    #[doc = "Software bit CDSVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSVAL_A {
    #[doc = "0: No card detected"]
    VALUE1 = 0,
    #[doc = "1: Card detected"]
    VALUE2 = 1,
}
impl From<CDSVAL_A> for bool {
    #[inline(always)]
    fn from(variant: CDSVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDSVAL` reader - SDMMC Write Protect Software Control"]
pub struct CDSVAL_R(crate::FieldReader<bool, CDSVAL_A>);
impl CDSVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDSVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSVAL_A {
        match self.bits {
            false => CDSVAL_A::VALUE1,
            true => CDSVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CDSVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CDSVAL_A::VALUE2
    }
}
impl core::ops::Deref for CDSVAL_R {
    type Target = crate::FieldReader<bool, CDSVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDSVAL` writer - SDMMC Write Protect Software Control"]
pub struct CDSVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDSVAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No card detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSVAL_A::VALUE1)
    }
    #[doc = "Card detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSVAL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    pub fn wpsel(&self) -> WPSEL_R {
        WPSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn wpsval(&self) -> WPSVAL_R {
        WPSVAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    pub fn cdsel(&self) -> CDSEL_R {
        CDSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn cdsval(&self) -> CDSVAL_R {
        CDSVAL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    pub fn wpsel(&mut self) -> WPSEL_W {
        WPSEL_W { w: self }
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn wpsval(&mut self) -> WPSVAL_W {
        WPSVAL_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    pub fn cdsel(&mut self) -> CDSEL_W {
        CDSEL_W { w: self }
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn cdsval(&mut self) -> CDSVAL_W {
        CDSVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMMC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_con](index.html) module"]
pub struct SDMMC_CON_SPEC;
impl crate::RegisterSpec for SDMMC_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_con::R](R) reader structure"]
impl crate::Readable for SDMMC_CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_con::W](W) writer structure"]
impl crate::Writable for SDMMC_CON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_CON to value 0"]
impl crate::Resettable for SDMMC_CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
