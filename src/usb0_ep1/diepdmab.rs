#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DIEPDMAB {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DMABUFFERADDRR {
    bits: u32,
}
impl DMABUFFERADDRR {
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
    #[doc = "Bits 0:31 - DMA Buffer Address"]
    #[inline]
    pub fn dmabuffer_addr(&self) -> DMABUFFERADDRR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DMABUFFERADDRR { bits }
    }
}
