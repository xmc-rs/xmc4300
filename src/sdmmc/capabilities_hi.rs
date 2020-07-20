#[doc = "Reader of register CAPABILITIES_HI"]
pub type R = crate::R<u32, super::CAPABILITIES_HI>;
#[doc = "SDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SDR50_SUPPORT`"]
pub type SDR50_SUPPORT_R = crate::R<bool, SDR50_SUPPORT_A>;
impl SDR50_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SDR50_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SDR50_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDR50_SUPPORT_A::VALUE1
    }
}
#[doc = "SDR104 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SDR104_SUPPORT`"]
pub type SDR104_SUPPORT_R = crate::R<bool, SDR104_SUPPORT_A>;
impl SDR104_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SDR104_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SDR104_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDR104_SUPPORT_A::VALUE1
    }
}
#[doc = "DDR50 Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `DDR50_SUPPORT`"]
pub type DDR50_SUPPORT_R = crate::R<bool, DDR50_SUPPORT_A>;
impl DDR50_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DDR50_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DDR50_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DDR50_SUPPORT_A::VALUE1
    }
}
#[doc = "Driver Type A Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `DRV_A_SUPPORT`"]
pub type DRV_A_SUPPORT_R = crate::R<bool, DRV_A_SUPPORT_A>;
impl DRV_A_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DRV_A_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DRV_A_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DRV_A_SUPPORT_A::VALUE1
    }
}
#[doc = "Driver Type C Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `DRV_C_SUPPORT`"]
pub type DRV_C_SUPPORT_R = crate::R<bool, DRV_C_SUPPORT_A>;
impl DRV_C_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DRV_C_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DRV_C_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DRV_C_SUPPORT_A::VALUE1
    }
}
#[doc = "Driver Type D Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `DRV_D_SUPPORT`"]
pub type DRV_D_SUPPORT_R = crate::R<bool, DRV_D_SUPPORT_A>;
impl DRV_D_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DRV_D_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DRV_D_SUPPORT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DRV_D_SUPPORT_A::VALUE1
    }
}
#[doc = "Timer count for Re-Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `TIM_CNT_RETUNE`"]
pub type TIM_CNT_RETUNE_R = crate::R<u8, TIM_CNT_RETUNE_A>;
impl TIM_CNT_RETUNE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIM_CNT_RETUNE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIM_CNT_RETUNE_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIM_CNT_RETUNE_A::VALUE1
    }
}
#[doc = "Use Tuning for SDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `USE_TUNING_SDR50`"]
pub type USE_TUNING_SDR50_R = crate::R<bool, USE_TUNING_SDR50_A>;
impl USE_TUNING_SDR50_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, USE_TUNING_SDR50_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(USE_TUNING_SDR50_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USE_TUNING_SDR50_A::VALUE1
    }
}
#[doc = "Re-tuning modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RE_TUNING_MODES`"]
pub type RE_TUNING_MODES_R = crate::R<u8, RE_TUNING_MODES_A>;
impl RE_TUNING_MODES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RE_TUNING_MODES_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RE_TUNING_MODES_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RE_TUNING_MODES_A::VALUE1
    }
}
#[doc = "Clock Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `CLK_MULT`"]
pub type CLK_MULT_R = crate::R<u8, CLK_MULT_A>;
impl CLK_MULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_MULT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_MULT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLK_MULT_A::VALUE1
    }
}
impl R {
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> SDR50_SUPPORT_R {
        SDR50_SUPPORT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> SDR104_SUPPORT_R {
        SDR104_SUPPORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> DDR50_SUPPORT_R {
        DDR50_SUPPORT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support"]
    #[inline(always)]
    pub fn drv_a_support(&self) -> DRV_A_SUPPORT_R {
        DRV_A_SUPPORT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline(always)]
    pub fn drv_c_support(&self) -> DRV_C_SUPPORT_R {
        DRV_C_SUPPORT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline(always)]
    pub fn drv_d_support(&self) -> DRV_D_SUPPORT_R {
        DRV_D_SUPPORT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Timer count for Re-Tuning"]
    #[inline(always)]
    pub fn tim_cnt_retune(&self) -> TIM_CNT_RETUNE_R {
        TIM_CNT_RETUNE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn use_tuning_sdr50(&self) -> USE_TUNING_SDR50_R {
        USE_TUNING_SDR50_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Re-tuning modes"]
    #[inline(always)]
    pub fn re_tuning_modes(&self) -> RE_TUNING_MODES_R {
        RE_TUNING_MODES_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clk_mult(&self) -> CLK_MULT_R {
        CLK_MULT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
