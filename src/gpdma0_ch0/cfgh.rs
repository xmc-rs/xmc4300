#[doc = "Register `CFGH` reader"]
pub struct R(crate::R<CFGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGH` writer"]
pub struct W(crate::W<CFGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGH_SPEC>;
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
impl From<crate::W<CFGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEST_PER` reader - Destination Peripheral"]
pub struct DEST_PER_R(crate::FieldReader<u8, u8>);
impl DEST_PER_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEST_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEST_PER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEST_PER` writer - Destination Peripheral"]
pub struct DEST_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `SRC_PER` reader - Source Peripheral"]
pub struct SRC_PER_R(crate::FieldReader<u8, u8>);
impl SRC_PER_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRC_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_PER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_PER` writer - Source Peripheral"]
pub struct SRC_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `SS_UPD_EN` reader - Source Status Update Enable"]
pub struct SS_UPD_EN_R(crate::FieldReader<bool, bool>);
impl SS_UPD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SS_UPD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_UPD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS_UPD_EN` writer - Source Status Update Enable"]
pub struct SS_UPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_UPD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DS_UPD_EN` reader - Destination Status Update Enable"]
pub struct DS_UPD_EN_R(crate::FieldReader<bool, bool>);
impl DS_UPD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DS_UPD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS_UPD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS_UPD_EN` writer - Destination Status Update Enable"]
pub struct DS_UPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_UPD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PROTCTL` reader - Protection Control"]
pub struct PROTCTL_R(crate::FieldReader<u8, u8>);
impl PROTCTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PROTCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROTCTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROTCTL` writer - Protection Control"]
pub struct PROTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "FIFO Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_MODE_A {
    #[doc = "0: Space/data available for single AHB transfer of the specified transfer width."]
    VALUE1 = 0,
    #[doc = "1: Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    VALUE2 = 1,
}
impl From<FIFO_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_MODE` reader - FIFO Mode Select"]
pub struct FIFO_MODE_R(crate::FieldReader<bool, FIFO_MODE_A>);
impl FIFO_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_MODE_A {
        match self.bits {
            false => FIFO_MODE_A::VALUE1,
            true => FIFO_MODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FIFO_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FIFO_MODE_A::VALUE2
    }
}
impl core::ops::Deref for FIFO_MODE_R {
    type Target = crate::FieldReader<bool, FIFO_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_MODE` writer - FIFO Mode Select"]
pub struct FIFO_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FIFO_MODE_A::VALUE1)
    }
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FIFO_MODE_A::VALUE2)
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
#[doc = "Flow Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCMODE_A {
    #[doc = "0: Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    VALUE1 = 0,
    #[doc = "1: Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    VALUE2 = 1,
}
impl From<FCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FCMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCMODE` reader - Flow Control Mode"]
pub struct FCMODE_R(crate::FieldReader<bool, FCMODE_A>);
impl FCMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCMODE_A {
        match self.bits {
            false => FCMODE_A::VALUE1,
            true => FCMODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FCMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FCMODE_A::VALUE2
    }
}
impl core::ops::Deref for FCMODE_R {
    type Target = crate::FieldReader<bool, FCMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCMODE` writer - Flow Control Mode"]
pub struct FCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCMODE_A::VALUE1)
    }
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCMODE_A::VALUE2)
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
impl R {
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    pub fn dest_per(&self) -> DEST_PER_R {
        DEST_PER_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    pub fn src_per(&self) -> SRC_PER_R {
        SRC_PER_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Source Status Update Enable"]
    #[inline(always)]
    pub fn ss_upd_en(&self) -> SS_UPD_EN_R {
        SS_UPD_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Destination Status Update Enable"]
    #[inline(always)]
    pub fn ds_upd_en(&self) -> DS_UPD_EN_R {
        DS_UPD_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&self) -> PROTCTL_R {
        PROTCTL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FIFO_MODE_R {
        FIFO_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    pub fn fcmode(&self) -> FCMODE_R {
        FCMODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    pub fn dest_per(&mut self) -> DEST_PER_W {
        DEST_PER_W { w: self }
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    pub fn src_per(&mut self) -> SRC_PER_W {
        SRC_PER_W { w: self }
    }
    #[doc = "Bit 6 - Source Status Update Enable"]
    #[inline(always)]
    pub fn ss_upd_en(&mut self) -> SS_UPD_EN_W {
        SS_UPD_EN_W { w: self }
    }
    #[doc = "Bit 5 - Destination Status Update Enable"]
    #[inline(always)]
    pub fn ds_upd_en(&mut self) -> DS_UPD_EN_W {
        DS_UPD_EN_W { w: self }
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&mut self) -> PROTCTL_W {
        PROTCTL_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&mut self) -> FIFO_MODE_W {
        FIFO_MODE_W { w: self }
    }
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    pub fn fcmode(&mut self) -> FCMODE_W {
        FCMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgh](index.html) module"]
pub struct CFGH_SPEC;
impl crate::RegisterSpec for CFGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgh::R](R) reader structure"]
impl crate::Readable for CFGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgh::W](W) writer structure"]
impl crate::Writable for CFGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGH to value 0x04"]
impl crate::Resettable for CFGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
