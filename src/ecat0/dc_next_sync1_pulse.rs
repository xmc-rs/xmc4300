#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DC_NEXT_SYNC1_PULSE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DC_NEXT_SYNC1_PULSER {
    bits: u32,
}
impl DC_NEXT_SYNC1_PULSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - System time of next SYNC1 pulse in ns"]
    #[inline]
    pub fn dc_next_sync1_pulse(&self) -> DC_NEXT_SYNC1_PULSER {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DC_NEXT_SYNC1_PULSER { bits }
    }
}
