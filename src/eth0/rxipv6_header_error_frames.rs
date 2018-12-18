#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXIPV6_HEADER_ERROR_FRAMES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RXIPV6HDRERRFRMR {
    bits: u32,
}
impl RXIPV6HDRERRFRMR {
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
    #[doc = "Bits 0:31 - This field indicates the number of IPv6 datagrams received with header errors (length or version mismatch)."]
    #[inline]
    pub fn rxipv6hdrerrfrm(&self) -> RXIPV6HDRERRFRMR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RXIPV6HDRERRFRMR { bits }
    }
}
