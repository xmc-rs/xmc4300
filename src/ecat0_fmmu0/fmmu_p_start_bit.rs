#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FMMU_P_START_BIT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct P_START_BITR {
    bits: u8,
}
impl P_START_BITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:2 - Physical starting bit as target of logical start bit mapping"]
    #[inline]
    pub fn p_start_bit(&self) -> P_START_BITR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        P_START_BITR { bits }
    }
}
