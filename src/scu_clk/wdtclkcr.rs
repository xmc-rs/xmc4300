#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WDTCLKCR {
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
#[doc = r" Value of the field"]
pub struct WDTDIVR {
    bits: u8,
}
impl WDTDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WDTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTSELR {
    #[doc = "fOFI clock"]
    CONST_00,
    #[doc = "fSTDBY clock"]
    CONST_01,
    #[doc = "fPLL clock"]
    CONST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WDTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDTSELR::CONST_00 => 0,
            WDTSELR::CONST_01 => 1,
            WDTSELR::CONST_10 => 2,
            WDTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDTSELR {
        match value {
            0 => WDTSELR::CONST_00,
            1 => WDTSELR::CONST_01,
            2 => WDTSELR::CONST_10,
            i => WDTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == WDTSELR::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == WDTSELR::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == WDTSELR::CONST_10
    }
}
#[doc = r" Proxy"]
pub struct _WDTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDTSEL`"]
pub enum WDTSELW {
    #[doc = "fOFI clock"]
    CONST_00,
    #[doc = "fSTDBY clock"]
    CONST_01,
    #[doc = "fPLL clock"]
    CONST_10,
}
impl WDTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WDTSELW::CONST_00 => 0,
            WDTSELW::CONST_01 => 1,
            WDTSELW::CONST_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fOFI clock"]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(WDTSELW::CONST_00)
    }
    #[doc = "fSTDBY clock"]
    #[inline]
    pub fn const_01(self) -> &'a mut W {
        self.variant(WDTSELW::CONST_01)
    }
    #[doc = "fPLL clock"]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(WDTSELW::CONST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline]
    pub fn wdtdiv(&self) -> WDTDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WDTDIVR { bits }
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline]
    pub fn wdtsel(&self) -> WDTSELR {
        WDTSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline]
    pub fn wdtdiv(&mut self) -> _WDTDIVW {
        _WDTDIVW { w: self }
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline]
    pub fn wdtsel(&mut self) -> _WDTSELW {
        _WDTSELW { w: self }
    }
}
