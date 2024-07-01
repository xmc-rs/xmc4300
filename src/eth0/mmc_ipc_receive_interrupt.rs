#[doc = "Register `MMC_IPC_RECEIVE_INTERRUPT` reader"]
pub type R = crate::R<MMC_IPC_RECEIVE_INTERRUPT_SPEC>;
#[doc = "Field `RXIPV4GFIS` reader - MMC Receive IPV4 Good Frame Counter Interrupt Status"]
pub type RXIPV4GFIS_R = crate::BitReader;
#[doc = "Field `RXIPV4HERFIS` reader - MMC Receive IPV4 Header Error Frame Counter Interrupt Status"]
pub type RXIPV4HERFIS_R = crate::BitReader;
#[doc = "Field `RXIPV4NOPAYFIS` reader - MMC Receive IPV4 No Payload Frame Counter Interrupt Status"]
pub type RXIPV4NOPAYFIS_R = crate::BitReader;
#[doc = "Field `RXIPV4FRAGFIS` reader - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status"]
pub type RXIPV4FRAGFIS_R = crate::BitReader;
#[doc = "Field `RXIPV4UDSBLFIS` reader - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status"]
pub type RXIPV4UDSBLFIS_R = crate::BitReader;
#[doc = "Field `RXIPV6GFIS` reader - MMC Receive IPV6 Good Frame Counter Interrupt Status"]
pub type RXIPV6GFIS_R = crate::BitReader;
#[doc = "Field `RXIPV6HERFIS` reader - MMC Receive IPV6 Header Error Frame Counter Interrupt Status"]
pub type RXIPV6HERFIS_R = crate::BitReader;
#[doc = "Field `RXIPV6NOPAYFIS` reader - MMC Receive IPV6 No Payload Frame Counter Interrupt Status"]
pub type RXIPV6NOPAYFIS_R = crate::BitReader;
#[doc = "Field `RXUDPGFIS` reader - MMC Receive UDP Good Frame Counter Interrupt Status"]
pub type RXUDPGFIS_R = crate::BitReader;
#[doc = "Field `RXUDPERFIS` reader - MMC Receive UDP Error Frame Counter Interrupt Status"]
pub type RXUDPERFIS_R = crate::BitReader;
#[doc = "Field `RXTCPGFIS` reader - MMC Receive TCP Good Frame Counter Interrupt Status"]
pub type RXTCPGFIS_R = crate::BitReader;
#[doc = "Field `RXTCPERFIS` reader - MMC Receive TCP Error Frame Counter Interrupt Status"]
pub type RXTCPERFIS_R = crate::BitReader;
#[doc = "Field `RXICMPGFIS` reader - MMC Receive ICMP Good Frame Counter Interrupt Status"]
pub type RXICMPGFIS_R = crate::BitReader;
#[doc = "Field `RXICMPERFIS` reader - MMC Receive ICMP Error Frame Counter Interrupt Status"]
pub type RXICMPERFIS_R = crate::BitReader;
#[doc = "Field `RXIPV4GOIS` reader - MMC Receive IPV4 Good Octet Counter Interrupt Status"]
pub type RXIPV4GOIS_R = crate::BitReader;
#[doc = "Field `RXIPV4HEROIS` reader - MMC Receive IPV4 Header Error Octet Counter Interrupt Status"]
pub type RXIPV4HEROIS_R = crate::BitReader;
#[doc = "Field `RXIPV4NOPAYOIS` reader - MMC Receive IPV4 No Payload Octet Counter Interrupt Status"]
pub type RXIPV4NOPAYOIS_R = crate::BitReader;
#[doc = "Field `RXIPV4FRAGOIS` reader - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status"]
pub type RXIPV4FRAGOIS_R = crate::BitReader;
#[doc = "Field `RXIPV4UDSBLOIS` reader - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status"]
pub type RXIPV4UDSBLOIS_R = crate::BitReader;
#[doc = "Field `RXIPV6GOIS` reader - MMC Receive IPV6 Good Octet Counter Interrupt Status"]
pub type RXIPV6GOIS_R = crate::BitReader;
#[doc = "Field `RXIPV6HEROIS` reader - MMC Receive IPV6 Header Error Octet Counter Interrupt Status"]
pub type RXIPV6HEROIS_R = crate::BitReader;
#[doc = "Field `RXIPV6NOPAYOIS` reader - MMC Receive IPV6 No Payload Octet Counter Interrupt Status"]
pub type RXIPV6NOPAYOIS_R = crate::BitReader;
#[doc = "Field `RXUDPGOIS` reader - MMC Receive UDP Good Octet Counter Interrupt Status"]
pub type RXUDPGOIS_R = crate::BitReader;
#[doc = "Field `RXUDPEROIS` reader - MMC Receive UDP Error Octet Counter Interrupt Status"]
pub type RXUDPEROIS_R = crate::BitReader;
#[doc = "Field `RXTCPGOIS` reader - MMC Receive TCP Good Octet Counter Interrupt Status"]
pub type RXTCPGOIS_R = crate::BitReader;
#[doc = "Field `RXTCPEROIS` reader - MMC Receive TCP Error Octet Counter Interrupt Status"]
pub type RXTCPEROIS_R = crate::BitReader;
#[doc = "Field `RXICMPGOIS` reader - MMC Receive ICMP Good Octet Counter Interrupt Status"]
pub type RXICMPGOIS_R = crate::BitReader;
#[doc = "Field `RXICMPEROIS` reader - MMC Receive ICMP Error Octet Counter Interrupt Status"]
pub type RXICMPEROIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4gfis(&self) -> RXIPV4GFIS_R {
        RXIPV4GFIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4herfis(&self) -> RXIPV4HERFIS_R {
        RXIPV4HERFIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4nopayfis(&self) -> RXIPV4NOPAYFIS_R {
        RXIPV4NOPAYFIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4fragfis(&self) -> RXIPV4FRAGFIS_R {
        RXIPV4FRAGFIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4udsblfis(&self) -> RXIPV4UDSBLFIS_R {
        RXIPV4UDSBLFIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6gfis(&self) -> RXIPV6GFIS_R {
        RXIPV6GFIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6herfis(&self) -> RXIPV6HERFIS_R {
        RXIPV6HERFIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6nopayfis(&self) -> RXIPV6NOPAYFIS_R {
        RXIPV6NOPAYFIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudpgfis(&self) -> RXUDPGFIS_R {
        RXUDPGFIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudperfis(&self) -> RXUDPERFIS_R {
        RXUDPERFIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcpgfis(&self) -> RXTCPGFIS_R {
        RXTCPGFIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcperfis(&self) -> RXTCPERFIS_R {
        RXTCPERFIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmpgfis(&self) -> RXICMPGFIS_R {
        RXICMPGFIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmperfis(&self) -> RXICMPERFIS_R {
        RXICMPERFIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4gois(&self) -> RXIPV4GOIS_R {
        RXIPV4GOIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4herois(&self) -> RXIPV4HEROIS_R {
        RXIPV4HEROIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4nopayois(&self) -> RXIPV4NOPAYOIS_R {
        RXIPV4NOPAYOIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4fragois(&self) -> RXIPV4FRAGOIS_R {
        RXIPV4FRAGOIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4udsblois(&self) -> RXIPV4UDSBLOIS_R {
        RXIPV4UDSBLOIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6gois(&self) -> RXIPV6GOIS_R {
        RXIPV6GOIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6herois(&self) -> RXIPV6HEROIS_R {
        RXIPV6HEROIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6nopayois(&self) -> RXIPV6NOPAYOIS_R {
        RXIPV6NOPAYOIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudpgois(&self) -> RXUDPGOIS_R {
        RXUDPGOIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudperois(&self) -> RXUDPEROIS_R {
        RXUDPEROIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcpgois(&self) -> RXTCPGOIS_R {
        RXTCPGOIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcperois(&self) -> RXTCPEROIS_R {
        RXTCPEROIS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmpgois(&self) -> RXICMPGOIS_R {
        RXICMPGOIS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmperois(&self) -> RXICMPEROIS_R {
        RXICMPEROIS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_ipc_receive_interrupt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_IPC_RECEIVE_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_IPC_RECEIVE_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_ipc_receive_interrupt::R`](R) reader structure"]
impl crate::Readable for MMC_IPC_RECEIVE_INTERRUPT_SPEC {}
#[doc = "`reset()` method sets MMC_IPC_RECEIVE_INTERRUPT to value 0"]
impl crate::Resettable for MMC_IPC_RECEIVE_INTERRUPT_SPEC {
    const RESET_VALUE: u32 = 0;
}
