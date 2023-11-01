#[doc = "Register `FEATURE` reader"]
pub type R = crate::R<FEATURE_SPEC>;
#[doc = "Field `FMMU` reader - FMMU Operation"]
pub type FMMU_R = crate::BitReader<FMMU_A>;
#[doc = "FMMU Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMMU_A {
    #[doc = "0: Bit oriented"]
    VALUE1 = 0,
    #[doc = "1: Byte oriented"]
    VALUE2 = 1,
}
impl From<FMMU_A> for bool {
    #[inline(always)]
    fn from(variant: FMMU_A) -> Self {
        variant as u8 != 0
    }
}
impl FMMU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMMU_A {
        match self.bits {
            false => FMMU_A::VALUE1,
            true => FMMU_A::VALUE2,
        }
    }
    #[doc = "Bit oriented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FMMU_A::VALUE1
    }
    #[doc = "Byte oriented"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FMMU_A::VALUE2
    }
}
#[doc = "Field `CLKS` reader - Distributed Clocks"]
pub type CLKS_R = crate::BitReader<CLKS_A>;
#[doc = "Distributed Clocks\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKS_A {
    #[doc = "0: Not available"]
    VALUE1 = 0,
    #[doc = "1: Available"]
    VALUE2 = 1,
}
impl From<CLKS_A> for bool {
    #[inline(always)]
    fn from(variant: CLKS_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKS_A {
        match self.bits {
            false => CLKS_A::VALUE1,
            true => CLKS_A::VALUE2,
        }
    }
    #[doc = "Not available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKS_A::VALUE1
    }
    #[doc = "Available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKS_A::VALUE2
    }
}
#[doc = "Field `CLKS_W` reader - Distributed Clocks (width)"]
pub type CLKS_W_R = crate::BitReader<CLKS_W_A>;
#[doc = "Distributed Clocks (width)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKS_W_A {
    #[doc = "0: 32 bits"]
    VALUE1 = 0,
    #[doc = "1: 64 bits"]
    VALUE2 = 1,
}
impl From<CLKS_W_A> for bool {
    #[inline(always)]
    fn from(variant: CLKS_W_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKS_W_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKS_W_A {
        match self.bits {
            false => CLKS_W_A::VALUE1,
            true => CLKS_W_A::VALUE2,
        }
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKS_W_A::VALUE1
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKS_W_A::VALUE2
    }
}
#[doc = "Field `LJ_EBUS` reader - Low Jitter EBUS"]
pub type LJ_EBUS_R = crate::BitReader<LJ_EBUS_A>;
#[doc = "Low Jitter EBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LJ_EBUS_A {
    #[doc = "0: Not available, standard jitter"]
    VALUE1 = 0,
    #[doc = "1: Available, jitter minimized"]
    VALUE2 = 1,
}
impl From<LJ_EBUS_A> for bool {
    #[inline(always)]
    fn from(variant: LJ_EBUS_A) -> Self {
        variant as u8 != 0
    }
}
impl LJ_EBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LJ_EBUS_A {
        match self.bits {
            false => LJ_EBUS_A::VALUE1,
            true => LJ_EBUS_A::VALUE2,
        }
    }
    #[doc = "Not available, standard jitter"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LJ_EBUS_A::VALUE1
    }
    #[doc = "Available, jitter minimized"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LJ_EBUS_A::VALUE2
    }
}
#[doc = "Field `ELD_EBUS` reader - Enhanced Link Detection EBUS"]
pub type ELD_EBUS_R = crate::BitReader<ELD_EBUS_A>;
#[doc = "Enhanced Link Detection EBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELD_EBUS_A {
    #[doc = "0: Not available"]
    VALUE1 = 0,
    #[doc = "1: Available"]
    VALUE2 = 1,
}
impl From<ELD_EBUS_A> for bool {
    #[inline(always)]
    fn from(variant: ELD_EBUS_A) -> Self {
        variant as u8 != 0
    }
}
impl ELD_EBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ELD_EBUS_A {
        match self.bits {
            false => ELD_EBUS_A::VALUE1,
            true => ELD_EBUS_A::VALUE2,
        }
    }
    #[doc = "Not available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ELD_EBUS_A::VALUE1
    }
    #[doc = "Available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ELD_EBUS_A::VALUE2
    }
}
#[doc = "Field `ELD_MII` reader - Enhanced Link Detection MII"]
pub type ELD_MII_R = crate::BitReader<ELD_MII_A>;
#[doc = "Enhanced Link Detection MII\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELD_MII_A {
    #[doc = "0: Not available"]
    VALUE1 = 0,
    #[doc = "1: Available"]
    VALUE2 = 1,
}
impl From<ELD_MII_A> for bool {
    #[inline(always)]
    fn from(variant: ELD_MII_A) -> Self {
        variant as u8 != 0
    }
}
impl ELD_MII_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ELD_MII_A {
        match self.bits {
            false => ELD_MII_A::VALUE1,
            true => ELD_MII_A::VALUE2,
        }
    }
    #[doc = "Not available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ELD_MII_A::VALUE1
    }
    #[doc = "Available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ELD_MII_A::VALUE2
    }
}
#[doc = "Field `SH_FCSE` reader - Separate Handling of FCS Errors"]
pub type SH_FCSE_R = crate::BitReader<SH_FCSE_A>;
#[doc = "Separate Handling of FCS Errors\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SH_FCSE_A {
    #[doc = "0: Not supported"]
    VALUE1 = 0,
    #[doc = "1: Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter"]
    VALUE2 = 1,
}
impl From<SH_FCSE_A> for bool {
    #[inline(always)]
    fn from(variant: SH_FCSE_A) -> Self {
        variant as u8 != 0
    }
}
impl SH_FCSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SH_FCSE_A {
        match self.bits {
            false => SH_FCSE_A::VALUE1,
            true => SH_FCSE_A::VALUE2,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SH_FCSE_A::VALUE1
    }
    #[doc = "Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SH_FCSE_A::VALUE2
    }
}
#[doc = "Field `EDC_SYNCA` reader - Enhanced DC SYNC Activation"]
pub type EDC_SYNCA_R = crate::BitReader<EDC_SYNCA_A>;
#[doc = "Enhanced DC SYNC Activation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDC_SYNCA_A {
    #[doc = "0: Not available"]
    VALUE1 = 0,
    #[doc = "1: Available"]
    VALUE2 = 1,
}
impl From<EDC_SYNCA_A> for bool {
    #[inline(always)]
    fn from(variant: EDC_SYNCA_A) -> Self {
        variant as u8 != 0
    }
}
impl EDC_SYNCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDC_SYNCA_A {
        match self.bits {
            false => EDC_SYNCA_A::VALUE1,
            true => EDC_SYNCA_A::VALUE2,
        }
    }
    #[doc = "Not available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EDC_SYNCA_A::VALUE1
    }
    #[doc = "Available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EDC_SYNCA_A::VALUE2
    }
}
#[doc = "Field `LRW_CS` reader - EtherCAT LRW command support"]
pub type LRW_CS_R = crate::BitReader<LRW_CS_A>;
#[doc = "EtherCAT LRW command support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRW_CS_A {
    #[doc = "0: Supported"]
    VALUE1 = 0,
    #[doc = "1: Not supported"]
    VALUE2 = 1,
}
impl From<LRW_CS_A> for bool {
    #[inline(always)]
    fn from(variant: LRW_CS_A) -> Self {
        variant as u8 != 0
    }
}
impl LRW_CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LRW_CS_A {
        match self.bits {
            false => LRW_CS_A::VALUE1,
            true => LRW_CS_A::VALUE2,
        }
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LRW_CS_A::VALUE1
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LRW_CS_A::VALUE2
    }
}
#[doc = "Field `RW_CS` reader - EtherCAT read/write command support (BRW, APRW, FPRW)"]
pub type RW_CS_R = crate::BitReader<RW_CS_A>;
#[doc = "EtherCAT read/write command support (BRW, APRW, FPRW)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RW_CS_A {
    #[doc = "0: Supported"]
    VALUE1 = 0,
    #[doc = "1: Not supported"]
    VALUE2 = 1,
}
impl From<RW_CS_A> for bool {
    #[inline(always)]
    fn from(variant: RW_CS_A) -> Self {
        variant as u8 != 0
    }
}
impl RW_CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RW_CS_A {
        match self.bits {
            false => RW_CS_A::VALUE1,
            true => RW_CS_A::VALUE2,
        }
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RW_CS_A::VALUE1
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RW_CS_A::VALUE2
    }
}
#[doc = "Field `FX_CONF` reader - Fixed FMMU/SyncManager configuration"]
pub type FX_CONF_R = crate::BitReader<FX_CONF_A>;
#[doc = "Fixed FMMU/SyncManager configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FX_CONF_A {
    #[doc = "0: Variable configuration"]
    VALUE1 = 0,
    #[doc = "1: Fixed configuration (refer to documentation of supporting ESCs)"]
    VALUE2 = 1,
}
impl From<FX_CONF_A> for bool {
    #[inline(always)]
    fn from(variant: FX_CONF_A) -> Self {
        variant as u8 != 0
    }
}
impl FX_CONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FX_CONF_A {
        match self.bits {
            false => FX_CONF_A::VALUE1,
            true => FX_CONF_A::VALUE2,
        }
    }
    #[doc = "Variable configuration"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FX_CONF_A::VALUE1
    }
    #[doc = "Fixed configuration (refer to documentation of supporting ESCs)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FX_CONF_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - FMMU Operation"]
    #[inline(always)]
    pub fn fmmu(&self) -> FMMU_R {
        FMMU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Distributed Clocks"]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Distributed Clocks (width)"]
    #[inline(always)]
    pub fn clks_w(&self) -> CLKS_W_R {
        CLKS_W_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Jitter EBUS"]
    #[inline(always)]
    pub fn lj_ebus(&self) -> LJ_EBUS_R {
        LJ_EBUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enhanced Link Detection EBUS"]
    #[inline(always)]
    pub fn eld_ebus(&self) -> ELD_EBUS_R {
        ELD_EBUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enhanced Link Detection MII"]
    #[inline(always)]
    pub fn eld_mii(&self) -> ELD_MII_R {
        ELD_MII_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Separate Handling of FCS Errors"]
    #[inline(always)]
    pub fn sh_fcse(&self) -> SH_FCSE_R {
        SH_FCSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enhanced DC SYNC Activation"]
    #[inline(always)]
    pub fn edc_synca(&self) -> EDC_SYNCA_R {
        EDC_SYNCA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EtherCAT LRW command support"]
    #[inline(always)]
    pub fn lrw_cs(&self) -> LRW_CS_R {
        LRW_CS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EtherCAT read/write command support (BRW, APRW, FPRW)"]
    #[inline(always)]
    pub fn rw_cs(&self) -> RW_CS_R {
        RW_CS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fixed FMMU/SyncManager configuration"]
    #[inline(always)]
    pub fn fx_conf(&self) -> FX_CONF_R {
        FX_CONF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "ESC Features Supported\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FEATURE_SPEC;
impl crate::RegisterSpec for FEATURE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`feature::R`](R) reader structure"]
impl crate::Readable for FEATURE_SPEC {}
#[doc = "`reset()` method sets FEATURE to value 0x01cc"]
impl crate::Resettable for FEATURE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01cc;
}
