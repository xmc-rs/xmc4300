#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INPR {
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
#[doc = "Possible values of the field `TSINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSINPR {
    #[doc = "Output SR0 becomes activated."]
    VALUE1,
    #[doc = "Output SR1 becomes activated."]
    VALUE2,
    #[doc = "Output SR2 becomes activated."]
    VALUE3,
    #[doc = "Output SR3 becomes activated."]
    VALUE4,
    #[doc = "Output SR4 becomes activated."]
    VALUE5,
    #[doc = "Output SR5 becomes activated."]
    VALUE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSINPR::VALUE1 => 0,
            TSINPR::VALUE2 => 1,
            TSINPR::VALUE3 => 2,
            TSINPR::VALUE4 => 3,
            TSINPR::VALUE5 => 4,
            TSINPR::VALUE6 => 5,
            TSINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSINPR {
        match value {
            0 => TSINPR::VALUE1,
            1 => TSINPR::VALUE2,
            2 => TSINPR::VALUE3,
            3 => TSINPR::VALUE4,
            4 => TSINPR::VALUE5,
            5 => TSINPR::VALUE6,
            i => TSINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TSINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TSINPR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == TSINPR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == TSINPR::VALUE6
    }
}
#[doc = r" Value of the field"]
pub struct TBINPR {
    bits: u8,
}
impl TBINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RINPR {
    bits: u8,
}
impl RINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AINPR {
    bits: u8,
}
impl AINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PINPR {
    bits: u8,
}
impl PINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TSINP`"]
pub enum TSINPW {
    #[doc = "Output SR0 becomes activated."]
    VALUE1,
    #[doc = "Output SR1 becomes activated."]
    VALUE2,
    #[doc = "Output SR2 becomes activated."]
    VALUE3,
    #[doc = "Output SR3 becomes activated."]
    VALUE4,
    #[doc = "Output SR4 becomes activated."]
    VALUE5,
    #[doc = "Output SR5 becomes activated."]
    VALUE6,
}
impl TSINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSINPW::VALUE1 => 0,
            TSINPW::VALUE2 => 1,
            TSINPW::VALUE3 => 2,
            TSINPW::VALUE4 => 3,
            TSINPW::VALUE5 => 4,
            TSINPW::VALUE6 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSINPW<'a> {
    w: &'a mut W,
}
impl<'a> _TSINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSINPW::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSINPW::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSINPW::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSINPW::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(TSINPW::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(TSINPW::VALUE6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TBINPW<'a> {
    w: &'a mut W,
}
impl<'a> _TBINPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RINPW<'a> {
    w: &'a mut W,
}
impl<'a> _RINPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AINPW<'a> {
    w: &'a mut W,
}
impl<'a> _AINPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINPW<'a> {
    w: &'a mut W,
}
impl<'a> _PINPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Transmit Shift Interrupt Node Pointer"]
    #[inline]
    pub fn tsinp(&self) -> TSINPR {
        TSINPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Transmit Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn tbinp(&self) -> TBINPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TBINPR { bits }
    }
    #[doc = "Bits 8:10 - Receive Interrupt Node Pointer"]
    #[inline]
    pub fn rinp(&self) -> RINPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RINPR { bits }
    }
    #[doc = "Bits 12:14 - Alternative Receive Interrupt Node Pointer"]
    #[inline]
    pub fn ainp(&self) -> AINPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AINPR { bits }
    }
    #[doc = "Bits 16:18 - Transmit Shift Interrupt Node Pointer"]
    #[inline]
    pub fn pinp(&self) -> PINPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PINPR { bits }
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
    #[doc = "Bits 0:2 - Transmit Shift Interrupt Node Pointer"]
    #[inline]
    pub fn tsinp(&mut self) -> _TSINPW {
        _TSINPW { w: self }
    }
    #[doc = "Bits 4:6 - Transmit Buffer Interrupt Node Pointer"]
    #[inline]
    pub fn tbinp(&mut self) -> _TBINPW {
        _TBINPW { w: self }
    }
    #[doc = "Bits 8:10 - Receive Interrupt Node Pointer"]
    #[inline]
    pub fn rinp(&mut self) -> _RINPW {
        _RINPW { w: self }
    }
    #[doc = "Bits 12:14 - Alternative Receive Interrupt Node Pointer"]
    #[inline]
    pub fn ainp(&mut self) -> _AINPW {
        _AINPW { w: self }
    }
    #[doc = "Bits 16:18 - Transmit Shift Interrupt Node Pointer"]
    #[inline]
    pub fn pinp(&mut self) -> _PINPW {
        _PINPW { w: self }
    }
}
