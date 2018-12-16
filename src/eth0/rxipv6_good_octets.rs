#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXIPV6_GOOD_OCTETS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RXIPV6GDOCTR {
    bits: u32,
}
impl RXIPV6GDOCTR {
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
    #[doc = "Bits 0:31 - Thsi field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data."]
    #[inline]
    pub fn rxipv6gdoct(&self) -> RXIPV6GDOCTR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RXIPV6GDOCTR { bits }
    }
}
