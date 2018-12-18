#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXIPV4_GOOD_FRAMES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RXIPV4GDFRMR {
    bits: u32,
}
impl RXIPV4GDFRMR {
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
    #[doc = "Bits 0:31 - This field indicates the number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload."]
    #[inline]
    pub fn rxipv4gdfrm(&self) -> RXIPV4GDFRMR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RXIPV4GDFRMR { bits }
    }
}
