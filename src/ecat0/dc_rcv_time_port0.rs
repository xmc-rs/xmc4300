#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DC_RCV_TIME_PORT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct LOCAL_TIME_P0R {
    bits: u32,
}
impl LOCAL_TIME_P0R {
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
    #[doc = "Bits 0:31 - Write by EtherCAT master"]
    #[inline]
    pub fn local_time_p0(&self) -> LOCAL_TIME_P0R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        LOCAL_TIME_P0R { bits }
    }
}
