#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TX1024_MAXOCTGBR {
    bits: u32,
}
impl TX1024_MAXOCTGBR {
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
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline]
    pub fn tx1024_maxoctgb(&self) -> TX1024_MAXOCTGBR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TX1024_MAXOCTGBR { bits }
    }
}
