#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GRXSTSP_DEVICEMODE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct EPNUMR {
    bits: u8,
}
impl EPNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BCNTR {
    bits: u16,
}
impl BCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DPID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPIDR {
    #[doc = "DATA0"]
    VALUE1,
    #[doc = "DATA1"]
    VALUE2,
    #[doc = "DATA2"]
    VALUE3,
    #[doc = "MDATA"]
    VALUE4,
}
impl DPIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DPIDR::VALUE1 => 0,
            DPIDR::VALUE2 => 2,
            DPIDR::VALUE3 => 1,
            DPIDR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DPIDR {
        match value {
            0 => DPIDR::VALUE1,
            2 => DPIDR::VALUE2,
            1 => DPIDR::VALUE3,
            3 => DPIDR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DPIDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DPIDR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DPIDR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DPIDR::VALUE4
    }
}
#[doc = "Possible values of the field `PktSts`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKTSTSR {
    #[doc = "Global OUT NAK (triggers an interrupt)"]
    VALUE1,
    #[doc = "OUT data packet received"]
    VALUE2,
    #[doc = "OUT transfer completed (triggers an interrupt)"]
    VALUE3,
    #[doc = "SETUP transaction completed (triggers an interrupt)"]
    VALUE4,
    #[doc = "SETUP data packet received"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PKTSTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PKTSTSR::VALUE1 => 1,
            PKTSTSR::VALUE2 => 2,
            PKTSTSR::VALUE3 => 3,
            PKTSTSR::VALUE4 => 4,
            PKTSTSR::VALUE5 => 6,
            PKTSTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PKTSTSR {
        match value {
            1 => PKTSTSR::VALUE1,
            2 => PKTSTSR::VALUE2,
            3 => PKTSTSR::VALUE3,
            4 => PKTSTSR::VALUE4,
            6 => PKTSTSR::VALUE5,
            i => PKTSTSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PKTSTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PKTSTSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PKTSTSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PKTSTSR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PKTSTSR::VALUE5
    }
}
#[doc = r" Value of the field"]
pub struct FNR {
    bits: u8,
}
impl FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Endpoint Number"]
    #[inline]
    pub fn epnum(&self) -> EPNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPNUMR { bits }
    }
    #[doc = "Bits 4:14 - Byte Count"]
    #[inline]
    pub fn bcnt(&self) -> BCNTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BCNTR { bits }
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline]
    pub fn dpid(&self) -> DPIDR {
        DPIDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:20 - Packet Status"]
    #[inline]
    pub fn pkt_sts(&self) -> PKTSTSR {
        PKTSTSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline]
    pub fn fn_(&self) -> FNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FNR { bits }
    }
}
