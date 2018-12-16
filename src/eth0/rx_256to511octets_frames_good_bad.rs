#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RX_256TO511OCTETS_FRAMES_GOOD_BAD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RX256_511OCTGBR {
    bits: u32,
}
impl RX256_511OCTGBR {
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
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble."]
    #[inline]
    pub fn rx256_511octgb(&self) -> RX256_511OCTGBR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RX256_511OCTGBR { bits }
    }
}
