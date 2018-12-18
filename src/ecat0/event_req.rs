#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::EVENT_REQ {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `DC_LE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_LER {
    #[doc = "No change on DC Latch Inputs"]
    VALUE1,
    #[doc = "At least one change on DC Latch Inputs"]
    VALUE2,
}
impl DC_LER {
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
            DC_LER::VALUE1 => false,
            DC_LER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DC_LER {
        match value {
            false => DC_LER::VALUE1,
            true => DC_LER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DC_LER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DC_LER::VALUE2
    }
}
#[doc = "Possible values of the field `DL_SE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DL_SER {
    #[doc = "No change in DL Status"]
    VALUE1,
    #[doc = "DL Status change"]
    VALUE2,
}
impl DL_SER {
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
            DL_SER::VALUE1 => false,
            DL_SER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DL_SER {
        match value {
            false => DL_SER::VALUE1,
            true => DL_SER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DL_SER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DL_SER::VALUE2
    }
}
#[doc = "Possible values of the field `AL_SE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AL_SER {
    #[doc = "No change in AL Status"]
    VALUE1,
    #[doc = "AL Status change"]
    VALUE2,
}
impl AL_SER {
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
            AL_SER::VALUE1 => false,
            AL_SER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AL_SER {
        match value {
            false => AL_SER::VALUE1,
            true => AL_SER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AL_SER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AL_SER::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_0R {
    #[doc = "No Sync Channel 0 event"]
    VALUE1,
    #[doc = "Sync Channel 0 event pending"]
    VALUE2,
}
impl MIR_0R {
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
            MIR_0R::VALUE1 => false,
            MIR_0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_0R {
        match value {
            false => MIR_0R::VALUE1,
            true => MIR_0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_0R::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_1R {
    #[doc = "No Sync Channel 1 event"]
    VALUE1,
    #[doc = "Sync Channel 1 event pending"]
    VALUE2,
}
impl MIR_1R {
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
            MIR_1R::VALUE1 => false,
            MIR_1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_1R {
        match value {
            false => MIR_1R::VALUE1,
            true => MIR_1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_1R::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_2R {
    #[doc = "No Sync Channel 2 event"]
    VALUE1,
    #[doc = "Sync Channel 2 event pending"]
    VALUE2,
}
impl MIR_2R {
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
            MIR_2R::VALUE1 => false,
            MIR_2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_2R {
        match value {
            false => MIR_2R::VALUE1,
            true => MIR_2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_2R::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_3R {
    #[doc = "No Sync Channel 3 event"]
    VALUE1,
    #[doc = "Sync Channel 3event pending"]
    VALUE2,
}
impl MIR_3R {
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
            MIR_3R::VALUE1 => false,
            MIR_3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_3R {
        match value {
            false => MIR_3R::VALUE1,
            true => MIR_3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_3R::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_4R {
    #[doc = "No Sync Channel 4 event"]
    VALUE1,
    #[doc = "Sync Channel 4 event pending"]
    VALUE2,
}
impl MIR_4R {
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
            MIR_4R::VALUE1 => false,
            MIR_4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_4R {
        match value {
            false => MIR_4R::VALUE1,
            true => MIR_4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_4R::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_5R {
    #[doc = "No Sync Channel 5 event"]
    VALUE1,
    #[doc = "Sync Channel 5 event pending"]
    VALUE2,
}
impl MIR_5R {
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
            MIR_5R::VALUE1 => false,
            MIR_5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_5R {
        match value {
            false => MIR_5R::VALUE1,
            true => MIR_5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_5R::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_6R {
    #[doc = "No Sync Channel 6 event"]
    VALUE1,
    #[doc = "Sync Channel 6 event pending"]
    VALUE2,
}
impl MIR_6R {
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
            MIR_6R::VALUE1 => false,
            MIR_6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_6R {
        match value {
            false => MIR_6R::VALUE1,
            true => MIR_6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_6R::VALUE2
    }
}
#[doc = "Possible values of the field `MIR_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_7R {
    #[doc = "No Sync Channel 7 event"]
    VALUE1,
    #[doc = "Sync Channel 7 event pending"]
    VALUE2,
}
impl MIR_7R {
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
            MIR_7R::VALUE1 => false,
            MIR_7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIR_7R {
        match value {
            false => MIR_7R::VALUE1,
            true => MIR_7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIR_7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIR_7R::VALUE2
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
    pub fn dc_le(&self) -> DC_LER {
        DC_LER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - DL Status event"]
    #[inline]
    pub fn dl_se(&self) -> DL_SER {
        DL_SER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - AL Status event"]
    #[inline]
    pub fn al_se(&self) -> AL_SER {
        AL_SER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_0(&self) -> MIR_0R {
        MIR_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_1(&self) -> MIR_1R {
        MIR_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_2(&self) -> MIR_2R {
        MIR_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_3(&self) -> MIR_3R {
        MIR_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_4(&self) -> MIR_4R {
        MIR_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_5(&self) -> MIR_5R {
        MIR_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_6(&self) -> MIR_6R {
        MIR_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Mirrors values of each SyncManager Status"]
    #[inline]
    pub fn mir_7(&self) -> MIR_7R {
        MIR_7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
