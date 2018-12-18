#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHENREG {
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
#[doc = "Possible values of the field `CH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHR {
    #[doc = "Disable the Channel"]
    VALUE1,
    #[doc = "Enable the Channel"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHR::VALUE1 => 0,
            CHR::VALUE2 => 1,
            CHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHR {
        match value {
            0 => CHR::VALUE1,
            1 => CHR::VALUE2,
            i => CHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _WE_CHW<'a> {
    w: &'a mut W,
}
impl<'a> _WE_CHW<'a> {
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
#[doc = "Values that can be written to the field `CH`"]
pub enum CHW {
    #[doc = "Disable the Channel"]
    VALUE1,
    #[doc = "Enable the Channel"]
    VALUE2,
}
impl CHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHW::VALUE1 => 0,
            CHW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHW<'a> {
    w: &'a mut W,
}
impl<'a> _CHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable the Channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHW::VALUE1)
    }
    #[doc = "Enable the Channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHW::VALUE2)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline]
    pub fn ch(&self) -> CHR {
        CHR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 8:15 - Channel enable write enable"]
    #[inline]
    pub fn we_ch(&mut self) -> _WE_CHW {
        _WE_CHW { w: self }
    }
    #[doc = "Bits 0:7 - Enables/Disables the channel"]
    #[inline]
    pub fn ch(&mut self) -> _CHW {
        _CHW { w: self }
    }
}
