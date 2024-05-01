#[doc = "Register `DIEPTSIZ0` reader"]
pub type R = crate::R<Dieptsiz0Spec>;
#[doc = "Register `DIEPTSIZ0` writer"]
pub type W = crate::W<Dieptsiz0Spec>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XferSizeR = crate::FieldReader;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PktCntR = crate::FieldReader;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PktCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XferSizeR {
        XferSizeR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PktCntR {
        PktCntR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XferSizeW<Dieptsiz0Spec> {
        XferSizeW::new(self, 0)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PktCntW<Dieptsiz0Spec> {
        PktCntW::new(self, 19)
    }
}
#[doc = "Device IN Endpoint Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptsiz0Spec;
impl crate::RegisterSpec for Dieptsiz0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz0::R`](R) reader structure"]
impl crate::Readable for Dieptsiz0Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz0::W`](W) writer structure"]
impl crate::Writable for Dieptsiz0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ0 to value 0"]
impl crate::Resettable for Dieptsiz0Spec {
    const RESET_VALUE: u32 = 0;
}
