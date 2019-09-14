#[doc = "Reader of register FEATURE"]
pub type R = crate::R<u16, super::FEATURE>;
#[doc = "FMMU Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMMU_A {
    #[doc = "0: Bit oriented"]
    VALUE1,
    #[doc = "1: Byte oriented"]
    VALUE2,
}
impl From<FMMU_A> for bool {
    #[inline(always)]
    fn from(variant: FMMU_A) -> Self {
        match variant {
            FMMU_A::VALUE1 => false,
            FMMU_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FMMU`"]
pub type FMMU_R = crate::R<bool, FMMU_A>;
impl FMMU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMMU_A {
        match self.bits {
            false => FMMU_A::VALUE1,
            true => FMMU_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FMMU_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FMMU_A::VALUE2
    }
}
#[doc = "Distributed Clocks\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKS_A {
    #[doc = "0: Not available"]
    VALUE1,
    #[doc = "1: Available"]
    VALUE2,
}
impl From<CLKS_A> for bool {
    #[inline(always)]
    fn from(variant: CLKS_A) -> Self {
        match variant {
            CLKS_A::VALUE1 => false,
            CLKS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CLKS`"]
pub type CLKS_R = crate::R<bool, CLKS_A>;
impl CLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_A {
        match self.bits {
            false => CLKS_A::VALUE1,
            true => CLKS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKS_A::VALUE2
    }
}
#[doc = "Distributed Clocks (width)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKS_W_A {
    #[doc = "0: 32 bits"]
    VALUE1,
    #[doc = "1: 64 bits"]
    VALUE2,
}
impl From<CLKS_W_A> for bool {
    #[inline(always)]
    fn from(variant: CLKS_W_A) -> Self {
        match variant {
            CLKS_W_A::VALUE1 => false,
            CLKS_W_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CLKS_W`"]
pub type CLKS_W_R = crate::R<bool, CLKS_W_A>;
impl CLKS_W_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_W_A {
        match self.bits {
            false => CLKS_W_A::VALUE1,
            true => CLKS_W_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKS_W_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKS_W_A::VALUE2
    }
}
#[doc = "Low Jitter EBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LJ_EBUS_A {
    #[doc = "0: Not available, standard jitter"]
    VALUE1,
    #[doc = "1: Available, jitter minimized"]
    VALUE2,
}
impl From<LJ_EBUS_A> for bool {
    #[inline(always)]
    fn from(variant: LJ_EBUS_A) -> Self {
        match variant {
            LJ_EBUS_A::VALUE1 => false,
            LJ_EBUS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LJ_EBUS`"]
pub type LJ_EBUS_R = crate::R<bool, LJ_EBUS_A>;
impl LJ_EBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LJ_EBUS_A {
        match self.bits {
            false => LJ_EBUS_A::VALUE1,
            true => LJ_EBUS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LJ_EBUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LJ_EBUS_A::VALUE2
    }
}
#[doc = "Enhanced Link Detection EBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELD_EBUS_A {
    #[doc = "0: Not available"]
    VALUE1,
    #[doc = "1: Available"]
    VALUE2,
}
impl From<ELD_EBUS_A> for bool {
    #[inline(always)]
    fn from(variant: ELD_EBUS_A) -> Self {
        match variant {
            ELD_EBUS_A::VALUE1 => false,
            ELD_EBUS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ELD_EBUS`"]
pub type ELD_EBUS_R = crate::R<bool, ELD_EBUS_A>;
impl ELD_EBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELD_EBUS_A {
        match self.bits {
            false => ELD_EBUS_A::VALUE1,
            true => ELD_EBUS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ELD_EBUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ELD_EBUS_A::VALUE2
    }
}
#[doc = "Enhanced Link Detection MII\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELD_MII_A {
    #[doc = "0: Not available"]
    VALUE1,
    #[doc = "1: Available"]
    VALUE2,
}
impl From<ELD_MII_A> for bool {
    #[inline(always)]
    fn from(variant: ELD_MII_A) -> Self {
        match variant {
            ELD_MII_A::VALUE1 => false,
            ELD_MII_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ELD_MII`"]
pub type ELD_MII_R = crate::R<bool, ELD_MII_A>;
impl ELD_MII_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELD_MII_A {
        match self.bits {
            false => ELD_MII_A::VALUE1,
            true => ELD_MII_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ELD_MII_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ELD_MII_A::VALUE2
    }
}
#[doc = "Separate Handling of FCS Errors\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SH_FCSE_A {
    #[doc = "0: Not supported"]
    VALUE1,
    #[doc = "1: Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter"]
    VALUE2,
}
impl From<SH_FCSE_A> for bool {
    #[inline(always)]
    fn from(variant: SH_FCSE_A) -> Self {
        match variant {
            SH_FCSE_A::VALUE1 => false,
            SH_FCSE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SH_FCSE`"]
pub type SH_FCSE_R = crate::R<bool, SH_FCSE_A>;
impl SH_FCSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SH_FCSE_A {
        match self.bits {
            false => SH_FCSE_A::VALUE1,
            true => SH_FCSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SH_FCSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SH_FCSE_A::VALUE2
    }
}
#[doc = "Enhanced DC SYNC Activation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDC_SYNCA_A {
    #[doc = "0: Not available"]
    VALUE1,
    #[doc = "1: Available"]
    VALUE2,
}
impl From<EDC_SYNCA_A> for bool {
    #[inline(always)]
    fn from(variant: EDC_SYNCA_A) -> Self {
        match variant {
            EDC_SYNCA_A::VALUE1 => false,
            EDC_SYNCA_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EDC_SYNCA`"]
pub type EDC_SYNCA_R = crate::R<bool, EDC_SYNCA_A>;
impl EDC_SYNCA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDC_SYNCA_A {
        match self.bits {
            false => EDC_SYNCA_A::VALUE1,
            true => EDC_SYNCA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EDC_SYNCA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EDC_SYNCA_A::VALUE2
    }
}
#[doc = "EtherCAT LRW command support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRW_CS_A {
    #[doc = "0: Supported"]
    VALUE1,
    #[doc = "1: Not supported"]
    VALUE2,
}
impl From<LRW_CS_A> for bool {
    #[inline(always)]
    fn from(variant: LRW_CS_A) -> Self {
        match variant {
            LRW_CS_A::VALUE1 => false,
            LRW_CS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LRW_CS`"]
pub type LRW_CS_R = crate::R<bool, LRW_CS_A>;
impl LRW_CS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRW_CS_A {
        match self.bits {
            false => LRW_CS_A::VALUE1,
            true => LRW_CS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LRW_CS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LRW_CS_A::VALUE2
    }
}
#[doc = "EtherCAT read/write command support (BRW, APRW, FPRW)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RW_CS_A {
    #[doc = "0: Supported"]
    VALUE1,
    #[doc = "1: Not supported"]
    VALUE2,
}
impl From<RW_CS_A> for bool {
    #[inline(always)]
    fn from(variant: RW_CS_A) -> Self {
        match variant {
            RW_CS_A::VALUE1 => false,
            RW_CS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RW_CS`"]
pub type RW_CS_R = crate::R<bool, RW_CS_A>;
impl RW_CS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RW_CS_A {
        match self.bits {
            false => RW_CS_A::VALUE1,
            true => RW_CS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RW_CS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RW_CS_A::VALUE2
    }
}
#[doc = "Fixed FMMU/SyncManager configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FX_CONF_A {
    #[doc = "0: Variable configuration"]
    VALUE1,
    #[doc = "1: Fixed configuration (refer to documentation of supporting ESCs)"]
    VALUE2,
}
impl From<FX_CONF_A> for bool {
    #[inline(always)]
    fn from(variant: FX_CONF_A) -> Self {
        match variant {
            FX_CONF_A::VALUE1 => false,
            FX_CONF_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FX_CONF`"]
pub type FX_CONF_R = crate::R<bool, FX_CONF_A>;
impl FX_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FX_CONF_A {
        match self.bits {
            false => FX_CONF_A::VALUE1,
            true => FX_CONF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FX_CONF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FX_CONF_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - FMMU Operation"]
    #[inline(always)]
    pub fn fmmu(&self) -> FMMU_R {
        FMMU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Distributed Clocks"]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Distributed Clocks (width)"]
    #[inline(always)]
    pub fn clks_w(&self) -> CLKS_W_R {
        CLKS_W_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low Jitter EBUS"]
    #[inline(always)]
    pub fn lj_ebus(&self) -> LJ_EBUS_R {
        LJ_EBUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enhanced Link Detection EBUS"]
    #[inline(always)]
    pub fn eld_ebus(&self) -> ELD_EBUS_R {
        ELD_EBUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enhanced Link Detection MII"]
    #[inline(always)]
    pub fn eld_mii(&self) -> ELD_MII_R {
        ELD_MII_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Separate Handling of FCS Errors"]
    #[inline(always)]
    pub fn sh_fcse(&self) -> SH_FCSE_R {
        SH_FCSE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enhanced DC SYNC Activation"]
    #[inline(always)]
    pub fn edc_synca(&self) -> EDC_SYNCA_R {
        EDC_SYNCA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EtherCAT LRW command support"]
    #[inline(always)]
    pub fn lrw_cs(&self) -> LRW_CS_R {
        LRW_CS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EtherCAT read/write command support (BRW, APRW, FPRW)"]
    #[inline(always)]
    pub fn rw_cs(&self) -> RW_CS_R {
        RW_CS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fixed FMMU/SyncManager configuration"]
    #[inline(always)]
    pub fn fx_conf(&self) -> FX_CONF_R {
        FX_CONF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
