#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CURRENT_HOST_RECEIVE_BUFFER_ADDRESS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct CURRBUFAPTRR {
    bits: u32,
}
impl CURRBUFAPTRR {
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
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline]
    pub fn currbufaptr(&self) -> CURRBUFAPTRR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CURRBUFAPTRR { bits }
    }
}
