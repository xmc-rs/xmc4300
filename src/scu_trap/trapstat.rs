#[doc = "Register `TRAPSTAT` reader"]
pub struct R(crate::R<TRAPSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRAPSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRAPSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRAPSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOSCWDGT` reader - OSC_HP Oscillator Watchdog Trap Status"]
pub type SOSCWDGT_R = crate::BitReader<SOSCWDGT_A>;
#[doc = "OSC_HP Oscillator Watchdog Trap Status\n\nValue on reset: 0"]
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
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SOSCWDGT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SOSCWDGT_A::CONST_1
    }
}
#[doc = "Field `SVCOLCKT` reader - System VCO Lock Trap Status"]
pub type SVCOLCKT_R = crate::BitReader<SVCOLCKT_A>;
#[doc = "System VCO Lock Trap Status\n\nValue on reset: 0"]
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
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SVCOLCKT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SVCOLCKT_A::CONST_1
    }
}
#[doc = "Field `UVCOLCKT` reader - USB VCO Lock Trap Status"]
pub type UVCOLCKT_R = crate::BitReader<UVCOLCKT_A>;
#[doc = "USB VCO Lock Trap Status\n\nValue on reset: 0"]
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
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == UVCOLCKT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == UVCOLCKT_A::CONST_1
    }
}
#[doc = "Field `PET` reader - Parity Error Trap Status"]
pub type PET_R = crate::BitReader<PET_A>;
#[doc = "Parity Error Trap Status\n\nValue on reset: 0"]
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
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PET_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PET_A::CONST_1
    }
}
#[doc = "Field `BRWNT` reader - Brown Out Trap Status"]
pub type BRWNT_R = crate::BitReader<BRWNT_A>;
#[doc = "Brown Out Trap Status\n\nValue on reset: 0"]
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
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BRWNT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BRWNT_A::CONST_1
    }
}
#[doc = "Field `ULPWDGT` reader - OSC_ULP Oscillator Watchdog Trap Status"]
pub type ULPWDGT_R = crate::BitReader<ULPWDGT_A>;
#[doc = "OSC_ULP Oscillator Watchdog Trap Status\n\nValue on reset: 0"]
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
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDGT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDGT_A::CONST_1
    }
}
#[doc = "Field `BWERR0T` reader - Peripheral Bridge 0 Trap Status"]
pub type BWERR0T_R = crate::BitReader<BWERR0T_A>;
#[doc = "Peripheral Bridge 0 Trap Status\n\nValue on reset: 0"]
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
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR0T_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR0T_A::CONST_1
    }
}
#[doc = "Field `BWERR1T` reader - Peripheral Bridge 1 Trap Status"]
pub type BWERR1T_R = crate::BitReader<BWERR1T_A>;
#[doc = "Peripheral Bridge 1 Trap Status\n\nValue on reset: 0"]
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
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR1T_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR1T_A::CONST_1
    }
}
#[doc = "Field `ECAT0RST` reader - EtherCat Reset 0 Trap Status"]
pub type ECAT0RST_R = crate::BitReader<ECAT0RST_A>;
#[doc = "EtherCat Reset 0 Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0RST_A {
    #[doc = "0: No effect"]
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
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ECAT0RST_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ECAT0RST_A::CONST_1
    }
}
impl R {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Status"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SOSCWDGT_R {
        SOSCWDGT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Status"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SVCOLCKT_R {
        SVCOLCKT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Status"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UVCOLCKT_R {
        UVCOLCKT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Status"]
    #[inline(always)]
    pub fn pet(&self) -> PET_R {
        PET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Status"]
    #[inline(always)]
    pub fn brwnt(&self) -> BRWNT_R {
        BRWNT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Status"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> ULPWDGT_R {
        ULPWDGT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Status"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> BWERR0T_R {
        BWERR0T_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Status"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> BWERR1T_R {
        BWERR1T_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Status"]
    #[inline(always)]
    pub fn ecat0rst(&self) -> ECAT0RST_R {
        ECAT0RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Trap Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trapstat](index.html) module"]
pub struct TRAPSTAT_SPEC;
impl crate::RegisterSpec for TRAPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trapstat::R](R) reader structure"]
impl crate::Readable for TRAPSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRAPSTAT to value 0"]
impl crate::Resettable for TRAPSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
