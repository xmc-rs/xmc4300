#[doc = "Register `DTSCON` reader"]
pub struct R(crate::R<DTSCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSCON` writer"]
pub struct W(crate::W<DTSCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSCON_SPEC>;
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
impl From<crate::W<DTSCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sensor Power Down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWD_A {
    #[doc = "0: The DTS is powered"]
    CONST_0 = 0,
    #[doc = "1: The DTS is not powered"]
    CONST_1 = 1,
}
impl From<PWD_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWD` reader - Sensor Power Down"]
pub struct PWD_R(crate::FieldReader<bool, PWD_A>);
impl PWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_A {
        match self.bits {
            false => PWD_A::CONST_0,
            true => PWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == PWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == PWD_A::CONST_1
    }
}
impl core::ops::Deref for PWD_R {
    type Target = crate::FieldReader<bool, PWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWD` writer - Sensor Power Down"]
pub struct PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DTS is powered"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PWD_A::CONST_0)
    }
    #[doc = "The DTS is not powered"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PWD_A::CONST_1)
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
#[doc = "Sensor Measurement Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_AW {
    #[doc = "0: No DTS measurement is started"]
    CONST_0 = 0,
    #[doc = "1: DTS measurement is started"]
    CONST_1 = 1,
}
impl From<START_AW> for bool {
    #[inline(always)]
    fn from(variant: START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Sensor Measurement Start"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DTS measurement is started"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(START_AW::CONST_0)
    }
    #[doc = "DTS measurement is started"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(START_AW::CONST_1)
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
#[doc = "Field `OFFSET` reader - Offset Calibration Value"]
pub struct OFFSET_R(crate::FieldReader<u8, u8>);
impl OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET` writer - Offset Calibration Value"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | ((value as u32 & 0x7f) << 4);
        self.w
    }
}
#[doc = "Field `GAIN` reader - Gain Calibration Value"]
pub struct GAIN_R(crate::FieldReader<u8, u8>);
impl GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN` writer - Gain Calibration Value"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 11)) | ((value as u32 & 0x3f) << 11);
        self.w
    }
}
#[doc = "Field `REFTRIM` reader - Reference Trim Calibration Value"]
pub struct REFTRIM_R(crate::FieldReader<u8, u8>);
impl REFTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFTRIM` writer - Reference Trim Calibration Value"]
pub struct REFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> REFTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `BGTRIM` reader - Bandgap Trim Calibration Value"]
pub struct BGTRIM_R(crate::FieldReader<u8, u8>);
impl BGTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        BGTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGTRIM` writer - Bandgap Trim Calibration Value"]
pub struct BGTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BGTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    pub fn pwd(&self) -> PWD_R {
        PWD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    pub fn reftrim(&self) -> REFTRIM_R {
        REFTRIM_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    pub fn bgtrim(&self) -> BGTRIM_R {
        BGTRIM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    pub fn pwd(&mut self) -> PWD_W {
        PWD_W { w: self }
    }
    #[doc = "Bit 1 - Sensor Measurement Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    pub fn reftrim(&mut self) -> REFTRIM_W {
        REFTRIM_W { w: self }
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    pub fn bgtrim(&mut self) -> BGTRIM_W {
        BGTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Die Temperature Sensor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtscon](index.html) module"]
pub struct DTSCON_SPEC;
impl crate::RegisterSpec for DTSCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtscon::R](R) reader structure"]
impl crate::Readable for DTSCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtscon::W](W) writer structure"]
impl crate::Writable for DTSCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTSCON to value 0x01"]
impl crate::Resettable for DTSCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
