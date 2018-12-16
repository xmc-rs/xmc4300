#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRBPTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TDIPTRR {
    bits: u8,
}
impl TDIPTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TDOPTRR {
    bits: u8,
}
impl TDOPTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RDIPTRR {
    bits: u8,
}
impl RDIPTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RDOPTRR {
    bits: u8,
}
impl RDOPTRR {
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
    #[doc = "Bits 0:5 - Transmitter Data Input Pointer"]
    #[inline]
    pub fn tdiptr(&self) -> TDIPTRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TDIPTRR { bits }
    }
    #[doc = "Bits 8:13 - Transmitter Data Output Pointer"]
    #[inline]
    pub fn tdoptr(&self) -> TDOPTRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TDOPTRR { bits }
    }
    #[doc = "Bits 16:21 - Receiver Data Input Pointer"]
    #[inline]
    pub fn rdiptr(&self) -> RDIPTRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDIPTRR { bits }
    }
    #[doc = "Bits 24:29 - Receiver Data Output Pointer"]
    #[inline]
    pub fn rdoptr(&self) -> RDOPTRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDOPTRR { bits }
    }
}
