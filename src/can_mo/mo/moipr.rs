#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOIPR {
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
#[doc = "Possible values of the field `RXINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINPR {
    #[doc = "Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "Interrupt output line INT_O15 is selected."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXINPR::VALUE1 => 0,
            RXINPR::VALUE2 => 1,
            RXINPR::VALUE3 => 14,
            RXINPR::VALUE4 => 15,
            RXINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXINPR {
        match value {
            0 => RXINPR::VALUE1,
            1 => RXINPR::VALUE2,
            14 => RXINPR::VALUE3,
            15 => RXINPR::VALUE4,
            i => RXINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXINPR::VALUE4
    }
}
#[doc = "Possible values of the field `TXINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINPR {
    #[doc = "Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "Interrupt output line INT_O15 is selected."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXINPR::VALUE1 => 0,
            TXINPR::VALUE2 => 1,
            TXINPR::VALUE3 => 14,
            TXINPR::VALUE4 => 15,
            TXINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXINPR {
        match value {
            0 => TXINPR::VALUE1,
            1 => TXINPR::VALUE2,
            14 => TXINPR::VALUE3,
            15 => TXINPR::VALUE4,
            i => TXINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TXINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TXINPR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct MPNR {
    bits: u8,
}
impl MPNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CFCVALR {
    bits: u16,
}
impl CFCVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `RXINP`"]
pub enum RXINPW {
    #[doc = "Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl RXINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXINPW::VALUE1 => 0,
            RXINPW::VALUE2 => 1,
            RXINPW::VALUE3 => 14,
            RXINPW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXINPW<'a> {
    w: &'a mut W,
}
impl<'a> _RXINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXINPW::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXINPW::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXINPW::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXINPW::VALUE4)
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
#[doc = "Values that can be written to the field `TXINP`"]
pub enum TXINPW {
    #[doc = "Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl TXINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXINPW::VALUE1 => 0,
            TXINPW::VALUE2 => 1,
            TXINPW::VALUE3 => 14,
            TXINPW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXINPW<'a> {
    w: &'a mut W,
}
impl<'a> _TXINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXINPW::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXINPW::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TXINPW::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TXINPW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MPNW<'a> {
    w: &'a mut W,
}
impl<'a> _MPNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFCVALW<'a> {
    w: &'a mut W,
}
impl<'a> _CFCVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline]
    pub fn rxinp(&self) -> RXINPR {
        RXINPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline]
    pub fn txinp(&self) -> TXINPR {
        TXINPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline]
    pub fn mpn(&self) -> MPNR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MPNR { bits }
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline]
    pub fn cfcval(&self) -> CFCVALR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CFCVALR { bits }
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
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline]
    pub fn rxinp(&mut self) -> _RXINPW {
        _RXINPW { w: self }
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline]
    pub fn txinp(&mut self) -> _TXINPW {
        _TXINPW { w: self }
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline]
    pub fn mpn(&mut self) -> _MPNW {
        _MPNW { w: self }
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline]
    pub fn cfcval(&mut self) -> _CFCVALW {
        _CFCVALW { w: self }
    }
}
