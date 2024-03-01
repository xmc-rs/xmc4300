#[doc = "Register `MMC_CONTROL` reader"]
pub type R = crate::R<MmcControlSpec>;
#[doc = "Register `MMC_CONTROL` writer"]
pub type W = crate::W<MmcControlSpec>;
#[doc = "Field `CNTRST` reader - Counters Reset"]
pub type CntrstR = crate::BitReader;
#[doc = "Field `CNTRST` writer - Counters Reset"]
pub type CntrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTOPRO` reader - Counters Stop Rollover"]
pub type CntstoproR = crate::BitReader;
#[doc = "Field `CNTSTOPRO` writer - Counters Stop Rollover"]
pub type CntstoproW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTONRD` reader - Reset on Read"]
pub type RstonrdR = crate::BitReader;
#[doc = "Field `RSTONRD` writer - Reset on Read"]
pub type RstonrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTFREEZ` reader - MMC Counter Freeze"]
pub type CntfreezR = crate::BitReader;
#[doc = "Field `CNTFREEZ` writer - MMC Counter Freeze"]
pub type CntfreezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTPRST` reader - Counters Preset"]
pub type CntprstR = crate::BitReader;
#[doc = "Field `CNTPRST` writer - Counters Preset"]
pub type CntprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTPRSTLVL` reader - Full-Half Preset"]
pub type CntprstlvlR = crate::BitReader;
#[doc = "Field `CNTPRSTLVL` writer - Full-Half Preset"]
pub type CntprstlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCDBC` reader - Update MMC Counters for Dropped Broadcast Frames"]
pub type UcdbcR = crate::BitReader;
#[doc = "Field `UCDBC` writer - Update MMC Counters for Dropped Broadcast Frames"]
pub type UcdbcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn cntrst(&self) -> CntrstR {
        CntrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    pub fn cntstopro(&self) -> CntstoproR {
        CntstoproR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn rstonrd(&self) -> RstonrdR {
        RstonrdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn cntfreez(&self) -> CntfreezR {
        CntfreezR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&self) -> CntprstR {
        CntprstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full-Half Preset"]
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CntprstlvlR {
        CntprstlvlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn ucdbc(&self) -> UcdbcR {
        UcdbcR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cntrst(&mut self) -> CntrstW<MmcControlSpec> {
        CntrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    #[must_use]
    pub fn cntstopro(&mut self) -> CntstoproW<MmcControlSpec> {
        CntstoproW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    #[must_use]
    pub fn rstonrd(&mut self) -> RstonrdW<MmcControlSpec> {
        RstonrdW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn cntfreez(&mut self) -> CntfreezW<MmcControlSpec> {
        CntfreezW::new(self, 3)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CntprstW<MmcControlSpec> {
        CntprstW::new(self, 4)
    }
    #[doc = "Bit 5 - Full-Half Preset"]
    #[inline(always)]
    #[must_use]
    pub fn cntprstlvl(&mut self) -> CntprstlvlW<MmcControlSpec> {
        CntprstlvlW::new(self, 5)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    #[must_use]
    pub fn ucdbc(&mut self) -> UcdbcW<MmcControlSpec> {
        UcdbcW::new(self, 8)
    }
}
#[doc = "MMC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcControlSpec;
impl crate::RegisterSpec for MmcControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_control::R`](R) reader structure"]
impl crate::Readable for MmcControlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_control::W`](W) writer structure"]
impl crate::Writable for MmcControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_CONTROL to value 0"]
impl crate::Resettable for MmcControlSpec {
    const RESET_VALUE: u32 = 0;
}
