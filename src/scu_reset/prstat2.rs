#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTAT2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `WDTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTRSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl WDTRSR {
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
            WDTRSR::CONST_0 => false,
            WDTRSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTRSR {
        match value {
            false => WDTRSR::CONST_0,
            true => WDTRSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == WDTRSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == WDTRSR::CONST_1
    }
}
#[doc = "Possible values of the field `ETH0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl ETH0RSR {
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
            ETH0RSR::CONST_0 => false,
            ETH0RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETH0RSR {
        match value {
            false => ETH0RSR::CONST_0,
            true => ETH0RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ETH0RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ETH0RSR::CONST_1
    }
}
#[doc = "Possible values of the field `DMA0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl DMA0RSR {
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
            DMA0RSR::CONST_0 => false,
            DMA0RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA0RSR {
        match value {
            false => DMA0RSR::CONST_0,
            true => DMA0RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == DMA0RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == DMA0RSR::CONST_1
    }
}
#[doc = "Possible values of the field `FCERS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCERSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl FCERSR {
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
            FCERSR::CONST_0 => false,
            FCERSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCERSR {
        match value {
            false => FCERSR::CONST_0,
            true => FCERSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == FCERSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == FCERSR::CONST_1
    }
}
#[doc = "Possible values of the field `USBRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl USBRSR {
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
            USBRSR::CONST_0 => false,
            USBRSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBRSR {
        match value {
            false => USBRSR::CONST_0,
            true => USBRSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USBRSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USBRSR::CONST_1
    }
}
#[doc = "Possible values of the field `ECAT0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RSR {
    #[doc = "Reset de-asserted"]
    CONST_0,
    #[doc = "Reset asserted"]
    CONST_1,
}
impl ECAT0RSR {
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
            ECAT0RSR::CONST_0 => false,
            ECAT0RSR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECAT0RSR {
        match value {
            false => ECAT0RSR::CONST_0,
            true => ECAT0RSR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ECAT0RSR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ECAT0RSR::CONST_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - WDT Reset Status"]
    #[inline]
    pub fn wdtrs(&self) -> WDTRSR {
        WDTRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - ETH0 Reset Status"]
    #[inline]
    pub fn eth0rs(&self) -> ETH0RSR {
        ETH0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DMA0 Reset Status"]
    #[inline]
    pub fn dma0rs(&self) -> DMA0RSR {
        DMA0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - FCE Reset Status"]
    #[inline]
    pub fn fcers(&self) -> FCERSR {
        FCERSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - USB Reset Status"]
    #[inline]
    pub fn usbrs(&self) -> USBRSR {
        USBRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - ECAT0 Reset Status"]
    #[inline]
    pub fn ecat0rs(&self) -> ECAT0RSR {
        ECAT0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
