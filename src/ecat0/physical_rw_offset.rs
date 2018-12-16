#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::PHYSICAL_RW_OFFSET {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct OFFSETR {
    bits: u16,
}
impl OFFSETR {
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
    #[doc = "Bits 0:15 - Offset of R/W Commands (FPRW, APRW) between Read address and Write address"]
    #[inline]
    pub fn offset(&self) -> OFFSETR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        OFFSETR { bits }
    }
}
