#[doc = "Register `ATIM1` reader"]
pub struct R(crate::R<ATIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATIM1` writer"]
pub struct W(crate::W<ATIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATIM1_SPEC>;
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
impl From<crate::W<ATIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMO` reader - Alarm Month Compare Value"]
pub type AMO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMO` writer - Alarm Month Compare Value"]
pub type AMO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATIM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `AYE` reader - Alarm Year Compare Value"]
pub type AYE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AYE` writer - Alarm Year Compare Value"]
pub type AYE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATIM1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 8:11 - Alarm Month Compare Value"]
    #[inline(always)]
    pub fn amo(&self) -> AMO_R {
        AMO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Alarm Year Compare Value"]
    #[inline(always)]
    pub fn aye(&self) -> AYE_R {
        AYE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:11 - Alarm Month Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn amo(&mut self) -> AMO_W<8> {
        AMO_W::new(self)
    }
    #[doc = "Bits 16:31 - Alarm Year Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn aye(&mut self) -> AYE_W<16> {
        AYE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Alarm Time Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atim1](index.html) module"]
pub struct ATIM1_SPEC;
impl crate::RegisterSpec for ATIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atim1::R](R) reader structure"]
impl crate::Readable for ATIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atim1::W](W) writer structure"]
impl crate::Writable for ATIM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATIM1 to value 0"]
impl crate::Resettable for ATIM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
