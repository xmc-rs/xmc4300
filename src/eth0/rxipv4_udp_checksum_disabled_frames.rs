#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RXIPV4UDSBLFRMR {
    bits: u32,
}
impl RXIPV4UDSBLFRMR {
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
    #[doc = "Bits 0:31 - This field indicates the number of received good IPv4 datagrams which have the UDP payload with checksum disabled."]
    #[inline]
    pub fn rxipv4udsblfrm(&self) -> RXIPV4UDSBLFRMR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RXIPV4UDSBLFRMR { bits }
    }
}
