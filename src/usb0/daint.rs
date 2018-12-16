#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DAINT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct INEPINTR {
    bits: u16,
}
impl INEPINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUTEPINTR {
    bits: u16,
}
impl OUTEPINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - IN Endpoint Interrupt Bits"]
    #[inline]
    pub fn in_ep_int(&self) -> INEPINTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INEPINTR { bits }
    }
    #[doc = "Bits 16:31 - OUT Endpoint Interrupt Bits"]
    #[inline]
    pub fn out_epint(&self) -> OUTEPINTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        OUTEPINTR { bits }
    }
}
