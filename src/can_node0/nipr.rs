#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NIPR {
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
#[doc = "Possible values of the field `ALINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALINPR {
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
impl ALINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALINPR::VALUE1 => 0,
            ALINPR::VALUE2 => 1,
            ALINPR::VALUE3 => 14,
            ALINPR::VALUE4 => 15,
            ALINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALINPR {
        match value {
            0 => ALINPR::VALUE1,
            1 => ALINPR::VALUE2,
            14 => ALINPR::VALUE3,
            15 => ALINPR::VALUE4,
            i => ALINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ALINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ALINPR::VALUE4
    }
}
#[doc = "Possible values of the field `LECINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LECINPR {
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
impl LECINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LECINPR::VALUE1 => 0,
            LECINPR::VALUE2 => 1,
            LECINPR::VALUE3 => 14,
            LECINPR::VALUE4 => 15,
            LECINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LECINPR {
        match value {
            0 => LECINPR::VALUE1,
            1 => LECINPR::VALUE2,
            14 => LECINPR::VALUE3,
            15 => LECINPR::VALUE4,
            i => LECINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LECINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LECINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LECINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LECINPR::VALUE4
    }
}
#[doc = "Possible values of the field `TRINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRINPR {
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
impl TRINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRINPR::VALUE1 => 0,
            TRINPR::VALUE2 => 1,
            TRINPR::VALUE3 => 14,
            TRINPR::VALUE4 => 15,
            TRINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRINPR {
        match value {
            0 => TRINPR::VALUE1,
            1 => TRINPR::VALUE2,
            14 => TRINPR::VALUE3,
            15 => TRINPR::VALUE4,
            i => TRINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TRINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TRINPR::VALUE4
    }
}
#[doc = "Possible values of the field `CFCINP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFCINPR {
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
impl CFCINPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFCINPR::VALUE1 => 0,
            CFCINPR::VALUE2 => 1,
            CFCINPR::VALUE3 => 14,
            CFCINPR::VALUE4 => 15,
            CFCINPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFCINPR {
        match value {
            0 => CFCINPR::VALUE1,
            1 => CFCINPR::VALUE2,
            14 => CFCINPR::VALUE3,
            15 => CFCINPR::VALUE4,
            i => CFCINPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CFCINPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CFCINPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CFCINPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CFCINPR::VALUE4
    }
}
#[doc = "Values that can be written to the field `ALINP`"]
pub enum ALINPW {
    #[doc = "Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl ALINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALINPW::VALUE1 => 0,
            ALINPW::VALUE2 => 1,
            ALINPW::VALUE3 => 14,
            ALINPW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALINPW<'a> {
    w: &'a mut W,
}
impl<'a> _ALINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALINPW::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALINPW::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ALINPW::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ALINPW::VALUE4)
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
#[doc = "Values that can be written to the field `LECINP`"]
pub enum LECINPW {
    #[doc = "Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl LECINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LECINPW::VALUE1 => 0,
            LECINPW::VALUE2 => 1,
            LECINPW::VALUE3 => 14,
            LECINPW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LECINPW<'a> {
    w: &'a mut W,
}
impl<'a> _LECINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LECINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LECINPW::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LECINPW::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LECINPW::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(LECINPW::VALUE4)
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
#[doc = "Values that can be written to the field `TRINP`"]
pub enum TRINPW {
    #[doc = "Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl TRINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRINPW::VALUE1 => 0,
            TRINPW::VALUE2 => 1,
            TRINPW::VALUE3 => 14,
            TRINPW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRINPW<'a> {
    w: &'a mut W,
}
impl<'a> _TRINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRINPW::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRINPW::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRINPW::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRINPW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFCINP`"]
pub enum CFCINPW {
    #[doc = "Interrupt output line INT_O0 is selected."]
    VALUE1,
    #[doc = "Interrupt output line INT_O1 is selected."]
    VALUE2,
    #[doc = "Interrupt output line INT_O14 is selected."]
    VALUE3,
    #[doc = "Interrupt output line INT_O15 is selected."]
    VALUE4,
}
impl CFCINPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFCINPW::VALUE1 => 0,
            CFCINPW::VALUE2 => 1,
            CFCINPW::VALUE3 => 14,
            CFCINPW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFCINPW<'a> {
    w: &'a mut W,
}
impl<'a> _CFCINPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFCINPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCINPW::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCINPW::VALUE2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFCINPW::VALUE3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFCINPW::VALUE4)
    }
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
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline]
    pub fn alinp(&self) -> ALINPR {
        ALINPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline]
    pub fn lecinp(&self) -> LECINPR {
        LECINPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline]
    pub fn trinp(&self) -> TRINPR {
        TRINPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline]
    pub fn cfcinp(&self) -> CFCINPR {
        CFCINPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline]
    pub fn alinp(&mut self) -> _ALINPW {
        _ALINPW { w: self }
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline]
    pub fn lecinp(&mut self) -> _LECINPW {
        _LECINPW { w: self }
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline]
    pub fn trinp(&mut self) -> _TRINPW {
        _TRINPW { w: self }
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline]
    pub fn cfcinp(&mut self) -> _CFCINPW {
        _CFCINPW { w: self }
    }
}
