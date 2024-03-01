#[doc = "Register `MMC_IPC_RECEIVE_INTERRUPT` reader"]
pub type R = crate::R<MmcIpcReceiveInterruptSpec>;
#[doc = "Field `RXIPV4GFIS` reader - MMC Receive IPV4 Good Frame Counter Interrupt Status"]
pub type Rxipv4gfisR = crate::BitReader;
#[doc = "Field `RXIPV4HERFIS` reader - MMC Receive IPV4 Header Error Frame Counter Interrupt Status"]
pub type Rxipv4herfisR = crate::BitReader;
#[doc = "Field `RXIPV4NOPAYFIS` reader - MMC Receive IPV4 No Payload Frame Counter Interrupt Status"]
pub type Rxipv4nopayfisR = crate::BitReader;
#[doc = "Field `RXIPV4FRAGFIS` reader - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status"]
pub type Rxipv4fragfisR = crate::BitReader;
#[doc = "Field `RXIPV4UDSBLFIS` reader - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status"]
pub type Rxipv4udsblfisR = crate::BitReader;
#[doc = "Field `RXIPV6GFIS` reader - MMC Receive IPV6 Good Frame Counter Interrupt Status"]
pub type Rxipv6gfisR = crate::BitReader;
#[doc = "Field `RXIPV6HERFIS` reader - MMC Receive IPV6 Header Error Frame Counter Interrupt Status"]
pub type Rxipv6herfisR = crate::BitReader;
#[doc = "Field `RXIPV6NOPAYFIS` reader - MMC Receive IPV6 No Payload Frame Counter Interrupt Status"]
pub type Rxipv6nopayfisR = crate::BitReader;
#[doc = "Field `RXUDPGFIS` reader - MMC Receive UDP Good Frame Counter Interrupt Status"]
pub type RxudpgfisR = crate::BitReader;
#[doc = "Field `RXUDPERFIS` reader - MMC Receive UDP Error Frame Counter Interrupt Status"]
pub type RxudperfisR = crate::BitReader;
#[doc = "Field `RXTCPGFIS` reader - MMC Receive TCP Good Frame Counter Interrupt Status"]
pub type RxtcpgfisR = crate::BitReader;
#[doc = "Field `RXTCPERFIS` reader - MMC Receive TCP Error Frame Counter Interrupt Status"]
pub type RxtcperfisR = crate::BitReader;
#[doc = "Field `RXICMPGFIS` reader - MMC Receive ICMP Good Frame Counter Interrupt Status"]
pub type RxicmpgfisR = crate::BitReader;
#[doc = "Field `RXICMPERFIS` reader - MMC Receive ICMP Error Frame Counter Interrupt Status"]
pub type RxicmperfisR = crate::BitReader;
#[doc = "Field `RXIPV4GOIS` reader - MMC Receive IPV4 Good Octet Counter Interrupt Status"]
pub type Rxipv4goisR = crate::BitReader;
#[doc = "Field `RXIPV4HEROIS` reader - MMC Receive IPV4 Header Error Octet Counter Interrupt Status"]
pub type Rxipv4heroisR = crate::BitReader;
#[doc = "Field `RXIPV4NOPAYOIS` reader - MMC Receive IPV4 No Payload Octet Counter Interrupt Status"]
pub type Rxipv4nopayoisR = crate::BitReader;
#[doc = "Field `RXIPV4FRAGOIS` reader - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status"]
pub type Rxipv4fragoisR = crate::BitReader;
#[doc = "Field `RXIPV4UDSBLOIS` reader - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status"]
pub type Rxipv4udsbloisR = crate::BitReader;
#[doc = "Field `RXIPV6GOIS` reader - MMC Receive IPV6 Good Octet Counter Interrupt Status"]
pub type Rxipv6goisR = crate::BitReader;
#[doc = "Field `RXIPV6HEROIS` reader - MMC Receive IPV6 Header Error Octet Counter Interrupt Status"]
pub type Rxipv6heroisR = crate::BitReader;
#[doc = "Field `RXIPV6NOPAYOIS` reader - MMC Receive IPV6 No Payload Octet Counter Interrupt Status"]
pub type Rxipv6nopayoisR = crate::BitReader;
#[doc = "Field `RXUDPGOIS` reader - MMC Receive UDP Good Octet Counter Interrupt Status"]
pub type RxudpgoisR = crate::BitReader;
#[doc = "Field `RXUDPEROIS` reader - MMC Receive UDP Error Octet Counter Interrupt Status"]
pub type RxudperoisR = crate::BitReader;
#[doc = "Field `RXTCPGOIS` reader - MMC Receive TCP Good Octet Counter Interrupt Status"]
pub type RxtcpgoisR = crate::BitReader;
#[doc = "Field `RXTCPEROIS` reader - MMC Receive TCP Error Octet Counter Interrupt Status"]
pub type RxtcperoisR = crate::BitReader;
#[doc = "Field `RXICMPGOIS` reader - MMC Receive ICMP Good Octet Counter Interrupt Status"]
pub type RxicmpgoisR = crate::BitReader;
#[doc = "Field `RXICMPEROIS` reader - MMC Receive ICMP Error Octet Counter Interrupt Status"]
pub type RxicmperoisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4gfis(&self) -> Rxipv4gfisR {
        Rxipv4gfisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4herfis(&self) -> Rxipv4herfisR {
        Rxipv4herfisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4nopayfis(&self) -> Rxipv4nopayfisR {
        Rxipv4nopayfisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4fragfis(&self) -> Rxipv4fragfisR {
        Rxipv4fragfisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4udsblfis(&self) -> Rxipv4udsblfisR {
        Rxipv4udsblfisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6gfis(&self) -> Rxipv6gfisR {
        Rxipv6gfisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6herfis(&self) -> Rxipv6herfisR {
        Rxipv6herfisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6nopayfis(&self) -> Rxipv6nopayfisR {
        Rxipv6nopayfisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudpgfis(&self) -> RxudpgfisR {
        RxudpgfisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudperfis(&self) -> RxudperfisR {
        RxudperfisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcpgfis(&self) -> RxtcpgfisR {
        RxtcpgfisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcperfis(&self) -> RxtcperfisR {
        RxtcperfisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmpgfis(&self) -> RxicmpgfisR {
        RxicmpgfisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmperfis(&self) -> RxicmperfisR {
        RxicmperfisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4gois(&self) -> Rxipv4goisR {
        Rxipv4goisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4herois(&self) -> Rxipv4heroisR {
        Rxipv4heroisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4nopayois(&self) -> Rxipv4nopayoisR {
        Rxipv4nopayoisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4fragois(&self) -> Rxipv4fragoisR {
        Rxipv4fragoisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv4udsblois(&self) -> Rxipv4udsbloisR {
        Rxipv4udsbloisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6gois(&self) -> Rxipv6goisR {
        Rxipv6goisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6herois(&self) -> Rxipv6heroisR {
        Rxipv6heroisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxipv6nopayois(&self) -> Rxipv6nopayoisR {
        Rxipv6nopayoisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudpgois(&self) -> RxudpgoisR {
        RxudpgoisR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxudperois(&self) -> RxudperoisR {
        RxudperoisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcpgois(&self) -> RxtcpgoisR {
        RxtcpgoisR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxtcperois(&self) -> RxtcperoisR {
        RxtcperoisR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmpgois(&self) -> RxicmpgoisR {
        RxicmpgoisR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxicmperois(&self) -> RxicmperoisR {
        RxicmperoisR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_receive_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcIpcReceiveInterruptSpec;
impl crate::RegisterSpec for MmcIpcReceiveInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_ipc_receive_interrupt::R`](R) reader structure"]
impl crate::Readable for MmcIpcReceiveInterruptSpec {}
#[doc = "`reset()` method sets MMC_IPC_RECEIVE_INTERRUPT to value 0"]
impl crate::Resettable for MmcIpcReceiveInterruptSpec {
    const RESET_VALUE: u32 = 0;
}
