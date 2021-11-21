#[doc = "Register `DOEPTSIZ_ISO` reader"]
pub struct R(crate::R<DOEPTSIZ_ISO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPTSIZ_ISO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPTSIZ_ISO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPTSIZ_ISO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPTSIZ_ISO` writer"]
pub struct W(crate::W<DOEPTSIZ_ISO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPTSIZ_ISO_SPEC>;
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
impl From<crate::W<DOEPTSIZ_ISO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPTSIZ_ISO_SPEC>) -> Self {
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
#[doc = "Received Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXDPID_A {
    #[doc = "0: DATA0"]
    VALUE1 = 0,
    #[doc = "1: DATA2"]
    VALUE2 = 1,
    #[doc = "2: DATA1"]
    VALUE3 = 2,
    #[doc = "3: MDATA"]
    VALUE4 = 3,
}
impl From<RXDPID_A> for u8 {
    #[inline(always)]
    fn from(variant: RXDPID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RxDPID` reader - Received Data PID"]
pub struct RXDPID_R(crate::FieldReader<u8, RXDPID_A>);
impl RXDPID_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXDPID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDPID_A {
        match self.bits {
            0 => RXDPID_A::VALUE1,
            1 => RXDPID_A::VALUE2,
            2 => RXDPID_A::VALUE3,
            3 => RXDPID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RXDPID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RXDPID_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == RXDPID_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == RXDPID_A::VALUE4
    }
}
impl core::ops::Deref for RXDPID_R {
    type Target = crate::FieldReader<u8, RXDPID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Bits 29:30 - Received Data PID"]
    #[inline(always)]
    pub fn rx_dpid(&self) -> RXDPID_R {
        RXDPID_R::new(((self.bits >> 29) & 0x03) as u8)
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz_iso](index.html) module"]
pub struct DOEPTSIZ_ISO_SPEC;
impl crate::RegisterSpec for DOEPTSIZ_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doeptsiz_iso::R](R) reader structure"]
impl crate::Readable for DOEPTSIZ_ISO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doeptsiz_iso::W](W) writer structure"]
impl crate::Writable for DOEPTSIZ_ISO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPTSIZ_ISO to value 0"]
impl crate::Resettable for DOEPTSIZ_ISO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
