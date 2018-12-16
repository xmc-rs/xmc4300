#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FMMU_NUM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct NUM_FMMUR {
    bits: u8,
}
impl NUM_FMMUR {
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
    #[doc = "Bits 0:7 - Number of supported FMMU channels (or entities) of the EtherCAT Slave Controller"]
    #[inline]
    pub fn num_fmmu(&self) -> NUM_FMMUR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        NUM_FMMUR { bits }
    }
}
