#[doc = "Register `DOEPTSIZ_CONTROL` reader"]
pub struct R(crate::R<DOEPTSIZ_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPTSIZ_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPTSIZ_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPTSIZ_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPTSIZ_CONTROL` writer"]
pub struct W(crate::W<DOEPTSIZ_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPTSIZ_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DOEPTSIZ_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPTSIZ_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ_CONTROL_SPEC, u32, u32, 19, O>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ_CONTROL_SPEC, u16, u16, 10, O>;
#[doc = "Field `SUPCnt` reader - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
pub type SUPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUPCnt` writer - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
pub type SUPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ_CONTROL_SPEC, u8, u8, 2, O>;
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
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<0> {
        XFER_SIZE_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<19> {
        PKT_CNT_W::new(self)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt(&mut self) -> SUPCNT_W<29> {
        SUPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Transfer Size Register \\[CONT\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz_control](index.html) module"]
pub struct DOEPTSIZ_CONTROL_SPEC;
impl crate::RegisterSpec for DOEPTSIZ_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doeptsiz_control::R](R) reader structure"]
impl crate::Readable for DOEPTSIZ_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doeptsiz_control::W](W) writer structure"]
impl crate::Writable for DOEPTSIZ_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ_CONTROL to value 0"]
impl crate::Resettable for DOEPTSIZ_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
