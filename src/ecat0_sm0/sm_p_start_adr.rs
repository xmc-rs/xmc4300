#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::SM_P_START_ADR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct FIRST_BYTER {
    bits: u16,
}
impl FIRST_BYTER {
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
    #[doc = "Bits 0:15 - Specifies first byte that will be handled by SyncManager"]
    #[inline]
    pub fn first_byte(&self) -> FIRST_BYTER {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        FIRST_BYTER { bits }
    }
}
