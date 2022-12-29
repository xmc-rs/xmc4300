#[doc = "Register `MOFGPR` reader"]
pub struct R(crate::R<MOFGPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOFGPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOFGPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOFGPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOFGPR` writer"]
pub struct W(crate::W<MOFGPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOFGPR_SPEC>;
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
impl From<crate::W<MOFGPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOFGPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOT` reader - Bottom Pointer"]
pub type BOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOT` writer - Bottom Pointer"]
pub type BOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOFGPR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TOP` reader - Top Pointer"]
pub type TOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOP` writer - Top Pointer"]
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOFGPR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CUR` reader - Current Object Pointer"]
pub type CUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CUR` writer - Current Object Pointer"]
pub type CUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOFGPR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SEL` reader - Object Select Pointer"]
pub type SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL` writer - Object Select Pointer"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOFGPR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    pub fn bot(&self) -> BOT_R {
        BOT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    pub fn cur(&self) -> CUR_R {
        CUR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bot(&mut self) -> BOT_W<0> {
        BOT_W::new(self)
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<8> {
        TOP_W::new(self)
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn cur(&mut self) -> CUR_W<16> {
        CUR_W::new(self)
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<24> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object FIFO/Gateway Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mofgpr](index.html) module"]
pub struct MOFGPR_SPEC;
impl crate::RegisterSpec for MOFGPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mofgpr::R](R) reader structure"]
impl crate::Readable for MOFGPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mofgpr::W](W) writer structure"]
impl crate::Writable for MOFGPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOFGPR to value 0"]
impl crate::Resettable for MOFGPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
