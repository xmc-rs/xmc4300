#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEP_ADR {
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
#[doc = "Possible values of the field `EEPROM_ADDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEPROM_ADDRR {
    #[doc = "First word (= 16 bits)"]
    VALUE1,
    #[doc = "Second word"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl EEPROM_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            EEPROM_ADDRR::VALUE1 => 0,
            EEPROM_ADDRR::VALUE2 => 1,
            EEPROM_ADDRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> EEPROM_ADDRR {
        match value {
            0 => EEPROM_ADDRR::VALUE1,
            1 => EEPROM_ADDRR::VALUE2,
            i => EEPROM_ADDRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EEPROM_ADDRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EEPROM_ADDRR::VALUE2
    }
}
#[doc = "Values that can be written to the field `EEPROM_ADDR`"]
pub enum EEPROM_ADDRW {
    #[doc = "First word (= 16 bits)"]
    VALUE1,
    #[doc = "Second word"]
    VALUE2,
}
impl EEPROM_ADDRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            EEPROM_ADDRW::VALUE1 => 0,
            EEPROM_ADDRW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEPROM_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_ADDRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEPROM_ADDRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "First word (= 16 bits)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EEPROM_ADDRW::VALUE1)
    }
    #[doc = "Second word"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EEPROM_ADDRW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline]
    pub fn eeprom_addr(&self) -> EEPROM_ADDRR {
        EEPROM_ADDRR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - EEPROM Address"]
    #[inline]
    pub fn eeprom_addr(&mut self) -> _EEPROM_ADDRW {
        _EEPROM_ADDRW { w: self }
    }
}
