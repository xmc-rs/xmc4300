#[doc = "Register `FEATURE` reader"]
pub struct R(crate::R<FEATURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEATURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEATURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEATURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "FMMU Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FMMU` reader - FMMU Operation"]
pub struct FMMU_R(crate::FieldReader<bool, FMMU_A>);
impl FMMU_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMMU_R(crate::FieldReader::new(bits))
    }
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
        **self == FMMU_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FMMU_A::VALUE2
    }
}
impl core::ops::Deref for FMMU_R {
    type Target = crate::FieldReader<bool, FMMU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Distributed Clocks\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CLKS` reader - Distributed Clocks"]
pub struct CLKS_R(crate::FieldReader<bool, CLKS_A>);
impl CLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKS_R(crate::FieldReader::new(bits))
    }
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
        **self == CLKS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CLKS_A::VALUE2
    }
}
impl core::ops::Deref for CLKS_R {
    type Target = crate::FieldReader<bool, CLKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Distributed Clocks (width)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CLKS_W` reader - Distributed Clocks (width)"]
pub struct CLKS_W_R(crate::FieldReader<bool, CLKS_W_A>);
impl CLKS_W_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKS_W_R(crate::FieldReader::new(bits))
    }
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
        **self == CLKS_W_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CLKS_W_A::VALUE2
    }
}
impl core::ops::Deref for CLKS_W_R {
    type Target = crate::FieldReader<bool, CLKS_W_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Low Jitter EBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LJ_EBUS` reader - Low Jitter EBUS"]
pub struct LJ_EBUS_R(crate::FieldReader<bool, LJ_EBUS_A>);
impl LJ_EBUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LJ_EBUS_R(crate::FieldReader::new(bits))
    }
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
        **self == LJ_EBUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LJ_EBUS_A::VALUE2
    }
}
impl core::ops::Deref for LJ_EBUS_R {
    type Target = crate::FieldReader<bool, LJ_EBUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enhanced Link Detection EBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ELD_EBUS` reader - Enhanced Link Detection EBUS"]
pub struct ELD_EBUS_R(crate::FieldReader<bool, ELD_EBUS_A>);
impl ELD_EBUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELD_EBUS_R(crate::FieldReader::new(bits))
    }
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
        **self == ELD_EBUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ELD_EBUS_A::VALUE2
    }
}
impl core::ops::Deref for ELD_EBUS_R {
    type Target = crate::FieldReader<bool, ELD_EBUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enhanced Link Detection MII\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ELD_MII` reader - Enhanced Link Detection MII"]
pub struct ELD_MII_R(crate::FieldReader<bool, ELD_MII_A>);
impl ELD_MII_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELD_MII_R(crate::FieldReader::new(bits))
    }
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
        **self == ELD_MII_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ELD_MII_A::VALUE2
    }
}
impl core::ops::Deref for ELD_MII_R {
    type Target = crate::FieldReader<bool, ELD_MII_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Separate Handling of FCS Errors\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SH_FCSE` reader - Separate Handling of FCS Errors"]
pub struct SH_FCSE_R(crate::FieldReader<bool, SH_FCSE_A>);
impl SH_FCSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SH_FCSE_R(crate::FieldReader::new(bits))
    }
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
        **self == SH_FCSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SH_FCSE_A::VALUE2
    }
}
impl core::ops::Deref for SH_FCSE_R {
    type Target = crate::FieldReader<bool, SH_FCSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enhanced DC SYNC Activation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EDC_SYNCA` reader - Enhanced DC SYNC Activation"]
pub struct EDC_SYNCA_R(crate::FieldReader<bool, EDC_SYNCA_A>);
impl EDC_SYNCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDC_SYNCA_R(crate::FieldReader::new(bits))
    }
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
        **self == EDC_SYNCA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EDC_SYNCA_A::VALUE2
    }
}
impl core::ops::Deref for EDC_SYNCA_R {
    type Target = crate::FieldReader<bool, EDC_SYNCA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "EtherCAT LRW command support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LRW_CS` reader - EtherCAT LRW command support"]
pub struct LRW_CS_R(crate::FieldReader<bool, LRW_CS_A>);
impl LRW_CS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRW_CS_R(crate::FieldReader::new(bits))
    }
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
        **self == LRW_CS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LRW_CS_A::VALUE2
    }
}
impl core::ops::Deref for LRW_CS_R {
    type Target = crate::FieldReader<bool, LRW_CS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "EtherCAT read/write command support (BRW, APRW, FPRW)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RW_CS` reader - EtherCAT read/write command support (BRW, APRW, FPRW)"]
pub struct RW_CS_R(crate::FieldReader<bool, RW_CS_A>);
impl RW_CS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RW_CS_R(crate::FieldReader::new(bits))
    }
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
        **self == RW_CS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RW_CS_A::VALUE2
    }
}
impl core::ops::Deref for RW_CS_R {
    type Target = crate::FieldReader<bool, RW_CS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fixed FMMU/SyncManager configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FX_CONF` reader - Fixed FMMU/SyncManager configuration"]
pub struct FX_CONF_R(crate::FieldReader<bool, FX_CONF_A>);
impl FX_CONF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FX_CONF_R(crate::FieldReader::new(bits))
    }
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
        **self == FX_CONF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FX_CONF_A::VALUE2
    }
}
impl core::ops::Deref for FX_CONF_R {
    type Target = crate::FieldReader<bool, FX_CONF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "ESC Features Supported\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feature](index.html) module"]
pub struct FEATURE_SPEC;
impl crate::RegisterSpec for FEATURE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [feature::R](R) reader structure"]
impl crate::Readable for FEATURE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FEATURE to value 0x01cc"]
impl crate::Resettable for FEATURE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01cc
    }
}
