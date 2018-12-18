#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::EVENT_MASK {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `DC_LE_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_LE_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl DC_LE_MASKR {
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
            DC_LE_MASKR::VALUE1 => false,
            DC_LE_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DC_LE_MASKR {
        match value {
            false => DC_LE_MASKR::VALUE1,
            true => DC_LE_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DC_LE_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DC_LE_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `DL_SE_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DL_SE_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl DL_SE_MASKR {
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
            DL_SE_MASKR::VALUE1 => false,
            DL_SE_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DL_SE_MASKR {
        match value {
            false => DL_SE_MASKR::VALUE1,
            true => DL_SE_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DL_SE_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DL_SE_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `AL_SE_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AL_SE_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl AL_SE_MASKR {
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
            AL_SE_MASKR::VALUE1 => false,
            AL_SE_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AL_SE_MASKR {
        match value {
            false => AL_SE_MASKR::VALUE1,
            true => AL_SE_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AL_SE_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AL_SE_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_0_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_0_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl MIR_0_MASKR {
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
            MIR_0_MASKR::VALUE1 => false,
            MIR_0_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_0_MASKR {
        match value {
            false => MIR_0_MASKR::VALUE1,
            true => MIR_0_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_0_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_0_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_1_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_1_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl MIR_1_MASKR {
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
            MIR_1_MASKR::VALUE1 => false,
            MIR_1_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_1_MASKR {
        match value {
            false => MIR_1_MASKR::VALUE1,
            true => MIR_1_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_1_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_1_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_2_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_2_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl MIR_2_MASKR {
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
            MIR_2_MASKR::VALUE1 => false,
            MIR_2_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_2_MASKR {
        match value {
            false => MIR_2_MASKR::VALUE1,
            true => MIR_2_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_2_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_2_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_3_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_3_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl MIR_3_MASKR {
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
            MIR_3_MASKR::VALUE1 => false,
            MIR_3_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_3_MASKR {
        match value {
            false => MIR_3_MASKR::VALUE1,
            true => MIR_3_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_3_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_3_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_4_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_4_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl MIR_4_MASKR {
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
            MIR_4_MASKR::VALUE1 => false,
            MIR_4_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_4_MASKR {
        match value {
            false => MIR_4_MASKR::VALUE1,
            true => MIR_4_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_4_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_4_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_5_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_5_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl MIR_5_MASKR {
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
            MIR_5_MASKR::VALUE1 => false,
            MIR_5_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_5_MASKR {
        match value {
            false => MIR_5_MASKR::VALUE1,
            true => MIR_5_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_5_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_5_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_6_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_6_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl MIR_6_MASKR {
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
            MIR_6_MASKR::VALUE1 => false,
            MIR_6_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_6_MASKR {
        match value {
            false => MIR_6_MASKR::VALUE1,
            true => MIR_6_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_6_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_6_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_7_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_7_MASKR {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    VALUE2,
}
impl MIR_7_MASKR {
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
            MIR_7_MASKR::VALUE1 => false,
            MIR_7_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_7_MASKR {
        match value {
            false => MIR_7_MASKR::VALUE1,
            true => MIR_7_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_7_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_7_MASKR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - DC Latch event"]
    #[inline]
    pub fn dc_le_mask(&self) -> DC_LE_MASKR {
        DC_LE_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - DL Status event"]
    #[inline]
    pub fn dl_se_mask(&self) -> DL_SE_MASKR {
        DL_SE_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - AL Status event"]
    #[inline]
    pub fn al_se_mask(&self) -> AL_SE_MASKR {
        AL_SE_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_0_mask(&self) -> MIR_0_MASKR {
        MIR_0_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_1_mask(&self) -> MIR_1_MASKR {
        MIR_1_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_2_mask(&self) -> MIR_2_MASKR {
        MIR_2_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_3_mask(&self) -> MIR_3_MASKR {
        MIR_3_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_4_mask(&self) -> MIR_4_MASKR {
        MIR_4_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_5_mask(&self) -> MIR_5_MASKR {
        MIR_5_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_6_mask(&self) -> MIR_6_MASKR {
        MIR_6_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_7_mask(&self) -> MIR_7_MASKR {
        MIR_7_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
