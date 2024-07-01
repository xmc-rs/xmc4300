#[doc = "Register `CAPABILITIES_HI` reader"]
pub type R = crate::R<CAPABILITIES_HI_SPEC>;
#[doc = "SDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDR50_SUPPORT_A {
    #[doc = "0: SDR50 is not supported"]
    VALUE1 = 0,
}
impl From<SDR50_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: SDR50_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR50_SUPPORT` reader - SDR50 Support"]
pub type SDR50_SUPPORT_R = crate::BitReader<SDR50_SUPPORT_A>;
impl SDR50_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDR50_SUPPORT_A> {
        match self.bits {
            false => Some(SDR50_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "SDR50 is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDR50_SUPPORT_A::VALUE1
    }
}
#[doc = "SDR104 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDR104_SUPPORT_A {
    #[doc = "0: SDR104 is not supported"]
    VALUE1 = 0,
}
impl From<SDR104_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: SDR104_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR104_SUPPORT` reader - SDR104 Support"]
pub type SDR104_SUPPORT_R = crate::BitReader<SDR104_SUPPORT_A>;
impl SDR104_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDR104_SUPPORT_A> {
        match self.bits {
            false => Some(SDR104_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "SDR104 is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDR104_SUPPORT_A::VALUE1
    }
}
#[doc = "DDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDR50_SUPPORT_A {
    #[doc = "0: DDR50 is not supported"]
    VALUE1 = 0,
}
impl From<DDR50_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: DDR50_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR50_SUPPORT` reader - DDR50 Support"]
pub type DDR50_SUPPORT_R = crate::BitReader<DDR50_SUPPORT_A>;
impl DDR50_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DDR50_SUPPORT_A> {
        match self.bits {
            false => Some(DDR50_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "DDR50 is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DDR50_SUPPORT_A::VALUE1
    }
}
#[doc = "Driver Type A Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRV_A_SUPPORT_A {
    #[doc = "0: Driver Type A is not supported"]
    VALUE1 = 0,
}
impl From<DRV_A_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: DRV_A_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRV_A_SUPPORT` reader - Driver Type A Support"]
pub type DRV_A_SUPPORT_R = crate::BitReader<DRV_A_SUPPORT_A>;
impl DRV_A_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DRV_A_SUPPORT_A> {
        match self.bits {
            false => Some(DRV_A_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Driver Type A is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DRV_A_SUPPORT_A::VALUE1
    }
}
#[doc = "Driver Type C Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRV_C_SUPPORT_A {
    #[doc = "0: Driver Type C is not supported"]
    VALUE1 = 0,
}
impl From<DRV_C_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: DRV_C_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRV_C_SUPPORT` reader - Driver Type C Support"]
pub type DRV_C_SUPPORT_R = crate::BitReader<DRV_C_SUPPORT_A>;
impl DRV_C_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DRV_C_SUPPORT_A> {
        match self.bits {
            false => Some(DRV_C_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Driver Type C is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DRV_C_SUPPORT_A::VALUE1
    }
}
#[doc = "Driver Type D Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRV_D_SUPPORT_A {
    #[doc = "0: Driver Type D is not supported"]
    VALUE1 = 0,
}
impl From<DRV_D_SUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: DRV_D_SUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRV_D_SUPPORT` reader - Driver Type D Support"]
pub type DRV_D_SUPPORT_R = crate::BitReader<DRV_D_SUPPORT_A>;
impl DRV_D_SUPPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DRV_D_SUPPORT_A> {
        match self.bits {
            false => Some(DRV_D_SUPPORT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Driver Type D is not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DRV_D_SUPPORT_A::VALUE1
    }
}
#[doc = "Timer count for Re-Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIM_CNT_RETUNE_A {
    #[doc = "0: Get information via other source"]
    VALUE1 = 0,
}
impl From<TIM_CNT_RETUNE_A> for u8 {
    #[inline(always)]
    fn from(variant: TIM_CNT_RETUNE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIM_CNT_RETUNE_A {
    type Ux = u8;
}
impl crate::IsEnum for TIM_CNT_RETUNE_A {}
#[doc = "Field `TIM_CNT_RETUNE` reader - Timer count for Re-Tuning"]
pub type TIM_CNT_RETUNE_R = crate::FieldReader<TIM_CNT_RETUNE_A>;
impl TIM_CNT_RETUNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM_CNT_RETUNE_A> {
        match self.bits {
            0 => Some(TIM_CNT_RETUNE_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Get information via other source"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIM_CNT_RETUNE_A::VALUE1
    }
}
#[doc = "Use Tuning for SDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USE_TUNING_SDR50_A {
    #[doc = "0: SDR50 does not require tuning"]
    VALUE1 = 0,
}
impl From<USE_TUNING_SDR50_A> for bool {
    #[inline(always)]
    fn from(variant: USE_TUNING_SDR50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE_TUNING_SDR50` reader - Use Tuning for SDR50"]
pub type USE_TUNING_SDR50_R = crate::BitReader<USE_TUNING_SDR50_A>;
impl USE_TUNING_SDR50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USE_TUNING_SDR50_A> {
        match self.bits {
            false => Some(USE_TUNING_SDR50_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "SDR50 does not require tuning"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USE_TUNING_SDR50_A::VALUE1
    }
}
#[doc = "Re-tuning modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RE_TUNING_MODES_A {
    #[doc = "0: Mode 1"]
    VALUE1 = 0,
}
impl From<RE_TUNING_MODES_A> for u8 {
    #[inline(always)]
    fn from(variant: RE_TUNING_MODES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RE_TUNING_MODES_A {
    type Ux = u8;
}
impl crate::IsEnum for RE_TUNING_MODES_A {}
#[doc = "Field `RE_TUNING_MODES` reader - Re-tuning modes"]
pub type RE_TUNING_MODES_R = crate::FieldReader<RE_TUNING_MODES_A>;
impl RE_TUNING_MODES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RE_TUNING_MODES_A> {
        match self.bits {
            0 => Some(RE_TUNING_MODES_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Mode 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RE_TUNING_MODES_A::VALUE1
    }
}
#[doc = "Clock Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_MULT_A {
    #[doc = "0: Clock Multiplier not supported"]
    VALUE1 = 0,
}
impl From<CLK_MULT_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_MULT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK_MULT_A {
    type Ux = u8;
}
impl crate::IsEnum for CLK_MULT_A {}
#[doc = "Field `CLK_MULT` reader - Clock Multiplier"]
pub type CLK_MULT_R = crate::FieldReader<CLK_MULT_A>;
impl CLK_MULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLK_MULT_A> {
        match self.bits {
            0 => Some(CLK_MULT_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Clock Multiplier not supported"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLK_MULT_A::VALUE1
    }
}
impl R {
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> SDR50_SUPPORT_R {
        SDR50_SUPPORT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> SDR104_SUPPORT_R {
        SDR104_SUPPORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> DDR50_SUPPORT_R {
        DDR50_SUPPORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support"]
    #[inline(always)]
    pub fn drv_a_support(&self) -> DRV_A_SUPPORT_R {
        DRV_A_SUPPORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline(always)]
    pub fn drv_c_support(&self) -> DRV_C_SUPPORT_R {
        DRV_C_SUPPORT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline(always)]
    pub fn drv_d_support(&self) -> DRV_D_SUPPORT_R {
        DRV_D_SUPPORT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timer count for Re-Tuning"]
    #[inline(always)]
    pub fn tim_cnt_retune(&self) -> TIM_CNT_RETUNE_R {
        TIM_CNT_RETUNE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn use_tuning_sdr50(&self) -> USE_TUNING_SDR50_R {
        USE_TUNING_SDR50_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Re-tuning modes"]
    #[inline(always)]
    pub fn re_tuning_modes(&self) -> RE_TUNING_MODES_R {
        RE_TUNING_MODES_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clk_mult(&self) -> CLK_MULT_R {
        CLK_MULT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Capabilities Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities_hi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPABILITIES_HI_SPEC;
impl crate::RegisterSpec for CAPABILITIES_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities_hi::R`](R) reader structure"]
impl crate::Readable for CAPABILITIES_HI_SPEC {}
#[doc = "`reset()` method sets CAPABILITIES_HI to value 0x0300_0000"]
impl crate::Resettable for CAPABILITIES_HI_SPEC {
    const RESET_VALUE: u32 = 0x0300_0000;
}
