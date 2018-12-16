#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::PDI_CONFIG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `BUS_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_CLKR {
    #[doc = "asyncronous"]
    VALUE1,
    #[doc = "values 1-31 is used for synchronous multiplication factor (N*25Mhz)"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BUS_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BUS_CLKR::VALUE1 => 0,
            BUS_CLKR::VALUE2 => 1,
            BUS_CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BUS_CLKR {
        match value {
            0 => BUS_CLKR::VALUE1,
            1 => BUS_CLKR::VALUE2,
            i => BUS_CLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUS_CLKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUS_CLKR::VALUE2
    }
}
#[doc = "Possible values of the field `OC_BUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC_BUSR {
    #[doc = "Altera Avalon"]
    VALUE1,
    #[doc = "AXI"]
    VALUE2,
    #[doc = "Xilinx PLB v4.6"]
    VALUE3,
    #[doc = "Xilinx OPB"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OC_BUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OC_BUSR::VALUE1 => 0,
            OC_BUSR::VALUE2 => 1,
            OC_BUSR::VALUE3 => 2,
            OC_BUSR::VALUE4 => 4,
            OC_BUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OC_BUSR {
        match value {
            0 => OC_BUSR::VALUE1,
            1 => OC_BUSR::VALUE2,
            2 => OC_BUSR::VALUE3,
            4 => OC_BUSR::VALUE4,
            i => OC_BUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OC_BUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OC_BUSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == OC_BUSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == OC_BUSR::VALUE4
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:4 - On-chip bus clock"]
    #[inline]
    pub fn bus_clk(&self) -> BUS_CLKR {
        BUS_CLKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 5:7 - On-chip bus"]
    #[inline]
    pub fn oc_bus(&self) -> OC_BUSR {
        OC_BUSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
