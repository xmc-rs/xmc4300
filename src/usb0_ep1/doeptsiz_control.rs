#[doc = "Register `DOEPTSIZ_CONTROL` reader"]
pub type R = crate::R<DoeptsizControlSpec>;
#[doc = "Register `DOEPTSIZ_CONTROL` writer"]
pub type W = crate::W<DoeptsizControlSpec>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XferSizeR = crate::FieldReader<u32>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PktCntR = crate::FieldReader<u16>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PktCntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SUPCnt` reader - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
pub type SupcntR = crate::FieldReader;
#[doc = "Field `SUPCnt` writer - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
pub type SupcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XferSizeR {
        XferSizeR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PktCntR {
        PktCntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
    #[inline(always)]
    pub fn supcnt(&self) -> SupcntR {
        SupcntR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XferSizeW<DoeptsizControlSpec> {
        XferSizeW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PktCntW<DoeptsizControlSpec> {
        PktCntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt(&mut self) -> SupcntW<DoeptsizControlSpec> {
        SupcntW::new(self, 29)
    }
}
#[doc = "Device Endpoint Transfer Size Register \\[CONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoeptsizControlSpec;
impl crate::RegisterSpec for DoeptsizControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz_control::R`](R) reader structure"]
impl crate::Readable for DoeptsizControlSpec {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz_control::W`](W) writer structure"]
impl crate::Writable for DoeptsizControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ_CONTROL to value 0"]
impl crate::Resettable for DoeptsizControlSpec {
    const RESET_VALUE: u32 = 0;
}
