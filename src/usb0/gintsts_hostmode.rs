#[doc = "Register `GINTSTS_HOSTMODE` reader"]
pub struct R(crate::R<GINTSTS_HOSTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTSTS_HOSTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTSTS_HOSTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTSTS_HOSTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTSTS_HOSTMODE` writer"]
pub struct W(crate::W<GINTSTS_HOSTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTSTS_HOSTMODE_SPEC>;
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
impl From<crate::W<GINTSTS_HOSTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTSTS_HOSTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Current Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURMOD_A {
    #[doc = "0: Device mode"]
    VALUE1 = 0,
    #[doc = "1: Host mode"]
    VALUE2 = 1,
}
impl From<CURMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CURMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CurMod` reader - Current Mode of Operation"]
pub struct CURMOD_R(crate::FieldReader<bool, CURMOD_A>);
impl CURMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CURMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURMOD_A {
        match self.bits {
            false => CURMOD_A::VALUE1,
            true => CURMOD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CURMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CURMOD_A::VALUE2
    }
}
impl core::ops::Deref for CURMOD_R {
    type Target = crate::FieldReader<bool, CURMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ModeMis` reader - Mode Mismatch Interrupt"]
pub struct MODEMIS_R(crate::FieldReader<bool, bool>);
impl MODEMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ModeMis` writer - Mode Mismatch Interrupt"]
pub struct MODEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMIS_W<'a> {
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
#[doc = "Field `OTGInt` reader - OTG Interrupt"]
pub struct OTGINT_R(crate::FieldReader<bool, bool>);
impl OTGINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Sof` reader - Start of Frame"]
pub struct SOF_R(crate::FieldReader<bool, bool>);
impl SOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Sof` writer - Start of Frame"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
#[doc = "Field `RxFLvl` reader - RxFIFO Non-Empty"]
pub struct RXFLVL_R(crate::FieldReader<bool, bool>);
impl RXFLVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `incomplP` reader - Incomplete Periodic Transfer"]
pub struct INCOMPLP_R(crate::FieldReader<bool, bool>);
impl INCOMPLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCOMPLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCOMPLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `incomplP` writer - Incomplete Periodic Transfer"]
pub struct INCOMPLP_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPLP_W<'a> {
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
#[doc = "Field `PrtInt` reader - Host Port Interrupt"]
pub struct PRTINT_R(crate::FieldReader<bool, bool>);
impl PRTINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRTINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HChInt` reader - Host Channels Interrupt"]
pub struct HCHINT_R(crate::FieldReader<bool, bool>);
impl HCHINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCHINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCHINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTxFEmp` reader - Periodic TxFIFO Empty"]
pub struct PTXFEMP_R(crate::FieldReader<bool, bool>);
impl PTXFEMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTXFEMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFEMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ConIDStsChng` reader - Connector ID Status Change"]
pub struct CONIDSTSCHNG_R(crate::FieldReader<bool, bool>);
impl CONIDSTSCHNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONIDSTSCHNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONIDSTSCHNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ConIDStsChng` writer - Connector ID Status Change"]
pub struct CONIDSTSCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONIDSTSCHNG_W<'a> {
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
#[doc = "Field `DisconnInt` reader - Disconnect Detected Interrupt"]
pub struct DISCONNINT_R(crate::FieldReader<bool, bool>);
impl DISCONNINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCONNINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCONNINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DisconnInt` writer - Disconnect Detected Interrupt"]
pub struct DISCONNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONNINT_W<'a> {
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
#[doc = "Field `SessReqInt` reader - Session Request/New Session Detected Interrupt"]
pub struct SESSREQINT_R(crate::FieldReader<bool, bool>);
impl SESSREQINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESSREQINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESSREQINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SessReqInt` writer - Session Request/New Session Detected Interrupt"]
pub struct SESSREQINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSREQINT_W<'a> {
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
#[doc = "Field `WkUpInt` reader - Resume/Remote Wakeup Detected Interrupt"]
pub struct WKUPINT_R(crate::FieldReader<bool, bool>);
impl WKUPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WkUpInt` writer - Resume/Remote Wakeup Detected Interrupt"]
pub struct WKUPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINT_W<'a> {
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
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn cur_mod(&self) -> CURMOD_R {
        CURMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn mode_mis(&self) -> MODEMIS_R {
        MODEMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rx_flvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incompl_p(&self) -> INCOMPLP_R {
        INCOMPLP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt"]
    #[inline(always)]
    pub fn prt_int(&self) -> PRTINT_R {
        PRTINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt"]
    #[inline(always)]
    pub fn hch_int(&self) -> HCHINT_R {
        HCHINT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty"]
    #[inline(always)]
    pub fn ptx_femp(&self) -> PTXFEMP_R {
        PTXFEMP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn con_idsts_chng(&self) -> CONIDSTSCHNG_R {
        CONIDSTSCHNG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    pub fn disconn_int(&self) -> DISCONNINT_R {
        DISCONNINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sess_req_int(&self) -> SESSREQINT_R {
        SESSREQINT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wk_up_int(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn mode_mis(&mut self) -> MODEMIS_W {
        MODEMIS_W { w: self }
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incompl_p(&mut self) -> INCOMPLP_W {
        INCOMPLP_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn con_idsts_chng(&mut self) -> CONIDSTSCHNG_W {
        CONIDSTSCHNG_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    pub fn disconn_int(&mut self) -> DISCONNINT_W {
        DISCONNINT_W { w: self }
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sess_req_int(&mut self) -> SESSREQINT_W {
        SESSREQINT_W { w: self }
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wk_up_int(&mut self) -> WKUPINT_W {
        WKUPINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register \\[HOSTMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintsts_hostmode](index.html) module"]
pub struct GINTSTS_HOSTMODE_SPEC;
impl crate::RegisterSpec for GINTSTS_HOSTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintsts_hostmode::R](R) reader structure"]
impl crate::Readable for GINTSTS_HOSTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintsts_hostmode::W](W) writer structure"]
impl crate::Writable for GINTSTS_HOSTMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTSTS_HOSTMODE to value 0x1400_0020"]
impl crate::Resettable for GINTSTS_HOSTMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1400_0020
    }
}
