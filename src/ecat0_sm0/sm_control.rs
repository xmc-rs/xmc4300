#[doc = "Register `SM_CONTROL` reader"]
pub type R = crate::R<SM_CONTROL_SPEC>;
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OP_MODE_A {
    #[doc = "0: Buffered (3 buffer mode)"]
    VALUE1 = 0,
    #[doc = "2: Mailbox (Single buffer mode)"]
    VALUE3 = 2,
}
impl From<OP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OP_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OP_MODE_A {
    type Ux = u8;
}
impl crate::IsEnum for OP_MODE_A {}
#[doc = "Field `OP_MODE` reader - Operation Mode"]
pub type OP_MODE_R = crate::FieldReader<OP_MODE_A>;
impl OP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OP_MODE_A> {
        match self.bits {
            0 => Some(OP_MODE_A::VALUE1),
            2 => Some(OP_MODE_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Buffered (3 buffer mode)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OP_MODE_A::VALUE1
    }
    #[doc = "Mailbox (Single buffer mode)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == OP_MODE_A::VALUE3
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIR_A {
    #[doc = "0: Read: ECAT read access, PDI write access"]
    VALUE1 = 0,
    #[doc = "1: Write: ECAT write access, PDI read access"]
    VALUE2 = 1,
}
impl From<DIR_A> for u8 {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIR_A {
    type Ux = u8;
}
impl crate::IsEnum for DIR_A {}
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::FieldReader<DIR_A>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIR_A> {
        match self.bits {
            0 => Some(DIR_A::VALUE1),
            1 => Some(DIR_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Read: ECAT read access, PDI write access"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIR_A::VALUE1
    }
    #[doc = "Write: ECAT write access, PDI read access"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIR_A::VALUE2
    }
}
#[doc = "Interrupt in ECAT Event Request Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_ECAT_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<INT_ECAT_A> for bool {
    #[inline(always)]
    fn from(variant: INT_ECAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_ECAT` reader - Interrupt in ECAT Event Request Register"]
pub type INT_ECAT_R = crate::BitReader<INT_ECAT_A>;
impl INT_ECAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_ECAT_A {
        match self.bits {
            false => INT_ECAT_A::VALUE1,
            true => INT_ECAT_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INT_ECAT_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INT_ECAT_A::VALUE2
    }
}
#[doc = "Interrupt in PDI Event Request Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_PDI_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<INT_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: INT_PDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_PDI` reader - Interrupt in PDI Event Request Register"]
pub type INT_PDI_R = crate::BitReader<INT_PDI_A>;
impl INT_PDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_PDI_A {
        match self.bits {
            false => INT_PDI_A::VALUE1,
            true => INT_PDI_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INT_PDI_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INT_PDI_A::VALUE2
    }
}
#[doc = "Watchdog Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WD_TRG_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<WD_TRG_A> for bool {
    #[inline(always)]
    fn from(variant: WD_TRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WD_TRG` reader - Watchdog Trigger Enable"]
pub type WD_TRG_R = crate::BitReader<WD_TRG_A>;
impl WD_TRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WD_TRG_A {
        match self.bits {
            false => WD_TRG_A::VALUE1,
            true => WD_TRG_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WD_TRG_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WD_TRG_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn op_mode(&self) -> OP_MODE_R {
        OP_MODE_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Interrupt in ECAT Event Request Register"]
    #[inline(always)]
    pub fn int_ecat(&self) -> INT_ECAT_R {
        INT_ECAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt in PDI Event Request Register"]
    #[inline(always)]
    pub fn int_pdi(&self) -> INT_PDI_R {
        INT_PDI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Trigger Enable"]
    #[inline(always)]
    pub fn wd_trg(&self) -> WD_TRG_R {
        WD_TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Control Register SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_CONTROL_SPEC;
impl crate::RegisterSpec for SM_CONTROL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sm_control::R`](R) reader structure"]
impl crate::Readable for SM_CONTROL_SPEC {}
#[doc = "`reset()` method sets SM_CONTROL to value 0"]
impl crate::Resettable for SM_CONTROL_SPEC {
    const RESET_VALUE: u8 = 0;
}
