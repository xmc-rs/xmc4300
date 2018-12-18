#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::LOST_LINK_COUNT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct LL_COUNTERR {
    bits: u8,
}
impl LL_COUNTERR {
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
    #[doc = "Bits 0:7 - Lost Link counter of Port p"]
    #[inline]
    pub fn ll_counter(&self) -> LL_COUNTERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        LL_COUNTERR { bits }
    }
}
