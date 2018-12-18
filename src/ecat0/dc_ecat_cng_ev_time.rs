#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DC_ECAT_CNG_EV_TIME {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct ECAT_CNG_EV_TIMER {
    bits: u32,
}
impl ECAT_CNG_EV_TIMER {
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
    #[doc = "Bits 0:31 - Register captures local time of the beginning of the frame which causes at least one SyncManager to assert an ECAT event"]
    #[inline]
    pub fn ecat_cng_ev_time(&self) -> ECAT_CNG_EV_TIMER {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ECAT_CNG_EV_TIMER { bits }
    }
}
