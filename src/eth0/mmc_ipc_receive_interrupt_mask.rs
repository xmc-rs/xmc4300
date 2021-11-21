#[doc = "Register `MMC_IPC_RECEIVE_INTERRUPT_MASK` reader"]
pub struct R(crate::R<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_IPC_RECEIVE_INTERRUPT_MASK` writer"]
pub struct W(crate::W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXIPV4GFIM` reader - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
pub struct RXIPV4GFIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4GFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4GFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4GFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4GFIM` writer - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
pub struct RXIPV4GFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4GFIM_W<'a> {
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
#[doc = "Field `RXIPV4HERFIM` reader - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
pub struct RXIPV4HERFIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4HERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4HERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4HERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4HERFIM` writer - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
pub struct RXIPV4HERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4HERFIM_W<'a> {
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
#[doc = "Field `RXIPV4NOPAYFIM` reader - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
pub struct RXIPV4NOPAYFIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4NOPAYFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4NOPAYFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4NOPAYFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4NOPAYFIM` writer - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
pub struct RXIPV4NOPAYFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4NOPAYFIM_W<'a> {
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
#[doc = "Field `RXIPV4FRAGFIM` reader - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
pub struct RXIPV4FRAGFIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4FRAGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4FRAGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4FRAGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4FRAGFIM` writer - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
pub struct RXIPV4FRAGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4FRAGFIM_W<'a> {
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
#[doc = "Field `RXIPV4UDSBLFIM` reader - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
pub struct RXIPV4UDSBLFIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4UDSBLFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4UDSBLFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4UDSBLFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4UDSBLFIM` writer - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
pub struct RXIPV4UDSBLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4UDSBLFIM_W<'a> {
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
#[doc = "Field `RXIPV6GFIM` reader - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
pub struct RXIPV6GFIM_R(crate::FieldReader<bool, bool>);
impl RXIPV6GFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6GFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6GFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6GFIM` writer - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
pub struct RXIPV6GFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6GFIM_W<'a> {
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
#[doc = "Field `RXIPV6HERFIM` reader - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
pub struct RXIPV6HERFIM_R(crate::FieldReader<bool, bool>);
impl RXIPV6HERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6HERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6HERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6HERFIM` writer - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
pub struct RXIPV6HERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6HERFIM_W<'a> {
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
#[doc = "Field `RXIPV6NOPAYFIM` reader - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
pub struct RXIPV6NOPAYFIM_R(crate::FieldReader<bool, bool>);
impl RXIPV6NOPAYFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6NOPAYFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6NOPAYFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6NOPAYFIM` writer - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
pub struct RXIPV6NOPAYFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6NOPAYFIM_W<'a> {
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
#[doc = "Field `RXUDPGFIM` reader - MMC Receive UDP Good Frame Counter Interrupt Mask"]
pub struct RXUDPGFIM_R(crate::FieldReader<bool, bool>);
impl RXUDPGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUDPGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUDPGFIM` writer - MMC Receive UDP Good Frame Counter Interrupt Mask"]
pub struct RXUDPGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDPGFIM_W<'a> {
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
#[doc = "Field `RXUDPERFIM` reader - MMC Receive UDP Error Frame Counter Interrupt Mask"]
pub struct RXUDPERFIM_R(crate::FieldReader<bool, bool>);
impl RXUDPERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUDPERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUDPERFIM` writer - MMC Receive UDP Error Frame Counter Interrupt Mask"]
pub struct RXUDPERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDPERFIM_W<'a> {
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
#[doc = "Field `RXTCPGFIM` reader - MMC Receive TCP Good Frame Counter Interrupt Mask"]
pub struct RXTCPGFIM_R(crate::FieldReader<bool, bool>);
impl RXTCPGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTCPGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTCPGFIM` writer - MMC Receive TCP Good Frame Counter Interrupt Mask"]
pub struct RXTCPGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTCPGFIM_W<'a> {
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
#[doc = "Field `RXTCPERFIM` reader - MMC Receive TCP Error Frame Counter Interrupt Mask"]
pub struct RXTCPERFIM_R(crate::FieldReader<bool, bool>);
impl RXTCPERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTCPERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTCPERFIM` writer - MMC Receive TCP Error Frame Counter Interrupt Mask"]
pub struct RXTCPERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTCPERFIM_W<'a> {
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
#[doc = "Field `RXICMPGFIM` reader - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
pub struct RXICMPGFIM_R(crate::FieldReader<bool, bool>);
impl RXICMPGFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXICMPGFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPGFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXICMPGFIM` writer - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
pub struct RXICMPGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXICMPGFIM_W<'a> {
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
#[doc = "Field `RXICMPERFIM` reader - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
pub struct RXICMPERFIM_R(crate::FieldReader<bool, bool>);
impl RXICMPERFIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXICMPERFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPERFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXICMPERFIM` writer - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
pub struct RXICMPERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXICMPERFIM_W<'a> {
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
#[doc = "Field `RXIPV4GOIM` reader - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
pub struct RXIPV4GOIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4GOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4GOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4GOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4GOIM` writer - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
pub struct RXIPV4GOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4GOIM_W<'a> {
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
#[doc = "Field `RXIPV4HEROIM` reader - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
pub struct RXIPV4HEROIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4HEROIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4HEROIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4HEROIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4HEROIM` writer - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
pub struct RXIPV4HEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4HEROIM_W<'a> {
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
#[doc = "Field `RXIPV4NOPAYOIM` reader - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
pub struct RXIPV4NOPAYOIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4NOPAYOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4NOPAYOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4NOPAYOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4NOPAYOIM` writer - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
pub struct RXIPV4NOPAYOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4NOPAYOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RXIPV4FRAGOIM` reader - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
pub struct RXIPV4FRAGOIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4FRAGOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4FRAGOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4FRAGOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4FRAGOIM` writer - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
pub struct RXIPV4FRAGOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4FRAGOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RXIPV4UDSBLOIM` reader - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
pub struct RXIPV4UDSBLOIM_R(crate::FieldReader<bool, bool>);
impl RXIPV4UDSBLOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4UDSBLOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4UDSBLOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4UDSBLOIM` writer - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
pub struct RXIPV4UDSBLOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4UDSBLOIM_W<'a> {
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
#[doc = "Field `RXIPV6GOIM` reader - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
pub struct RXIPV6GOIM_R(crate::FieldReader<bool, bool>);
impl RXIPV6GOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6GOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6GOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6GOIM` writer - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
pub struct RXIPV6GOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6GOIM_W<'a> {
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
#[doc = "Field `RXIPV6HEROIM` reader - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
pub struct RXIPV6HEROIM_R(crate::FieldReader<bool, bool>);
impl RXIPV6HEROIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6HEROIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6HEROIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6HEROIM` writer - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
pub struct RXIPV6HEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6HEROIM_W<'a> {
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
#[doc = "Field `RXIPV6NOPAYOIM` reader - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
pub struct RXIPV6NOPAYOIM_R(crate::FieldReader<bool, bool>);
impl RXIPV6NOPAYOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6NOPAYOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6NOPAYOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6NOPAYOIM` writer - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
pub struct RXIPV6NOPAYOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6NOPAYOIM_W<'a> {
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
#[doc = "Field `RXUDPGOIM` reader - MMC Receive UDP Good Octet Counter Interrupt Mask"]
pub struct RXUDPGOIM_R(crate::FieldReader<bool, bool>);
impl RXUDPGOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUDPGOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPGOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUDPGOIM` writer - MMC Receive UDP Good Octet Counter Interrupt Mask"]
pub struct RXUDPGOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDPGOIM_W<'a> {
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
#[doc = "Field `RXUDPEROIM` reader - MMC Receive UDP Error Octet Counter Interrupt Mask"]
pub struct RXUDPEROIM_R(crate::FieldReader<bool, bool>);
impl RXUDPEROIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUDPEROIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPEROIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUDPEROIM` writer - MMC Receive UDP Error Octet Counter Interrupt Mask"]
pub struct RXUDPEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDPEROIM_W<'a> {
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
#[doc = "Field `RXTCPGOIM` reader - MMC Receive TCP Good Octet Counter Interrupt Mask"]
pub struct RXTCPGOIM_R(crate::FieldReader<bool, bool>);
impl RXTCPGOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTCPGOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPGOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTCPGOIM` writer - MMC Receive TCP Good Octet Counter Interrupt Mask"]
pub struct RXTCPGOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTCPGOIM_W<'a> {
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
#[doc = "Field `RXTCPEROIM` reader - MMC Receive TCP Error Octet Counter Interrupt Mask"]
pub struct RXTCPEROIM_R(crate::FieldReader<bool, bool>);
impl RXTCPEROIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTCPEROIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPEROIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTCPEROIM` writer - MMC Receive TCP Error Octet Counter Interrupt Mask"]
pub struct RXTCPEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTCPEROIM_W<'a> {
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
#[doc = "Field `RXICMPGOIM` reader - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
pub struct RXICMPGOIM_R(crate::FieldReader<bool, bool>);
impl RXICMPGOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXICMPGOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPGOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXICMPGOIM` writer - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
pub struct RXICMPGOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXICMPGOIM_W<'a> {
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
#[doc = "Field `RXICMPEROIM` reader - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
pub struct RXICMPEROIM_R(crate::FieldReader<bool, bool>);
impl RXICMPEROIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXICMPEROIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPEROIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXICMPEROIM` writer - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
pub struct RXICMPEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXICMPEROIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4gfim(&self) -> RXIPV4GFIM_R {
        RXIPV4GFIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4herfim(&self) -> RXIPV4HERFIM_R {
        RXIPV4HERFIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayfim(&self) -> RXIPV4NOPAYFIM_R {
        RXIPV4NOPAYFIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragfim(&self) -> RXIPV4FRAGFIM_R {
        RXIPV4FRAGFIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsblfim(&self) -> RXIPV4UDSBLFIM_R {
        RXIPV4UDSBLFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6gfim(&self) -> RXIPV6GFIM_R {
        RXIPV6GFIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6herfim(&self) -> RXIPV6HERFIM_R {
        RXIPV6HERFIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayfim(&self) -> RXIPV6NOPAYFIM_R {
        RXIPV6NOPAYFIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgfim(&self) -> RXUDPGFIM_R {
        RXUDPGFIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperfim(&self) -> RXUDPERFIM_R {
        RXUDPERFIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgfim(&self) -> RXTCPGFIM_R {
        RXTCPGFIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperfim(&self) -> RXTCPERFIM_R {
        RXTCPERFIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgfim(&self) -> RXICMPGFIM_R {
        RXICMPGFIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperfim(&self) -> RXICMPERFIM_R {
        RXICMPERFIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4goim(&self) -> RXIPV4GOIM_R {
        RXIPV4GOIM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4heroim(&self) -> RXIPV4HEROIM_R {
        RXIPV4HEROIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayoim(&self) -> RXIPV4NOPAYOIM_R {
        RXIPV4NOPAYOIM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragoim(&self) -> RXIPV4FRAGOIM_R {
        RXIPV4FRAGOIM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsbloim(&self) -> RXIPV4UDSBLOIM_R {
        RXIPV4UDSBLOIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6goim(&self) -> RXIPV6GOIM_R {
        RXIPV6GOIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6heroim(&self) -> RXIPV6HEROIM_R {
        RXIPV6HEROIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayoim(&self) -> RXIPV6NOPAYOIM_R {
        RXIPV6NOPAYOIM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgoim(&self) -> RXUDPGOIM_R {
        RXUDPGOIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperoim(&self) -> RXUDPEROIM_R {
        RXUDPEROIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgoim(&self) -> RXTCPGOIM_R {
        RXTCPGOIM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperoim(&self) -> RXTCPEROIM_R {
        RXTCPEROIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgoim(&self) -> RXICMPGOIM_R {
        RXICMPGOIM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperoim(&self) -> RXICMPEROIM_R {
        RXICMPEROIM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4gfim(&mut self) -> RXIPV4GFIM_W {
        RXIPV4GFIM_W { w: self }
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4herfim(&mut self) -> RXIPV4HERFIM_W {
        RXIPV4HERFIM_W { w: self }
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayfim(&mut self) -> RXIPV4NOPAYFIM_W {
        RXIPV4NOPAYFIM_W { w: self }
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragfim(&mut self) -> RXIPV4FRAGFIM_W {
        RXIPV4FRAGFIM_W { w: self }
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsblfim(&mut self) -> RXIPV4UDSBLFIM_W {
        RXIPV4UDSBLFIM_W { w: self }
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6gfim(&mut self) -> RXIPV6GFIM_W {
        RXIPV6GFIM_W { w: self }
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6herfim(&mut self) -> RXIPV6HERFIM_W {
        RXIPV6HERFIM_W { w: self }
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayfim(&mut self) -> RXIPV6NOPAYFIM_W {
        RXIPV6NOPAYFIM_W { w: self }
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgfim(&mut self) -> RXUDPGFIM_W {
        RXUDPGFIM_W { w: self }
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperfim(&mut self) -> RXUDPERFIM_W {
        RXUDPERFIM_W { w: self }
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgfim(&mut self) -> RXTCPGFIM_W {
        RXTCPGFIM_W { w: self }
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperfim(&mut self) -> RXTCPERFIM_W {
        RXTCPERFIM_W { w: self }
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgfim(&mut self) -> RXICMPGFIM_W {
        RXICMPGFIM_W { w: self }
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperfim(&mut self) -> RXICMPERFIM_W {
        RXICMPERFIM_W { w: self }
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4goim(&mut self) -> RXIPV4GOIM_W {
        RXIPV4GOIM_W { w: self }
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4heroim(&mut self) -> RXIPV4HEROIM_W {
        RXIPV4HEROIM_W { w: self }
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayoim(&mut self) -> RXIPV4NOPAYOIM_W {
        RXIPV4NOPAYOIM_W { w: self }
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragoim(&mut self) -> RXIPV4FRAGOIM_W {
        RXIPV4FRAGOIM_W { w: self }
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsbloim(&mut self) -> RXIPV4UDSBLOIM_W {
        RXIPV4UDSBLOIM_W { w: self }
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6goim(&mut self) -> RXIPV6GOIM_W {
        RXIPV6GOIM_W { w: self }
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6heroim(&mut self) -> RXIPV6HEROIM_W {
        RXIPV6HEROIM_W { w: self }
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayoim(&mut self) -> RXIPV6NOPAYOIM_W {
        RXIPV6NOPAYOIM_W { w: self }
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgoim(&mut self) -> RXUDPGOIM_W {
        RXUDPGOIM_W { w: self }
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperoim(&mut self) -> RXUDPEROIM_W {
        RXUDPEROIM_W { w: self }
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgoim(&mut self) -> RXTCPGOIM_W {
        RXTCPGOIM_W { w: self }
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperoim(&mut self) -> RXTCPEROIM_W {
        RXTCPEROIM_W { w: self }
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgoim(&mut self) -> RXICMPGOIM_W {
        RXICMPGOIM_W { w: self }
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperoim(&mut self) -> RXICMPEROIM_W {
        RXICMPEROIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_ipc_receive_interrupt_mask](index.html) module"]
pub struct MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_ipc_receive_interrupt_mask::R](R) reader structure"]
impl crate::Readable for MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_ipc_receive_interrupt_mask::W](W) writer structure"]
impl crate::Writable for MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_IPC_RECEIVE_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
