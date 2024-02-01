#[doc = "Register `DOEPTSIZ_CONTROL` reader"]
pub type R = crate::R<DOEPTSIZ_CONTROL_SPEC>;
#[doc = "Register `DOEPTSIZ_CONTROL` writer"]
pub type W = crate::W<DOEPTSIZ_CONTROL_SPEC>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SUPCnt` reader - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
pub type SUPCNT_R = crate::FieldReader;
#[doc = "Field `SUPCnt` writer - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
pub type SUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFER_SIZE_R {
        XFER_SIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKT_CNT_R {
        PKT_CNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<DOEPTSIZ_CONTROL_SPEC> {
        XFER_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<DOEPTSIZ_CONTROL_SPEC> {
        PKT_CNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt(&mut self) -> SUPCNT_W<DOEPTSIZ_CONTROL_SPEC> {
        SUPCNT_W::new(self, 29)
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
#[doc = "Device Endpoint Transfer Size Register \\[CONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ_CONTROL_SPEC;
impl crate::RegisterSpec for DOEPTSIZ_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz_control::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz_control::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ_CONTROL to value 0"]
impl crate::Resettable for DOEPTSIZ_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
