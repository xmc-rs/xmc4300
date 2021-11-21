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
pub struct XFERSIZE_R(crate::FieldReader<u32, u32>);
impl XFERSIZE_R {
    pub(crate) fn new(bits: u32) -> Self {
        XFERSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFERSIZE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XferSize` writer - Transfer Size"]
pub struct XFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | (value as u32 & 0x0007_ffff);
        self.w
    }
}
#[doc = "Field `PktCnt` reader - Packet Count"]
pub struct PKTCNT_R(crate::FieldReader<u16, u16>);
impl PKTCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PktCnt` writer - Packet Count"]
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | ((value as u32 & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Field `SUPCnt` reader - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
pub struct SUPCNT_R(crate::FieldReader<u8, u8>);
impl SUPCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUPCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUPCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUPCnt` writer - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
pub struct SUPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&mut self) -> XFERSIZE_W {
        XFERSIZE_W { w: self }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    #[doc = "Bits 29:30 - SETUP Packet Count: 0b00=1 packet, 0b00=2 packets, 0b00=3 packets,"]
    #[inline(always)]
    pub fn supcnt(&mut self) -> SUPCNT_W {
        SUPCNT_W { w: self }
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
}
#[doc = "`reset()` method sets DOEPTSIZ_CONTROL to value 0"]
impl crate::Resettable for DOEPTSIZ_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
