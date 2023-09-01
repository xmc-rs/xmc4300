#[doc = "Register `DIEPTSIZ0` reader"]
pub type R = crate::R<DIEPTSIZ0_SPEC>;
#[doc = "Register `DIEPTSIZ0` writer"]
pub type W = crate::W<DIEPTSIZ0_SPEC>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFER_SIZE_R {
        XFER_SIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKT_CNT_R {
        PKT_CNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<DIEPTSIZ0_SPEC, 0> {
        XFER_SIZE_W::new(self)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<DIEPTSIZ0_SPEC, 19> {
        PKT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device IN Endpoint Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ0_SPEC;
impl crate::RegisterSpec for DIEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz0::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz0::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ0 to value 0"]
impl crate::Resettable for DIEPTSIZ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
