#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCDMA_SCATGATHER {
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
#[doc = "Possible values of the field `CTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTDR {
    #[doc = "1 descriptor"]
    VALUE1,
    #[doc = "64 descriptors"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CTDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTDR::VALUE1 => 0,
            CTDR::VALUE2 => 63,
            CTDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTDR {
        match value {
            0 => CTDR::VALUE1,
            63 => CTDR::VALUE2,
            i => CTDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CTDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CTDR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct DMAADDRR {
    bits: u32,
}
impl DMAADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CTD`"]
pub enum CTDW {
    #[doc = "1 descriptor"]
    VALUE1,
    #[doc = "64 descriptors"]
    VALUE2,
}
impl CTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTDW::VALUE1 => 0,
            CTDW::VALUE2 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTDW<'a> {
    w: &'a mut W,
}
impl<'a> _CTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 descriptor"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTDW::VALUE1)
    }
    #[doc = "64 descriptors"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTDW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 8388607;
        const OFFSET: u8 = 9;
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
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline]
    pub fn ctd(&self) -> CTDR {
        CTDR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline]
    pub fn dmaaddr(&self) -> DMAADDRR {
        let bits = {
            const MASK: u32 = 8388607;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DMAADDRR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline]
    pub fn ctd(&mut self) -> _CTDW {
        _CTDW { w: self }
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline]
    pub fn dmaaddr(&mut self) -> _DMAADDRW {
        _DMAADDRW { w: self }
    }
}
