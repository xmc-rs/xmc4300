#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAX_CURRENT_CAP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct MAX_CURRENT_FOR_3_3VR {
    bits: u8,
}
impl MAX_CURRENT_FOR_3_3VR {
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
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline]
    pub fn max_current_for_3_3v(&self) -> MAX_CURRENT_FOR_3_3VR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_CURRENT_FOR_3_3VR { bits }
    }
}
