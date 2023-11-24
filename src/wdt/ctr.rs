#[doc = "Register `CTR` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `ENB` reader - Enable"]
pub type ENB_R = crate::BitReader;
#[doc = "Field `ENB` writer - Enable"]
pub type ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE` reader - Pre-warning"]
pub type PRE_R = crate::BitReader;
#[doc = "Field `PRE` writer - Pre-warning"]
pub type PRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP` reader - Debug Suspend"]
pub type DSP_R = crate::BitReader;
#[doc = "Field `DSP` writer - Debug Suspend"]
pub type DSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPW` reader - Service Indication Pulse Width"]
pub type SPW_R = crate::FieldReader;
#[doc = "Field `SPW` writer - Service Indication Pulse Width"]
pub type SPW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pre-warning"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Suspend"]
    #[inline(always)]
    pub fn dsp(&self) -> DSP_R {
        DSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Service Indication Pulse Width"]
    #[inline(always)]
    pub fn spw(&self) -> SPW_R {
        SPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<CTR_SPEC> {
        ENB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pre-warning"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<CTR_SPEC> {
        PRE_W::new(self, 1)
    }
    #[doc = "Bit 4 - Debug Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn dsp(&mut self) -> DSP_W<CTR_SPEC> {
        DSP_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Service Indication Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn spw(&mut self) -> SPW_W<CTR_SPEC> {
        SPW_W::new(self, 8)
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
#[doc = "WDT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
