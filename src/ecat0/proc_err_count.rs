#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::PROC_ERR_COUNT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct UNIT_ERRORR {
    bits: u8,
}
impl UNIT_ERRORR {
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
    #[doc = "Bits 0:7 - ECAT Processing Unit error counter"]
    #[inline]
    pub fn unit_error(&self) -> UNIT_ERRORR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        UNIT_ERRORR { bits }
    }
}
