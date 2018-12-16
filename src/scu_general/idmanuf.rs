#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IDMANUF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DEPTR {
    bits: u8,
}
impl DEPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MANUFR {
    bits: u16,
}
impl MANUFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Department Identification Number"]
    #[inline]
    pub fn dept(&self) -> DEPTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEPTR { bits }
    }
    #[doc = "Bits 5:15 - Manufacturer Identification Number"]
    #[inline]
    pub fn manuf(&self) -> MANUFR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MANUFR { bits }
    }
}
