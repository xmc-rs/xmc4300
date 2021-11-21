#[doc = "Register `NVIC_IABR3` reader"]
pub struct R(crate::R<NVIC_IABR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IABR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IABR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IABR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IABR3` writer"]
pub struct W(crate::W<NVIC_IABR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IABR3_SPEC>;
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
impl From<crate::W<NVIC_IABR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IABR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt active flags:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum ACTIVE_A {
    #[doc = "0: interrupt not active"]
    VALUE1 = 0,
    #[doc = "1: interrupt active"]
    VALUE2 = 1,
}
impl From<ACTIVE_A> for u32 {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACTIVE` reader - Interrupt active flags:"]
pub struct ACTIVE_R(crate::FieldReader<u32, ACTIVE_A>);
impl ACTIVE_R {
    pub(crate) fn new(bits: u32) -> Self {
        ACTIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACTIVE_A> {
        match self.bits {
            0 => Some(ACTIVE_A::VALUE1),
            1 => Some(ACTIVE_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ACTIVE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ACTIVE_A::VALUE2
    }
}
impl core::ops::Deref for ACTIVE_R {
    type Target = crate::FieldReader<u32, ACTIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE` writer - Interrupt active flags:"]
pub struct ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTIVE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "interrupt not active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACTIVE_A::VALUE1)
    }
    #[doc = "interrupt active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACTIVE_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt active flags:"]
    #[inline(always)]
    pub fn active(&mut self) -> ACTIVE_W {
        ACTIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Active Bit Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iabr3](index.html) module"]
pub struct NVIC_IABR3_SPEC;
impl crate::RegisterSpec for NVIC_IABR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_iabr3::R](R) reader structure"]
impl crate::Readable for NVIC_IABR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_iabr3::W](W) writer structure"]
impl crate::Writable for NVIC_IABR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IABR3 to value 0"]
impl crate::Resettable for NVIC_IABR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
