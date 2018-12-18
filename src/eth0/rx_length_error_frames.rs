#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RX_LENGTH_ERROR_FRAMES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RXLENERRR {
    bits: u32,
}
impl RXLENERRR {
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
    #[doc = "Bits 0:31 - This field indicates the number of frames received with length error (Length type field not equal to frame size) for all frames with valid length field."]
    #[inline]
    pub fn rxlenerr(&self) -> RXLENERRR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RXLENERRR { bits }
    }
}
