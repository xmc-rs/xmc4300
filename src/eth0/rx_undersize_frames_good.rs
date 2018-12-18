#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RX_UNDERSIZE_FRAMES_GOOD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RXUNDERSZGR {
    bits: u32,
}
impl RXUNDERSZGR {
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
    #[doc = "Bits 0:31 - This field indicates the number of frames received with length less than 64 bytes and without errors."]
    #[inline]
    pub fn rxunderszg(&self) -> RXUNDERSZGR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RXUNDERSZGR { bits }
    }
}
