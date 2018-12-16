#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TX_CARRIER_ERROR_FRAMES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TXCARRR {
    bits: u32,
}
impl TXCARRR {
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
    #[doc = "Bits 0:31 - This field indicates the number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
    #[inline]
    pub fn txcarr(&self) -> TXCARRR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TXCARRR { bits }
    }
}
