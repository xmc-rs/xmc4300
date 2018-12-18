#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct MISFRMCNTR {
    bits: u16,
}
impl MISFRMCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MISCNTOVFR {
    bits: bool,
}
impl MISCNTOVFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct OVFFRMCNTR {
    bits: u16,
}
impl OVFFRMCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OVFCNTOVFR {
    bits: bool,
}
impl OVFCNTOVFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the RAM Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
    #[inline]
    pub fn misfrmcnt(&self) -> MISFRMCNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MISFRMCNTR { bits }
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline]
    pub fn miscntovf(&self) -> MISCNTOVFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MISCNTOVFR { bits }
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application. The counter is cleared when this register is read."]
    #[inline]
    pub fn ovffrmcnt(&self) -> OVFFRMCNTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        OVFFRMCNTR { bits }
    }
    #[doc = "Bit 28 - Overflow bit for FIFO Overflow Counter"]
    #[inline]
    pub fn ovfcntovf(&self) -> OVFCNTOVFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVFCNTOVFR { bits }
    }
}
