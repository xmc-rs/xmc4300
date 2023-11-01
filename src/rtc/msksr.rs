#[doc = "Register `MSKSR` reader"]
pub type R = crate::R<MSKSR_SPEC>;
#[doc = "Register `MSKSR` writer"]
pub type W = crate::W<MSKSR_SPEC>;
#[doc = "Field `MPSE` reader - Periodic Seconds Interrupt Mask"]
pub type MPSE_R = crate::BitReader;
#[doc = "Field `MPSE` writer - Periodic Seconds Interrupt Mask"]
pub type MPSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPMI` reader - Periodic Minutes Interrupt Mask"]
pub type MPMI_R = crate::BitReader;
#[doc = "Field `MPMI` writer - Periodic Minutes Interrupt Mask"]
pub type MPMI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPHO` reader - Periodic Hours Interrupt Mask"]
pub type MPHO_R = crate::BitReader;
#[doc = "Field `MPHO` writer - Periodic Hours Interrupt Mask"]
pub type MPHO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPDA` reader - Periodic Days Interrupt Mask"]
pub type MPDA_R = crate::BitReader;
#[doc = "Field `MPDA` writer - Periodic Days Interrupt Mask"]
pub type MPDA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPMO` reader - Periodic Months Interrupt Mask"]
pub type MPMO_R = crate::BitReader;
#[doc = "Field `MPMO` writer - Periodic Months Interrupt Mask"]
pub type MPMO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPYE` reader - Periodic Years Interrupt Mask"]
pub type MPYE_R = crate::BitReader;
#[doc = "Field `MPYE` writer - Periodic Years Interrupt Mask"]
pub type MPYE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAI` reader - Alarm Interrupt Mask"]
pub type MAI_R = crate::BitReader;
#[doc = "Field `MAI` writer - Alarm Interrupt Mask"]
pub type MAI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn mpse(&mut self) -> MPSE_W<MSKSR_SPEC, 0> {
        MPSE_W::new(self)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpmi(&mut self) -> MPMI_W<MSKSR_SPEC, 1> {
        MPMI_W::new(self)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpho(&mut self) -> MPHO_W<MSKSR_SPEC, 2> {
        MPHO_W::new(self)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpda(&mut self) -> MPDA_W<MSKSR_SPEC, 3> {
        MPDA_W::new(self)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpmo(&mut self) -> MPMO_W<MSKSR_SPEC, 5> {
        MPMO_W::new(self)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpye(&mut self) -> MPYE_W<MSKSR_SPEC, 6> {
        MPYE_W::new(self)
    }
    #[doc = "Bit 8 - Alarm Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mai(&mut self) -> MAI_W<MSKSR_SPEC, 8> {
        MAI_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC Service Request Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msksr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msksr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSKSR_SPEC;
impl crate::RegisterSpec for MSKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msksr::R`](R) reader structure"]
impl crate::Readable for MSKSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msksr::W`](W) writer structure"]
impl crate::Writable for MSKSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSKSR to value 0"]
impl crate::Resettable for MSKSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
