#[doc = "Register `FEATURE` reader"]
pub type R = crate::R<FeatureSpec>;
#[doc = "FMMU Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmmu {
    #[doc = "0: Bit oriented"]
    Value1 = 0,
    #[doc = "1: Byte oriented"]
    Value2 = 1,
}
impl From<Fmmu> for bool {
    #[inline(always)]
    fn from(variant: Fmmu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMMU` reader - FMMU Operation"]
pub type FmmuR = crate::BitReader<Fmmu>;
impl FmmuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmmu {
        match self.bits {
            false => Fmmu::Value1,
            true => Fmmu::Value2,
        }
    }
    #[doc = "Bit oriented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fmmu::Value1
    }
    #[doc = "Byte oriented"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fmmu::Value2
    }
}
#[doc = "Distributed Clocks\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clks {
    #[doc = "0: Not available"]
    Value1 = 0,
    #[doc = "1: Available"]
    Value2 = 1,
}
impl From<Clks> for bool {
    #[inline(always)]
    fn from(variant: Clks) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKS` reader - Distributed Clocks"]
pub type ClksR = crate::BitReader<Clks>;
impl ClksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clks {
        match self.bits {
            false => Clks::Value1,
            true => Clks::Value2,
        }
    }
    #[doc = "Not available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Clks::Value1
    }
    #[doc = "Available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Clks::Value2
    }
}
#[doc = "Distributed Clocks (width)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClksW {
    #[doc = "0: 32 bits"]
    Value1 = 0,
    #[doc = "1: 64 bits"]
    Value2 = 1,
}
impl From<ClksW> for bool {
    #[inline(always)]
    fn from(variant: ClksW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKS_W` reader - Distributed Clocks (width)"]
pub type ClksWR = crate::BitReader<ClksW>;
impl ClksWR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClksW {
        match self.bits {
            false => ClksW::Value1,
            true => ClksW::Value2,
        }
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ClksW::Value1
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ClksW::Value2
    }
}
#[doc = "Low Jitter EBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LjEbus {
    #[doc = "0: Not available, standard jitter"]
    Value1 = 0,
    #[doc = "1: Available, jitter minimized"]
    Value2 = 1,
}
impl From<LjEbus> for bool {
    #[inline(always)]
    fn from(variant: LjEbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LJ_EBUS` reader - Low Jitter EBUS"]
pub type LjEbusR = crate::BitReader<LjEbus>;
impl LjEbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LjEbus {
        match self.bits {
            false => LjEbus::Value1,
            true => LjEbus::Value2,
        }
    }
    #[doc = "Not available, standard jitter"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LjEbus::Value1
    }
    #[doc = "Available, jitter minimized"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LjEbus::Value2
    }
}
#[doc = "Enhanced Link Detection EBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EldEbus {
    #[doc = "0: Not available"]
    Value1 = 0,
    #[doc = "1: Available"]
    Value2 = 1,
}
impl From<EldEbus> for bool {
    #[inline(always)]
    fn from(variant: EldEbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELD_EBUS` reader - Enhanced Link Detection EBUS"]
pub type EldEbusR = crate::BitReader<EldEbus>;
impl EldEbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EldEbus {
        match self.bits {
            false => EldEbus::Value1,
            true => EldEbus::Value2,
        }
    }
    #[doc = "Not available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EldEbus::Value1
    }
    #[doc = "Available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EldEbus::Value2
    }
}
#[doc = "Enhanced Link Detection MII\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EldMii {
    #[doc = "0: Not available"]
    Value1 = 0,
    #[doc = "1: Available"]
    Value2 = 1,
}
impl From<EldMii> for bool {
    #[inline(always)]
    fn from(variant: EldMii) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELD_MII` reader - Enhanced Link Detection MII"]
pub type EldMiiR = crate::BitReader<EldMii>;
impl EldMiiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EldMii {
        match self.bits {
            false => EldMii::Value1,
            true => EldMii::Value2,
        }
    }
    #[doc = "Not available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EldMii::Value1
    }
    #[doc = "Available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EldMii::Value2
    }
}
#[doc = "Separate Handling of FCS Errors\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShFcse {
    #[doc = "0: Not supported"]
    Value1 = 0,
    #[doc = "1: Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter"]
    Value2 = 1,
}
impl From<ShFcse> for bool {
    #[inline(always)]
    fn from(variant: ShFcse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SH_FCSE` reader - Separate Handling of FCS Errors"]
pub type ShFcseR = crate::BitReader<ShFcse>;
impl ShFcseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShFcse {
        match self.bits {
            false => ShFcse::Value1,
            true => ShFcse::Value2,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ShFcse::Value1
    }
    #[doc = "Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ShFcse::Value2
    }
}
#[doc = "Enhanced DC SYNC Activation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdcSynca {
    #[doc = "0: Not available"]
    Value1 = 0,
    #[doc = "1: Available"]
    Value2 = 1,
}
impl From<EdcSynca> for bool {
    #[inline(always)]
    fn from(variant: EdcSynca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDC_SYNCA` reader - Enhanced DC SYNC Activation"]
pub type EdcSyncaR = crate::BitReader<EdcSynca>;
impl EdcSyncaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdcSynca {
        match self.bits {
            false => EdcSynca::Value1,
            true => EdcSynca::Value2,
        }
    }
    #[doc = "Not available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EdcSynca::Value1
    }
    #[doc = "Available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EdcSynca::Value2
    }
}
#[doc = "EtherCAT LRW command support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LrwCs {
    #[doc = "0: Supported"]
    Value1 = 0,
    #[doc = "1: Not supported"]
    Value2 = 1,
}
impl From<LrwCs> for bool {
    #[inline(always)]
    fn from(variant: LrwCs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRW_CS` reader - EtherCAT LRW command support"]
pub type LrwCsR = crate::BitReader<LrwCs>;
impl LrwCsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LrwCs {
        match self.bits {
            false => LrwCs::Value1,
            true => LrwCs::Value2,
        }
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LrwCs::Value1
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LrwCs::Value2
    }
}
#[doc = "EtherCAT read/write command support (BRW, APRW, FPRW)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RwCs {
    #[doc = "0: Supported"]
    Value1 = 0,
    #[doc = "1: Not supported"]
    Value2 = 1,
}
impl From<RwCs> for bool {
    #[inline(always)]
    fn from(variant: RwCs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RW_CS` reader - EtherCAT read/write command support (BRW, APRW, FPRW)"]
pub type RwCsR = crate::BitReader<RwCs>;
impl RwCsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RwCs {
        match self.bits {
            false => RwCs::Value1,
            true => RwCs::Value2,
        }
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RwCs::Value1
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RwCs::Value2
    }
}
#[doc = "Fixed FMMU/SyncManager configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FxConf {
    #[doc = "0: Variable configuration"]
    Value1 = 0,
    #[doc = "1: Fixed configuration (refer to documentation of supporting ESCs)"]
    Value2 = 1,
}
impl From<FxConf> for bool {
    #[inline(always)]
    fn from(variant: FxConf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FX_CONF` reader - Fixed FMMU/SyncManager configuration"]
pub type FxConfR = crate::BitReader<FxConf>;
impl FxConfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FxConf {
        match self.bits {
            false => FxConf::Value1,
            true => FxConf::Value2,
        }
    }
    #[doc = "Variable configuration"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FxConf::Value1
    }
    #[doc = "Fixed configuration (refer to documentation of supporting ESCs)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FxConf::Value2
    }
}
impl R {
    #[doc = "Bit 0 - FMMU Operation"]
    #[inline(always)]
    pub fn fmmu(&self) -> FmmuR {
        FmmuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Distributed Clocks"]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Distributed Clocks (width)"]
    #[inline(always)]
    pub fn clks_w(&self) -> ClksWR {
        ClksWR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Jitter EBUS"]
    #[inline(always)]
    pub fn lj_ebus(&self) -> LjEbusR {
        LjEbusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enhanced Link Detection EBUS"]
    #[inline(always)]
    pub fn eld_ebus(&self) -> EldEbusR {
        EldEbusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enhanced Link Detection MII"]
    #[inline(always)]
    pub fn eld_mii(&self) -> EldMiiR {
        EldMiiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Separate Handling of FCS Errors"]
    #[inline(always)]
    pub fn sh_fcse(&self) -> ShFcseR {
        ShFcseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enhanced DC SYNC Activation"]
    #[inline(always)]
    pub fn edc_synca(&self) -> EdcSyncaR {
        EdcSyncaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EtherCAT LRW command support"]
    #[inline(always)]
    pub fn lrw_cs(&self) -> LrwCsR {
        LrwCsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EtherCAT read/write command support (BRW, APRW, FPRW)"]
    #[inline(always)]
    pub fn rw_cs(&self) -> RwCsR {
        RwCsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fixed FMMU/SyncManager configuration"]
    #[inline(always)]
    pub fn fx_conf(&self) -> FxConfR {
        FxConfR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "ESC Features Supported\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeatureSpec;
impl crate::RegisterSpec for FeatureSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`feature::R`](R) reader structure"]
impl crate::Readable for FeatureSpec {}
#[doc = "`reset()` method sets FEATURE to value 0x01cc"]
impl crate::Resettable for FeatureSpec {
    const RESET_VALUE: u16 = 0x01cc;
}
