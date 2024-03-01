#[doc = "Register `MSKSR` reader"]
pub type R = crate::R<MsksrSpec>;
#[doc = "Register `MSKSR` writer"]
pub type W = crate::W<MsksrSpec>;
#[doc = "Field `MPSE` reader - Periodic Seconds Interrupt Mask"]
pub type MpseR = crate::BitReader;
#[doc = "Field `MPSE` writer - Periodic Seconds Interrupt Mask"]
pub type MpseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPMI` reader - Periodic Minutes Interrupt Mask"]
pub type MpmiR = crate::BitReader;
#[doc = "Field `MPMI` writer - Periodic Minutes Interrupt Mask"]
pub type MpmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPHO` reader - Periodic Hours Interrupt Mask"]
pub type MphoR = crate::BitReader;
#[doc = "Field `MPHO` writer - Periodic Hours Interrupt Mask"]
pub type MphoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPDA` reader - Periodic Days Interrupt Mask"]
pub type MpdaR = crate::BitReader;
#[doc = "Field `MPDA` writer - Periodic Days Interrupt Mask"]
pub type MpdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPMO` reader - Periodic Months Interrupt Mask"]
pub type MpmoR = crate::BitReader;
#[doc = "Field `MPMO` writer - Periodic Months Interrupt Mask"]
pub type MpmoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPYE` reader - Periodic Years Interrupt Mask"]
pub type MpyeR = crate::BitReader;
#[doc = "Field `MPYE` writer - Periodic Years Interrupt Mask"]
pub type MpyeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAI` reader - Alarm Interrupt Mask"]
pub type MaiR = crate::BitReader;
#[doc = "Field `MAI` writer - Alarm Interrupt Mask"]
pub type MaiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Mask"]
    #[inline(always)]
    pub fn mpse(&self) -> MpseR {
        MpseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Mask"]
    #[inline(always)]
    pub fn mpmi(&self) -> MpmiR {
        MpmiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Mask"]
    #[inline(always)]
    pub fn mpho(&self) -> MphoR {
        MphoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Mask"]
    #[inline(always)]
    pub fn mpda(&self) -> MpdaR {
        MpdaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Mask"]
    #[inline(always)]
    pub fn mpmo(&self) -> MpmoR {
        MpmoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Mask"]
    #[inline(always)]
    pub fn mpye(&self) -> MpyeR {
        MpyeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn mai(&self) -> MaiR {
        MaiR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Seconds Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpse(&mut self) -> MpseW<MsksrSpec> {
        MpseW::new(self, 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpmi(&mut self) -> MpmiW<MsksrSpec> {
        MpmiW::new(self, 1)
    }
    #[doc = "Bit 2 - Periodic Hours Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpho(&mut self) -> MphoW<MsksrSpec> {
        MphoW::new(self, 2)
    }
    #[doc = "Bit 3 - Periodic Days Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpda(&mut self) -> MpdaW<MsksrSpec> {
        MpdaW::new(self, 3)
    }
    #[doc = "Bit 5 - Periodic Months Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpmo(&mut self) -> MpmoW<MsksrSpec> {
        MpmoW::new(self, 5)
    }
    #[doc = "Bit 6 - Periodic Years Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mpye(&mut self) -> MpyeW<MsksrSpec> {
        MpyeW::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mai(&mut self) -> MaiW<MsksrSpec> {
        MaiW::new(self, 8)
    }
}
#[doc = "RTC Service Request Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msksr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msksr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsksrSpec;
impl crate::RegisterSpec for MsksrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msksr::R`](R) reader structure"]
impl crate::Readable for MsksrSpec {}
#[doc = "`write(|w| ..)` method takes [`msksr::W`](W) writer structure"]
impl crate::Writable for MsksrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSKSR to value 0"]
impl crate::Resettable for MsksrSpec {
    const RESET_VALUE: u32 = 0;
}
