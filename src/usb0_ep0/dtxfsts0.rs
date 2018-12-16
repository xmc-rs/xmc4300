#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DTXFSTS0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `INEPTxFSpcAvail`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INEPTXFSPCAVAILR {
    #[doc = "Endpoint TxFIFO is full"]
    VALUE1,
    #[doc = "1 word available"]
    VALUE2,
    #[doc = "2 words available"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl INEPTXFSPCAVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            INEPTXFSPCAVAILR::VALUE1 => 0,
            INEPTXFSPCAVAILR::VALUE2 => 1,
            INEPTXFSPCAVAILR::VALUE3 => 2,
            INEPTXFSPCAVAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> INEPTXFSPCAVAILR {
        match value {
            0 => INEPTXFSPCAVAILR::VALUE1,
            1 => INEPTXFSPCAVAILR::VALUE2,
            2 => INEPTXFSPCAVAILR::VALUE3,
            i => INEPTXFSPCAVAILR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INEPTXFSPCAVAILR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INEPTXFSPCAVAILR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == INEPTXFSPCAVAILR::VALUE3
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail"]
    #[inline]
    pub fn ineptx_fspc_avail(&self) -> INEPTXFSPCAVAILR {
        INEPTXFSPCAVAILR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
