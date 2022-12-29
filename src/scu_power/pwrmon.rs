#[doc = "Register `PWRMON` reader"]
pub struct R(crate::R<PWRMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRMON` writer"]
pub struct W(crate::W<PWRMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRMON_SPEC>;
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
impl From<crate::W<PWRMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRS` reader - Threshold"]
pub type THRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRS` writer - Threshold"]
pub type THRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWRMON_SPEC, u8, u8, 8, O>;
#[doc = "Field `INTV` reader - Interval"]
pub type INTV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTV` writer - Interval"]
pub type INTV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWRMON_SPEC, u8, u8, 8, O>;
#[doc = "Field `ENB` reader - Enable"]
pub type ENB_R = crate::BitReader<bool>;
#[doc = "Field `ENB` writer - Enable"]
pub type ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRMON_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    pub fn thrs(&self) -> THRS_R {
        THRS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    pub fn intv(&self) -> INTV_R {
        INTV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thrs(&mut self) -> THRS_W<0> {
        THRS_W::new(self)
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    #[must_use]
    pub fn intv(&mut self) -> INTV_W<8> {
        INTV_W::new(self)
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<16> {
        ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Monitor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrmon](index.html) module"]
pub struct PWRMON_SPEC;
impl crate::RegisterSpec for PWRMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrmon::R](R) reader structure"]
impl crate::Readable for PWRMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrmon::W](W) writer structure"]
impl crate::Writable for PWRMON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRMON to value 0"]
impl crate::Resettable for PWRMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
