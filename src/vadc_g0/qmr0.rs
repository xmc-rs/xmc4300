#[doc = "Register `QMR0` reader"]
pub struct R(crate::R<QMR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QMR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QMR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QMR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QMR0` writer"]
pub struct W(crate::W<QMR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QMR0_SPEC>;
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
impl From<crate::W<QMR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QMR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENGT_A {
    #[doc = "0: No conversion requests are issued"]
    VALUE1 = 0,
    #[doc = "1: Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    VALUE2 = 1,
    #[doc = "2: Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    VALUE3 = 2,
    #[doc = "3: Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
    VALUE4 = 3,
}
impl From<ENGT_A> for u8 {
    #[inline(always)]
    fn from(variant: ENGT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ENGT` reader - Enable Gate"]
pub struct ENGT_R(crate::FieldReader<u8, ENGT_A>);
impl ENGT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENGT_A {
        match self.bits {
            0 => ENGT_A::VALUE1,
            1 => ENGT_A::VALUE2,
            2 => ENGT_A::VALUE3,
            3 => ENGT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ENGT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ENGT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ENGT_A::VALUE4
    }
}
impl core::ops::Deref for ENGT_R {
    type Target = crate::FieldReader<u8, ENGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENGT` writer - Enable Gate"]
pub struct ENGT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENGT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No conversion requests are issued"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE1)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE2)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE3)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ENGT_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Enable External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENTR_A {
    #[doc = "0: External trigger disabled"]
    VALUE1 = 0,
    #[doc = "1: The selected edge at the selected trigger input signal REQTR generates the trigger event"]
    VALUE2 = 1,
}
impl From<ENTR_A> for bool {
    #[inline(always)]
    fn from(variant: ENTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENTR` reader - Enable External Trigger"]
pub struct ENTR_R(crate::FieldReader<bool, ENTR_A>);
impl ENTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENTR_A {
        match self.bits {
            false => ENTR_A::VALUE1,
            true => ENTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ENTR_A::VALUE2
    }
}
impl core::ops::Deref for ENTR_R {
    type Target = crate::FieldReader<bool, ENTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENTR` writer - Enable External Trigger"]
pub struct ENTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External trigger disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENTR_A::VALUE1)
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the trigger event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENTR_A::VALUE2)
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
#[doc = "Clear Valid Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRV_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: The next pending valid queue entry in the sequence and the event flag EV are cleared. If there is a valid entry in the queue backup register (QBUR.V = 1), this entry is cleared, otherwise the entry in queue register 0 is cleared."]
    VALUE2 = 1,
}
impl From<CLRV_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRV` writer - Clear Valid Bit"]
pub struct CLRV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLRV_AW::VALUE1)
    }
    #[doc = "The next pending valid queue entry in the sequence and the event flag EV are cleared. If there is a valid entry in the queue backup register (QBUR.V = 1), this entry is cleared, otherwise the entry in queue register 0 is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLRV_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Trigger Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TREV_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Generate a trigger event by software"]
    VALUE2 = 1,
}
impl From<TREV_AW> for bool {
    #[inline(always)]
    fn from(variant: TREV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TREV` writer - Trigger Event"]
pub struct TREV_W<'a> {
    w: &'a mut W,
}
impl<'a> TREV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TREV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TREV_AW::VALUE1)
    }
    #[doc = "Generate a trigger event by software"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TREV_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Flush Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLUSH_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear all queue entries (including backup stage) and the event flag EV. The queue contains no more valid entry."]
    VALUE2 = 1,
}
impl From<FLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: FLUSH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH` writer - Flush Queue"]
pub struct FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLUSH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLUSH_AW::VALUE1)
    }
    #[doc = "Clear all queue entries (including backup stage) and the event flag EV. The queue contains no more valid entry."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLUSH_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Clear Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit EV"]
    VALUE2 = 1,
}
impl From<CEV_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV` writer - Clear Event Flag"]
pub struct CEV_W<'a> {
    w: &'a mut W,
}
impl<'a> CEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEV_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV_AW::VALUE1)
    }
    #[doc = "Clear bit EV"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV_AW::VALUE2)
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
#[doc = "Repeat Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPTDIS_A {
    #[doc = "0: A cancelled conversion is repeated"]
    VALUE1 = 0,
    #[doc = "1: A cancelled conversion is discarded"]
    VALUE2 = 1,
}
impl From<RPTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RPTDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPTDIS` reader - Repeat Disable"]
pub struct RPTDIS_R(crate::FieldReader<bool, RPTDIS_A>);
impl RPTDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPTDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPTDIS_A {
        match self.bits {
            false => RPTDIS_A::VALUE1,
            true => RPTDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RPTDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RPTDIS_A::VALUE2
    }
}
impl core::ops::Deref for RPTDIS_R {
    type Target = crate::FieldReader<bool, RPTDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPTDIS` writer - Repeat Disable"]
pub struct RPTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RPTDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPTDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A cancelled conversion is repeated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RPTDIS_A::VALUE1)
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RPTDIS_A::VALUE2)
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
impl R {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    pub fn engt(&self) -> ENGT_R {
        ENGT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    pub fn entr(&self) -> ENTR_R {
        ENTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    pub fn rptdis(&self) -> RPTDIS_R {
        RPTDIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    pub fn engt(&mut self) -> ENGT_W {
        ENGT_W { w: self }
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    pub fn entr(&mut self) -> ENTR_W {
        ENTR_W { w: self }
    }
    #[doc = "Bit 8 - Clear Valid Bit"]
    #[inline(always)]
    pub fn clrv(&mut self) -> CLRV_W {
        CLRV_W { w: self }
    }
    #[doc = "Bit 9 - Trigger Event"]
    #[inline(always)]
    pub fn trev(&mut self) -> TREV_W {
        TREV_W { w: self }
    }
    #[doc = "Bit 10 - Flush Queue"]
    #[inline(always)]
    pub fn flush(&mut self) -> FLUSH_W {
        FLUSH_W { w: self }
    }
    #[doc = "Bit 11 - Clear Event Flag"]
    #[inline(always)]
    pub fn cev(&mut self) -> CEV_W {
        CEV_W { w: self }
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    pub fn rptdis(&mut self) -> RPTDIS_W {
        RPTDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Queue 0 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmr0](index.html) module"]
pub struct QMR0_SPEC;
impl crate::RegisterSpec for QMR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qmr0::R](R) reader structure"]
impl crate::Readable for QMR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qmr0::W](W) writer structure"]
impl crate::Writable for QMR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QMR0 to value 0"]
impl crate::Resettable for QMR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
