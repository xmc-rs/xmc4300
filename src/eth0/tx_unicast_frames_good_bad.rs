#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TX_UNICAST_FRAMES_GOOD_BAD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TXUCASTGBR {
    bits: u32,
}
impl TXUCASTGBR {
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
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad unicast frames."]
    #[inline]
    pub fn txucastgb(&self) -> TXUCASTGBR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TXUCASTGBR { bits }
    }
}
