#[doc = "Register `OMR` writer"]
pub type W = crate::W<OMR_SPEC>;
#[doc = "Field `PS0` writer - Port n Set Bit 0"]
pub type PS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS1` writer - Port n Set Bit 1"]
pub type PS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2` writer - Port n Set Bit 2"]
pub type PS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS3` writer - Port n Set Bit 3"]
pub type PS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS4` writer - Port n Set Bit 4"]
pub type PS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS5` writer - Port n Set Bit 5"]
pub type PS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS6` writer - Port n Set Bit 6"]
pub type PS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS7` writer - Port n Set Bit 7"]
pub type PS7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS8` writer - Port n Set Bit 8"]
pub type PS8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS9` writer - Port n Set Bit 9"]
pub type PS9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS10` writer - Port n Set Bit 10"]
pub type PS10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS11` writer - Port n Set Bit 11"]
pub type PS11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS12` writer - Port n Set Bit 12"]
pub type PS12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS13` writer - Port n Set Bit 13"]
pub type PS13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS14` writer - Port n Set Bit 14"]
pub type PS14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS15` writer - Port n Set Bit 15"]
pub type PS15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR0` writer - Port n Reset Bit 0"]
pub type PR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1` writer - Port n Reset Bit 1"]
pub type PR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR2` writer - Port n Reset Bit 2"]
pub type PR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR3` writer - Port n Reset Bit 3"]
pub type PR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR4` writer - Port n Reset Bit 4"]
pub type PR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR5` writer - Port n Reset Bit 5"]
pub type PR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR6` writer - Port n Reset Bit 6"]
pub type PR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR7` writer - Port n Reset Bit 7"]
pub type PR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR8` writer - Port n Reset Bit 8"]
pub type PR8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR9` writer - Port n Reset Bit 9"]
pub type PR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR10` writer - Port n Reset Bit 10"]
pub type PR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR11` writer - Port n Reset Bit 11"]
pub type PR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR12` writer - Port n Reset Bit 12"]
pub type PR12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR13` writer - Port n Reset Bit 13"]
pub type PR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR14` writer - Port n Reset Bit 14"]
pub type PR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR15` writer - Port n Reset Bit 15"]
pub type PR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port n Set Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ps0(&mut self) -> PS0_W<OMR_SPEC> {
        PS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port n Set Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ps1(&mut self) -> PS1_W<OMR_SPEC> {
        PS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port n Set Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ps2(&mut self) -> PS2_W<OMR_SPEC> {
        PS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port n Set Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ps3(&mut self) -> PS3_W<OMR_SPEC> {
        PS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port n Set Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ps4(&mut self) -> PS4_W<OMR_SPEC> {
        PS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port n Set Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ps5(&mut self) -> PS5_W<OMR_SPEC> {
        PS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port n Set Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ps6(&mut self) -> PS6_W<OMR_SPEC> {
        PS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port n Set Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ps7(&mut self) -> PS7_W<OMR_SPEC> {
        PS7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port n Set Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ps8(&mut self) -> PS8_W<OMR_SPEC> {
        PS8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port n Set Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ps9(&mut self) -> PS9_W<OMR_SPEC> {
        PS9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port n Set Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ps10(&mut self) -> PS10_W<OMR_SPEC> {
        PS10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port n Set Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ps11(&mut self) -> PS11_W<OMR_SPEC> {
        PS11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port n Set Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ps12(&mut self) -> PS12_W<OMR_SPEC> {
        PS12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port n Set Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ps13(&mut self) -> PS13_W<OMR_SPEC> {
        PS13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port n Set Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ps14(&mut self) -> PS14_W<OMR_SPEC> {
        PS14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port n Set Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ps15(&mut self) -> PS15_W<OMR_SPEC> {
        PS15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Port n Reset Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pr0(&mut self) -> PR0_W<OMR_SPEC> {
        PR0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Port n Reset Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pr1(&mut self) -> PR1_W<OMR_SPEC> {
        PR1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Port n Reset Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pr2(&mut self) -> PR2_W<OMR_SPEC> {
        PR2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Port n Reset Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pr3(&mut self) -> PR3_W<OMR_SPEC> {
        PR3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Port n Reset Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pr4(&mut self) -> PR4_W<OMR_SPEC> {
        PR4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Port n Reset Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pr5(&mut self) -> PR5_W<OMR_SPEC> {
        PR5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Port n Reset Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pr6(&mut self) -> PR6_W<OMR_SPEC> {
        PR6_W::new(self, 22)
    }
    #[doc = "Bit 23 - Port n Reset Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pr7(&mut self) -> PR7_W<OMR_SPEC> {
        PR7_W::new(self, 23)
    }
    #[doc = "Bit 24 - Port n Reset Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pr8(&mut self) -> PR8_W<OMR_SPEC> {
        PR8_W::new(self, 24)
    }
    #[doc = "Bit 25 - Port n Reset Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pr9(&mut self) -> PR9_W<OMR_SPEC> {
        PR9_W::new(self, 25)
    }
    #[doc = "Bit 26 - Port n Reset Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pr10(&mut self) -> PR10_W<OMR_SPEC> {
        PR10_W::new(self, 26)
    }
    #[doc = "Bit 27 - Port n Reset Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pr11(&mut self) -> PR11_W<OMR_SPEC> {
        PR11_W::new(self, 27)
    }
    #[doc = "Bit 28 - Port n Reset Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pr12(&mut self) -> PR12_W<OMR_SPEC> {
        PR12_W::new(self, 28)
    }
    #[doc = "Bit 29 - Port n Reset Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pr13(&mut self) -> PR13_W<OMR_SPEC> {
        PR13_W::new(self, 29)
    }
    #[doc = "Bit 30 - Port n Reset Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pr14(&mut self) -> PR14_W<OMR_SPEC> {
        PR14_W::new(self, 30)
    }
    #[doc = "Bit 31 - Port n Reset Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pr15(&mut self) -> PR15_W<OMR_SPEC> {
        PR15_W::new(self, 31)
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
#[doc = "Port 3 Output Modification Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OMR_SPEC;
impl crate::RegisterSpec for OMR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`omr::W`](W) writer structure"]
impl crate::Writable for OMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OMR to value 0"]
impl crate::Resettable for OMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
