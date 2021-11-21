#[doc = "Register `KSCFG` reader"]
pub struct R(crate::R<KSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KSCFG` writer"]
pub struct W(crate::W<KSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KSCFG_SPEC>;
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
impl From<crate::W<KSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEN_A {
    #[doc = "0: The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    VALUE1 = 0,
    #[doc = "1: The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    VALUE2 = 1,
}
impl From<MODEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODEN` reader - Module Enable"]
pub struct MODEN_R(crate::FieldReader<bool, MODEN_A>);
impl MODEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEN_A {
        match self.bits {
            false => MODEN_A::VALUE1,
            true => MODEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MODEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MODEN_A::VALUE2
    }
}
impl core::ops::Deref for MODEN_R {
    type Target = crate::FieldReader<bool, MODEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODEN` writer - Module Enable"]
pub struct MODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODEN_A::VALUE1)
    }
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODEN_A::VALUE2)
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
#[doc = "Bit Protection for MODEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPMODEN_AW {
    #[doc = "0: MODEN is not changed."]
    VALUE1 = 0,
    #[doc = "1: MODEN is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPMODEN_AW> for bool {
    #[inline(always)]
    fn from(variant: BPMODEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPMODEN` writer - Bit Protection for MODEN"]
pub struct BPMODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BPMODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPMODEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MODEN is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPMODEN_AW::VALUE1)
    }
    #[doc = "MODEN is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPMODEN_AW::VALUE2)
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
#[doc = "Normal Operation Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NOMCFG_A {
    #[doc = "0: Run mode 0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Run mode 1 is selected."]
    VALUE2 = 1,
    #[doc = "2: Stop mode 0 is selected."]
    VALUE3 = 2,
    #[doc = "3: Stop mode 1 is selected."]
    VALUE4 = 3,
}
impl From<NOMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: NOMCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NOMCFG` reader - Normal Operation Mode Configuration"]
pub struct NOMCFG_R(crate::FieldReader<u8, NOMCFG_A>);
impl NOMCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        NOMCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOMCFG_A {
        match self.bits {
            0 => NOMCFG_A::VALUE1,
            1 => NOMCFG_A::VALUE2,
            2 => NOMCFG_A::VALUE3,
            3 => NOMCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NOMCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NOMCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == NOMCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == NOMCFG_A::VALUE4
    }
}
impl core::ops::Deref for NOMCFG_R {
    type Target = crate::FieldReader<u8, NOMCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOMCFG` writer - Normal Operation Mode Configuration"]
pub struct NOMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> NOMCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOMCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Run mode 0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE1)
    }
    #[doc = "Run mode 1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE2)
    }
    #[doc = "Stop mode 0 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE3)
    }
    #[doc = "Stop mode 1 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Bit Protection for NOMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPNOM_AW {
    #[doc = "0: NOMCFG is not changed."]
    VALUE1 = 0,
    #[doc = "1: NOMCFG is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPNOM_AW> for bool {
    #[inline(always)]
    fn from(variant: BPNOM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPNOM` writer - Bit Protection for NOMCFG"]
pub struct BPNOM_W<'a> {
    w: &'a mut W,
}
impl<'a> BPNOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPNOM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "NOMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPNOM_AW::VALUE1)
    }
    #[doc = "NOMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPNOM_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SUMCFG` reader - Suspend Mode Configuration"]
pub struct SUMCFG_R(crate::FieldReader<u8, u8>);
impl SUMCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUMCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUMCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUMCFG` writer - Suspend Mode Configuration"]
pub struct SUMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUMCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Bit Protection for SUMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPSUM_AW {
    #[doc = "0: SUMCFG is not changed."]
    VALUE1 = 0,
    #[doc = "1: SUMCFG is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPSUM_AW> for bool {
    #[inline(always)]
    fn from(variant: BPSUM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPSUM` writer - Bit Protection for SUMCFG"]
pub struct BPSUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BPSUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPSUM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SUMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPSUM_AW::VALUE1)
    }
    #[doc = "SUMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPSUM_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn moden(&self) -> MODEN_R {
        MODEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    pub fn nomcfg(&self) -> NOMCFG_R {
        NOMCFG_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn sumcfg(&self) -> SUMCFG_R {
        SUMCFG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn moden(&mut self) -> MODEN_W {
        MODEN_W { w: self }
    }
    #[doc = "Bit 1 - Bit Protection for MODEN"]
    #[inline(always)]
    pub fn bpmoden(&mut self) -> BPMODEN_W {
        BPMODEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    pub fn nomcfg(&mut self) -> NOMCFG_W {
        NOMCFG_W { w: self }
    }
    #[doc = "Bit 7 - Bit Protection for NOMCFG"]
    #[inline(always)]
    pub fn bpnom(&mut self) -> BPNOM_W {
        BPNOM_W { w: self }
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn sumcfg(&mut self) -> SUMCFG_W {
        SUMCFG_W { w: self }
    }
    #[doc = "Bit 11 - Bit Protection for SUMCFG"]
    #[inline(always)]
    pub fn bpsum(&mut self) -> BPSUM_W {
        BPSUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Kernel State Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kscfg](index.html) module"]
pub struct KSCFG_SPEC;
impl crate::RegisterSpec for KSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kscfg::R](R) reader structure"]
impl crate::Readable for KSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kscfg::W](W) writer structure"]
impl crate::Writable for KSCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KSCFG to value 0"]
impl crate::Resettable for KSCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
