#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::RX_ERR_COUNT1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct INVALID_FRAMER {
    bits: u8,
}
impl INVALID_FRAMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RX_ERRORR {
    bits: u8,
}
impl RX_ERRORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:7 - Invalid frame counter of Port y"]
    #[inline]
    pub fn invalid_frame(&self) -> INVALID_FRAMER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        INVALID_FRAMER { bits }
    }
    #[doc = "Bits 8:15 - RX Error counter of Port y"]
    #[inline]
    pub fn rx_error(&self) -> RX_ERRORR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        RX_ERRORR { bits }
    }
}
