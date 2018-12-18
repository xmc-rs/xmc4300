#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PMUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUSR {
    #[doc = "Period match while counting up not detected"]
    VALUE1,
    #[doc = "Period match while counting up detected"]
    VALUE2,
}
impl PMUSR {
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
            PMUSR::VALUE1 => false,
            PMUSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMUSR {
        match value {
            false => PMUSR::VALUE1,
            true => PMUSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PMUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PMUSR::VALUE2
    }
}
#[doc = "Possible values of the field `OMDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OMDSR {
    #[doc = "One match while counting down not detected"]
    VALUE1,
    #[doc = "One match while counting down detected"]
    VALUE2,
}
impl OMDSR {
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
            OMDSR::VALUE1 => false,
            OMDSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OMDSR {
        match value {
            false => OMDSR::VALUE1,
            true => OMDSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OMDSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OMDSR::VALUE2
    }
}
#[doc = "Possible values of the field `CMU1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMU1SR {
    #[doc = "Compare match while counting up not detected"]
    VALUE1,
    #[doc = "Compare match while counting up detected"]
    VALUE2,
}
impl CMU1SR {
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
            CMU1SR::VALUE1 => false,
            CMU1SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMU1SR {
        match value {
            false => CMU1SR::VALUE1,
            true => CMU1SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMU1SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMU1SR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD1SR {
    #[doc = "Compare match while counting down not detected"]
    VALUE1,
    #[doc = "Compare match while counting down detected"]
    VALUE2,
}
impl CMD1SR {
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
            CMD1SR::VALUE1 => false,
            CMD1SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD1SR {
        match value {
            false => CMD1SR::VALUE1,
            true => CMD1SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD1SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD1SR::VALUE2
    }
}
#[doc = "Possible values of the field `CMU2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMU2SR {
    #[doc = "Compare match while counting up not detected"]
    VALUE1,
    #[doc = "Compare match while counting up detected"]
    VALUE2,
}
impl CMU2SR {
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
            CMU2SR::VALUE1 => false,
            CMU2SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMU2SR {
        match value {
            false => CMU2SR::VALUE1,
            true => CMU2SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMU2SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMU2SR::VALUE2
    }
}
#[doc = "Possible values of the field `CMD2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD2SR {
    #[doc = "Compare match while counting down not detected"]
    VALUE1,
    #[doc = "Compare match while counting down detected"]
    VALUE2,
}
impl CMD2SR {
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
            CMD2SR::VALUE1 => false,
            CMD2SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD2SR {
        match value {
            false => CMD2SR::VALUE1,
            true => CMD2SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD2SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD2SR::VALUE2
    }
}
#[doc = "Possible values of the field `E0AS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E0ASR {
    #[doc = "Event 0 not detected"]
    VALUE1,
    #[doc = "Event 0 detected"]
    VALUE2,
}
impl E0ASR {
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
            E0ASR::VALUE1 => false,
            E0ASR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E0ASR {
        match value {
            false => E0ASR::VALUE1,
            true => E0ASR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == E0ASR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == E0ASR::VALUE2
    }
}
#[doc = "Possible values of the field `E1AS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E1ASR {
    #[doc = "Event 1 not detected"]
    VALUE1,
    #[doc = "Event 1 detected"]
    VALUE2,
}
impl E1ASR {
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
            E1ASR::VALUE1 => false,
            E1ASR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E1ASR {
        match value {
            false => E1ASR::VALUE1,
            true => E1ASR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == E1ASR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == E1ASR::VALUE2
    }
}
#[doc = "Possible values of the field `E2AS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E2ASR {
    #[doc = "Event 2 not detected"]
    VALUE1,
    #[doc = "Event 2 detected"]
    VALUE2,
}
impl E2ASR {
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
            E2ASR::VALUE1 => false,
            E2ASR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E2ASR {
        match value {
            false => E2ASR::VALUE1,
            true => E2ASR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == E2ASR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == E2ASR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct TRPFR {
    bits: bool,
}
impl TRPFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Period Match while Counting Up"]
    #[inline]
    pub fn pmus(&self) -> PMUSR {
        PMUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - One Match while Counting Down"]
    #[inline]
    pub fn omds(&self) -> OMDSR {
        OMDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 1 Compare Match while Counting Up"]
    #[inline]
    pub fn cmu1s(&self) -> CMU1SR {
        CMU1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 1 Compare Match while Counting Down"]
    #[inline]
    pub fn cmd1s(&self) -> CMD1SR {
        CMD1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 2 Compare Match while Counting Up"]
    #[inline]
    pub fn cmu2s(&self) -> CMU2SR {
        CMU2SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 2 Compare Match while Counting Down"]
    #[inline]
    pub fn cmd2s(&self) -> CMD2SR {
        CMD2SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Event 0 Detection Status"]
    #[inline]
    pub fn e0as(&self) -> E0ASR {
        E0ASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Event 1 Detection Status"]
    #[inline]
    pub fn e1as(&self) -> E1ASR {
        E1ASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Event 2 Detection Status"]
    #[inline]
    pub fn e2as(&self) -> E2ASR {
        E2ASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Trap Flag Status"]
    #[inline]
    pub fn trpf(&self) -> TRPFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRPFR { bits }
    }
}
