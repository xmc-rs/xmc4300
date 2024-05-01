#[doc = "Register `CAPABILITIES_HI` reader"]
pub type R = crate::R<CapabilitiesHiSpec>;
#[doc = "SDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr50Support {
    #[doc = "0: SDR50 is not supported"]
    Value1 = 0,
}
impl From<Sdr50Support> for bool {
    #[inline(always)]
    fn from(variant: Sdr50Support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR50_SUPPORT` reader - SDR50 Support"]
pub type Sdr50SupportR = crate::BitReader<Sdr50Support>;
impl Sdr50SupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sdr50Support> {
        match self.bits {
            false => Some(Sdr50Support::Value1),
            _ => None,
        }
    }
    #[doc = "SDR50 is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sdr50Support::Value1
    }
}
#[doc = "SDR104 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr104Support {
    #[doc = "0: SDR104 is not supported"]
    Value1 = 0,
}
impl From<Sdr104Support> for bool {
    #[inline(always)]
    fn from(variant: Sdr104Support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR104_SUPPORT` reader - SDR104 Support"]
pub type Sdr104SupportR = crate::BitReader<Sdr104Support>;
impl Sdr104SupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sdr104Support> {
        match self.bits {
            false => Some(Sdr104Support::Value1),
            _ => None,
        }
    }
    #[doc = "SDR104 is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sdr104Support::Value1
    }
}
#[doc = "DDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddr50Support {
    #[doc = "0: DDR50 is not supported"]
    Value1 = 0,
}
impl From<Ddr50Support> for bool {
    #[inline(always)]
    fn from(variant: Ddr50Support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR50_SUPPORT` reader - DDR50 Support"]
pub type Ddr50SupportR = crate::BitReader<Ddr50Support>;
impl Ddr50SupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ddr50Support> {
        match self.bits {
            false => Some(Ddr50Support::Value1),
            _ => None,
        }
    }
    #[doc = "DDR50 is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ddr50Support::Value1
    }
}
#[doc = "Driver Type A Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrvASupport {
    #[doc = "0: Driver Type A is not supported"]
    Value1 = 0,
}
impl From<DrvASupport> for bool {
    #[inline(always)]
    fn from(variant: DrvASupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRV_A_SUPPORT` reader - Driver Type A Support"]
pub type DrvASupportR = crate::BitReader<DrvASupport>;
impl DrvASupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DrvASupport> {
        match self.bits {
            false => Some(DrvASupport::Value1),
            _ => None,
        }
    }
    #[doc = "Driver Type A is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DrvASupport::Value1
    }
}
#[doc = "Driver Type C Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrvCSupport {
    #[doc = "0: Driver Type C is not supported"]
    Value1 = 0,
}
impl From<DrvCSupport> for bool {
    #[inline(always)]
    fn from(variant: DrvCSupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRV_C_SUPPORT` reader - Driver Type C Support"]
pub type DrvCSupportR = crate::BitReader<DrvCSupport>;
impl DrvCSupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DrvCSupport> {
        match self.bits {
            false => Some(DrvCSupport::Value1),
            _ => None,
        }
    }
    #[doc = "Driver Type C is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DrvCSupport::Value1
    }
}
#[doc = "Driver Type D Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrvDSupport {
    #[doc = "0: Driver Type D is not supported"]
    Value1 = 0,
}
impl From<DrvDSupport> for bool {
    #[inline(always)]
    fn from(variant: DrvDSupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRV_D_SUPPORT` reader - Driver Type D Support"]
pub type DrvDSupportR = crate::BitReader<DrvDSupport>;
impl DrvDSupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DrvDSupport> {
        match self.bits {
            false => Some(DrvDSupport::Value1),
            _ => None,
        }
    }
    #[doc = "Driver Type D is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DrvDSupport::Value1
    }
}
#[doc = "Timer count for Re-Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TimCntRetune {
    #[doc = "0: Get information via other source"]
    Value1 = 0,
}
impl From<TimCntRetune> for u8 {
    #[inline(always)]
    fn from(variant: TimCntRetune) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TimCntRetune {
    type Ux = u8;
}
impl crate::IsEnum for TimCntRetune {}
#[doc = "Field `TIM_CNT_RETUNE` reader - Timer count for Re-Tuning"]
pub type TimCntRetuneR = crate::FieldReader<TimCntRetune>;
impl TimCntRetuneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TimCntRetune> {
        match self.bits {
            0 => Some(TimCntRetune::Value1),
            _ => None,
        }
    }
    #[doc = "Get information via other source"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TimCntRetune::Value1
    }
}
#[doc = "Use Tuning for SDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseTuningSdr50 {
    #[doc = "0: SDR50 does not require tuning"]
    Value1 = 0,
}
impl From<UseTuningSdr50> for bool {
    #[inline(always)]
    fn from(variant: UseTuningSdr50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE_TUNING_SDR50` reader - Use Tuning for SDR50"]
pub type UseTuningSdr50R = crate::BitReader<UseTuningSdr50>;
impl UseTuningSdr50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UseTuningSdr50> {
        match self.bits {
            false => Some(UseTuningSdr50::Value1),
            _ => None,
        }
    }
    #[doc = "SDR50 does not require tuning"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UseTuningSdr50::Value1
    }
}
#[doc = "Re-tuning modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReTuningModes {
    #[doc = "0: Mode 1"]
    Value1 = 0,
}
impl From<ReTuningModes> for u8 {
    #[inline(always)]
    fn from(variant: ReTuningModes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ReTuningModes {
    type Ux = u8;
}
impl crate::IsEnum for ReTuningModes {}
#[doc = "Field `RE_TUNING_MODES` reader - Re-tuning modes"]
pub type ReTuningModesR = crate::FieldReader<ReTuningModes>;
impl ReTuningModesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ReTuningModes> {
        match self.bits {
            0 => Some(ReTuningModes::Value1),
            _ => None,
        }
    }
    #[doc = "Mode 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ReTuningModes::Value1
    }
}
#[doc = "Clock Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMult {
    #[doc = "0: Clock Multiplier not supported"]
    Value1 = 0,
}
impl From<ClkMult> for u8 {
    #[inline(always)]
    fn from(variant: ClkMult) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkMult {
    type Ux = u8;
}
impl crate::IsEnum for ClkMult {}
#[doc = "Field `CLK_MULT` reader - Clock Multiplier"]
pub type ClkMultR = crate::FieldReader<ClkMult>;
impl ClkMultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMult> {
        match self.bits {
            0 => Some(ClkMult::Value1),
            _ => None,
        }
    }
    #[doc = "Clock Multiplier not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ClkMult::Value1
    }
}
impl R {
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> Sdr50SupportR {
        Sdr50SupportR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> Sdr104SupportR {
        Sdr104SupportR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> Ddr50SupportR {
        Ddr50SupportR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support"]
    #[inline(always)]
    pub fn drv_a_support(&self) -> DrvASupportR {
        DrvASupportR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline(always)]
    pub fn drv_c_support(&self) -> DrvCSupportR {
        DrvCSupportR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline(always)]
    pub fn drv_d_support(&self) -> DrvDSupportR {
        DrvDSupportR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timer count for Re-Tuning"]
    #[inline(always)]
    pub fn tim_cnt_retune(&self) -> TimCntRetuneR {
        TimCntRetuneR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn use_tuning_sdr50(&self) -> UseTuningSdr50R {
        UseTuningSdr50R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Re-tuning modes"]
    #[inline(always)]
    pub fn re_tuning_modes(&self) -> ReTuningModesR {
        ReTuningModesR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clk_mult(&self) -> ClkMultR {
        ClkMultR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Capabilities Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities_hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapabilitiesHiSpec;
impl crate::RegisterSpec for CapabilitiesHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities_hi::R`](R) reader structure"]
impl crate::Readable for CapabilitiesHiSpec {}
#[doc = "`reset()` method sets CAPABILITIES_HI to value 0x0300_0000"]
impl crate::Resettable for CapabilitiesHiSpec {
    const RESET_VALUE: u32 = 0x0300_0000;
}
