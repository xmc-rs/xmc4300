#[doc = "Register `CTR` reader"]
pub type R = crate::R<CtrSpec>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CtrSpec>;
#[doc = "Field `ENB` reader - Enable"]
pub type EnbR = crate::BitReader;
#[doc = "Field `ENB` writer - Enable"]
pub type EnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE` reader - Pre-warning"]
pub type PreR = crate::BitReader;
#[doc = "Field `PRE` writer - Pre-warning"]
pub type PreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP` reader - Debug Suspend"]
pub type DspR = crate::BitReader;
#[doc = "Field `DSP` writer - Debug Suspend"]
pub type DspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPW` reader - Service Indication Pulse Width"]
pub type SpwR = crate::FieldReader;
#[doc = "Field `SPW` writer - Service Indication Pulse Width"]
pub type SpwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pre-warning"]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Suspend"]
    #[inline(always)]
    pub fn dsp(&self) -> DspR {
        DspR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Service Indication Pulse Width"]
    #[inline(always)]
    pub fn spw(&self) -> SpwR {
        SpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> EnbW<CtrSpec> {
        EnbW::new(self, 0)
    }
    #[doc = "Bit 1 - Pre-warning"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PreW<CtrSpec> {
        PreW::new(self, 1)
    }
    #[doc = "Bit 4 - Debug Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn dsp(&mut self) -> DspW<CtrSpec> {
        DspW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Service Indication Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn spw(&mut self) -> SpwW<CtrSpec> {
        SpwW::new(self, 8)
    }
}
#[doc = "WDT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrSpec;
impl crate::RegisterSpec for CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CtrSpec {
    const RESET_VALUE: u32 = 0;
}
