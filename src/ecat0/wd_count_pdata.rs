#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::WD_COUNT_PDATA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct WD_COUNTER_PDR {
    bits: u8,
}
impl WD_COUNTER_PDR {
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
    #[doc = "Bits 0:7 - Watchdog Counter Process Data"]
    #[inline]
    pub fn wd_counter_pd(&self) -> WD_COUNTER_PDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        WD_COUNTER_PDR { bits }
    }
}
