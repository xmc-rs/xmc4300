#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::SYNC_MANAGER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct NUM_SMR {
    bits: u8,
}
impl NUM_SMR {
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
    #[doc = "Bits 0:7 - Number of supported SyncManager channels (or entities) of the EtherCAT Slave Controller"]
    #[inline]
    pub fn num_sm(&self) -> NUM_SMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        NUM_SMR { bits }
    }
}
