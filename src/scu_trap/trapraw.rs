#[doc = "Register `TRAPRAW` reader"]
pub type R = crate::R<TRAPRAW_SPEC>;
#[doc = "Field `SOSCWDGT` reader - OSC_HP Oscillator Watchdog Trap Raw Status"]
pub type SOSCWDGT_R = crate::BitReader<SOSCWDGT_A>;
#[doc = "OSC_HP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSCWDGT_A {
    #[doc = "0: No pending trap request"]
    CONST_0 = 0,
    #[doc = "1: Pending trap request"]
    CONST_1 = 1,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        variant as u8 != 0
    }
}
impl SOSCWDGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCWDGT_A {
        match self.bits {
            false => SOSCWDGT_A::CONST_0,
            true => SOSCWDGT_A::CONST_1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SOSCWDGT_A::CONST_0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SOSCWDGT_A::CONST_1
    }
}
#[doc = "Field `SVCOLCKT` reader - System VCO Lock Trap Raw Status"]
pub type SVCOLCKT_R = crate::BitReader<SVCOLCKT_A>;
#[doc = "System VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    CONST_0 = 0,
    #[doc = "1: Pending trap request"]
    CONST_1 = 1,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
impl SVCOLCKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCOLCKT_A {
        match self.bits {
            false => SVCOLCKT_A::CONST_0,
            true => SVCOLCKT_A::CONST_1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SVCOLCKT_A::CONST_0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SVCOLCKT_A::CONST_1
    }
}
#[doc = "Field `UVCOLCKT` reader - USB VCO Lock Trap Raw Status"]
pub type UVCOLCKT_R = crate::BitReader<UVCOLCKT_A>;
#[doc = "USB VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    CONST_0 = 0,
    #[doc = "1: Pending trap request"]
    CONST_1 = 1,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
impl UVCOLCKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UVCOLCKT_A {
        match self.bits {
            false => UVCOLCKT_A::CONST_0,
            true => UVCOLCKT_A::CONST_1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == UVCOLCKT_A::CONST_0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == UVCOLCKT_A::CONST_1
    }
}
#[doc = "Field `PET` reader - Parity Error Trap Raw Status"]
pub type PET_R = crate::BitReader<PET_A>;
#[doc = "Parity Error Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PET_A {
    #[doc = "0: No pending trap request"]
    CONST_0 = 0,
    #[doc = "1: Pending trap request"]
    CONST_1 = 1,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        variant as u8 != 0
    }
}
impl PET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PET_A {
        match self.bits {
            false => PET_A::CONST_0,
            true => PET_A::CONST_1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PET_A::CONST_0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PET_A::CONST_1
    }
}
#[doc = "Field `BRWNT` reader - Brown Out Trap Raw Status"]
pub type BRWNT_R = crate::BitReader<BRWNT_A>;
#[doc = "Brown Out Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRWNT_A {
    #[doc = "0: No pending trap request"]
    CONST_0 = 0,
    #[doc = "1: Pending trap request"]
    CONST_1 = 1,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        variant as u8 != 0
    }
}
impl BRWNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRWNT_A {
        match self.bits {
            false => BRWNT_A::CONST_0,
            true => BRWNT_A::CONST_1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BRWNT_A::CONST_0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BRWNT_A::CONST_1
    }
}
#[doc = "Field `ULPWDGT` reader - OSC_ULP Oscillator Watchdog Trap Raw Status"]
pub type ULPWDGT_R = crate::BitReader<ULPWDGT_A>;
#[doc = "OSC_ULP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDGT_A {
    #[doc = "0: No pending trap request"]
    CONST_0 = 0,
    #[doc = "1: Pending trap request"]
    CONST_1 = 1,
}
impl From<ULPWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_A) -> Self {
        variant as u8 != 0
    }
}
impl ULPWDGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPWDGT_A {
        match self.bits {
            false => ULPWDGT_A::CONST_0,
            true => ULPWDGT_A::CONST_1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDGT_A::CONST_0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDGT_A::CONST_1
    }
}
#[doc = "Field `BWERR0T` reader - Peripheral Bridge 0 Trap Raw Status"]
pub type BWERR0T_R = crate::BitReader<BWERR0T_A>;
#[doc = "Peripheral Bridge 0 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR0T_A {
    #[doc = "0: No pending trap request"]
    CONST_0 = 0,
    #[doc = "1: Pending trap request"]
    CONST_1 = 1,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        variant as u8 != 0
    }
}
impl BWERR0T_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR0T_A {
        match self.bits {
            false => BWERR0T_A::CONST_0,
            true => BWERR0T_A::CONST_1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR0T_A::CONST_0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR0T_A::CONST_1
    }
}
#[doc = "Field `BWERR1T` reader - Peripheral Bridge 1 Trap Raw Status"]
pub type BWERR1T_R = crate::BitReader<BWERR1T_A>;
#[doc = "Peripheral Bridge 1 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR1T_A {
    #[doc = "0: No pending trap request"]
    CONST_0 = 0,
    #[doc = "1: Pending trap request"]
    CONST_1 = 1,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        variant as u8 != 0
    }
}
impl BWERR1T_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR1T_A {
        match self.bits {
            false => BWERR1T_A::CONST_0,
            true => BWERR1T_A::CONST_1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR1T_A::CONST_0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR1T_A::CONST_1
    }
}
#[doc = "Field `ECAT0RST` reader - EtherCat Reset 0 Trap Raw Status"]
pub type ECAT0RST_R = crate::BitReader<ECAT0RST_A>;
#[doc = "EtherCat Reset 0 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0RST_A {
    #[doc = "0: No pending trap request"]
    CONST_0 = 0,
    #[doc = "1: Pending trap request"]
    CONST_1 = 1,
}
impl From<ECAT0RST_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RST_A) -> Self {
        variant as u8 != 0
    }
}
impl ECAT0RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECAT0RST_A {
        match self.bits {
            false => ECAT0RST_A::CONST_0,
            true => ECAT0RST_A::CONST_1,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ECAT0RST_A::CONST_0
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ECAT0RST_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SOSCWDGT_R {
        SOSCWDGT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SVCOLCKT_R {
        SVCOLCKT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UVCOLCKT_R {
        UVCOLCKT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Raw Status"]
    #[inline(always)]
    pub fn pet(&self) -> PET_R {
        PET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Raw Status"]
    #[inline(always)]
    pub fn brwnt(&self) -> BRWNT_R {
        BRWNT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> ULPWDGT_R {
        ULPWDGT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> BWERR0T_R {
        BWERR0T_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> BWERR1T_R {
        BWERR1T_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Raw Status"]
    #[inline(always)]
    pub fn ecat0rst(&self) -> ECAT0RST_R {
        ECAT0RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Trap Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapraw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRAPRAW_SPEC;
impl crate::RegisterSpec for TRAPRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trapraw::R`](R) reader structure"]
impl crate::Readable for TRAPRAW_SPEC {}
#[doc = "`reset()` method sets TRAPRAW to value 0"]
impl crate::Resettable for TRAPRAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
