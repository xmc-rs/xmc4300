#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAPABILITIES_HI {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SDR50_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR50_SUPPORTR {
    #[doc = "SDR50 is not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SDR50_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SDR50_SUPPORTR::VALUE1 => false,
            SDR50_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDR50_SUPPORTR {
        match value {
            false => SDR50_SUPPORTR::VALUE1,
            i => SDR50_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDR50_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `SDR104_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR104_SUPPORTR {
    #[doc = "SDR104 is not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SDR104_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SDR104_SUPPORTR::VALUE1 => false,
            SDR104_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDR104_SUPPORTR {
        match value {
            false => SDR104_SUPPORTR::VALUE1,
            i => SDR104_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDR104_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `DDR50_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR50_SUPPORTR {
    #[doc = "DDR50 is not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DDR50_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DDR50_SUPPORTR::VALUE1 => false,
            DDR50_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DDR50_SUPPORTR {
        match value {
            false => DDR50_SUPPORTR::VALUE1,
            i => DDR50_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DDR50_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `DRV_A_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRV_A_SUPPORTR {
    #[doc = "Driver Type A is not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DRV_A_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DRV_A_SUPPORTR::VALUE1 => false,
            DRV_A_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRV_A_SUPPORTR {
        match value {
            false => DRV_A_SUPPORTR::VALUE1,
            i => DRV_A_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DRV_A_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `DRV_C_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRV_C_SUPPORTR {
    #[doc = "Driver Type C is not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DRV_C_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DRV_C_SUPPORTR::VALUE1 => false,
            DRV_C_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRV_C_SUPPORTR {
        match value {
            false => DRV_C_SUPPORTR::VALUE1,
            i => DRV_C_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DRV_C_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `DRV_D_SUPPORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRV_D_SUPPORTR {
    #[doc = "Driver Type D is not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DRV_D_SUPPORTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DRV_D_SUPPORTR::VALUE1 => false,
            DRV_D_SUPPORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRV_D_SUPPORTR {
        match value {
            false => DRV_D_SUPPORTR::VALUE1,
            i => DRV_D_SUPPORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DRV_D_SUPPORTR::VALUE1
    }
}
#[doc = "Possible values of the field `TIM_CNT_RETUNE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM_CNT_RETUNER {
    #[doc = "Get information via other source"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIM_CNT_RETUNER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIM_CNT_RETUNER::VALUE1 => 0,
            TIM_CNT_RETUNER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIM_CNT_RETUNER {
        match value {
            0 => TIM_CNT_RETUNER::VALUE1,
            i => TIM_CNT_RETUNER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TIM_CNT_RETUNER::VALUE1
    }
}
#[doc = "Possible values of the field `USE_TUNING_SDR50`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USE_TUNING_SDR50R {
    #[doc = "SDR50 does not require tuning"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl USE_TUNING_SDR50R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USE_TUNING_SDR50R::VALUE1 => false,
            USE_TUNING_SDR50R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USE_TUNING_SDR50R {
        match value {
            false => USE_TUNING_SDR50R::VALUE1,
            i => USE_TUNING_SDR50R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USE_TUNING_SDR50R::VALUE1
    }
}
#[doc = "Possible values of the field `RE_TUNING_MODES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_TUNING_MODESR {
    #[doc = "Mode 1"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RE_TUNING_MODESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RE_TUNING_MODESR::VALUE1 => 0,
            RE_TUNING_MODESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RE_TUNING_MODESR {
        match value {
            0 => RE_TUNING_MODESR::VALUE1,
            i => RE_TUNING_MODESR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RE_TUNING_MODESR::VALUE1
    }
}
#[doc = "Possible values of the field `CLK_MULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_MULTR {
    #[doc = "Clock Multiplier not supported"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_MULTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_MULTR::VALUE1 => 0,
            CLK_MULTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_MULTR {
        match value {
            0 => CLK_MULTR::VALUE1,
            i => CLK_MULTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLK_MULTR::VALUE1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline]
    pub fn sdr50_support(&self) -> SDR50_SUPPORTR {
        SDR50_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline]
    pub fn sdr104_support(&self) -> SDR104_SUPPORTR {
        SDR104_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline]
    pub fn ddr50_support(&self) -> DDR50_SUPPORTR {
        DDR50_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Driver Type A Support"]
    #[inline]
    pub fn drv_a_support(&self) -> DRV_A_SUPPORTR {
        DRV_A_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline]
    pub fn drv_c_support(&self) -> DRV_C_SUPPORTR {
        DRV_C_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline]
    pub fn drv_d_support(&self) -> DRV_D_SUPPORTR {
        DRV_D_SUPPORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Timer count for Re-Tuning"]
    #[inline]
    pub fn tim_cnt_retune(&self) -> TIM_CNT_RETUNER {
        TIM_CNT_RETUNER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline]
    pub fn use_tuning_sdr50(&self) -> USE_TUNING_SDR50R {
        USE_TUNING_SDR50R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Re-tuning modes"]
    #[inline]
    pub fn re_tuning_modes(&self) -> RE_TUNING_MODESR {
        RE_TUNING_MODESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline]
    pub fn clk_mult(&self) -> CLK_MULTR {
        CLK_MULTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
