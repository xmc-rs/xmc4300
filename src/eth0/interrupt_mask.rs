#[doc = "Register `INTERRUPT_MASK` reader"]
pub struct R(crate::R<INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT_MASK` writer"]
pub struct W(crate::W<INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMTIM` reader - PMT Interrupt Mask"]
pub type PMTIM_R = crate::BitReader<bool>;
#[doc = "Field `PMTIM` writer - PMT Interrupt Mask"]
pub type PMTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_MASK_SPEC, bool, O>;
#[doc = "Field `TSIM` reader - Timestamp Interrupt Mask"]
pub type TSIM_R = crate::BitReader<bool>;
#[doc = "Field `TSIM` writer - Timestamp Interrupt Mask"]
pub type TSIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&self) -> TSIM_R {
        TSIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pmtim(&mut self) -> PMTIM_W<3> {
        PMTIM_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn tsim(&mut self) -> TSIM_W<9> {
        TSIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_mask](index.html) module"]
pub struct INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_mask::R](R) reader structure"]
impl crate::Readable for INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_mask::W](W) writer structure"]
impl crate::Writable for INTERRUPT_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRUPT_MASK to value 0"]
impl crate::Resettable for INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
