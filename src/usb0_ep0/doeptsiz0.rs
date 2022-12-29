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
pub type XFER_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ0_SPEC, u8, u8, 7, O>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SUPCnt` reader - SETUP Packet Count"]
pub type SUPCNT_R = crate::FieldReader<u8, SUPCNT_A>;
#[doc = "SETUP Packet Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SUPCNT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SUPCNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUPCNT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUPCNT_A::VALUE3
    }
}
#[doc = "Field `SUPCnt` writer - SETUP Packet Count"]
pub type SUPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ0_SPEC, u8, SUPCNT_A, 2, O>;
impl<'a, const O: u8> SUPCNT_W<'a, O> {
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
}
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
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<0> {
        XFER_SIZE_W::new(self)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<19> {
        PKT_CNT_W::new(self)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ0 to value 0"]
impl crate::Resettable for DOEPTSIZ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
