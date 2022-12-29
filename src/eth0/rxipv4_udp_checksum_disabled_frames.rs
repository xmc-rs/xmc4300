#[doc = "Register `RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES` reader"]
pub struct R(crate::R<RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXIPV4UDSBLFRM` reader - This field indicates the number of received good IPv4 datagrams which have the UDP payload with checksum disabled."]
pub type RXIPV4UDSBLFRM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good IPv4 datagrams which have the UDP payload with checksum disabled."]
    #[inline(always)]
    pub fn rxipv4udsblfrm(&self) -> RXIPV4UDSBLFRM_R {
        RXIPV4UDSBLFRM_R::new(self.bits)
    }
}
#[doc = "Receive IPV4 UDP Checksum Disabled Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxipv4_udp_checksum_disabled_frames](index.html) module"]
pub struct RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC;
impl crate::RegisterSpec for RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxipv4_udp_checksum_disabled_frames::R](R) reader structure"]
impl crate::Readable for RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES to value 0"]
impl crate::Resettable for RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
