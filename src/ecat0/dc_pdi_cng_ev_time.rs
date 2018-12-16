#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DC_PDI_CNG_EV_TIME {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct PDI_CNG_EV_TIMER {
    bits: u32,
}
impl PDI_CNG_EV_TIMER {
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
    #[doc = "Bits 0:31 - Register captures local time when at least one SyncManager asserts an PDI buffer change event"]
    #[inline]
    pub fn pdi_cng_ev_time(&self) -> PDI_CNG_EV_TIMER {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        PDI_CNG_EV_TIMER { bits }
    }
}
