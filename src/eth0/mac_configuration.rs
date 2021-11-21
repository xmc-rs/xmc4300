#[doc = "Register `MAC_CONFIGURATION` reader"]
pub struct R(crate::R<MAC_CONFIGURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_CONFIGURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_CONFIGURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_CONFIGURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_CONFIGURATION` writer"]
pub struct W(crate::W<MAC_CONFIGURATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_CONFIGURATION_SPEC>;
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
impl From<crate::W<MAC_CONFIGURATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_CONFIGURATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit Frames"]
pub struct PRELEN_R(crate::FieldReader<u8, u8>);
impl PRELEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit Frames"]
pub struct PRELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `RE` reader - Receiver Enable"]
pub struct RE_R(crate::FieldReader<bool, bool>);
impl RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RE` writer - Receiver Enable"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
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
#[doc = "Field `TE` reader - Transmitter Enable"]
pub struct TE_R(crate::FieldReader<bool, bool>);
impl TE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE` writer - Transmitter Enable"]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
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
#[doc = "Field `DC` reader - Deferral Check"]
pub struct DC_R(crate::FieldReader<bool, bool>);
impl DC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC` writer - Deferral Check"]
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
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
#[doc = "Field `BL` reader - Back-Off Limit"]
pub struct BL_R(crate::FieldReader<u8, u8>);
impl BL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BL` writer - Back-Off Limit"]
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping"]
pub struct ACS_R(crate::FieldReader<bool, bool>);
impl ACS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping"]
pub struct ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACS_W<'a> {
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
#[doc = "Field `DR` reader - Disable Retry"]
pub struct DR_R(crate::FieldReader<bool, bool>);
impl DR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR` writer - Disable Retry"]
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
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
#[doc = "Field `IPC` reader - Checksum Offload"]
pub struct IPC_R(crate::FieldReader<bool, bool>);
impl IPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC` writer - Checksum Offload"]
pub struct IPC_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC_W<'a> {
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
#[doc = "Field `DM` reader - Duplex Mode"]
pub struct DM_R(crate::FieldReader<bool, bool>);
impl DM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM` writer - Duplex Mode"]
pub struct DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_W<'a> {
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
#[doc = "Field `LM` reader - Loopback Mode"]
pub struct LM_R(crate::FieldReader<bool, bool>);
impl LM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LM` writer - Loopback Mode"]
pub struct LM_W<'a> {
    w: &'a mut W,
}
impl<'a> LM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DO` reader - Disable Receive Own"]
pub struct DO_R(crate::FieldReader<bool, bool>);
impl DO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DO` writer - Disable Receive Own"]
pub struct DO_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FES` reader - Speed"]
pub struct FES_R(crate::FieldReader<bool, bool>);
impl FES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FES` writer - Speed"]
pub struct FES_W<'a> {
    w: &'a mut W,
}
impl<'a> FES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission"]
pub struct DCRS_R(crate::FieldReader<bool, bool>);
impl DCRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission"]
pub struct DCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRS_W<'a> {
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
#[doc = "Field `IFG` reader - Inter-Frame Gap"]
pub struct IFG_R(crate::FieldReader<u8, u8>);
impl IFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFG` writer - Inter-Frame Gap"]
pub struct IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `JE` reader - Jumbo Frame Enable"]
pub struct JE_R(crate::FieldReader<bool, bool>);
impl JE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JE` writer - Jumbo Frame Enable"]
pub struct JE_W<'a> {
    w: &'a mut W,
}
impl<'a> JE_W<'a> {
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
#[doc = "Field `BE` reader - Frame Burst Enable"]
pub struct BE_R(crate::FieldReader<bool, bool>);
impl BE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JD` reader - Jabber Disable"]
pub struct JD_R(crate::FieldReader<bool, bool>);
impl JD_R {
    pub(crate) fn new(bits: bool) -> Self {
        JD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JD` writer - Jabber Disable"]
pub struct JD_W<'a> {
    w: &'a mut W,
}
impl<'a> JD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `WD` reader - Watchdog Disable"]
pub struct WD_R(crate::FieldReader<bool, bool>);
impl WD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WD` writer - Watchdog Disable"]
pub struct WD_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_W<'a> {
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
#[doc = "Field `TC` reader - Transmit Configuration in RMII"]
pub struct TC_R(crate::FieldReader<bool, bool>);
impl TC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CST` reader - CRC Stripping of Type Frames"]
pub struct CST_R(crate::FieldReader<bool, bool>);
impl CST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CST` writer - CRC Stripping of Type Frames"]
pub struct CST_W<'a> {
    w: &'a mut W,
}
impl<'a> CST_W<'a> {
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
#[doc = "Field `TWOKPE` reader - IEEE 802.3as support for 2K packets Enable"]
pub struct TWOKPE_R(crate::FieldReader<bool, bool>);
impl TWOKPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWOKPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWOKPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWOKPE` writer - IEEE 802.3as support for 2K packets Enable"]
pub struct TWOKPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TWOKPE_W<'a> {
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
#[doc = "Field `SARC` reader - Source Address Insertion or Replacement Control"]
pub struct SARC_R(crate::FieldReader<u8, u8>);
impl SARC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SARC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Frame Burst Enable"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmit Configuration in RMII"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CRC Stripping of Type Frames"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IEEE 802.3as support for 2K packets Enable"]
    #[inline(always)]
    pub fn twokpe(&self) -> TWOKPE_R {
        TWOKPE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control"]
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn prelen(&mut self) -> PRELEN_W {
        PRELEN_W { w: self }
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W {
        ACS_W { w: self }
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W {
        IPC_W { w: self }
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W {
        DM_W { w: self }
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W {
        LM_W { w: self }
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn do_(&mut self) -> DO_W {
        DO_W { w: self }
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W {
        FES_W { w: self }
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&mut self) -> DCRS_W {
        DCRS_W { w: self }
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W {
        IFG_W { w: self }
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn je(&mut self) -> JE_W {
        JE_W { w: self }
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W {
        JD_W { w: self }
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W {
        WD_W { w: self }
    }
    #[doc = "Bit 25 - CRC Stripping of Type Frames"]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W {
        CST_W { w: self }
    }
    #[doc = "Bit 27 - IEEE 802.3as support for 2K packets Enable"]
    #[inline(always)]
    pub fn twokpe(&mut self) -> TWOKPE_W {
        TWOKPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_configuration](index.html) module"]
pub struct MAC_CONFIGURATION_SPEC;
impl crate::RegisterSpec for MAC_CONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_configuration::R](R) reader structure"]
impl crate::Readable for MAC_CONFIGURATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_configuration::W](W) writer structure"]
impl crate::Writable for MAC_CONFIGURATION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_CONFIGURATION to value 0x8000"]
impl crate::Resettable for MAC_CONFIGURATION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
