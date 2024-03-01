#[doc = "Register `OMR` writer"]
pub type W = crate::W<OmrSpec>;
#[doc = "Field `PS0` writer - Port n Set Bit 0"]
pub type Ps0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS1` writer - Port n Set Bit 1"]
pub type Ps1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2` writer - Port n Set Bit 2"]
pub type Ps2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS3` writer - Port n Set Bit 3"]
pub type Ps3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS4` writer - Port n Set Bit 4"]
pub type Ps4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS5` writer - Port n Set Bit 5"]
pub type Ps5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS6` writer - Port n Set Bit 6"]
pub type Ps6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS7` writer - Port n Set Bit 7"]
pub type Ps7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS8` writer - Port n Set Bit 8"]
pub type Ps8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS9` writer - Port n Set Bit 9"]
pub type Ps9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS10` writer - Port n Set Bit 10"]
pub type Ps10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS11` writer - Port n Set Bit 11"]
pub type Ps11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS12` writer - Port n Set Bit 12"]
pub type Ps12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS13` writer - Port n Set Bit 13"]
pub type Ps13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS14` writer - Port n Set Bit 14"]
pub type Ps14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS15` writer - Port n Set Bit 15"]
pub type Ps15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR0` writer - Port n Reset Bit 0"]
pub type Pr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1` writer - Port n Reset Bit 1"]
pub type Pr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR2` writer - Port n Reset Bit 2"]
pub type Pr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR3` writer - Port n Reset Bit 3"]
pub type Pr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR4` writer - Port n Reset Bit 4"]
pub type Pr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR5` writer - Port n Reset Bit 5"]
pub type Pr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR6` writer - Port n Reset Bit 6"]
pub type Pr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR7` writer - Port n Reset Bit 7"]
pub type Pr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR8` writer - Port n Reset Bit 8"]
pub type Pr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR9` writer - Port n Reset Bit 9"]
pub type Pr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR10` writer - Port n Reset Bit 10"]
pub type Pr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR11` writer - Port n Reset Bit 11"]
pub type Pr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR12` writer - Port n Reset Bit 12"]
pub type Pr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR13` writer - Port n Reset Bit 13"]
pub type Pr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR14` writer - Port n Reset Bit 14"]
pub type Pr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR15` writer - Port n Reset Bit 15"]
pub type Pr15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port n Set Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ps0(&mut self) -> Ps0W<OmrSpec> {
        Ps0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port n Set Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ps1(&mut self) -> Ps1W<OmrSpec> {
        Ps1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port n Set Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ps2(&mut self) -> Ps2W<OmrSpec> {
        Ps2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port n Set Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ps3(&mut self) -> Ps3W<OmrSpec> {
        Ps3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port n Set Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ps4(&mut self) -> Ps4W<OmrSpec> {
        Ps4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port n Set Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ps5(&mut self) -> Ps5W<OmrSpec> {
        Ps5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port n Set Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ps6(&mut self) -> Ps6W<OmrSpec> {
        Ps6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port n Set Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ps7(&mut self) -> Ps7W<OmrSpec> {
        Ps7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port n Set Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ps8(&mut self) -> Ps8W<OmrSpec> {
        Ps8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port n Set Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ps9(&mut self) -> Ps9W<OmrSpec> {
        Ps9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port n Set Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ps10(&mut self) -> Ps10W<OmrSpec> {
        Ps10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port n Set Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ps11(&mut self) -> Ps11W<OmrSpec> {
        Ps11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port n Set Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ps12(&mut self) -> Ps12W<OmrSpec> {
        Ps12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port n Set Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ps13(&mut self) -> Ps13W<OmrSpec> {
        Ps13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port n Set Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ps14(&mut self) -> Ps14W<OmrSpec> {
        Ps14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port n Set Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ps15(&mut self) -> Ps15W<OmrSpec> {
        Ps15W::new(self, 15)
    }
    #[doc = "Bit 16 - Port n Reset Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pr0(&mut self) -> Pr0W<OmrSpec> {
        Pr0W::new(self, 16)
    }
    #[doc = "Bit 17 - Port n Reset Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pr1(&mut self) -> Pr1W<OmrSpec> {
        Pr1W::new(self, 17)
    }
    #[doc = "Bit 18 - Port n Reset Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pr2(&mut self) -> Pr2W<OmrSpec> {
        Pr2W::new(self, 18)
    }
    #[doc = "Bit 19 - Port n Reset Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pr3(&mut self) -> Pr3W<OmrSpec> {
        Pr3W::new(self, 19)
    }
    #[doc = "Bit 20 - Port n Reset Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pr4(&mut self) -> Pr4W<OmrSpec> {
        Pr4W::new(self, 20)
    }
    #[doc = "Bit 21 - Port n Reset Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pr5(&mut self) -> Pr5W<OmrSpec> {
        Pr5W::new(self, 21)
    }
    #[doc = "Bit 22 - Port n Reset Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pr6(&mut self) -> Pr6W<OmrSpec> {
        Pr6W::new(self, 22)
    }
    #[doc = "Bit 23 - Port n Reset Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pr7(&mut self) -> Pr7W<OmrSpec> {
        Pr7W::new(self, 23)
    }
    #[doc = "Bit 24 - Port n Reset Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pr8(&mut self) -> Pr8W<OmrSpec> {
        Pr8W::new(self, 24)
    }
    #[doc = "Bit 25 - Port n Reset Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pr9(&mut self) -> Pr9W<OmrSpec> {
        Pr9W::new(self, 25)
    }
    #[doc = "Bit 26 - Port n Reset Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pr10(&mut self) -> Pr10W<OmrSpec> {
        Pr10W::new(self, 26)
    }
    #[doc = "Bit 27 - Port n Reset Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pr11(&mut self) -> Pr11W<OmrSpec> {
        Pr11W::new(self, 27)
    }
    #[doc = "Bit 28 - Port n Reset Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pr12(&mut self) -> Pr12W<OmrSpec> {
        Pr12W::new(self, 28)
    }
    #[doc = "Bit 29 - Port n Reset Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pr13(&mut self) -> Pr13W<OmrSpec> {
        Pr13W::new(self, 29)
    }
    #[doc = "Bit 30 - Port n Reset Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pr14(&mut self) -> Pr14W<OmrSpec> {
        Pr14W::new(self, 30)
    }
    #[doc = "Bit 31 - Port n Reset Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pr15(&mut self) -> Pr15W<OmrSpec> {
        Pr15W::new(self, 31)
    }
}
#[doc = "Port 1 Output Modification Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OmrSpec;
impl crate::RegisterSpec for OmrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`omr::W`](W) writer structure"]
impl crate::Writable for OmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OMR to value 0"]
impl crate::Resettable for OmrSpec {
    const RESET_VALUE: u32 = 0;
}
