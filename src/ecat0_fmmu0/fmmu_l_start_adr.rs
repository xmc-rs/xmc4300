#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FMMU_L_START_ADR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct L_START_ADDRR {
    bits: u32,
}
impl L_START_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Logical start address within the EtherCAT Address Space"]
    #[inline]
    pub fn l_start_addr(&self) -> L_START_ADDRR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        L_START_ADDRR { bits }
    }
}
