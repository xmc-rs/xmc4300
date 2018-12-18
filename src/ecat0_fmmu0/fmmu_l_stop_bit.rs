#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FMMU_L_STOP_BIT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct L_STOP_BITR {
    bits: u8,
}
impl L_STOP_BITR {
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
    #[doc = "Bits 0:2 - Last logical bit that shall be mapped"]
    #[inline]
    pub fn l_stop_bit(&self) -> L_STOP_BITR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        L_STOP_BITR { bits }
    }
}
