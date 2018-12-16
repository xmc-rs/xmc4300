#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPTXSTS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `PTxFSpcAvail`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTXFSPCAVAILR {
    #[doc = "Periodic TxFIFO is full"]
    VALUE1,
    #[doc = "1 word available"]
    VALUE2,
    #[doc = "2 words available"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl PTXFSPCAVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            PTXFSPCAVAILR::VALUE1 => 0,
            PTXFSPCAVAILR::VALUE2 => 1,
            PTXFSPCAVAILR::VALUE3 => 2,
            PTXFSPCAVAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> PTXFSPCAVAILR {
        match value {
            0 => PTXFSPCAVAILR::VALUE1,
            1 => PTXFSPCAVAILR::VALUE2,
            2 => PTXFSPCAVAILR::VALUE3,
            i => PTXFSPCAVAILR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PTXFSPCAVAILR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PTXFSPCAVAILR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PTXFSPCAVAILR::VALUE3
    }
}
#[doc = "Possible values of the field `PTxQSpcAvail`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTXQSPCAVAILR {
    #[doc = "Periodic Transmit Request Queue is full"]
    VALUE1,
    #[doc = "1 location available"]
    VALUE2,
    #[doc = "2 locations available"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTXQSPCAVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTXQSPCAVAILR::VALUE1 => 0,
            PTXQSPCAVAILR::VALUE2 => 1,
            PTXQSPCAVAILR::VALUE3 => 2,
            PTXQSPCAVAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTXQSPCAVAILR {
        match value {
            0 => PTXQSPCAVAILR::VALUE1,
            1 => PTXQSPCAVAILR::VALUE2,
            2 => PTXQSPCAVAILR::VALUE3,
            i => PTXQSPCAVAILR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PTXQSPCAVAILR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PTXQSPCAVAILR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PTXQSPCAVAILR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct PTXQTOPR {
    bits: u8,
}
impl PTXQTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `PTxFSpcAvail`"]
pub enum PTXFSPCAVAILW {
    #[doc = "Periodic TxFIFO is full"]
    VALUE1,
    #[doc = "1 word available"]
    VALUE2,
    #[doc = "2 words available"]
    VALUE3,
}
impl PTXFSPCAVAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            PTXFSPCAVAILW::VALUE1 => 0,
            PTXFSPCAVAILW::VALUE2 => 1,
            PTXFSPCAVAILW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTXFSPCAVAILW<'a> {
    w: &'a mut W,
}
impl<'a> _PTXFSPCAVAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTXFSPCAVAILW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Periodic TxFIFO is full"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PTXFSPCAVAILW::VALUE1)
    }
    #[doc = "1 word available"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PTXFSPCAVAILW::VALUE2)
    }
    #[doc = "2 words available"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PTXFSPCAVAILW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline]
    pub fn ptx_fspc_avail(&self) -> PTXFSPCAVAILR {
        PTXFSPCAVAILR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:23 - Periodic Transmit Request Queue Space Available"]
    #[inline]
    pub fn ptx_qspc_avail(&self) -> PTXQSPCAVAILR {
        PTXQSPCAVAILR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Top of the Periodic Transmit Request Queue"]
    #[inline]
    pub fn ptx_qtop(&self) -> PTXQTOPR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PTXQTOPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 524544 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline]
    pub fn ptx_fspc_avail(&mut self) -> _PTXFSPCAVAILW {
        _PTXFSPCAVAILW { w: self }
    }
}
