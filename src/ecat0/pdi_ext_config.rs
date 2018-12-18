#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::PDI_EXT_CONFIG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `R_Pref`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_PREFR {
    #[doc = "4 cycles"]
    VALUE1,
    #[doc = "1 cycle (typical)"]
    VALUE2,
    #[doc = "2 cycles"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl R_PREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R_PREFR::VALUE1 => 0,
            R_PREFR::VALUE2 => 1,
            R_PREFR::VALUE3 => 2,
            R_PREFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R_PREFR {
        match value {
            0 => R_PREFR::VALUE1,
            1 => R_PREFR::VALUE2,
            2 => R_PREFR::VALUE3,
            i => R_PREFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == R_PREFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == R_PREFR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == R_PREFR::VALUE3
    }
}
#[doc = "Possible values of the field `SUB_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUB_TYPER {
    #[doc = "AXI3"]
    VALUE1,
    #[doc = "AXI4"]
    VALUE2,
    #[doc = "AXI4 Lite"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SUB_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUB_TYPER::VALUE1 => 0,
            SUB_TYPER::VALUE2 => 1,
            SUB_TYPER::VALUE3 => 2,
            SUB_TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUB_TYPER {
        match value {
            0 => SUB_TYPER::VALUE1,
            1 => SUB_TYPER::VALUE2,
            2 => SUB_TYPER::VALUE3,
            i => SUB_TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SUB_TYPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SUB_TYPER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SUB_TYPER::VALUE3
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:1 - Read Prefetch Size"]
    #[inline]
    pub fn r_pref(&self) -> R_PREFR {
        R_PREFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:10 - On-chip Sub Type for AXI"]
    #[inline]
    pub fn sub_type(&self) -> SUB_TYPER {
        SUB_TYPER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
