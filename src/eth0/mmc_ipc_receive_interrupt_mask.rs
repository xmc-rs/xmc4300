#[doc = "Register `MMC_IPC_RECEIVE_INTERRUPT_MASK` reader"]
pub type R = crate::R<MmcIpcReceiveInterruptMaskSpec>;
#[doc = "Register `MMC_IPC_RECEIVE_INTERRUPT_MASK` writer"]
pub type W = crate::W<MmcIpcReceiveInterruptMaskSpec>;
#[doc = "Field `RXIPV4GFIM` reader - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
pub type Rxipv4gfimR = crate::BitReader;
#[doc = "Field `RXIPV4GFIM` writer - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
pub type Rxipv4gfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4HERFIM` reader - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
pub type Rxipv4herfimR = crate::BitReader;
#[doc = "Field `RXIPV4HERFIM` writer - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
pub type Rxipv4herfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4NOPAYFIM` reader - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
pub type Rxipv4nopayfimR = crate::BitReader;
#[doc = "Field `RXIPV4NOPAYFIM` writer - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
pub type Rxipv4nopayfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4FRAGFIM` reader - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
pub type Rxipv4fragfimR = crate::BitReader;
#[doc = "Field `RXIPV4FRAGFIM` writer - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
pub type Rxipv4fragfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4UDSBLFIM` reader - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
pub type Rxipv4udsblfimR = crate::BitReader;
#[doc = "Field `RXIPV4UDSBLFIM` writer - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
pub type Rxipv4udsblfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6GFIM` reader - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
pub type Rxipv6gfimR = crate::BitReader;
#[doc = "Field `RXIPV6GFIM` writer - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
pub type Rxipv6gfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6HERFIM` reader - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
pub type Rxipv6herfimR = crate::BitReader;
#[doc = "Field `RXIPV6HERFIM` writer - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
pub type Rxipv6herfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6NOPAYFIM` reader - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
pub type Rxipv6nopayfimR = crate::BitReader;
#[doc = "Field `RXIPV6NOPAYFIM` writer - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
pub type Rxipv6nopayfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUDPGFIM` reader - MMC Receive UDP Good Frame Counter Interrupt Mask"]
pub type RxudpgfimR = crate::BitReader;
#[doc = "Field `RXUDPGFIM` writer - MMC Receive UDP Good Frame Counter Interrupt Mask"]
pub type RxudpgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUDPERFIM` reader - MMC Receive UDP Error Frame Counter Interrupt Mask"]
pub type RxudperfimR = crate::BitReader;
#[doc = "Field `RXUDPERFIM` writer - MMC Receive UDP Error Frame Counter Interrupt Mask"]
pub type RxudperfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTCPGFIM` reader - MMC Receive TCP Good Frame Counter Interrupt Mask"]
pub type RxtcpgfimR = crate::BitReader;
#[doc = "Field `RXTCPGFIM` writer - MMC Receive TCP Good Frame Counter Interrupt Mask"]
pub type RxtcpgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTCPERFIM` reader - MMC Receive TCP Error Frame Counter Interrupt Mask"]
pub type RxtcperfimR = crate::BitReader;
#[doc = "Field `RXTCPERFIM` writer - MMC Receive TCP Error Frame Counter Interrupt Mask"]
pub type RxtcperfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICMPGFIM` reader - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
pub type RxicmpgfimR = crate::BitReader;
#[doc = "Field `RXICMPGFIM` writer - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
pub type RxicmpgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICMPERFIM` reader - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
pub type RxicmperfimR = crate::BitReader;
#[doc = "Field `RXICMPERFIM` writer - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
pub type RxicmperfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4GOIM` reader - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
pub type Rxipv4goimR = crate::BitReader;
#[doc = "Field `RXIPV4GOIM` writer - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
pub type Rxipv4goimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4HEROIM` reader - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
pub type Rxipv4heroimR = crate::BitReader;
#[doc = "Field `RXIPV4HEROIM` writer - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
pub type Rxipv4heroimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4NOPAYOIM` reader - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
pub type Rxipv4nopayoimR = crate::BitReader;
#[doc = "Field `RXIPV4NOPAYOIM` writer - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
pub type Rxipv4nopayoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4FRAGOIM` reader - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
pub type Rxipv4fragoimR = crate::BitReader;
#[doc = "Field `RXIPV4FRAGOIM` writer - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
pub type Rxipv4fragoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4UDSBLOIM` reader - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
pub type Rxipv4udsbloimR = crate::BitReader;
#[doc = "Field `RXIPV4UDSBLOIM` writer - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
pub type Rxipv4udsbloimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6GOIM` reader - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
pub type Rxipv6goimR = crate::BitReader;
#[doc = "Field `RXIPV6GOIM` writer - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
pub type Rxipv6goimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6HEROIM` reader - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
pub type Rxipv6heroimR = crate::BitReader;
#[doc = "Field `RXIPV6HEROIM` writer - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
pub type Rxipv6heroimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6NOPAYOIM` reader - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
pub type Rxipv6nopayoimR = crate::BitReader;
#[doc = "Field `RXIPV6NOPAYOIM` writer - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
pub type Rxipv6nopayoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUDPGOIM` reader - MMC Receive UDP Good Octet Counter Interrupt Mask"]
pub type RxudpgoimR = crate::BitReader;
#[doc = "Field `RXUDPGOIM` writer - MMC Receive UDP Good Octet Counter Interrupt Mask"]
pub type RxudpgoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUDPEROIM` reader - MMC Receive UDP Error Octet Counter Interrupt Mask"]
pub type RxudperoimR = crate::BitReader;
#[doc = "Field `RXUDPEROIM` writer - MMC Receive UDP Error Octet Counter Interrupt Mask"]
pub type RxudperoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTCPGOIM` reader - MMC Receive TCP Good Octet Counter Interrupt Mask"]
pub type RxtcpgoimR = crate::BitReader;
#[doc = "Field `RXTCPGOIM` writer - MMC Receive TCP Good Octet Counter Interrupt Mask"]
pub type RxtcpgoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTCPEROIM` reader - MMC Receive TCP Error Octet Counter Interrupt Mask"]
pub type RxtcperoimR = crate::BitReader;
#[doc = "Field `RXTCPEROIM` writer - MMC Receive TCP Error Octet Counter Interrupt Mask"]
pub type RxtcperoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICMPGOIM` reader - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
pub type RxicmpgoimR = crate::BitReader;
#[doc = "Field `RXICMPGOIM` writer - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
pub type RxicmpgoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICMPEROIM` reader - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
pub type RxicmperoimR = crate::BitReader;
#[doc = "Field `RXICMPEROIM` writer - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
pub type RxicmperoimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4gfim(&self) -> Rxipv4gfimR {
        Rxipv4gfimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4herfim(&self) -> Rxipv4herfimR {
        Rxipv4herfimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayfim(&self) -> Rxipv4nopayfimR {
        Rxipv4nopayfimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragfim(&self) -> Rxipv4fragfimR {
        Rxipv4fragfimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsblfim(&self) -> Rxipv4udsblfimR {
        Rxipv4udsblfimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6gfim(&self) -> Rxipv6gfimR {
        Rxipv6gfimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6herfim(&self) -> Rxipv6herfimR {
        Rxipv6herfimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayfim(&self) -> Rxipv6nopayfimR {
        Rxipv6nopayfimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgfim(&self) -> RxudpgfimR {
        RxudpgfimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperfim(&self) -> RxudperfimR {
        RxudperfimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgfim(&self) -> RxtcpgfimR {
        RxtcpgfimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperfim(&self) -> RxtcperfimR {
        RxtcperfimR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgfim(&self) -> RxicmpgfimR {
        RxicmpgfimR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperfim(&self) -> RxicmperfimR {
        RxicmperfimR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4goim(&self) -> Rxipv4goimR {
        Rxipv4goimR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4heroim(&self) -> Rxipv4heroimR {
        Rxipv4heroimR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayoim(&self) -> Rxipv4nopayoimR {
        Rxipv4nopayoimR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragoim(&self) -> Rxipv4fragoimR {
        Rxipv4fragoimR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsbloim(&self) -> Rxipv4udsbloimR {
        Rxipv4udsbloimR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6goim(&self) -> Rxipv6goimR {
        Rxipv6goimR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6heroim(&self) -> Rxipv6heroimR {
        Rxipv6heroimR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayoim(&self) -> Rxipv6nopayoimR {
        Rxipv6nopayoimR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgoim(&self) -> RxudpgoimR {
        RxudpgoimR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperoim(&self) -> RxudperoimR {
        RxudperoimR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgoim(&self) -> RxtcpgoimR {
        RxtcpgoimR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperoim(&self) -> RxtcperoimR {
        RxtcperoimR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgoim(&self) -> RxicmpgoimR {
        RxicmpgoimR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperoim(&self) -> RxicmperoimR {
        RxicmperoimR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4gfim(&mut self) -> Rxipv4gfimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4gfimW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4herfim(&mut self) -> Rxipv4herfimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4herfimW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4nopayfim(&mut self) -> Rxipv4nopayfimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4nopayfimW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4fragfim(&mut self) -> Rxipv4fragfimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4fragfimW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4udsblfim(&mut self) -> Rxipv4udsblfimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4udsblfimW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6gfim(&mut self) -> Rxipv6gfimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv6gfimW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6herfim(&mut self) -> Rxipv6herfimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv6herfimW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6nopayfim(&mut self) -> Rxipv6nopayfimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv6nopayfimW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxudpgfim(&mut self) -> RxudpgfimW<MmcIpcReceiveInterruptMaskSpec> {
        RxudpgfimW::new(self, 8)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxudperfim(&mut self) -> RxudperfimW<MmcIpcReceiveInterruptMaskSpec> {
        RxudperfimW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxtcpgfim(&mut self) -> RxtcpgfimW<MmcIpcReceiveInterruptMaskSpec> {
        RxtcpgfimW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxtcperfim(&mut self) -> RxtcperfimW<MmcIpcReceiveInterruptMaskSpec> {
        RxtcperfimW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxicmpgfim(&mut self) -> RxicmpgfimW<MmcIpcReceiveInterruptMaskSpec> {
        RxicmpgfimW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxicmperfim(&mut self) -> RxicmperfimW<MmcIpcReceiveInterruptMaskSpec> {
        RxicmperfimW::new(self, 13)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4goim(&mut self) -> Rxipv4goimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4goimW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4heroim(&mut self) -> Rxipv4heroimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4heroimW::new(self, 17)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4nopayoim(&mut self) -> Rxipv4nopayoimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4nopayoimW::new(self, 18)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4fragoim(&mut self) -> Rxipv4fragoimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4fragoimW::new(self, 19)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4udsbloim(&mut self) -> Rxipv4udsbloimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv4udsbloimW::new(self, 20)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6goim(&mut self) -> Rxipv6goimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv6goimW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6heroim(&mut self) -> Rxipv6heroimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv6heroimW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6nopayoim(&mut self) -> Rxipv6nopayoimW<MmcIpcReceiveInterruptMaskSpec> {
        Rxipv6nopayoimW::new(self, 23)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxudpgoim(&mut self) -> RxudpgoimW<MmcIpcReceiveInterruptMaskSpec> {
        RxudpgoimW::new(self, 24)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxudperoim(&mut self) -> RxudperoimW<MmcIpcReceiveInterruptMaskSpec> {
        RxudperoimW::new(self, 25)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxtcpgoim(&mut self) -> RxtcpgoimW<MmcIpcReceiveInterruptMaskSpec> {
        RxtcpgoimW::new(self, 26)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxtcperoim(&mut self) -> RxtcperoimW<MmcIpcReceiveInterruptMaskSpec> {
        RxtcperoimW::new(self, 27)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxicmpgoim(&mut self) -> RxicmpgoimW<MmcIpcReceiveInterruptMaskSpec> {
        RxicmpgoimW::new(self, 28)
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxicmperoim(&mut self) -> RxicmperoimW<MmcIpcReceiveInterruptMaskSpec> {
        RxicmperoimW::new(self, 29)
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_receive_interrupt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_ipc_receive_interrupt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcIpcReceiveInterruptMaskSpec;
impl crate::RegisterSpec for MmcIpcReceiveInterruptMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_ipc_receive_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for MmcIpcReceiveInterruptMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_ipc_receive_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for MmcIpcReceiveInterruptMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_IPC_RECEIVE_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MmcIpcReceiveInterruptMaskSpec {
    const RESET_VALUE: u32 = 0;
}
