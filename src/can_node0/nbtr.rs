#[doc = "Register `NBTR` reader"]
pub struct R(crate::R<NBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NBTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NBTR` writer"]
pub struct W(crate::W<NBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NBTR_SPEC>;
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
impl From<crate::W<NBTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - Baud Rate Prescaler"]
pub type BRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRP` writer - Baud Rate Prescaler"]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTR_SPEC, u8, u8, 6, O>;
#[doc = "Field `SJW` reader - (Re) Synchronization Jump Width"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - (Re) Synchronization Jump Width"]
pub type SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TSEG1` reader - Time Segment Before Sample Point"]
pub type TSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG1` writer - Time Segment Before Sample Point"]
pub type TSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TSEG2` reader - Time Segment After Sample Point"]
pub type TSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG2` writer - Time Segment After Sample Point"]
pub type TSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DIV8` reader - Divide Prescaler Clock by 8"]
pub type DIV8_R = crate::BitReader<DIV8_A>;
#[doc = "Divide Prescaler Clock by 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIV8_A {
    #[doc = "0: A time quantum lasts (BRP+1) clock cycles."]
    VALUE1 = 0,
    #[doc = "1: A time quantum lasts 8 (BRP+1) clock cycles."]
    VALUE2 = 1,
}
impl From<DIV8_A> for bool {
    #[inline(always)]
    fn from(variant: DIV8_A) -> Self {
        variant as u8 != 0
    }
}
impl DIV8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV8_A {
        match self.bits {
            false => DIV8_A::VALUE1,
            true => DIV8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIV8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV8_A::VALUE2
    }
}
#[doc = "Field `DIV8` writer - Divide Prescaler Clock by 8"]
pub type DIV8_W<'a, const O: u8> = crate::BitWriter<'a, u32, NBTR_SPEC, DIV8_A, O>;
impl<'a, const O: u8> DIV8_W<'a, O> {
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIV8_A::VALUE1)
    }
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIV8_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    pub fn div8(&self) -> DIV8_R {
        DIV8_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<0> {
        BRP_W::new(self)
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<6> {
        SJW_W::new(self)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg1(&mut self) -> TSEG1_W<8> {
        TSEG1_W::new(self)
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<12> {
        TSEG2_W::new(self)
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    #[must_use]
    pub fn div8(&mut self) -> DIV8_W<15> {
        DIV8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbtr](index.html) module"]
pub struct NBTR_SPEC;
impl crate::RegisterSpec for NBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nbtr::R](R) reader structure"]
impl crate::Readable for NBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nbtr::W](W) writer structure"]
impl crate::Writable for NBTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NBTR to value 0"]
impl crate::Resettable for NBTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
