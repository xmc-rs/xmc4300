#[doc = "Register `SM_CONTROL` reader"]
pub type R = crate::R<SmControlSpec>;
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OpMode {
    #[doc = "0: Buffered (3 buffer mode)"]
    Value1 = 0,
    #[doc = "2: Mailbox (Single buffer mode)"]
    Value3 = 2,
}
impl From<OpMode> for u8 {
    #[inline(always)]
    fn from(variant: OpMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OpMode {
    type Ux = u8;
}
impl crate::IsEnum for OpMode {}
#[doc = "Field `OP_MODE` reader - Operation Mode"]
pub type OpModeR = crate::FieldReader<OpMode>;
impl OpModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OpMode> {
        match self.bits {
            0 => Some(OpMode::Value1),
            2 => Some(OpMode::Value3),
            _ => None,
        }
    }
    #[doc = "Buffered (3 buffer mode)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OpMode::Value1
    }
    #[doc = "Mailbox (Single buffer mode)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == OpMode::Value3
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dir {
    #[doc = "0: Read: ECAT read access, PDI write access"]
    Value1 = 0,
    #[doc = "1: Write: ECAT write access, PDI read access"]
    Value2 = 1,
}
impl From<Dir> for u8 {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dir {
    type Ux = u8;
}
impl crate::IsEnum for Dir {}
#[doc = "Field `DIR` reader - Direction"]
pub type DirR = crate::FieldReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dir> {
        match self.bits {
            0 => Some(Dir::Value1),
            1 => Some(Dir::Value2),
            _ => None,
        }
    }
    #[doc = "Read: ECAT read access, PDI write access"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dir::Value1
    }
    #[doc = "Write: ECAT write access, PDI read access"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dir::Value2
    }
}
#[doc = "Interrupt in ECAT Event Request Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEcat {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<IntEcat> for bool {
    #[inline(always)]
    fn from(variant: IntEcat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_ECAT` reader - Interrupt in ECAT Event Request Register"]
pub type IntEcatR = crate::BitReader<IntEcat>;
impl IntEcatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEcat {
        match self.bits {
            false => IntEcat::Value1,
            true => IntEcat::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IntEcat::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IntEcat::Value2
    }
}
#[doc = "Interrupt in PDI Event Request Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntPdi {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<IntPdi> for bool {
    #[inline(always)]
    fn from(variant: IntPdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_PDI` reader - Interrupt in PDI Event Request Register"]
pub type IntPdiR = crate::BitReader<IntPdi>;
impl IntPdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntPdi {
        match self.bits {
            false => IntPdi::Value1,
            true => IntPdi::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IntPdi::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IntPdi::Value2
    }
}
#[doc = "Watchdog Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdTrg {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<WdTrg> for bool {
    #[inline(always)]
    fn from(variant: WdTrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WD_TRG` reader - Watchdog Trigger Enable"]
pub type WdTrgR = crate::BitReader<WdTrg>;
impl WdTrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdTrg {
        match self.bits {
            false => WdTrg::Value1,
            true => WdTrg::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WdTrg::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WdTrg::Value2
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn op_mode(&self) -> OpModeR {
        OpModeR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Interrupt in ECAT Event Request Register"]
    #[inline(always)]
    pub fn int_ecat(&self) -> IntEcatR {
        IntEcatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt in PDI Event Request Register"]
    #[inline(always)]
    pub fn int_pdi(&self) -> IntPdiR {
        IntPdiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Trigger Enable"]
    #[inline(always)]
    pub fn wd_trg(&self) -> WdTrgR {
        WdTrgR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Control Register SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmControlSpec;
impl crate::RegisterSpec for SmControlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sm_control::R`](R) reader structure"]
impl crate::Readable for SmControlSpec {}
#[doc = "`reset()` method sets SM_CONTROL to value 0"]
impl crate::Resettable for SmControlSpec {
    const RESET_VALUE: u8 = 0;
}
