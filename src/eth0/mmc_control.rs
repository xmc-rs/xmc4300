#[doc = "Register `MMC_CONTROL` reader"]
pub type R = crate::R<MMC_CONTROL_SPEC>;
#[doc = "Register `MMC_CONTROL` writer"]
pub type W = crate::W<MMC_CONTROL_SPEC>;
#[doc = "Field `CNTRST` reader - Counters Reset"]
pub type CNTRST_R = crate::BitReader;
#[doc = "Field `CNTRST` writer - Counters Reset"]
pub type CNTRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTSTOPRO` reader - Counters Stop Rollover"]
pub type CNTSTOPRO_R = crate::BitReader;
#[doc = "Field `CNTSTOPRO` writer - Counters Stop Rollover"]
pub type CNTSTOPRO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTONRD` reader - Reset on Read"]
pub type RSTONRD_R = crate::BitReader;
#[doc = "Field `RSTONRD` writer - Reset on Read"]
pub type RSTONRD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTFREEZ` reader - MMC Counter Freeze"]
pub type CNTFREEZ_R = crate::BitReader;
#[doc = "Field `CNTFREEZ` writer - MMC Counter Freeze"]
pub type CNTFREEZ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTPRST` reader - Counters Preset"]
pub type CNTPRST_R = crate::BitReader;
#[doc = "Field `CNTPRST` writer - Counters Preset"]
pub type CNTPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTPRSTLVL` reader - Full-Half Preset"]
pub type CNTPRSTLVL_R = crate::BitReader;
#[doc = "Field `CNTPRSTLVL` writer - Full-Half Preset"]
pub type CNTPRSTLVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UCDBC` reader - Update MMC Counters for Dropped Broadcast Frames"]
pub type UCDBC_R = crate::BitReader;
#[doc = "Field `UCDBC` writer - Update MMC Counters for Dropped Broadcast Frames"]
pub type UCDBC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn cntrst(&self) -> CNTRST_R {
        CNTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    pub fn cntstopro(&self) -> CNTSTOPRO_R {
        CNTSTOPRO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn rstonrd(&self) -> RSTONRD_R {
        RSTONRD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn cntfreez(&self) -> CNTFREEZ_R {
        CNTFREEZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full-Half Preset"]
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CNTPRSTLVL_R {
        CNTPRSTLVL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn ucdbc(&self) -> UCDBC_R {
        UCDBC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cntrst(&mut self) -> CNTRST_W<MMC_CONTROL_SPEC, 0> {
        CNTRST_W::new(self)
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    #[must_use]
    pub fn cntstopro(&mut self) -> CNTSTOPRO_W<MMC_CONTROL_SPEC, 1> {
        CNTSTOPRO_W::new(self)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    #[must_use]
    pub fn rstonrd(&mut self) -> RSTONRD_W<MMC_CONTROL_SPEC, 2> {
        RSTONRD_W::new(self)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn cntfreez(&mut self) -> CNTFREEZ_W<MMC_CONTROL_SPEC, 3> {
        CNTFREEZ_W::new(self)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CNTPRST_W<MMC_CONTROL_SPEC, 4> {
        CNTPRST_W::new(self)
    }
    #[doc = "Bit 5 - Full-Half Preset"]
    #[inline(always)]
    #[must_use]
    pub fn cntprstlvl(&mut self) -> CNTPRSTLVL_W<MMC_CONTROL_SPEC, 5> {
        CNTPRSTLVL_W::new(self)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    #[must_use]
    pub fn ucdbc(&mut self) -> UCDBC_W<MMC_CONTROL_SPEC, 8> {
        UCDBC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MMC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_CONTROL_SPEC;
impl crate::RegisterSpec for MMC_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_control::R`](R) reader structure"]
impl crate::Readable for MMC_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmc_control::W`](W) writer structure"]
impl crate::Writable for MMC_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMC_CONTROL to value 0"]
impl crate::Resettable for MMC_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
