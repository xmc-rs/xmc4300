#[doc = "Register `MOCTR` writer"]
pub type W = crate::W<MOCTR_SPEC>;
#[doc = "Field `RESRXPND` writer - Reset/Set Receive Pending"]
pub type RESRXPND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXPND` writer - Reset/Set Transmit Pending"]
pub type RESTXPND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRXUPD` writer - Reset/Set Receive Updating"]
pub type RESRXUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESNEWDAT` writer - Reset/Set New Data"]
pub type RESNEWDAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESMSGLST` writer - Reset/Set Message Lost"]
pub type RESMSGLST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESMSGVAL` writer - Reset/Set Message Valid"]
pub type RESMSGVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRTSEL` writer - Reset/Set Receive/Transmit Selected"]
pub type RESRTSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRXEN` writer - Reset/Set Receive Enable"]
pub type RESRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXRQ` writer - Reset/Set Transmit Request"]
pub type RESTXRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXEN0` writer - Reset/Set Transmit Enable 0"]
pub type RESTXEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXEN1` writer - Reset/Set Transmit Enable 1"]
pub type RESTXEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESDIR` writer - Reset/Set Message Direction"]
pub type RESDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXPND` writer - Reset/Set Receive Pending"]
pub type SETRXPND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXPND` writer - Reset/Set Transmit Pending"]
pub type SETTXPND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXUPD` writer - Reset/Set Receive Updating"]
pub type SETRXUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETNEWDAT` writer - Reset/Set New Data"]
pub type SETNEWDAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETMSGLST` writer - Reset/Set Message Lost"]
pub type SETMSGLST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETMSGVAL` writer - Reset/Set Message Valid"]
pub type SETMSGVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRTSEL` writer - Reset/Set Receive/Transmit Selected"]
pub type SETRTSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXEN` writer - Reset/Set Receive Enable"]
pub type SETRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXRQ` writer - Reset/Set Transmit Request"]
pub type SETTXRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXEN0` writer - Reset/Set Transmit Enable 0"]
pub type SETTXEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXEN1` writer - Reset/Set Transmit Enable 1"]
pub type SETTXEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDIR` writer - Reset/Set Message Direction"]
pub type SETDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Reset/Set Receive Pending"]
    #[inline(always)]
    pub fn resrxpnd(&mut self) -> RESRXPND_W<MOCTR_SPEC> {
        RESRXPND_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reset/Set Transmit Pending"]
    #[inline(always)]
    pub fn restxpnd(&mut self) -> RESTXPND_W<MOCTR_SPEC> {
        RESTXPND_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset/Set Receive Updating"]
    #[inline(always)]
    pub fn resrxupd(&mut self) -> RESRXUPD_W<MOCTR_SPEC> {
        RESRXUPD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reset/Set New Data"]
    #[inline(always)]
    pub fn resnewdat(&mut self) -> RESNEWDAT_W<MOCTR_SPEC> {
        RESNEWDAT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reset/Set Message Lost"]
    #[inline(always)]
    pub fn resmsglst(&mut self) -> RESMSGLST_W<MOCTR_SPEC> {
        RESMSGLST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Reset/Set Message Valid"]
    #[inline(always)]
    pub fn resmsgval(&mut self) -> RESMSGVAL_W<MOCTR_SPEC> {
        RESMSGVAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset/Set Receive/Transmit Selected"]
    #[inline(always)]
    pub fn resrtsel(&mut self) -> RESRTSEL_W<MOCTR_SPEC> {
        RESRTSEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reset/Set Receive Enable"]
    #[inline(always)]
    pub fn resrxen(&mut self) -> RESRXEN_W<MOCTR_SPEC> {
        RESRXEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Reset/Set Transmit Request"]
    #[inline(always)]
    pub fn restxrq(&mut self) -> RESTXRQ_W<MOCTR_SPEC> {
        RESTXRQ_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reset/Set Transmit Enable 0"]
    #[inline(always)]
    pub fn restxen0(&mut self) -> RESTXEN0_W<MOCTR_SPEC> {
        RESTXEN0_W::new(self, 9)
    }
    #[doc = "Bit 10 - Reset/Set Transmit Enable 1"]
    #[inline(always)]
    pub fn restxen1(&mut self) -> RESTXEN1_W<MOCTR_SPEC> {
        RESTXEN1_W::new(self, 10)
    }
    #[doc = "Bit 11 - Reset/Set Message Direction"]
    #[inline(always)]
    pub fn resdir(&mut self) -> RESDIR_W<MOCTR_SPEC> {
        RESDIR_W::new(self, 11)
    }
    #[doc = "Bit 16 - Reset/Set Receive Pending"]
    #[inline(always)]
    pub fn setrxpnd(&mut self) -> SETRXPND_W<MOCTR_SPEC> {
        SETRXPND_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset/Set Transmit Pending"]
    #[inline(always)]
    pub fn settxpnd(&mut self) -> SETTXPND_W<MOCTR_SPEC> {
        SETTXPND_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset/Set Receive Updating"]
    #[inline(always)]
    pub fn setrxupd(&mut self) -> SETRXUPD_W<MOCTR_SPEC> {
        SETRXUPD_W::new(self, 18)
    }
    #[doc = "Bit 19 - Reset/Set New Data"]
    #[inline(always)]
    pub fn setnewdat(&mut self) -> SETNEWDAT_W<MOCTR_SPEC> {
        SETNEWDAT_W::new(self, 19)
    }
    #[doc = "Bit 20 - Reset/Set Message Lost"]
    #[inline(always)]
    pub fn setmsglst(&mut self) -> SETMSGLST_W<MOCTR_SPEC> {
        SETMSGLST_W::new(self, 20)
    }
    #[doc = "Bit 21 - Reset/Set Message Valid"]
    #[inline(always)]
    pub fn setmsgval(&mut self) -> SETMSGVAL_W<MOCTR_SPEC> {
        SETMSGVAL_W::new(self, 21)
    }
    #[doc = "Bit 22 - Reset/Set Receive/Transmit Selected"]
    #[inline(always)]
    pub fn setrtsel(&mut self) -> SETRTSEL_W<MOCTR_SPEC> {
        SETRTSEL_W::new(self, 22)
    }
    #[doc = "Bit 23 - Reset/Set Receive Enable"]
    #[inline(always)]
    pub fn setrxen(&mut self) -> SETRXEN_W<MOCTR_SPEC> {
        SETRXEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Reset/Set Transmit Request"]
    #[inline(always)]
    pub fn settxrq(&mut self) -> SETTXRQ_W<MOCTR_SPEC> {
        SETTXRQ_W::new(self, 24)
    }
    #[doc = "Bit 25 - Reset/Set Transmit Enable 0"]
    #[inline(always)]
    pub fn settxen0(&mut self) -> SETTXEN0_W<MOCTR_SPEC> {
        SETTXEN0_W::new(self, 25)
    }
    #[doc = "Bit 26 - Reset/Set Transmit Enable 1"]
    #[inline(always)]
    pub fn settxen1(&mut self) -> SETTXEN1_W<MOCTR_SPEC> {
        SETTXEN1_W::new(self, 26)
    }
    #[doc = "Bit 27 - Reset/Set Message Direction"]
    #[inline(always)]
    pub fn setdir(&mut self) -> SETDIR_W<MOCTR_SPEC> {
        SETDIR_W::new(self, 27)
    }
}
#[doc = "Message Object Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moctr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOCTR_SPEC;
impl crate::RegisterSpec for MOCTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`moctr::W`](W) writer structure"]
impl crate::Writable for MOCTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOCTR to value 0"]
impl crate::Resettable for MOCTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
