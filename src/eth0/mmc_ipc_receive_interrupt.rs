#[doc = "Register `MMC_IPC_RECEIVE_INTERRUPT` reader"]
pub struct R(crate::R<MMC_IPC_RECEIVE_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_IPC_RECEIVE_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_IPC_RECEIVE_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_IPC_RECEIVE_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXIPV4GFIS` reader - MMC Receive IPV4 Good Frame Counter Interrupt Status"]
pub struct RXIPV4GFIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4GFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4GFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4GFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4HERFIS` reader - MMC Receive IPV4 Header Error Frame Counter Interrupt Status"]
pub struct RXIPV4HERFIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4HERFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4HERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4HERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4NOPAYFIS` reader - MMC Receive IPV4 No Payload Frame Counter Interrupt Status"]
pub struct RXIPV4NOPAYFIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4NOPAYFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4NOPAYFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4NOPAYFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4FRAGFIS` reader - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status"]
pub struct RXIPV4FRAGFIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4FRAGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4FRAGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4FRAGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4UDSBLFIS` reader - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status"]
pub struct RXIPV4UDSBLFIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4UDSBLFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4UDSBLFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4UDSBLFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6GFIS` reader - MMC Receive IPV6 Good Frame Counter Interrupt Status"]
pub struct RXIPV6GFIS_R(crate::FieldReader<bool, bool>);
impl RXIPV6GFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6GFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6GFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6HERFIS` reader - MMC Receive IPV6 Header Error Frame Counter Interrupt Status"]
pub struct RXIPV6HERFIS_R(crate::FieldReader<bool, bool>);
impl RXIPV6HERFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6HERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6HERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6NOPAYFIS` reader - MMC Receive IPV6 No Payload Frame Counter Interrupt Status"]
pub struct RXIPV6NOPAYFIS_R(crate::FieldReader<bool, bool>);
impl RXIPV6NOPAYFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6NOPAYFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6NOPAYFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUDPGFIS` reader - MMC Receive UDP Good Frame Counter Interrupt Status"]
pub struct RXUDPGFIS_R(crate::FieldReader<bool, bool>);
impl RXUDPGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUDPGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUDPERFIS` reader - MMC Receive UDP Error Frame Counter Interrupt Status"]
pub struct RXUDPERFIS_R(crate::FieldReader<bool, bool>);
impl RXUDPERFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUDPERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTCPGFIS` reader - MMC Receive TCP Good Frame Counter Interrupt Status"]
pub struct RXTCPGFIS_R(crate::FieldReader<bool, bool>);
impl RXTCPGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTCPGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTCPERFIS` reader - MMC Receive TCP Error Frame Counter Interrupt Status"]
pub struct RXTCPERFIS_R(crate::FieldReader<bool, bool>);
impl RXTCPERFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTCPERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXICMPGFIS` reader - MMC Receive ICMP Good Frame Counter Interrupt Status"]
pub struct RXICMPGFIS_R(crate::FieldReader<bool, bool>);
impl RXICMPGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXICMPGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXICMPERFIS` reader - MMC Receive ICMP Error Frame Counter Interrupt Status"]
pub struct RXICMPERFIS_R(crate::FieldReader<bool, bool>);
impl RXICMPERFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXICMPERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4GOIS` reader - MMC Receive IPV4 Good Octet Counter Interrupt Status"]
pub struct RXIPV4GOIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4GOIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4GOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4GOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4HEROIS` reader - MMC Receive IPV4 Header Error Octet Counter Interrupt Status"]
pub struct RXIPV4HEROIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4HEROIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4HEROIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4HEROIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4NOPAYOIS` reader - MMC Receive IPV4 No Payload Octet Counter Interrupt Status"]
pub struct RXIPV4NOPAYOIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4NOPAYOIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4NOPAYOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4NOPAYOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4FRAGOIS` reader - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status"]
pub struct RXIPV4FRAGOIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4FRAGOIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4FRAGOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4FRAGOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV4UDSBLOIS` reader - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status"]
pub struct RXIPV4UDSBLOIS_R(crate::FieldReader<bool, bool>);
impl RXIPV4UDSBLOIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV4UDSBLOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV4UDSBLOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6GOIS` reader - MMC Receive IPV6 Good Octet Counter Interrupt Status"]
pub struct RXIPV6GOIS_R(crate::FieldReader<bool, bool>);
impl RXIPV6GOIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6GOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6GOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6HEROIS` reader - MMC Receive IPV6 Header Error Octet Counter Interrupt Status"]
pub struct RXIPV6HEROIS_R(crate::FieldReader<bool, bool>);
impl RXIPV6HEROIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6HEROIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6HEROIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIPV6NOPAYOIS` reader - MMC Receive IPV6 No Payload Octet Counter Interrupt Status"]
pub struct RXIPV6NOPAYOIS_R(crate::FieldReader<bool, bool>);
impl RXIPV6NOPAYOIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIPV6NOPAYOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIPV6NOPAYOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUDPGOIS` reader - MMC Receive UDP Good Octet Counter Interrupt Status"]
pub struct RXUDPGOIS_R(crate::FieldReader<bool, bool>);
impl RXUDPGOIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUDPGOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPGOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUDPEROIS` reader - MMC Receive UDP Error Octet Counter Interrupt Status"]
pub struct RXUDPEROIS_R(crate::FieldReader<bool, bool>);
impl RXUDPEROIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUDPEROIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUDPEROIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTCPGOIS` reader - MMC Receive TCP Good Octet Counter Interrupt Status"]
pub struct RXTCPGOIS_R(crate::FieldReader<bool, bool>);
impl RXTCPGOIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTCPGOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPGOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTCPEROIS` reader - MMC Receive TCP Error Octet Counter Interrupt Status"]
pub struct RXTCPEROIS_R(crate::FieldReader<bool, bool>);
impl RXTCPEROIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTCPEROIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTCPEROIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXICMPGOIS` reader - MMC Receive ICMP Good Octet Counter Interrupt Status"]
pub struct RXICMPGOIS_R(crate::FieldReader<bool, bool>);
impl RXICMPGOIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXICMPGOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPGOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXICMPEROIS` reader - MMC Receive ICMP Error Octet Counter Interrupt Status"]
pub struct RXICMPEROIS_R(crate::FieldReader<bool, bool>);
impl RXICMPEROIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXICMPEROIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXICMPEROIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4gfis(&self) -> RXIPV4GFIS_R {
        RXIPV4GFIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4herfis(&self) -> RXIPV4HERFIS_R {
        RXIPV4HERFIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4nopayfis(&self) -> RXIPV4NOPAYFIS_R {
        RXIPV4NOPAYFIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4fragfis(&self) -> RXIPV4FRAGFIS_R {
        RXIPV4FRAGFIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4udsblfis(&self) -> RXIPV4UDSBLFIS_R {
        RXIPV4UDSBLFIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6gfis(&self) -> RXIPV6GFIS_R {
        RXIPV6GFIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6herfis(&self) -> RXIPV6HERFIS_R {
        RXIPV6HERFIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6nopayfis(&self) -> RXIPV6NOPAYFIS_R {
        RXIPV6NOPAYFIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudpgfis(&self) -> RXUDPGFIS_R {
        RXUDPGFIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudperfis(&self) -> RXUDPERFIS_R {
        RXUDPERFIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcpgfis(&self) -> RXTCPGFIS_R {
        RXTCPGFIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcperfis(&self) -> RXTCPERFIS_R {
        RXTCPERFIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmpgfis(&self) -> RXICMPGFIS_R {
        RXICMPGFIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmperfis(&self) -> RXICMPERFIS_R {
        RXICMPERFIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4gois(&self) -> RXIPV4GOIS_R {
        RXIPV4GOIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4herois(&self) -> RXIPV4HEROIS_R {
        RXIPV4HEROIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4nopayois(&self) -> RXIPV4NOPAYOIS_R {
        RXIPV4NOPAYOIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4fragois(&self) -> RXIPV4FRAGOIS_R {
        RXIPV4FRAGOIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4udsblois(&self) -> RXIPV4UDSBLOIS_R {
        RXIPV4UDSBLOIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6gois(&self) -> RXIPV6GOIS_R {
        RXIPV6GOIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6herois(&self) -> RXIPV6HEROIS_R {
        RXIPV6HEROIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6nopayois(&self) -> RXIPV6NOPAYOIS_R {
        RXIPV6NOPAYOIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudpgois(&self) -> RXUDPGOIS_R {
        RXUDPGOIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudperois(&self) -> RXUDPEROIS_R {
        RXUDPEROIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcpgois(&self) -> RXTCPGOIS_R {
        RXTCPGOIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcperois(&self) -> RXTCPEROIS_R {
        RXTCPEROIS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmpgois(&self) -> RXICMPGOIS_R {
        RXICMPGOIS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmperois(&self) -> RXICMPEROIS_R {
        RXICMPEROIS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_ipc_receive_interrupt](index.html) module"]
pub struct MMC_IPC_RECEIVE_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_IPC_RECEIVE_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_ipc_receive_interrupt::R](R) reader structure"]
impl crate::Readable for MMC_IPC_RECEIVE_INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMC_IPC_RECEIVE_INTERRUPT to value 0"]
impl crate::Resettable for MMC_IPC_RECEIVE_INTERRUPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
