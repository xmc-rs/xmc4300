#[doc = "Register `CTLL` reader"]
pub struct R(crate::R<CTLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLL` writer"]
pub struct W(crate::W<CTLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLL_SPEC>;
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
impl From<crate::W<CTLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TT_FC` reader - Transfer Type and Flow Control"]
pub struct TT_FC_R(crate::FieldReader<u8, u8>);
impl TT_FC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TT_FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TT_FC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TT_FC` writer - Transfer Type and Flow Control"]
pub struct TT_FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TT_FC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `SRC_MSIZE` reader - Source Burst Transaction Length"]
pub struct SRC_MSIZE_R(crate::FieldReader<u8, u8>);
impl SRC_MSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRC_MSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_MSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_MSIZE` writer - Source Burst Transaction Length"]
pub struct SRC_MSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
#[doc = "Field `DEST_MSIZE` reader - Destination Burst Transaction Length"]
pub struct DEST_MSIZE_R(crate::FieldReader<u8, u8>);
impl DEST_MSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEST_MSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEST_MSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEST_MSIZE` writer - Destination Burst Transaction Length"]
pub struct DEST_MSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_MSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Source Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SINC_A {
    #[doc = "0: Increment"]
    VALUE1 = 0,
    #[doc = "1: Decrement"]
    VALUE2 = 1,
    #[doc = "2: No change"]
    VALUE3 = 2,
}
impl From<SINC_A> for u8 {
    #[inline(always)]
    fn from(variant: SINC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SINC` reader - Source Address Increment"]
pub struct SINC_R(crate::FieldReader<u8, SINC_A>);
impl SINC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SINC_A> {
        match self.bits {
            0 => Some(SINC_A::VALUE1),
            1 => Some(SINC_A::VALUE2),
            2 => Some(SINC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SINC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SINC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SINC_A::VALUE3
    }
}
impl core::ops::Deref for SINC_R {
    type Target = crate::FieldReader<u8, SINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINC` writer - Source Address Increment"]
pub struct SINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SINC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SINC_A::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SINC_A::VALUE2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SINC_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Destination Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DINC_A {
    #[doc = "0: Increment"]
    VALUE1 = 0,
    #[doc = "1: Decrement"]
    VALUE2 = 1,
    #[doc = "2: No change"]
    VALUE3 = 2,
}
impl From<DINC_A> for u8 {
    #[inline(always)]
    fn from(variant: DINC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DINC` reader - Destination Address Increment"]
pub struct DINC_R(crate::FieldReader<u8, DINC_A>);
impl DINC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DINC_A> {
        match self.bits {
            0 => Some(DINC_A::VALUE1),
            1 => Some(DINC_A::VALUE2),
            2 => Some(DINC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DINC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DINC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == DINC_A::VALUE3
    }
}
impl core::ops::Deref for DINC_R {
    type Target = crate::FieldReader<u8, DINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINC` writer - Destination Address Increment"]
pub struct DINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DINC_A::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DINC_A::VALUE2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DINC_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `SRC_TR_WIDTH` reader - Source Transfer Width"]
pub struct SRC_TR_WIDTH_R(crate::FieldReader<u8, u8>);
impl SRC_TR_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRC_TR_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_TR_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_TR_WIDTH` writer - Source Transfer Width"]
pub struct SRC_TR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_TR_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DST_TR_WIDTH` reader - Destination Transfer Width"]
pub struct DST_TR_WIDTH_R(crate::FieldReader<u8, u8>);
impl DST_TR_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DST_TR_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DST_TR_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_TR_WIDTH` writer - Destination Transfer Width"]
pub struct DST_TR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_TR_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `INT_EN` reader - Interrupt Enable Bit"]
pub struct INT_EN_R(crate::FieldReader<bool, bool>);
impl INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_EN` writer - Interrupt Enable Bit"]
pub struct INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EN_W<'a> {
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
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    pub fn tt_fc(&self) -> TT_FC_R {
        TT_FC_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&self) -> SRC_MSIZE_R {
        SRC_MSIZE_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dest_msize(&self) -> DEST_MSIZE_R {
        DEST_MSIZE_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SRC_TR_WIDTH_R {
        SRC_TR_WIDTH_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DST_TR_WIDTH_R {
        DST_TR_WIDTH_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    pub fn tt_fc(&mut self) -> TT_FC_W {
        TT_FC_W { w: self }
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&mut self) -> SRC_MSIZE_W {
        SRC_MSIZE_W { w: self }
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dest_msize(&mut self) -> DEST_MSIZE_W {
        DEST_MSIZE_W { w: self }
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W {
        SINC_W { w: self }
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W {
        DINC_W { w: self }
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&mut self) -> SRC_TR_WIDTH_W {
        SRC_TR_WIDTH_W { w: self }
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&mut self) -> DST_TR_WIDTH_W {
        DST_TR_WIDTH_W { w: self }
    }
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctll](index.html) module"]
pub struct CTLL_SPEC;
impl crate::RegisterSpec for CTLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctll::R](R) reader structure"]
impl crate::Readable for CTLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctll::W](W) writer structure"]
impl crate::Writable for CTLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLL to value 0x0030_4801"]
impl crate::Resettable for CTLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0030_4801
    }
}
