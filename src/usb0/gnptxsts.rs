#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GNPTXSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `NPTxFSpcAvail`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NPTXFSPCAVAILR {
    #[doc = "Non-periodic TxFIFO is full"]
    VALUE1,
    #[doc = "1 word available"]
    VALUE2,
    #[doc = "2 words available"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl NPTXFSPCAVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            NPTXFSPCAVAILR::VALUE1 => 0,
            NPTXFSPCAVAILR::VALUE2 => 1,
            NPTXFSPCAVAILR::VALUE3 => 2,
            NPTXFSPCAVAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> NPTXFSPCAVAILR {
        match value {
            0 => NPTXFSPCAVAILR::VALUE1,
            1 => NPTXFSPCAVAILR::VALUE2,
            2 => NPTXFSPCAVAILR::VALUE3,
            i => NPTXFSPCAVAILR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NPTXFSPCAVAILR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NPTXFSPCAVAILR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == NPTXFSPCAVAILR::VALUE3
    }
}
#[doc = "Possible values of the field `NPTxQSpcAvail`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NPTXQSPCAVAILR {
    #[doc = "Non-periodic Transmit Request Queue is full"]
    VALUE1,
    #[doc = "1 location available"]
    VALUE2,
    #[doc = "2 locations available"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NPTXQSPCAVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NPTXQSPCAVAILR::VALUE1 => 0,
            NPTXQSPCAVAILR::VALUE2 => 1,
            NPTXQSPCAVAILR::VALUE3 => 2,
            NPTXQSPCAVAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NPTXQSPCAVAILR {
        match value {
            0 => NPTXQSPCAVAILR::VALUE1,
            1 => NPTXQSPCAVAILR::VALUE2,
            2 => NPTXQSPCAVAILR::VALUE3,
            i => NPTXQSPCAVAILR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NPTXQSPCAVAILR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NPTXQSPCAVAILR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == NPTXQSPCAVAILR::VALUE3
    }
}
#[doc = "Possible values of the field `NPTxQTop`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NPTXQTOPR {
    #[doc = "IN/OUT token"]
    VALUE1,
    #[doc = "Zero-length transmit packet (device IN/host OUT)"]
    VALUE2,
    #[doc = "Channel halt command"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NPTXQTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NPTXQTOPR::VALUE1 => 0,
            NPTXQTOPR::VALUE2 => 1,
            NPTXQTOPR::VALUE4 => 3,
            NPTXQTOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NPTXQTOPR {
        match value {
            0 => NPTXQTOPR::VALUE1,
            1 => NPTXQTOPR::VALUE2,
            3 => NPTXQTOPR::VALUE4,
            i => NPTXQTOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NPTXQTOPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NPTXQTOPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == NPTXQTOPR::VALUE4
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Non-periodic TxFIFO Space Avail"]
    #[inline]
    pub fn nptx_fspc_avail(&self) -> NPTXFSPCAVAILR {
        NPTXFSPCAVAILR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:23 - Non-periodic Transmit Request Queue Space Available"]
    #[inline]
    pub fn nptx_qspc_avail(&self) -> NPTXQSPCAVAILR {
        NPTXQSPCAVAILR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:30 - Top of the Non-periodic Transmit Request Queue"]
    #[inline]
    pub fn nptx_qtop(&self) -> NPTXQTOPR {
        NPTXQTOPR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
