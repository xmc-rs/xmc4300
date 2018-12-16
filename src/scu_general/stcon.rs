#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STCON {
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
#[doc = "Possible values of the field `HWCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWCONR {
    #[doc = "Normal mode, JTAG"]
    CONST_00,
    #[doc = "ASC BSL enabled"]
    CONST_01,
    #[doc = "BMI customized boot enabled"]
    CONST_10,
    #[doc = "CAN BSL enabled"]
    CONST_11,
}
impl HWCONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HWCONR::CONST_00 => 0,
            HWCONR::CONST_01 => 1,
            HWCONR::CONST_10 => 2,
            HWCONR::CONST_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HWCONR {
        match value {
            0 => HWCONR::CONST_00,
            1 => HWCONR::CONST_01,
            2 => HWCONR::CONST_10,
            3 => HWCONR::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline]
    pub fn is_const_00(&self) -> bool {
        *self == HWCONR::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline]
    pub fn is_const_01(&self) -> bool {
        *self == HWCONR::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline]
    pub fn is_const_10(&self) -> bool {
        *self == HWCONR::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline]
    pub fn is_const_11(&self) -> bool {
        *self == HWCONR::CONST_11
    }
}
#[doc = "Possible values of the field `SWCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWCONR {
    #[doc = "Normal mode, boot from Boot ROM"]
    CONST_0000,
    #[doc = "ASC BSL enabled"]
    CONST_0001,
    #[doc = "BMI customized boot enabled"]
    CONST_0010,
    #[doc = "CAN BSL enabled"]
    CONST_0011,
    #[doc = "Boot from Code SRAM"]
    CONST_0100,
    #[doc = "Boot from alternate Flash Address 0"]
    CONST_1000,
    #[doc = "Boot from alternate Flash Address 1"]
    CONST_1100,
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    CONST_1110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SWCONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWCONR::CONST_0000 => 0,
            SWCONR::CONST_0001 => 1,
            SWCONR::CONST_0010 => 2,
            SWCONR::CONST_0011 => 3,
            SWCONR::CONST_0100 => 4,
            SWCONR::CONST_1000 => 8,
            SWCONR::CONST_1100 => 12,
            SWCONR::CONST_1110 => 14,
            SWCONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWCONR {
        match value {
            0 => SWCONR::CONST_0000,
            1 => SWCONR::CONST_0001,
            2 => SWCONR::CONST_0010,
            3 => SWCONR::CONST_0011,
            4 => SWCONR::CONST_0100,
            8 => SWCONR::CONST_1000,
            12 => SWCONR::CONST_1100,
            14 => SWCONR::CONST_1110,
            i => SWCONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0000`"]
    #[inline]
    pub fn is_const_0000(&self) -> bool {
        *self == SWCONR::CONST_0000
    }
    #[doc = "Checks if the value of the field is `CONST_0001`"]
    #[inline]
    pub fn is_const_0001(&self) -> bool {
        *self == SWCONR::CONST_0001
    }
    #[doc = "Checks if the value of the field is `CONST_0010`"]
    #[inline]
    pub fn is_const_0010(&self) -> bool {
        *self == SWCONR::CONST_0010
    }
    #[doc = "Checks if the value of the field is `CONST_0011`"]
    #[inline]
    pub fn is_const_0011(&self) -> bool {
        *self == SWCONR::CONST_0011
    }
    #[doc = "Checks if the value of the field is `CONST_0100`"]
    #[inline]
    pub fn is_const_0100(&self) -> bool {
        *self == SWCONR::CONST_0100
    }
    #[doc = "Checks if the value of the field is `CONST_1000`"]
    #[inline]
    pub fn is_const_1000(&self) -> bool {
        *self == SWCONR::CONST_1000
    }
    #[doc = "Checks if the value of the field is `CONST_1100`"]
    #[inline]
    pub fn is_const_1100(&self) -> bool {
        *self == SWCONR::CONST_1100
    }
    #[doc = "Checks if the value of the field is `CONST_1110`"]
    #[inline]
    pub fn is_const_1110(&self) -> bool {
        *self == SWCONR::CONST_1110
    }
}
#[doc = "Values that can be written to the field `SWCON`"]
pub enum SWCONW {
    #[doc = "Normal mode, boot from Boot ROM"]
    CONST_0000,
    #[doc = "ASC BSL enabled"]
    CONST_0001,
    #[doc = "BMI customized boot enabled"]
    CONST_0010,
    #[doc = "CAN BSL enabled"]
    CONST_0011,
    #[doc = "Boot from Code SRAM"]
    CONST_0100,
    #[doc = "Boot from alternate Flash Address 0"]
    CONST_1000,
    #[doc = "Boot from alternate Flash Address 1"]
    CONST_1100,
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    CONST_1110,
}
impl SWCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWCONW::CONST_0000 => 0,
            SWCONW::CONST_0001 => 1,
            SWCONW::CONST_0010 => 2,
            SWCONW::CONST_0011 => 3,
            SWCONW::CONST_0100 => 4,
            SWCONW::CONST_1000 => 8,
            SWCONW::CONST_1100 => 12,
            SWCONW::CONST_1110 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWCONW<'a> {
    w: &'a mut W,
}
impl<'a> _SWCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWCONW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline]
    pub fn const_0000(self) -> &'a mut W {
        self.variant(SWCONW::CONST_0000)
    }
    #[doc = "ASC BSL enabled"]
    #[inline]
    pub fn const_0001(self) -> &'a mut W {
        self.variant(SWCONW::CONST_0001)
    }
    #[doc = "BMI customized boot enabled"]
    #[inline]
    pub fn const_0010(self) -> &'a mut W {
        self.variant(SWCONW::CONST_0010)
    }
    #[doc = "CAN BSL enabled"]
    #[inline]
    pub fn const_0011(self) -> &'a mut W {
        self.variant(SWCONW::CONST_0011)
    }
    #[doc = "Boot from Code SRAM"]
    #[inline]
    pub fn const_0100(self) -> &'a mut W {
        self.variant(SWCONW::CONST_0100)
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline]
    pub fn const_1000(self) -> &'a mut W {
        self.variant(SWCONW::CONST_1000)
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline]
    pub fn const_1100(self) -> &'a mut W {
        self.variant(SWCONW::CONST_1100)
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline]
    pub fn const_1110(self) -> &'a mut W {
        self.variant(SWCONW::CONST_1110)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - HW Configuration"]
    #[inline]
    pub fn hwcon(&self) -> HWCONR {
        HWCONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline]
    pub fn swcon(&self) -> SWCONR {
        SWCONR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline]
    pub fn swcon(&mut self) -> _SWCONW {
        _SWCONW { w: self }
    }
}
