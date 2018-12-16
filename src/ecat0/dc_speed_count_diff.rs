#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::DC_SPEED_COUNT_DIFF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DEVIATIONR {
    bits: u16,
}
impl DEVIATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:15 - Representation of the deviation between local clock period and Reference Clock's clock period"]
    #[inline]
    pub fn deviation(&self) -> DEVIATIONR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        DEVIATIONR { bits }
    }
}
