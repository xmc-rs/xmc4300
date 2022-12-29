#[doc = "Register `MSKSR` reader"]
pub struct R(crate::R<MSKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSKSR` writer"]
pub struct W(crate::W<MSKSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSKSR_SPEC>;
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
impl From<crate::W<MSKSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSKSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPSE` reader - Periodic Seconds Interrupt Mask"]
pub type MPSE_R = crate::BitReader<bool>;
#[doc = "Field `MPSE` writer - Periodic Seconds Interrupt Mask"]
pub type MPSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSKSR_SPEC, bool, O>;
#[doc = "Field `MPMI` reader - Periodic Minutes Interrupt Mask"]
pub type MPMI_R = crate::BitReader<bool>;
#[doc = "Field `MPMI` writer - Periodic Minutes Interrupt Mask"]
pub type MPMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSKSR_SPEC, bool, O>;
#[doc = "Field `MPHO` reader - Periodic Hours Interrupt Mask"]
pub type MPHO_R = crate::BitReader<bool>;
#[doc = "Field `MPHO` writer - Periodic Hours Interrupt Mask"]
pub type MPHO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSKSR_SPEC, bool, O>;
#[doc = "Field `MPDA` reader - Periodic Days Interrupt Mask"]
pub type MPDA_R = crate::BitReader<bool>;
#[doc = "Field `MPDA` writer - Periodic Days Interrupt Mask"]
pub type MPDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSKSR_SPEC, bool, O>;
#[doc = "Field `MPMO` reader - Periodic Months Interrupt Mask"]
pub type MPMO_R = crate::BitReader<bool>;
#[doc = "Field `MPMO` writer - Periodic Months Interrupt Mask"]
pub type MPMO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSKSR_SPEC, bool, O>;
#[doc = "Field `MPYE` reader - Periodic Years Interrupt Mask"]
pub type MPYE_R = crate::BitReader<bool>;
#[doc = "Field `MPYE` writer - Periodic Years Interrupt Mask"]
pub type MPYE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSKSR_SPEC, bool, O>;
#[doc = "Field `MAI` reader - Alarm Interrupt Mask"]
pub type MAI_R = crate::BitReader<bool>;
#[doc = "Field `MAI` writer - Alarm Interrupt Mask"]
pub type MAI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSKSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Mask"]
    #[inline(always)]
    pub fn mpse(&self) -> MPSE_R {
        MPSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Mask"]
    #[inline(always)]
    pub fn mpmi(&self) -> MPMI_R {
        MPMI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Mask"]
    #[inline(always)]
    pub fn mpho(&self) -> MPHO_R {
        MPHO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Mask"]
    #[inline(always)]
    pub fn mpda(&self) -> MPDA_R {
        MPDA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Mask"]
    #[inline(always)]
    pub fn mpmo(&self) -> MPMO_R {
        MPMO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Mask"]
    #[inline(always)]
    pub fn mpye(&self) -> MPYE_R {
        MPYE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn mai(&self) -> MAI_R {
        MAI_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpse(&mut self) -> MPSE_W<0> {
        MPSE_W::new(self)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpmi(&mut self) -> MPMI_W<1> {
        MPMI_W::new(self)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpho(&mut self) -> MPHO_W<2> {
        MPHO_W::new(self)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpda(&mut self) -> MPDA_W<3> {
        MPDA_W::new(self)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpmo(&mut self) -> MPMO_W<5> {
        MPMO_W::new(self)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpye(&mut self) -> MPYE_W<6> {
        MPYE_W::new(self)
    }
    #[doc = "Bit 8 - Alarm Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mai(&mut self) -> MAI_W<8> {
        MAI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Service Request Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msksr](index.html) module"]
pub struct MSKSR_SPEC;
impl crate::RegisterSpec for MSKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msksr::R](R) reader structure"]
impl crate::Readable for MSKSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msksr::W](W) writer structure"]
impl crate::Writable for MSKSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSKSR to value 0"]
impl crate::Resettable for MSKSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
