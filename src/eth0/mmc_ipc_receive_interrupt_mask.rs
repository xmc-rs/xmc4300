#[doc = "Register `MMC_IPC_RECEIVE_INTERRUPT_MASK` reader"]
pub type R = crate::R<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>;
#[doc = "Register `MMC_IPC_RECEIVE_INTERRUPT_MASK` writer"]
pub type W = crate::W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>;
#[doc = "Field `RXIPV4GFIM` reader - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
pub type RXIPV4GFIM_R = crate::BitReader;
#[doc = "Field `RXIPV4GFIM` writer - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
pub type RXIPV4GFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4HERFIM` reader - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
pub type RXIPV4HERFIM_R = crate::BitReader;
#[doc = "Field `RXIPV4HERFIM` writer - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
pub type RXIPV4HERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4NOPAYFIM` reader - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
pub type RXIPV4NOPAYFIM_R = crate::BitReader;
#[doc = "Field `RXIPV4NOPAYFIM` writer - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
pub type RXIPV4NOPAYFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4FRAGFIM` reader - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
pub type RXIPV4FRAGFIM_R = crate::BitReader;
#[doc = "Field `RXIPV4FRAGFIM` writer - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
pub type RXIPV4FRAGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4UDSBLFIM` reader - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
pub type RXIPV4UDSBLFIM_R = crate::BitReader;
#[doc = "Field `RXIPV4UDSBLFIM` writer - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
pub type RXIPV4UDSBLFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6GFIM` reader - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
pub type RXIPV6GFIM_R = crate::BitReader;
#[doc = "Field `RXIPV6GFIM` writer - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
pub type RXIPV6GFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6HERFIM` reader - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
pub type RXIPV6HERFIM_R = crate::BitReader;
#[doc = "Field `RXIPV6HERFIM` writer - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
pub type RXIPV6HERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6NOPAYFIM` reader - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
pub type RXIPV6NOPAYFIM_R = crate::BitReader;
#[doc = "Field `RXIPV6NOPAYFIM` writer - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
pub type RXIPV6NOPAYFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUDPGFIM` reader - MMC Receive UDP Good Frame Counter Interrupt Mask"]
pub type RXUDPGFIM_R = crate::BitReader;
#[doc = "Field `RXUDPGFIM` writer - MMC Receive UDP Good Frame Counter Interrupt Mask"]
pub type RXUDPGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUDPERFIM` reader - MMC Receive UDP Error Frame Counter Interrupt Mask"]
pub type RXUDPERFIM_R = crate::BitReader;
#[doc = "Field `RXUDPERFIM` writer - MMC Receive UDP Error Frame Counter Interrupt Mask"]
pub type RXUDPERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTCPGFIM` reader - MMC Receive TCP Good Frame Counter Interrupt Mask"]
pub type RXTCPGFIM_R = crate::BitReader;
#[doc = "Field `RXTCPGFIM` writer - MMC Receive TCP Good Frame Counter Interrupt Mask"]
pub type RXTCPGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTCPERFIM` reader - MMC Receive TCP Error Frame Counter Interrupt Mask"]
pub type RXTCPERFIM_R = crate::BitReader;
#[doc = "Field `RXTCPERFIM` writer - MMC Receive TCP Error Frame Counter Interrupt Mask"]
pub type RXTCPERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICMPGFIM` reader - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
pub type RXICMPGFIM_R = crate::BitReader;
#[doc = "Field `RXICMPGFIM` writer - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
pub type RXICMPGFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICMPERFIM` reader - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
pub type RXICMPERFIM_R = crate::BitReader;
#[doc = "Field `RXICMPERFIM` writer - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
pub type RXICMPERFIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4GOIM` reader - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
pub type RXIPV4GOIM_R = crate::BitReader;
#[doc = "Field `RXIPV4GOIM` writer - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
pub type RXIPV4GOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4HEROIM` reader - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
pub type RXIPV4HEROIM_R = crate::BitReader;
#[doc = "Field `RXIPV4HEROIM` writer - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
pub type RXIPV4HEROIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4NOPAYOIM` reader - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
pub type RXIPV4NOPAYOIM_R = crate::BitReader;
#[doc = "Field `RXIPV4NOPAYOIM` writer - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
pub type RXIPV4NOPAYOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4FRAGOIM` reader - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
pub type RXIPV4FRAGOIM_R = crate::BitReader;
#[doc = "Field `RXIPV4FRAGOIM` writer - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
pub type RXIPV4FRAGOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV4UDSBLOIM` reader - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
pub type RXIPV4UDSBLOIM_R = crate::BitReader;
#[doc = "Field `RXIPV4UDSBLOIM` writer - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
pub type RXIPV4UDSBLOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6GOIM` reader - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
pub type RXIPV6GOIM_R = crate::BitReader;
#[doc = "Field `RXIPV6GOIM` writer - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
pub type RXIPV6GOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6HEROIM` reader - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
pub type RXIPV6HEROIM_R = crate::BitReader;
#[doc = "Field `RXIPV6HEROIM` writer - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
pub type RXIPV6HEROIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPV6NOPAYOIM` reader - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
pub type RXIPV6NOPAYOIM_R = crate::BitReader;
#[doc = "Field `RXIPV6NOPAYOIM` writer - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
pub type RXIPV6NOPAYOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUDPGOIM` reader - MMC Receive UDP Good Octet Counter Interrupt Mask"]
pub type RXUDPGOIM_R = crate::BitReader;
#[doc = "Field `RXUDPGOIM` writer - MMC Receive UDP Good Octet Counter Interrupt Mask"]
pub type RXUDPGOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUDPEROIM` reader - MMC Receive UDP Error Octet Counter Interrupt Mask"]
pub type RXUDPEROIM_R = crate::BitReader;
#[doc = "Field `RXUDPEROIM` writer - MMC Receive UDP Error Octet Counter Interrupt Mask"]
pub type RXUDPEROIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTCPGOIM` reader - MMC Receive TCP Good Octet Counter Interrupt Mask"]
pub type RXTCPGOIM_R = crate::BitReader;
#[doc = "Field `RXTCPGOIM` writer - MMC Receive TCP Good Octet Counter Interrupt Mask"]
pub type RXTCPGOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTCPEROIM` reader - MMC Receive TCP Error Octet Counter Interrupt Mask"]
pub type RXTCPEROIM_R = crate::BitReader;
#[doc = "Field `RXTCPEROIM` writer - MMC Receive TCP Error Octet Counter Interrupt Mask"]
pub type RXTCPEROIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICMPGOIM` reader - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
pub type RXICMPGOIM_R = crate::BitReader;
#[doc = "Field `RXICMPGOIM` writer - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
pub type RXICMPGOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICMPEROIM` reader - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
pub type RXICMPEROIM_R = crate::BitReader;
#[doc = "Field `RXICMPEROIM` writer - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
pub type RXICMPEROIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4gfim(&self) -> RXIPV4GFIM_R {
        RXIPV4GFIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4herfim(&self) -> RXIPV4HERFIM_R {
        RXIPV4HERFIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayfim(&self) -> RXIPV4NOPAYFIM_R {
        RXIPV4NOPAYFIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragfim(&self) -> RXIPV4FRAGFIM_R {
        RXIPV4FRAGFIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsblfim(&self) -> RXIPV4UDSBLFIM_R {
        RXIPV4UDSBLFIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6gfim(&self) -> RXIPV6GFIM_R {
        RXIPV6GFIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6herfim(&self) -> RXIPV6HERFIM_R {
        RXIPV6HERFIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayfim(&self) -> RXIPV6NOPAYFIM_R {
        RXIPV6NOPAYFIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgfim(&self) -> RXUDPGFIM_R {
        RXUDPGFIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperfim(&self) -> RXUDPERFIM_R {
        RXUDPERFIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgfim(&self) -> RXTCPGFIM_R {
        RXTCPGFIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperfim(&self) -> RXTCPERFIM_R {
        RXTCPERFIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgfim(&self) -> RXICMPGFIM_R {
        RXICMPGFIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperfim(&self) -> RXICMPERFIM_R {
        RXICMPERFIM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4goim(&self) -> RXIPV4GOIM_R {
        RXIPV4GOIM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4heroim(&self) -> RXIPV4HEROIM_R {
        RXIPV4HEROIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayoim(&self) -> RXIPV4NOPAYOIM_R {
        RXIPV4NOPAYOIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragoim(&self) -> RXIPV4FRAGOIM_R {
        RXIPV4FRAGOIM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsbloim(&self) -> RXIPV4UDSBLOIM_R {
        RXIPV4UDSBLOIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6goim(&self) -> RXIPV6GOIM_R {
        RXIPV6GOIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6heroim(&self) -> RXIPV6HEROIM_R {
        RXIPV6HEROIM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayoim(&self) -> RXIPV6NOPAYOIM_R {
        RXIPV6NOPAYOIM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgoim(&self) -> RXUDPGOIM_R {
        RXUDPGOIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperoim(&self) -> RXUDPEROIM_R {
        RXUDPEROIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgoim(&self) -> RXTCPGOIM_R {
        RXTCPGOIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperoim(&self) -> RXTCPEROIM_R {
        RXTCPEROIM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgoim(&self) -> RXICMPGOIM_R {
        RXICMPGOIM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperoim(&self) -> RXICMPEROIM_R {
        RXICMPEROIM_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4gfim(&mut self) -> RXIPV4GFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4GFIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4herfim(&mut self) -> RXIPV4HERFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4HERFIM_W::new(self, 1)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4nopayfim(&mut self) -> RXIPV4NOPAYFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4NOPAYFIM_W::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4fragfim(&mut self) -> RXIPV4FRAGFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4FRAGFIM_W::new(self, 3)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4udsblfim(&mut self) -> RXIPV4UDSBLFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4UDSBLFIM_W::new(self, 4)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6gfim(&mut self) -> RXIPV6GFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV6GFIM_W::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6herfim(&mut self) -> RXIPV6HERFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV6HERFIM_W::new(self, 6)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6nopayfim(&mut self) -> RXIPV6NOPAYFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV6NOPAYFIM_W::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxudpgfim(&mut self) -> RXUDPGFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXUDPGFIM_W::new(self, 8)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxudperfim(&mut self) -> RXUDPERFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXUDPERFIM_W::new(self, 9)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxtcpgfim(&mut self) -> RXTCPGFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXTCPGFIM_W::new(self, 10)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxtcperfim(&mut self) -> RXTCPERFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXTCPERFIM_W::new(self, 11)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxicmpgfim(&mut self) -> RXICMPGFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXICMPGFIM_W::new(self, 12)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxicmperfim(&mut self) -> RXICMPERFIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXICMPERFIM_W::new(self, 13)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4goim(&mut self) -> RXIPV4GOIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4GOIM_W::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4heroim(&mut self) -> RXIPV4HEROIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4HEROIM_W::new(self, 17)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4nopayoim(&mut self) -> RXIPV4NOPAYOIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4NOPAYOIM_W::new(self, 18)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4fragoim(&mut self) -> RXIPV4FRAGOIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4FRAGOIM_W::new(self, 19)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4udsbloim(&mut self) -> RXIPV4UDSBLOIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV4UDSBLOIM_W::new(self, 20)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6goim(&mut self) -> RXIPV6GOIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV6GOIM_W::new(self, 21)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6heroim(&mut self) -> RXIPV6HEROIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV6HEROIM_W::new(self, 22)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6nopayoim(&mut self) -> RXIPV6NOPAYOIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXIPV6NOPAYOIM_W::new(self, 23)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxudpgoim(&mut self) -> RXUDPGOIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXUDPGOIM_W::new(self, 24)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxudperoim(&mut self) -> RXUDPEROIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXUDPEROIM_W::new(self, 25)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxtcpgoim(&mut self) -> RXTCPGOIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXTCPGOIM_W::new(self, 26)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxtcperoim(&mut self) -> RXTCPEROIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXTCPEROIM_W::new(self, 27)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxicmpgoim(&mut self) -> RXICMPGOIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXICMPGOIM_W::new(self, 28)
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxicmperoim(&mut self) -> RXICMPEROIM_W<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC> {
        RXICMPEROIM_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_receive_interrupt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_ipc_receive_interrupt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_ipc_receive_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmc_ipc_receive_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_IPC_RECEIVE_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
