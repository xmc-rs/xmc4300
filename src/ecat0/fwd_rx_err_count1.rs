#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FWD_RX_ERR_COUNT1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct FORW_ERRORR {
    bits: u8,
}
impl FORW_ERRORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - Forwarded error counter of Port y"]
    #[inline]
    pub fn forw_error(&self) -> FORW_ERRORR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        FORW_ERRORR { bits }
    }
}
