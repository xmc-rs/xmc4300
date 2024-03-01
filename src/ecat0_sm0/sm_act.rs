#[doc = "Register `SM_ACT` reader"]
pub type R = crate::R<SmActSpec>;
#[doc = "SyncManager Enable/Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmEn {
    #[doc = "0: Disable: Access to Memory without SyncManager control"]
    Value1 = 0,
    #[doc = "1: Enable: SyncManager is active and controls Memory area set in configuration"]
    Value2 = 1,
}
impl From<SmEn> for bool {
    #[inline(always)]
    fn from(variant: SmEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM_EN` reader - SyncManager Enable/Disable"]
pub type SmEnR = crate::BitReader<SmEn>;
impl SmEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmEn {
        match self.bits {
            false => SmEn::Value1,
            true => SmEn::Value2,
        }
    }
    #[doc = "Disable: Access to Memory without SyncManager control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SmEn::Value1
    }
    #[doc = "Enable: SyncManager is active and controls Memory area set in configuration"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SmEn::Value2
    }
}
#[doc = "Field `REP_REQ` reader - Repeat Request"]
pub type RepReqR = crate::BitReader;
#[doc = "LatchEvent ECAT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeEcat {
    #[doc = "0: No"]
    Value1 = 0,
    #[doc = "1: Generate Latch event if EtherCAT master issues a buffer exchange"]
    Value2 = 1,
}
impl From<LeEcat> for bool {
    #[inline(always)]
    fn from(variant: LeEcat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LE_ECAT` reader - LatchEvent ECAT"]
pub type LeEcatR = crate::BitReader<LeEcat>;
impl LeEcatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LeEcat {
        match self.bits {
            false => LeEcat::Value1,
            true => LeEcat::Value2,
        }
    }
    #[doc = "No"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LeEcat::Value1
    }
    #[doc = "Generate Latch event if EtherCAT master issues a buffer exchange"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LeEcat::Value2
    }
}
#[doc = "LatchEvent PDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LePdi {
    #[doc = "0: No"]
    Value1 = 0,
    #[doc = "1: Generate Latch events if PDI issues a buffer exchange or if PDI accesses buffer start address"]
    Value2 = 1,
}
impl From<LePdi> for bool {
    #[inline(always)]
    fn from(variant: LePdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LE_PDI` reader - LatchEvent PDI"]
pub type LePdiR = crate::BitReader<LePdi>;
impl LePdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LePdi {
        match self.bits {
            false => LePdi::Value1,
            true => LePdi::Value2,
        }
    }
    #[doc = "No"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LePdi::Value1
    }
    #[doc = "Generate Latch events if PDI issues a buffer exchange or if PDI accesses buffer start address"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LePdi::Value2
    }
}
impl R {
    #[doc = "Bit 0 - SyncManager Enable/Disable"]
    #[inline(always)]
    pub fn sm_en(&self) -> SmEnR {
        SmEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeat Request"]
    #[inline(always)]
    pub fn rep_req(&self) -> RepReqR {
        RepReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LatchEvent ECAT"]
    #[inline(always)]
    pub fn le_ecat(&self) -> LeEcatR {
        LeEcatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LatchEvent PDI"]
    #[inline(always)]
    pub fn le_pdi(&self) -> LePdiR {
        LePdiR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Activate SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmActSpec;
impl crate::RegisterSpec for SmActSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sm_act::R`](R) reader structure"]
impl crate::Readable for SmActSpec {}
#[doc = "`reset()` method sets SM_ACT to value 0"]
impl crate::Resettable for SmActSpec {
    const RESET_VALUE: u8 = 0;
}
