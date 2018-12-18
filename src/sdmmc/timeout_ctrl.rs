#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TIMEOUT_CTRL {
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
#[doc = "Possible values of the field `DAT_TIMEOUT_CNT_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_TIMEOUT_CNT_VALR {
    #[doc = "TMCLK * 2^13"]
    VALUE1,
    #[doc = "TMCLK * 2^14"]
    VALUE2,
    #[doc = "TMCLK * 2^27"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DAT_TIMEOUT_CNT_VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DAT_TIMEOUT_CNT_VALR::VALUE1 => 0,
            DAT_TIMEOUT_CNT_VALR::VALUE2 => 1,
            DAT_TIMEOUT_CNT_VALR::VALUE3 => 14,
            DAT_TIMEOUT_CNT_VALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DAT_TIMEOUT_CNT_VALR {
        match value {
            0 => DAT_TIMEOUT_CNT_VALR::VALUE1,
            1 => DAT_TIMEOUT_CNT_VALR::VALUE2,
            14 => DAT_TIMEOUT_CNT_VALR::VALUE3,
            i => DAT_TIMEOUT_CNT_VALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DAT_TIMEOUT_CNT_VALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DAT_TIMEOUT_CNT_VALR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DAT_TIMEOUT_CNT_VALR::VALUE3
    }
}
#[doc = "Values that can be written to the field `DAT_TIMEOUT_CNT_VAL`"]
pub enum DAT_TIMEOUT_CNT_VALW {
    #[doc = "TMCLK * 2^13"]
    VALUE1,
    #[doc = "TMCLK * 2^14"]
    VALUE2,
    #[doc = "TMCLK * 2^27"]
    VALUE3,
}
impl DAT_TIMEOUT_CNT_VALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAT_TIMEOUT_CNT_VALW::VALUE1 => 0,
            DAT_TIMEOUT_CNT_VALW::VALUE2 => 1,
            DAT_TIMEOUT_CNT_VALW::VALUE3 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAT_TIMEOUT_CNT_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _DAT_TIMEOUT_CNT_VALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAT_TIMEOUT_CNT_VALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TMCLK * 2^13"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DAT_TIMEOUT_CNT_VALW::VALUE1)
    }
    #[doc = "TMCLK * 2^14"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DAT_TIMEOUT_CNT_VALW::VALUE2)
    }
    #[doc = "TMCLK * 2^27"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DAT_TIMEOUT_CNT_VALW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline]
    pub fn dat_timeout_cnt_val(&self) -> DAT_TIMEOUT_CNT_VALR {
        DAT_TIMEOUT_CNT_VALR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline]
    pub fn dat_timeout_cnt_val(&mut self) -> _DAT_TIMEOUT_CNT_VALW {
        _DAT_TIMEOUT_CNT_VALW { w: self }
    }
}
