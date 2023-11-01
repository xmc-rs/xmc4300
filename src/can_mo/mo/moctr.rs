#[doc = "Register `MOCTR` writer"]
pub type W = crate::W<MOCTR_SPEC>;
#[doc = "Field `RESRXPND` writer - Reset/Set Receive Pending"]
pub type RESRXPND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESTXPND` writer - Reset/Set Transmit Pending"]
pub type RESTXPND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESRXUPD` writer - Reset/Set Receive Updating"]
pub type RESRXUPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESNEWDAT` writer - Reset/Set New Data"]
pub type RESNEWDAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESMSGLST` writer - Reset/Set Message Lost"]
pub type RESMSGLST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESMSGVAL` writer - Reset/Set Message Valid"]
pub type RESMSGVAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESRTSEL` writer - Reset/Set Receive/Transmit Selected"]
pub type RESRTSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESRXEN` writer - Reset/Set Receive Enable"]
pub type RESRXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESTXRQ` writer - Reset/Set Transmit Request"]
pub type RESTXRQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESTXEN0` writer - Reset/Set Transmit Enable 0"]
pub type RESTXEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESTXEN1` writer - Reset/Set Transmit Enable 1"]
pub type RESTXEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESDIR` writer - Reset/Set Message Direction"]
pub type RESDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETRXPND` writer - Reset/Set Receive Pending"]
pub type SETRXPND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETTXPND` writer - Reset/Set Transmit Pending"]
pub type SETTXPND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETRXUPD` writer - Reset/Set Receive Updating"]
pub type SETRXUPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETNEWDAT` writer - Reset/Set New Data"]
pub type SETNEWDAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETMSGLST` writer - Reset/Set Message Lost"]
pub type SETMSGLST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETMSGVAL` writer - Reset/Set Message Valid"]
pub type SETMSGVAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETRTSEL` writer - Reset/Set Receive/Transmit Selected"]
pub type SETRTSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETRXEN` writer - Reset/Set Receive Enable"]
pub type SETRXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETTXRQ` writer - Reset/Set Transmit Request"]
pub type SETTXRQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETTXEN0` writer - Reset/Set Transmit Enable 0"]
pub type SETTXEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETTXEN1` writer - Reset/Set Transmit Enable 1"]
pub type SETTXEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETDIR` writer - Reset/Set Message Direction"]
pub type SETDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Reset/Set Receive Pending"]
    #[inline(always)]
    #[must_use]
    pub fn resrxpnd(&mut self) -> RESRXPND_W<MOCTR_SPEC, 0> {
        RESRXPND_W::new(self)
    }
    #[doc = "Bit 1 - Reset/Set Transmit Pending"]
    #[inline(always)]
    #[must_use]
    pub fn restxpnd(&mut self) -> RESTXPND_W<MOCTR_SPEC, 1> {
        RESTXPND_W::new(self)
    }
    #[doc = "Bit 2 - Reset/Set Receive Updating"]
    #[inline(always)]
    #[must_use]
    pub fn resrxupd(&mut self) -> RESRXUPD_W<MOCTR_SPEC, 2> {
        RESRXUPD_W::new(self)
    }
    #[doc = "Bit 3 - Reset/Set New Data"]
    #[inline(always)]
    #[must_use]
    pub fn resnewdat(&mut self) -> RESNEWDAT_W<MOCTR_SPEC, 3> {
        RESNEWDAT_W::new(self)
    }
    #[doc = "Bit 4 - Reset/Set Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn resmsglst(&mut self) -> RESMSGLST_W<MOCTR_SPEC, 4> {
        RESMSGLST_W::new(self)
    }
    #[doc = "Bit 5 - Reset/Set Message Valid"]
    #[inline(always)]
    #[must_use]
    pub fn resmsgval(&mut self) -> RESMSGVAL_W<MOCTR_SPEC, 5> {
        RESMSGVAL_W::new(self)
    }
    #[doc = "Bit 6 - Reset/Set Receive/Transmit Selected"]
    #[inline(always)]
    #[must_use]
    pub fn resrtsel(&mut self) -> RESRTSEL_W<MOCTR_SPEC, 6> {
        RESRTSEL_W::new(self)
    }
    #[doc = "Bit 7 - Reset/Set Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resrxen(&mut self) -> RESRXEN_W<MOCTR_SPEC, 7> {
        RESRXEN_W::new(self)
    }
    #[doc = "Bit 8 - Reset/Set Transmit Request"]
    #[inline(always)]
    #[must_use]
    pub fn restxrq(&mut self) -> RESTXRQ_W<MOCTR_SPEC, 8> {
        RESTXRQ_W::new(self)
    }
    #[doc = "Bit 9 - Reset/Set Transmit Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn restxen0(&mut self) -> RESTXEN0_W<MOCTR_SPEC, 9> {
        RESTXEN0_W::new(self)
    }
    #[doc = "Bit 10 - Reset/Set Transmit Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn restxen1(&mut self) -> RESTXEN1_W<MOCTR_SPEC, 10> {
        RESTXEN1_W::new(self)
    }
    #[doc = "Bit 11 - Reset/Set Message Direction"]
    #[inline(always)]
    #[must_use]
    pub fn resdir(&mut self) -> RESDIR_W<MOCTR_SPEC, 11> {
        RESDIR_W::new(self)
    }
    #[doc = "Bit 16 - Reset/Set Receive Pending"]
    #[inline(always)]
    #[must_use]
    pub fn setrxpnd(&mut self) -> SETRXPND_W<MOCTR_SPEC, 16> {
        SETRXPND_W::new(self)
    }
    #[doc = "Bit 17 - Reset/Set Transmit Pending"]
    #[inline(always)]
    #[must_use]
    pub fn settxpnd(&mut self) -> SETTXPND_W<MOCTR_SPEC, 17> {
        SETTXPND_W::new(self)
    }
    #[doc = "Bit 18 - Reset/Set Receive Updating"]
    #[inline(always)]
    #[must_use]
    pub fn setrxupd(&mut self) -> SETRXUPD_W<MOCTR_SPEC, 18> {
        SETRXUPD_W::new(self)
    }
    #[doc = "Bit 19 - Reset/Set New Data"]
    #[inline(always)]
    #[must_use]
    pub fn setnewdat(&mut self) -> SETNEWDAT_W<MOCTR_SPEC, 19> {
        SETNEWDAT_W::new(self)
    }
    #[doc = "Bit 20 - Reset/Set Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn setmsglst(&mut self) -> SETMSGLST_W<MOCTR_SPEC, 20> {
        SETMSGLST_W::new(self)
    }
    #[doc = "Bit 21 - Reset/Set Message Valid"]
    #[inline(always)]
    #[must_use]
    pub fn setmsgval(&mut self) -> SETMSGVAL_W<MOCTR_SPEC, 21> {
        SETMSGVAL_W::new(self)
    }
    #[doc = "Bit 22 - Reset/Set Receive/Transmit Selected"]
    #[inline(always)]
    #[must_use]
    pub fn setrtsel(&mut self) -> SETRTSEL_W<MOCTR_SPEC, 22> {
        SETRTSEL_W::new(self)
    }
    #[doc = "Bit 23 - Reset/Set Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn setrxen(&mut self) -> SETRXEN_W<MOCTR_SPEC, 23> {
        SETRXEN_W::new(self)
    }
    #[doc = "Bit 24 - Reset/Set Transmit Request"]
    #[inline(always)]
    #[must_use]
    pub fn settxrq(&mut self) -> SETTXRQ_W<MOCTR_SPEC, 24> {
        SETTXRQ_W::new(self)
    }
    #[doc = "Bit 25 - Reset/Set Transmit Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn settxen0(&mut self) -> SETTXEN0_W<MOCTR_SPEC, 25> {
        SETTXEN0_W::new(self)
    }
    #[doc = "Bit 26 - Reset/Set Transmit Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn settxen1(&mut self) -> SETTXEN1_W<MOCTR_SPEC, 26> {
        SETTXEN1_W::new(self)
    }
    #[doc = "Bit 27 - Reset/Set Message Direction"]
    #[inline(always)]
    #[must_use]
    pub fn setdir(&mut self) -> SETDIR_W<MOCTR_SPEC, 27> {
        SETDIR_W::new(self)
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
#[doc = "Message Object Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moctr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOCTR_SPEC;
impl crate::RegisterSpec for MOCTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`moctr::W`](W) writer structure"]
impl crate::Writable for MOCTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOCTR to value 0"]
impl crate::Resettable for MOCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
