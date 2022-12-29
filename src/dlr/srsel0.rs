#[doc = "Register `SRSEL0` reader"]
pub struct R(crate::R<SRSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSEL0` writer"]
pub struct W(crate::W<SRSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSEL0_SPEC>;
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
impl From<crate::W<SRSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS0` reader - Request Source for Line 0"]
pub type RS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS0` writer - Request Source for Line 0"]
pub type RS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS1` reader - Request Source for Line 1"]
pub type RS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS1` writer - Request Source for Line 1"]
pub type RS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS2` reader - Request Source for Line 2"]
pub type RS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS2` writer - Request Source for Line 2"]
pub type RS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS3` reader - Request Source for Line 3"]
pub type RS3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS3` writer - Request Source for Line 3"]
pub type RS3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS4` reader - Request Source for Line 4"]
pub type RS4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS4` writer - Request Source for Line 4"]
pub type RS4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS5` reader - Request Source for Line 5"]
pub type RS5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS5` writer - Request Source for Line 5"]
pub type RS5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS6` reader - Request Source for Line 6"]
pub type RS6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS6` writer - Request Source for Line 6"]
pub type RS6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS7` reader - Request Source for Line 7"]
pub type RS7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS7` writer - Request Source for Line 7"]
pub type RS7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Request Source for Line 0"]
    #[inline(always)]
    pub fn rs0(&self) -> RS0_R {
        RS0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Request Source for Line 1"]
    #[inline(always)]
    pub fn rs1(&self) -> RS1_R {
        RS1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Request Source for Line 2"]
    #[inline(always)]
    pub fn rs2(&self) -> RS2_R {
        RS2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 3"]
    #[inline(always)]
    pub fn rs3(&self) -> RS3_R {
        RS3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Request Source for Line 4"]
    #[inline(always)]
    pub fn rs4(&self) -> RS4_R {
        RS4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Request Source for Line 5"]
    #[inline(always)]
    pub fn rs5(&self) -> RS5_R {
        RS5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Request Source for Line 6"]
    #[inline(always)]
    pub fn rs6(&self) -> RS6_R {
        RS6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Request Source for Line 7"]
    #[inline(always)]
    pub fn rs7(&self) -> RS7_R {
        RS7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Request Source for Line 0"]
    #[inline(always)]
    #[must_use]
    pub fn rs0(&mut self) -> RS0_W<0> {
        RS0_W::new(self)
    }
    #[doc = "Bits 4:7 - Request Source for Line 1"]
    #[inline(always)]
    #[must_use]
    pub fn rs1(&mut self) -> RS1_W<4> {
        RS1_W::new(self)
    }
    #[doc = "Bits 8:11 - Request Source for Line 2"]
    #[inline(always)]
    #[must_use]
    pub fn rs2(&mut self) -> RS2_W<8> {
        RS2_W::new(self)
    }
    #[doc = "Bits 12:15 - Request Source for Line 3"]
    #[inline(always)]
    #[must_use]
    pub fn rs3(&mut self) -> RS3_W<12> {
        RS3_W::new(self)
    }
    #[doc = "Bits 16:19 - Request Source for Line 4"]
    #[inline(always)]
    #[must_use]
    pub fn rs4(&mut self) -> RS4_W<16> {
        RS4_W::new(self)
    }
    #[doc = "Bits 20:23 - Request Source for Line 5"]
    #[inline(always)]
    #[must_use]
    pub fn rs5(&mut self) -> RS5_W<20> {
        RS5_W::new(self)
    }
    #[doc = "Bits 24:27 - Request Source for Line 6"]
    #[inline(always)]
    #[must_use]
    pub fn rs6(&mut self) -> RS6_W<24> {
        RS6_W::new(self)
    }
    #[doc = "Bits 28:31 - Request Source for Line 7"]
    #[inline(always)]
    #[must_use]
    pub fn rs7(&mut self) -> RS7_W<28> {
        RS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service Request Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsel0](index.html) module"]
pub struct SRSEL0_SPEC;
impl crate::RegisterSpec for SRSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srsel0::R](R) reader structure"]
impl crate::Readable for SRSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srsel0::W](W) writer structure"]
impl crate::Writable for SRSEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSEL0 to value 0"]
impl crate::Resettable for SRSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
