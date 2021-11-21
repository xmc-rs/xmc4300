#[doc = "Register `GINTMSK_HOSTMODE` reader"]
pub struct R(crate::R<GINTMSK_HOSTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTMSK_HOSTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTMSK_HOSTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTMSK_HOSTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTMSK_HOSTMODE` writer"]
pub struct W(crate::W<GINTMSK_HOSTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTMSK_HOSTMODE_SPEC>;
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
impl From<crate::W<GINTMSK_HOSTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTMSK_HOSTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ModeMisMsk` reader - Mode Mismatch Interrupt Mask"]
pub struct MODEMISMSK_R(crate::FieldReader<bool, bool>);
impl MODEMISMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODEMISMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODEMISMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ModeMisMsk` writer - Mode Mismatch Interrupt Mask"]
pub struct MODEMISMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMISMSK_W<'a> {
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
#[doc = "Field `OTGIntMsk` reader - OTG Interrupt Mask"]
pub struct OTGINTMSK_R(crate::FieldReader<bool, bool>);
impl OTGINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGIntMsk` writer - OTG Interrupt Mask"]
pub struct OTGINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGINTMSK_W<'a> {
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
#[doc = "Field `SofMsk` reader - Start of Frame Mask"]
pub struct SOFMSK_R(crate::FieldReader<bool, bool>);
impl SOFMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SofMsk` writer - Start of Frame Mask"]
pub struct SOFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFMSK_W<'a> {
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
#[doc = "Field `RxFLvlMsk` reader - Receive FIFO Non-Empty Mask"]
pub struct RXFLVLMSK_R(crate::FieldReader<bool, bool>);
impl RXFLVLMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFLVLMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLVLMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RxFLvlMsk` writer - Receive FIFO Non-Empty Mask"]
pub struct RXFLVLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLVLMSK_W<'a> {
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
#[doc = "Field `incomplPMsk` reader - Incomplete Periodic Transfer Mask"]
pub struct INCOMPLPMSK_R(crate::FieldReader<bool, bool>);
impl INCOMPLPMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCOMPLPMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCOMPLPMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `incomplPMsk` writer - Incomplete Periodic Transfer Mask"]
pub struct INCOMPLPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPLPMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PrtIntMsk` reader - Host Port Interrupt Mask"]
pub struct PRTINTMSK_R(crate::FieldReader<bool, bool>);
impl PRTINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRTINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PrtIntMsk` writer - Host Port Interrupt Mask"]
pub struct PRTINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTINTMSK_W<'a> {
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
#[doc = "Field `HChIntMsk` reader - Host Channels Interrupt Mask"]
pub struct HCHINTMSK_R(crate::FieldReader<bool, bool>);
impl HCHINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCHINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCHINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HChIntMsk` writer - Host Channels Interrupt Mask"]
pub struct HCHINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> HCHINTMSK_W<'a> {
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
#[doc = "Field `PTxFEmpMsk` reader - Periodic TxFIFO Empty Mask"]
pub struct PTXFEMPMSK_R(crate::FieldReader<bool, bool>);
impl PTXFEMPMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTXFEMPMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFEMPMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTxFEmpMsk` writer - Periodic TxFIFO Empty Mask"]
pub struct PTXFEMPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFEMPMSK_W<'a> {
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
#[doc = "Field `ConIDStsChngMsk` reader - Connector ID Status Change Mask"]
pub struct CONIDSTSCHNGMSK_R(crate::FieldReader<bool, bool>);
impl CONIDSTSCHNGMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONIDSTSCHNGMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONIDSTSCHNGMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ConIDStsChngMsk` writer - Connector ID Status Change Mask"]
pub struct CONIDSTSCHNGMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CONIDSTSCHNGMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DisconnIntMsk` reader - Disconnect Detected Interrupt Mask"]
pub struct DISCONNINTMSK_R(crate::FieldReader<bool, bool>);
impl DISCONNINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCONNINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCONNINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DisconnIntMsk` writer - Disconnect Detected Interrupt Mask"]
pub struct DISCONNINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONNINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `SessReqIntMsk` reader - Session Request/New Session Detected Interrupt Mask"]
pub struct SESSREQINTMSK_R(crate::FieldReader<bool, bool>);
impl SESSREQINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESSREQINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESSREQINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SessReqIntMsk` writer - Session Request/New Session Detected Interrupt Mask"]
pub struct SESSREQINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSREQINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `WkUpIntMsk` reader - Resume/Remote Wakeup Detected Interrupt Mask"]
pub struct WKUPINTMSK_R(crate::FieldReader<bool, bool>);
impl WKUPINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WkUpIntMsk` writer - Resume/Remote Wakeup Detected Interrupt Mask"]
pub struct WKUPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn mode_mis_msk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgint_msk(&self) -> OTGINTMSK_R {
        OTGINTMSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sof_msk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rx_flvl_msk(&self) -> RXFLVLMSK_R {
        RXFLVLMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incompl_pmsk(&self) -> INCOMPLPMSK_R {
        INCOMPLPMSK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    pub fn prt_int_msk(&self) -> PRTINTMSK_R {
        PRTINTMSK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    pub fn hch_int_msk(&self) -> HCHINTMSK_R {
        HCHINTMSK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn ptx_femp_msk(&self) -> PTXFEMPMSK_R {
        PTXFEMPMSK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn con_idsts_chng_msk(&self) -> CONIDSTSCHNGMSK_R {
        CONIDSTSCHNGMSK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconn_int_msk(&self) -> DISCONNINTMSK_R {
        DISCONNINTMSK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sess_req_int_msk(&self) -> SESSREQINTMSK_R {
        SESSREQINTMSK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wk_up_int_msk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn mode_mis_msk(&mut self) -> MODEMISMSK_W {
        MODEMISMSK_W { w: self }
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgint_msk(&mut self) -> OTGINTMSK_W {
        OTGINTMSK_W { w: self }
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sof_msk(&mut self) -> SOFMSK_W {
        SOFMSK_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rx_flvl_msk(&mut self) -> RXFLVLMSK_W {
        RXFLVLMSK_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incompl_pmsk(&mut self) -> INCOMPLPMSK_W {
        INCOMPLPMSK_W { w: self }
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    pub fn prt_int_msk(&mut self) -> PRTINTMSK_W {
        PRTINTMSK_W { w: self }
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    pub fn hch_int_msk(&mut self) -> HCHINTMSK_W {
        HCHINTMSK_W { w: self }
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn ptx_femp_msk(&mut self) -> PTXFEMPMSK_W {
        PTXFEMPMSK_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn con_idsts_chng_msk(&mut self) -> CONIDSTSCHNGMSK_W {
        CONIDSTSCHNGMSK_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconn_int_msk(&mut self) -> DISCONNINTMSK_W {
        DISCONNINTMSK_W { w: self }
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sess_req_int_msk(&mut self) -> SESSREQINTMSK_W {
        SESSREQINTMSK_W { w: self }
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wk_up_int_msk(&mut self) -> WKUPINTMSK_W {
        WKUPINTMSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register \\[HOSTMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintmsk_hostmode](index.html) module"]
pub struct GINTMSK_HOSTMODE_SPEC;
impl crate::RegisterSpec for GINTMSK_HOSTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintmsk_hostmode::R](R) reader structure"]
impl crate::Readable for GINTMSK_HOSTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintmsk_hostmode::W](W) writer structure"]
impl crate::Writable for GINTMSK_HOSTMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTMSK_HOSTMODE to value 0"]
impl crate::Resettable for GINTMSK_HOSTMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
