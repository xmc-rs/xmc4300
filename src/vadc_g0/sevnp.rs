#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEVNP {
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
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
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
#[doc = "Possible values of the field `SEV1NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEV1NPR {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEV1NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEV1NPR::VALUE1 => 0,
            SEV1NPR::VALUE2 => 3,
            SEV1NPR::VALUE3 => 4,
            SEV1NPR::VALUE4 => 7,
            SEV1NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEV1NPR {
        match value {
            0 => SEV1NPR::VALUE1,
            3 => SEV1NPR::VALUE2,
            4 => SEV1NPR::VALUE3,
            7 => SEV1NPR::VALUE4,
            i => SEV1NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SEV1NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SEV1NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SEV1NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SEV1NPR::VALUE4
    }
}
#[doc = "Values that can be written to the field `SEV0NP`"]
pub enum SEV0NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
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
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV0NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV0NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SEV0NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
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
#[doc = "Values that can be written to the field `SEV1NP`"]
pub enum SEV1NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl SEV1NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEV1NPW::VALUE1 => 0,
            SEV1NPW::VALUE2 => 3,
            SEV1NPW::VALUE3 => 4,
            SEV1NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEV1NPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEV1NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEV1NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEV1NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEV1NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SEV1NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SEV1NPW::VALUE4)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Service Request Node Pointer Source Event i"]
    #[inline]
    pub fn sev0np(&self) -> SEV0NPR {
        SEV0NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Source Event i"]
    #[inline]
    pub fn sev1np(&self) -> SEV1NPR {
        SEV1NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - Service Request Node Pointer Source Event i"]
    #[inline]
    pub fn sev0np(&mut self) -> _SEV0NPW {
        _SEV0NPW { w: self }
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Source Event i"]
    #[inline]
    pub fn sev1np(&mut self) -> _SEV1NPW {
        _SEV1NPW { w: self }
    }
}
