#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTCLKCR {
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
#[doc = "Possible values of the field `ECKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECKSELR {
    #[doc = "fSYS clock"]
    CONST_00,
    #[doc = "fUSB clock divided according to ECKDIV bit field configuration"]
    CONST_10,
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    CONST_11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ECKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ECKSELR::CONST_00 => 0,
            ECKSELR::CONST_10 => 2,
            ECKSELR::CONST_11 => 3,
            ECKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ECKSELR {
        match value {
            0 => ECKSELR::CONST_00,
            2 => ECKSELR::CONST_10,
            3 => ECKSELR::CONST_11,
            i => ECKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == ECKSELR::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == ECKSELR::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline]
    pub fn is_const_11(&self) -> bool {
        *self == ECKSELR::CONST_11
    }
}
#[doc = r" Value of the field"]
pub struct ECKDIVR {
    bits: u16,
}
impl ECKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ECKSEL`"]
pub enum ECKSELW {
    #[doc = "fSYS clock"]
    CONST_00,
    #[doc = "fUSB clock divided according to ECKDIV bit field configuration"]
    CONST_10,
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    CONST_11,
}
impl ECKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ECKSELW::CONST_00 => 0,
            ECKSELW::CONST_10 => 2,
            ECKSELW::CONST_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ECKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fSYS clock"]
    #[inline]
    pub fn const_00(self) -> &'a mut W {
        self.variant(ECKSELW::CONST_00)
    }
    #[doc = "fUSB clock divided according to ECKDIV bit field configuration"]
    #[inline]
    pub fn const_10(self) -> &'a mut W {
        self.variant(ECKSELW::CONST_10)
    }
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    #[inline]
    pub fn const_11(self) -> &'a mut W {
        self.variant(ECKSELW::CONST_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ECKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ECKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
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
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline]
    pub fn ecksel(&self) -> ECKSELR {
        ECKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline]
    pub fn eckdiv(&self) -> ECKDIVR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ECKDIVR { bits }
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
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline]
    pub fn ecksel(&mut self) -> _ECKSELW {
        _ECKSELW { w: self }
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline]
    pub fn eckdiv(&mut self) -> _ECKDIVW {
        _ECKDIVW { w: self }
    }
}
