#[doc = "Register `MOCTR` writer"]
pub type W = crate::W<MoctrSpec>;
#[doc = "Field `RESRXPND` writer - Reset/Set Receive Pending"]
pub type ResrxpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXPND` writer - Reset/Set Transmit Pending"]
pub type RestxpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRXUPD` writer - Reset/Set Receive Updating"]
pub type ResrxupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESNEWDAT` writer - Reset/Set New Data"]
pub type ResnewdatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESMSGLST` writer - Reset/Set Message Lost"]
pub type ResmsglstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESMSGVAL` writer - Reset/Set Message Valid"]
pub type ResmsgvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRTSEL` writer - Reset/Set Receive/Transmit Selected"]
pub type ResrtselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRXEN` writer - Reset/Set Receive Enable"]
pub type ResrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXRQ` writer - Reset/Set Transmit Request"]
pub type RestxrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXEN0` writer - Reset/Set Transmit Enable 0"]
pub type Restxen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXEN1` writer - Reset/Set Transmit Enable 1"]
pub type Restxen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESDIR` writer - Reset/Set Message Direction"]
pub type ResdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXPND` writer - Reset/Set Receive Pending"]
pub type SetrxpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXPND` writer - Reset/Set Transmit Pending"]
pub type SettxpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXUPD` writer - Reset/Set Receive Updating"]
pub type SetrxupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETNEWDAT` writer - Reset/Set New Data"]
pub type SetnewdatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETMSGLST` writer - Reset/Set Message Lost"]
pub type SetmsglstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETMSGVAL` writer - Reset/Set Message Valid"]
pub type SetmsgvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRTSEL` writer - Reset/Set Receive/Transmit Selected"]
pub type SetrtselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXEN` writer - Reset/Set Receive Enable"]
pub type SetrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXRQ` writer - Reset/Set Transmit Request"]
pub type SettxrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXEN0` writer - Reset/Set Transmit Enable 0"]
pub type Settxen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXEN1` writer - Reset/Set Transmit Enable 1"]
pub type Settxen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDIR` writer - Reset/Set Message Direction"]
pub type SetdirW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Reset/Set Receive Pending"]
    #[inline(always)]
    #[must_use]
    pub fn resrxpnd(&mut self) -> ResrxpndW<MoctrSpec> {
        ResrxpndW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset/Set Transmit Pending"]
    #[inline(always)]
    #[must_use]
    pub fn restxpnd(&mut self) -> RestxpndW<MoctrSpec> {
        RestxpndW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset/Set Receive Updating"]
    #[inline(always)]
    #[must_use]
    pub fn resrxupd(&mut self) -> ResrxupdW<MoctrSpec> {
        ResrxupdW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset/Set New Data"]
    #[inline(always)]
    #[must_use]
    pub fn resnewdat(&mut self) -> ResnewdatW<MoctrSpec> {
        ResnewdatW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset/Set Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn resmsglst(&mut self) -> ResmsglstW<MoctrSpec> {
        ResmsglstW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset/Set Message Valid"]
    #[inline(always)]
    #[must_use]
    pub fn resmsgval(&mut self) -> ResmsgvalW<MoctrSpec> {
        ResmsgvalW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset/Set Receive/Transmit Selected"]
    #[inline(always)]
    #[must_use]
    pub fn resrtsel(&mut self) -> ResrtselW<MoctrSpec> {
        ResrtselW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset/Set Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resrxen(&mut self) -> ResrxenW<MoctrSpec> {
        ResrxenW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset/Set Transmit Request"]
    #[inline(always)]
    #[must_use]
    pub fn restxrq(&mut self) -> RestxrqW<MoctrSpec> {
        RestxrqW::new(self, 8)
    }
    #[doc = "Bit 9 - Reset/Set Transmit Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn restxen0(&mut self) -> Restxen0W<MoctrSpec> {
        Restxen0W::new(self, 9)
    }
    #[doc = "Bit 10 - Reset/Set Transmit Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn restxen1(&mut self) -> Restxen1W<MoctrSpec> {
        Restxen1W::new(self, 10)
    }
    #[doc = "Bit 11 - Reset/Set Message Direction"]
    #[inline(always)]
    #[must_use]
    pub fn resdir(&mut self) -> ResdirW<MoctrSpec> {
        ResdirW::new(self, 11)
    }
    #[doc = "Bit 16 - Reset/Set Receive Pending"]
    #[inline(always)]
    #[must_use]
    pub fn setrxpnd(&mut self) -> SetrxpndW<MoctrSpec> {
        SetrxpndW::new(self, 16)
    }
    #[doc = "Bit 17 - Reset/Set Transmit Pending"]
    #[inline(always)]
    #[must_use]
    pub fn settxpnd(&mut self) -> SettxpndW<MoctrSpec> {
        SettxpndW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset/Set Receive Updating"]
    #[inline(always)]
    #[must_use]
    pub fn setrxupd(&mut self) -> SetrxupdW<MoctrSpec> {
        SetrxupdW::new(self, 18)
    }
    #[doc = "Bit 19 - Reset/Set New Data"]
    #[inline(always)]
    #[must_use]
    pub fn setnewdat(&mut self) -> SetnewdatW<MoctrSpec> {
        SetnewdatW::new(self, 19)
    }
    #[doc = "Bit 20 - Reset/Set Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn setmsglst(&mut self) -> SetmsglstW<MoctrSpec> {
        SetmsglstW::new(self, 20)
    }
    #[doc = "Bit 21 - Reset/Set Message Valid"]
    #[inline(always)]
    #[must_use]
    pub fn setmsgval(&mut self) -> SetmsgvalW<MoctrSpec> {
        SetmsgvalW::new(self, 21)
    }
    #[doc = "Bit 22 - Reset/Set Receive/Transmit Selected"]
    #[inline(always)]
    #[must_use]
    pub fn setrtsel(&mut self) -> SetrtselW<MoctrSpec> {
        SetrtselW::new(self, 22)
    }
    #[doc = "Bit 23 - Reset/Set Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn setrxen(&mut self) -> SetrxenW<MoctrSpec> {
        SetrxenW::new(self, 23)
    }
    #[doc = "Bit 24 - Reset/Set Transmit Request"]
    #[inline(always)]
    #[must_use]
    pub fn settxrq(&mut self) -> SettxrqW<MoctrSpec> {
        SettxrqW::new(self, 24)
    }
    #[doc = "Bit 25 - Reset/Set Transmit Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn settxen0(&mut self) -> Settxen0W<MoctrSpec> {
        Settxen0W::new(self, 25)
    }
    #[doc = "Bit 26 - Reset/Set Transmit Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn settxen1(&mut self) -> Settxen1W<MoctrSpec> {
        Settxen1W::new(self, 26)
    }
    #[doc = "Bit 27 - Reset/Set Message Direction"]
    #[inline(always)]
    #[must_use]
    pub fn setdir(&mut self) -> SetdirW<MoctrSpec> {
        SetdirW::new(self, 27)
    }
}
#[doc = "Message Object Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moctr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoctrSpec;
impl crate::RegisterSpec for MoctrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`moctr::W`](W) writer structure"]
impl crate::Writable for MoctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOCTR to value 0"]
impl crate::Resettable for MoctrSpec {
    const RESET_VALUE: u32 = 0;
}
