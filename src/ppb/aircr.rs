#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AIRCR {
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
pub struct PRIGROUPR {
    bits: u8,
}
impl PRIGROUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENDIANNESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANNESSR {
    #[doc = "Little-endian"]
    VALUE1,
    #[doc = "Big-endian."]
    VALUE2,
}
impl ENDIANNESSR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ENDIANNESSR::VALUE1 => false,
            ENDIANNESSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDIANNESSR {
        match value {
            false => ENDIANNESSR::VALUE1,
            true => ENDIANNESSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENDIANNESSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENDIANNESSR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct VECTKEYR {
    bits: u16,
}
impl VECTKEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _VECTRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTRESETW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VECTCLRACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTCLRACTIVEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSRESETREQ`"]
pub enum SYSRESETREQW {
    #[doc = "no system reset request"]
    VALUE1,
    #[doc = "asserts a signal to the outer system that requests a reset."]
    VALUE2,
}
impl SYSRESETREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSRESETREQW::VALUE1 => false,
            SYSRESETREQW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSRESETREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSRESETREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSRESETREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no system reset request"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYSRESETREQW::VALUE1)
    }
    #[doc = "asserts a signal to the outer system that requests a reset."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYSRESETREQW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRIGROUPW<'a> {
    w: &'a mut W,
}
impl<'a> _PRIGROUPW<'a> {
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
pub struct _VECTKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTKEYW<'a> {
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
    #[doc = "Bits 8:10 - Interrupt priority grouping field"]
    #[inline]
    pub fn prigroup(&self) -> PRIGROUPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRIGROUPR { bits }
    }
    #[doc = "Bit 15 - Data endianness bit"]
    #[inline]
    pub fn endianness(&self) -> ENDIANNESSR {
        ENDIANNESSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline]
    pub fn vectkey(&self) -> VECTKEYR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VECTKEYR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4194631680 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Reserved for Debug use."]
    #[inline]
    pub fn vectreset(&mut self) -> _VECTRESETW {
        _VECTRESETW { w: self }
    }
    #[doc = "Bit 1 - Reserved for Debug use."]
    #[inline]
    pub fn vectclractive(&mut self) -> _VECTCLRACTIVEW {
        _VECTCLRACTIVEW { w: self }
    }
    #[doc = "Bit 2 - System reset request"]
    #[inline]
    pub fn sysresetreq(&mut self) -> _SYSRESETREQW {
        _SYSRESETREQW { w: self }
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field"]
    #[inline]
    pub fn prigroup(&mut self) -> _PRIGROUPW {
        _PRIGROUPW { w: self }
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline]
    pub fn vectkey(&mut self) -> _VECTKEYW {
        _VECTKEYW { w: self }
    }
}
