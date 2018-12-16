#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "No clock supplied"]
    VALUE1,
    #[doc = "fPERIPH"]
    VALUE2,
    #[doc = "fOHP"]
    VALUE3,
    #[doc = "hard wired to 0"]
    VALUE4,
    #[doc = "hard wired to 0"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSELR::VALUE1 => 0,
            CLKSELR::VALUE2 => 1,
            CLKSELR::VALUE3 => 2,
            CLKSELR::VALUE4 => 4,
            CLKSELR::VALUE5 => 8,
            CLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSELR {
        match value {
            0 => CLKSELR::VALUE1,
            1 => CLKSELR::VALUE2,
            2 => CLKSELR::VALUE3,
            4 => CLKSELR::VALUE4,
            8 => CLKSELR::VALUE5,
            i => CLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLKSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLKSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CLKSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CLKSELR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == CLKSELR::VALUE5
    }
}
#[doc = r" Value of the field"]
pub struct MPSELR {
    bits: u8,
}
impl MPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "No clock supplied"]
    VALUE1,
    #[doc = "fPERIPH"]
    VALUE2,
    #[doc = "fOHP"]
    VALUE3,
    #[doc = "hard wired to 0"]
    VALUE4,
    #[doc = "hard wired to 0"]
    VALUE5,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSELW::VALUE1 => 0,
            CLKSELW::VALUE2 => 1,
            CLKSELW::VALUE3 => 2,
            CLKSELW::VALUE4 => 4,
            CLKSELW::VALUE5 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No clock supplied"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKSELW::VALUE1)
    }
    #[doc = "fPERIPH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLKSELW::VALUE2)
    }
    #[doc = "fOHP"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLKSELW::VALUE3)
    }
    #[doc = "hard wired to 0"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLKSELW::VALUE4)
    }
    #[doc = "hard wired to 0"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(CLKSELW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MPSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - Baud Rate Logic Clock Select"]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Message Pending Selector"]
    #[inline]
    pub fn mpsel(&self) -> MPSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MPSELR { bits }
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
    #[doc = "Bits 0:3 - Baud Rate Logic Clock Select"]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bits 12:15 - Message Pending Selector"]
    #[inline]
    pub fn mpsel(&mut self) -> _MPSELW {
        _MPSELW { w: self }
    }
}
