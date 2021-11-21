#[doc = "Register `DOEPTSIZ0` reader"]
pub struct R(crate::R<DOEPTSIZ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPTSIZ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPTSIZ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPTSIZ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPTSIZ0` writer"]
pub struct W(crate::W<DOEPTSIZ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPTSIZ0_SPEC>;
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
impl From<crate::W<DOEPTSIZ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPTSIZ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferSize` reader - Transfer Size"]
pub struct XFERSIZE_R(crate::FieldReader<u8, u8>);
impl XFERSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        XFERSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFERSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `PktCnt` reader - Packet Count"]
pub struct PKTCNT_R(crate::FieldReader<u8, u8>);
impl PKTCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTCNT_R {
    type Target = crate::FieldReader<u8, u8>;
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
#[doc = "SETUP Packet Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUPCNT_A {
    #[doc = "1: 1 packet"]
    VALUE1 = 1,
    #[doc = "2: 2 packets"]
    VALUE2 = 2,
    #[doc = "3: 3 packets"]
    VALUE3 = 3,
}
impl From<SUPCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SUPCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SUPCnt` reader - SETUP Packet Count"]
pub struct SUPCNT_R(crate::FieldReader<u8, SUPCNT_A>);
impl SUPCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUPCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUPCNT_A> {
        match self.bits {
            1 => Some(SUPCNT_A::VALUE1),
            2 => Some(SUPCNT_A::VALUE2),
            3 => Some(SUPCNT_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SUPCNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SUPCNT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SUPCNT_A::VALUE3
    }
}
impl core::ops::Deref for SUPCNT_R {
    type Target = crate::FieldReader<u8, SUPCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUPCnt` writer - SETUP Packet Count"]
pub struct SUPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUPCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 packet"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUPCNT_A::VALUE1)
    }
    #[doc = "2 packets"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUPCNT_A::VALUE2)
    }
    #[doc = "3 packets"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUPCNT_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&mut self) -> XFERSIZE_W {
        XFERSIZE_W { w: self }
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
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
#[doc = "Device OUT Endpoint Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz0](index.html) module"]
pub struct DOEPTSIZ0_SPEC;
impl crate::RegisterSpec for DOEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doeptsiz0::R](R) reader structure"]
impl crate::Readable for DOEPTSIZ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doeptsiz0::W](W) writer structure"]
impl crate::Writable for DOEPTSIZ0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPTSIZ0 to value 0"]
impl crate::Resettable for DOEPTSIZ0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
