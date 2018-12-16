#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RX_128TO255OCTETS_FRAMES_GOOD_BAD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RX128_255OCTGBR {
    bits: u32,
}
impl RX128_255OCTGBR {
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
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 128 and 255 (inclusive) bytes, exclusive of preamble."]
    #[inline]
    pub fn rx128_255octgb(&self) -> RX128_255OCTGBR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RX128_255OCTGBR { bits }
    }
}
