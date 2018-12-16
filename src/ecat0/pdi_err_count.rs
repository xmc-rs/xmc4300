#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::PDI_ERR_COUNT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct PDI_ERROR_COUNTERR {
    bits: u8,
}
impl PDI_ERROR_COUNTERR {
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
    #[doc = "Bits 0:7 - PDI Error counter"]
    #[inline]
    pub fn pdi_error_counter(&self) -> PDI_ERROR_COUNTERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        PDI_ERROR_COUNTERR { bits }
    }
}
