#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EVRVADCSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct VADC13VR {
    bits: u8,
}
impl VADC13VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VADC33VR {
    bits: u8,
}
impl VADC33VR {
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
    #[doc = "Bits 0:7 - VADC 1.3 V Conversion Result"]
    #[inline]
    pub fn vadc13v(&self) -> VADC13VR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VADC13VR { bits }
    }
    #[doc = "Bits 8:15 - VADC 3.3 V Conversion Result"]
    #[inline]
    pub fn vadc33v(&self) -> VADC33VR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VADC33VR { bits }
    }
}
