#[doc = "Register `BUS_MODE` reader"]
pub struct R(crate::R<BUS_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_MODE` writer"]
pub struct W(crate::W<BUS_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_MODE_SPEC>;
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
impl From<crate::W<BUS_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWR` reader - Software Reset"]
pub struct SWR_R(crate::FieldReader<bool, bool>);
impl SWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWR` writer - Software Reset"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
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
#[doc = "Field `DA` reader - DMA Arbitration Scheme"]
pub struct DA_R(crate::FieldReader<bool, bool>);
impl DA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DA` writer - DMA Arbitration Scheme"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
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
#[doc = "Field `DSL` reader - Descriptor Skip Length"]
pub struct DSL_R(crate::FieldReader<u8, u8>);
impl DSL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSL` writer - Descriptor Skip Length"]
pub struct DSL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u32 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Field `ATDS` reader - Alternate Descriptor Size"]
pub struct ATDS_R(crate::FieldReader<bool, bool>);
impl ATDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATDS` writer - Alternate Descriptor Size"]
pub struct ATDS_W<'a> {
    w: &'a mut W,
}
impl<'a> ATDS_W<'a> {
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
#[doc = "Field `PBL` reader - Programmable Burst Length"]
pub struct PBL_R(crate::FieldReader<u8, u8>);
impl PBL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBL` writer - Programmable Burst Length"]
pub struct PBL_W<'a> {
    w: &'a mut W,
}
impl<'a> PBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `PR` reader - Priority Ratio"]
pub struct PR_R(crate::FieldReader<u8, u8>);
impl PR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR` writer - Priority Ratio"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `FB` reader - Fixed Burst"]
pub struct FB_R(crate::FieldReader<bool, bool>);
impl FB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB` writer - Fixed Burst"]
pub struct FB_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_W<'a> {
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
#[doc = "Field `RPBL` reader - Rx DMA PBL"]
pub struct RPBL_R(crate::FieldReader<u8, u8>);
impl RPBL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPBL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPBL` writer - Rx DMA PBL"]
pub struct RPBL_W<'a> {
    w: &'a mut W,
}
impl<'a> RPBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 17)) | ((value as u32 & 0x3f) << 17);
        self.w
    }
}
#[doc = "Field `USP` reader - Use Seperate PBL"]
pub struct USP_R(crate::FieldReader<bool, bool>);
impl USP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USP` writer - Use Seperate PBL"]
pub struct USP_W<'a> {
    w: &'a mut W,
}
impl<'a> USP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PBLX8` reader - 8xPBL Mode"]
pub struct PBLX8_R(crate::FieldReader<bool, bool>);
impl PBLX8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBLX8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBLX8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBLX8` writer - 8xPBL Mode"]
pub struct PBLX8_W<'a> {
    w: &'a mut W,
}
impl<'a> PBLX8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `AAL` reader - Address Aligned Beats"]
pub struct AAL_R(crate::FieldReader<bool, bool>);
impl AAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAL` writer - Address Aligned Beats"]
pub struct AAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `MB` reader - Mixed Burst"]
pub struct MB_R(crate::FieldReader<bool, bool>);
impl MB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB` writer - Mixed Burst"]
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `TXPR` reader - Transmit Priority"]
pub struct TXPR_R(crate::FieldReader<bool, bool>);
impl TXPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPR` writer - Transmit Priority"]
pub struct TXPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PRWG` reader - Channel Priority Weights"]
pub struct PRWG_R(crate::FieldReader<u8, u8>);
impl PRWG_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRWG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRWG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn atds(&self) -> ATDS_R {
        ATDS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rpbl(&self) -> RPBL_R {
        RPBL_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use Seperate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 8xPBL Mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights"]
    #[inline(always)]
    pub fn prwg(&self) -> PRWG_R {
        PRWG_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W { w: self }
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn atds(&mut self) -> ATDS_W {
        ATDS_W { w: self }
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W {
        PBL_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W { w: self }
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rpbl(&mut self) -> RPBL_W {
        RPBL_W { w: self }
    }
    #[doc = "Bit 23 - Use Seperate PBL"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W {
        USP_W { w: self }
    }
    #[doc = "Bit 24 - 8xPBL Mode"]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W {
        PBLX8_W { w: self }
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W {
        AAL_W { w: self }
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W {
        TXPR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_mode](index.html) module"]
pub struct BUS_MODE_SPEC;
impl crate::RegisterSpec for BUS_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_mode::R](R) reader structure"]
impl crate::Readable for BUS_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_mode::W](W) writer structure"]
impl crate::Writable for BUS_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUS_MODE to value 0x0002_0101"]
impl crate::Resettable for BUS_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0101
    }
}
