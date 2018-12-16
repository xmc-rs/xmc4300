#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VERSION {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct SNPSVERR {
    bits: u8,
}
impl SNPSVERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USERVERR {
    bits: u8,
}
impl USERVERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Synopsys-defined Version (3.7)"]
    #[inline]
    pub fn snpsver(&self) -> SNPSVERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SNPSVERR { bits }
    }
    #[doc = "Bits 8:15 - User-defined Version (Configured with the coreConsultant)"]
    #[inline]
    pub fn userver(&self) -> USERVERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USERVERR { bits }
    }
}
