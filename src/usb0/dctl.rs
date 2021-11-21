#[doc = "Register `DCTL` reader"]
pub struct R(crate::R<DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTL` writer"]
pub struct W(crate::W<DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTL_SPEC>;
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
impl From<crate::W<DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RmtWkUpSig` reader - Remote Wakeup Signaling"]
pub struct RMTWKUPSIG_R(crate::FieldReader<bool, bool>);
impl RMTWKUPSIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMTWKUPSIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMTWKUPSIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RmtWkUpSig` writer - Remote Wakeup Signaling"]
pub struct RMTWKUPSIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RMTWKUPSIG_W<'a> {
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
#[doc = "Soft Disconnect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFTDISCON_A {
    #[doc = "0: Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    VALUE1 = 0,
    #[doc = "1: The core drives a device disconnect event to the USB host."]
    VALUE2 = 1,
}
impl From<SFTDISCON_A> for bool {
    #[inline(always)]
    fn from(variant: SFTDISCON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SftDiscon` reader - Soft Disconnect"]
pub struct SFTDISCON_R(crate::FieldReader<bool, SFTDISCON_A>);
impl SFTDISCON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFTDISCON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFTDISCON_A {
        match self.bits {
            false => SFTDISCON_A::VALUE1,
            true => SFTDISCON_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SFTDISCON_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SFTDISCON_A::VALUE2
    }
}
impl core::ops::Deref for SFTDISCON_R {
    type Target = crate::FieldReader<bool, SFTDISCON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SftDiscon` writer - Soft Disconnect"]
pub struct SFTDISCON_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTDISCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFTDISCON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SFTDISCON_A::VALUE1)
    }
    #[doc = "The core drives a device disconnect event to the USB host."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SFTDISCON_A::VALUE2)
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
#[doc = "Global Non-periodic IN NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GNPINNAKSTS_A {
    #[doc = "0: A handshake is sent out based on the data availability in the transmit FIFO."]
    VALUE1 = 0,
    #[doc = "1: A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data availability in the transmit FIFO."]
    VALUE2 = 1,
}
impl From<GNPINNAKSTS_A> for bool {
    #[inline(always)]
    fn from(variant: GNPINNAKSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GNPINNakSts` reader - Global Non-periodic IN NAK Status"]
pub struct GNPINNAKSTS_R(crate::FieldReader<bool, GNPINNAKSTS_A>);
impl GNPINNAKSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        GNPINNAKSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GNPINNAKSTS_A {
        match self.bits {
            false => GNPINNAKSTS_A::VALUE1,
            true => GNPINNAKSTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == GNPINNAKSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GNPINNAKSTS_A::VALUE2
    }
}
impl core::ops::Deref for GNPINNAKSTS_R {
    type Target = crate::FieldReader<bool, GNPINNAKSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Global OUT NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GOUTNAKSTS_A {
    #[doc = "0: A handshake is sent based on the FIFO Status and the NAK and STALL bit settings."]
    VALUE1 = 0,
    #[doc = "1: No data is written to the RxFIFO, irrespective of space availability. Sends a NAK handshake on all packets, except on SETUP transactions. All isochronous OUT packets are dropped."]
    VALUE2 = 1,
}
impl From<GOUTNAKSTS_A> for bool {
    #[inline(always)]
    fn from(variant: GOUTNAKSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GOUTNakSts` reader - Global OUT NAK Status"]
pub struct GOUTNAKSTS_R(crate::FieldReader<bool, GOUTNAKSTS_A>);
impl GOUTNAKSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        GOUTNAKSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GOUTNAKSTS_A {
        match self.bits {
            false => GOUTNAKSTS_A::VALUE1,
            true => GOUTNAKSTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == GOUTNAKSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GOUTNAKSTS_A::VALUE2
    }
}
impl core::ops::Deref for GOUTNAKSTS_R {
    type Target = crate::FieldReader<bool, GOUTNAKSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SGNPInNak` writer - Set Global Non-periodic IN NAK"]
pub struct SGNPINNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGNPINNAK_W<'a> {
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
#[doc = "Field `CGNPInNak` writer - Clear Global Non-periodic IN NAK"]
pub struct CGNPINNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGNPINNAK_W<'a> {
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
#[doc = "Field `SGOUTNak` writer - Set Global OUT NAK"]
pub struct SGOUTNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGOUTNAK_W<'a> {
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
#[doc = "Field `CGOUTNak` writer - Clear Global OUT NAK"]
pub struct CGOUTNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGOUTNAK_W<'a> {
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
#[doc = "Global Multi Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GMC_A {
    #[doc = "0: Invalid."]
    VALUE1 = 0,
    #[doc = "1: 1 packet."]
    VALUE2 = 1,
    #[doc = "2: 2 packets."]
    VALUE3 = 2,
    #[doc = "3: 3 packets."]
    VALUE4 = 3,
}
impl From<GMC_A> for u8 {
    #[inline(always)]
    fn from(variant: GMC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GMC` reader - Global Multi Count"]
pub struct GMC_R(crate::FieldReader<u8, GMC_A>);
impl GMC_R {
    pub(crate) fn new(bits: u8) -> Self {
        GMC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GMC_A {
        match self.bits {
            0 => GMC_A::VALUE1,
            1 => GMC_A::VALUE2,
            2 => GMC_A::VALUE3,
            3 => GMC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == GMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GMC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == GMC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == GMC_A::VALUE4
    }
}
impl core::ops::Deref for GMC_R {
    type Target = crate::FieldReader<u8, GMC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GMC` writer - Global Multi Count"]
pub struct GMC_W<'a> {
    w: &'a mut W,
}
impl<'a> GMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GMC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Invalid."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GMC_A::VALUE1)
    }
    #[doc = "1 packet."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GMC_A::VALUE2)
    }
    #[doc = "2 packets."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GMC_A::VALUE3)
    }
    #[doc = "3 packets."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GMC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNRFRMNUM_A {
    #[doc = "0: Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    VALUE1 = 0,
    #[doc = "1: Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    VALUE2 = 1,
}
impl From<IGNRFRMNUM_A> for bool {
    #[inline(always)]
    fn from(variant: IGNRFRMNUM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IgnrFrmNum` reader - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
pub struct IGNRFRMNUM_R(crate::FieldReader<bool, IGNRFRMNUM_A>);
impl IGNRFRMNUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        IGNRFRMNUM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGNRFRMNUM_A {
        match self.bits {
            false => IGNRFRMNUM_A::VALUE1,
            true => IGNRFRMNUM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == IGNRFRMNUM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == IGNRFRMNUM_A::VALUE2
    }
}
impl core::ops::Deref for IGNRFRMNUM_R {
    type Target = crate::FieldReader<bool, IGNRFRMNUM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IgnrFrmNum` writer - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
pub struct IGNRFRMNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNRFRMNUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGNRFRMNUM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IGNRFRMNUM_A::VALUE1)
    }
    #[doc = "Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IGNRFRMNUM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `NakOnBble` reader - Set NAK automatically on babble"]
pub struct NAKONBBLE_R(crate::FieldReader<bool, bool>);
impl NAKONBBLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKONBBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKONBBLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NakOnBble` writer - Set NAK automatically on babble"]
pub struct NAKONBBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKONBBLE_W<'a> {
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
#[doc = "Enable continue on BNA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCONTONBNA_A {
    #[doc = "0: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    VALUE1 = 0,
    #[doc = "1: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    VALUE2 = 1,
}
impl From<ENCONTONBNA_A> for bool {
    #[inline(always)]
    fn from(variant: ENCONTONBNA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnContOnBNA` reader - Enable continue on BNA"]
pub struct ENCONTONBNA_R(crate::FieldReader<bool, ENCONTONBNA_A>);
impl ENCONTONBNA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENCONTONBNA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCONTONBNA_A {
        match self.bits {
            false => ENCONTONBNA_A::VALUE1,
            true => ENCONTONBNA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENCONTONBNA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ENCONTONBNA_A::VALUE2
    }
}
impl core::ops::Deref for ENCONTONBNA_R {
    type Target = crate::FieldReader<bool, ENCONTONBNA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EnContOnBNA` writer - Enable continue on BNA"]
pub struct ENCONTONBNA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCONTONBNA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENCONTONBNA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENCONTONBNA_A::VALUE1)
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENCONTONBNA_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmt_wk_up_sig(&self) -> RMTWKUPSIG_R {
        RMTWKUPSIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sft_discon(&self) -> SFTDISCON_R {
        SFTDISCON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline(always)]
    pub fn gnpinnak_sts(&self) -> GNPINNAKSTS_R {
        GNPINNAKSTS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline(always)]
    pub fn goutnak_sts(&self) -> GOUTNAKSTS_R {
        GOUTNAKSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Global Multi Count"]
    #[inline(always)]
    pub fn gmc(&self) -> GMC_R {
        GMC_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
    #[inline(always)]
    pub fn ignr_frm_num(&self) -> IGNRFRMNUM_R {
        IGNRFRMNUM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set NAK automatically on babble"]
    #[inline(always)]
    pub fn nak_on_bble(&self) -> NAKONBBLE_R {
        NAKONBBLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable continue on BNA"]
    #[inline(always)]
    pub fn en_cont_on_bna(&self) -> ENCONTONBNA_R {
        ENCONTONBNA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmt_wk_up_sig(&mut self) -> RMTWKUPSIG_W {
        RMTWKUPSIG_W { w: self }
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sft_discon(&mut self) -> SFTDISCON_W {
        SFTDISCON_W { w: self }
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn sgnpin_nak(&mut self) -> SGNPINNAK_W {
        SGNPINNAK_W { w: self }
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn cgnpin_nak(&mut self) -> CGNPINNAK_W {
        CGNPINNAK_W { w: self }
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline(always)]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W {
        SGOUTNAK_W { w: self }
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline(always)]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W {
        CGOUTNAK_W { w: self }
    }
    #[doc = "Bits 13:14 - Global Multi Count"]
    #[inline(always)]
    pub fn gmc(&mut self) -> GMC_W {
        GMC_W { w: self }
    }
    #[doc = "Bit 15 - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
    #[inline(always)]
    pub fn ignr_frm_num(&mut self) -> IGNRFRMNUM_W {
        IGNRFRMNUM_W { w: self }
    }
    #[doc = "Bit 16 - Set NAK automatically on babble"]
    #[inline(always)]
    pub fn nak_on_bble(&mut self) -> NAKONBBLE_W {
        NAKONBBLE_W { w: self }
    }
    #[doc = "Bit 17 - Enable continue on BNA"]
    #[inline(always)]
    pub fn en_cont_on_bna(&mut self) -> ENCONTONBNA_W {
        ENCONTONBNA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](index.html) module"]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctl::R](R) reader structure"]
impl crate::Readable for DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctl::W](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCTL to value 0x02"]
impl crate::Resettable for DCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
