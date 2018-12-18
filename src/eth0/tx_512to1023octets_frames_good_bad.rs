#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TX_512TO1023OCTETS_FRAMES_GOOD_BAD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TX512_1023OCTGBR {
    bits: u32,
}
impl TX512_1023OCTGBR {
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
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good and bad frames with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline]
    pub fn tx512_1023octgb(&self) -> TX512_1023OCTGBR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TX512_1023OCTGBR { bits }
    }
}
