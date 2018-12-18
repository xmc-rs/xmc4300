#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBEVNP {
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
#[doc = "Possible values of the field `SEV0NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV0NPR {
    #[doc = "Select shared service request line 0 of common service request group 0"]
    VALUE1,
    #[doc = "Select shared service request line 3 of common service request group 0"]
    VALUE2,
    #[doc = "Select shared service request line 0 of common service request group 1"]
    VALUE3,
    #[doc = "Select shared service request line 3 of common service request group 1"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEV0NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEV0NPR::VALUE1 => 0,
            SEV0NPR::VALUE2 => 3,
            SEV0NPR::VALUE3 => 4,
            SEV0NPR::VALUE4 => 7,
            SEV0NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEV0NPR {
        match value {
            0 => SEV0NPR::VALUE1,
            3 => SEV0NPR::VALUE2,
            4 => SEV0NPR::VALUE3,
            7 => SEV0NPR::VALUE4,
            i => SEV0NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEV0NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEV0NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SEV0NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SEV0NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV0NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV0NPR {
    #[doc = "Select shared service request line 0 of common service request group 0"]
    VALUE1,
    #[doc = "Select shared service request line 3 of common service request group 0"]
    VALUE2,
    #[doc = "Select shared service request line 0 of common service request group 1"]
    VALUE3,
    #[doc = "Select shared service request line 3 of common service request group 1"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REV0NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV0NPR::VALUE1 => 0,
            REV0NPR::VALUE2 => 3,
            REV0NPR::VALUE3 => 4,
            REV0NPR::VALUE4 => 7,
            REV0NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV0NPR {
        match value {
            0 => REV0NPR::VALUE1,
            3 => REV0NPR::VALUE2,
            4 => REV0NPR::VALUE3,
            7 => REV0NPR::VALUE4,
            i => REV0NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV0NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV0NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV0NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV0NPR::VALUE4
    }
}
#[doc = "Values that can be written to the field `SEV0NP`"]
pub enum SEV0NPW {
    #[doc = "Select shared service request line 0 of common service request group 0"]
    VALUE1,
    #[doc = "Select shared service request line 3 of common service request group 0"]
    VALUE2,
    #[doc = "Select shared service request line 0 of common service request group 1"]
    VALUE3,
    #[doc = "Select shared service request line 3 of common service request group 1"]
    VALUE4,
}
impl SEV0NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEV0NPW::VALUE1 => 0,
            SEV0NPW::VALUE2 => 3,
            SEV0NPW::VALUE3 => 4,
            SEV0NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEV0NPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEV0NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEV0NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0NPW::VALUE1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SEV0NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SEV0NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV0NP`"]
pub enum REV0NPW {
    #[doc = "Select shared service request line 0 of common service request group 0"]
    VALUE1,
    #[doc = "Select shared service request line 3 of common service request group 0"]
    VALUE2,
    #[doc = "Select shared service request line 0 of common service request group 1"]
    VALUE3,
    #[doc = "Select shared service request line 3 of common service request group 1"]
    VALUE4,
}
impl REV0NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV0NPW::VALUE1 => 0,
            REV0NPW::VALUE2 => 3,
            REV0NPW::VALUE3 => 4,
            REV0NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV0NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV0NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV0NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select shared service request line 0 of common service request group 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV0NPW::VALUE1)
    }
    #[doc = "Select shared service request line 3 of common service request group 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV0NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0 of common service request group 1"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV0NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3 of common service request group 1"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV0NPW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Service Request Node Pointer Backgr. Source"]
    #[inline]
    pub fn sev0np(&self) -> SEV0NPR {
        SEV0NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Backgr. Result"]
    #[inline]
    pub fn rev0np(&self) -> REV0NPR {
        REV0NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3 - Service Request Node Pointer Backgr. Source"]
    #[inline]
    pub fn sev0np(&mut self) -> _SEV0NPW {
        _SEV0NPW { w: self }
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Backgr. Result"]
    #[inline]
    pub fn rev0np(&mut self) -> _REV0NPW {
        _REV0NPW { w: self }
    }
}
