#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RECEIVE_TIME_PU {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RECEIVE_TIME_PUR {
    bits: u32,
}
impl RECEIVE_TIME_PUR {
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
    #[doc = "Bits 0:31 - Local time of the beginning of a frame"]
    #[inline]
    pub fn receive_time_pu(&self) -> RECEIVE_TIME_PUR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RECEIVE_TIME_PUR { bits }
    }
}
