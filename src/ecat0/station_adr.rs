#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::STATION_ADR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct NODE_ADDRR {
    bits: u16,
}
impl NODE_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:15 - Address used for node addressing (FPxx commands)"]
    #[inline]
    pub fn node_addr(&self) -> NODE_ADDRR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        NODE_ADDRR { bits }
    }
}
