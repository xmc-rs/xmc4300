#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REVNP1 {
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
#[doc = "Possible values of the field `REV8NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV8NPR {
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
impl REV8NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV8NPR::VALUE1 => 0,
            REV8NPR::VALUE2 => 3,
            REV8NPR::VALUE3 => 4,
            REV8NPR::VALUE4 => 7,
            REV8NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV8NPR {
        match value {
            0 => REV8NPR::VALUE1,
            3 => REV8NPR::VALUE2,
            4 => REV8NPR::VALUE3,
            7 => REV8NPR::VALUE4,
            i => REV8NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV8NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV8NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV8NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV8NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV9NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV9NPR {
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
impl REV9NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV9NPR::VALUE1 => 0,
            REV9NPR::VALUE2 => 3,
            REV9NPR::VALUE3 => 4,
            REV9NPR::VALUE4 => 7,
            REV9NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV9NPR {
        match value {
            0 => REV9NPR::VALUE1,
            3 => REV9NPR::VALUE2,
            4 => REV9NPR::VALUE3,
            7 => REV9NPR::VALUE4,
            i => REV9NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV9NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV9NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV9NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV9NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV10NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV10NPR {
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
impl REV10NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV10NPR::VALUE1 => 0,
            REV10NPR::VALUE2 => 3,
            REV10NPR::VALUE3 => 4,
            REV10NPR::VALUE4 => 7,
            REV10NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV10NPR {
        match value {
            0 => REV10NPR::VALUE1,
            3 => REV10NPR::VALUE2,
            4 => REV10NPR::VALUE3,
            7 => REV10NPR::VALUE4,
            i => REV10NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV10NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV10NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV10NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV10NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV11NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV11NPR {
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
impl REV11NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV11NPR::VALUE1 => 0,
            REV11NPR::VALUE2 => 3,
            REV11NPR::VALUE3 => 4,
            REV11NPR::VALUE4 => 7,
            REV11NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV11NPR {
        match value {
            0 => REV11NPR::VALUE1,
            3 => REV11NPR::VALUE2,
            4 => REV11NPR::VALUE3,
            7 => REV11NPR::VALUE4,
            i => REV11NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV11NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV11NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV11NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV11NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV12NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV12NPR {
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
impl REV12NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV12NPR::VALUE1 => 0,
            REV12NPR::VALUE2 => 3,
            REV12NPR::VALUE3 => 4,
            REV12NPR::VALUE4 => 7,
            REV12NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV12NPR {
        match value {
            0 => REV12NPR::VALUE1,
            3 => REV12NPR::VALUE2,
            4 => REV12NPR::VALUE3,
            7 => REV12NPR::VALUE4,
            i => REV12NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV12NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV12NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV12NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV12NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV13NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV13NPR {
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
impl REV13NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV13NPR::VALUE1 => 0,
            REV13NPR::VALUE2 => 3,
            REV13NPR::VALUE3 => 4,
            REV13NPR::VALUE4 => 7,
            REV13NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV13NPR {
        match value {
            0 => REV13NPR::VALUE1,
            3 => REV13NPR::VALUE2,
            4 => REV13NPR::VALUE3,
            7 => REV13NPR::VALUE4,
            i => REV13NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV13NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV13NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV13NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV13NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV14NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV14NPR {
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
impl REV14NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV14NPR::VALUE1 => 0,
            REV14NPR::VALUE2 => 3,
            REV14NPR::VALUE3 => 4,
            REV14NPR::VALUE4 => 7,
            REV14NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV14NPR {
        match value {
            0 => REV14NPR::VALUE1,
            3 => REV14NPR::VALUE2,
            4 => REV14NPR::VALUE3,
            7 => REV14NPR::VALUE4,
            i => REV14NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV14NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV14NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV14NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV14NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV15NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV15NPR {
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
impl REV15NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV15NPR::VALUE1 => 0,
            REV15NPR::VALUE2 => 3,
            REV15NPR::VALUE3 => 4,
            REV15NPR::VALUE4 => 7,
            REV15NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV15NPR {
        match value {
            0 => REV15NPR::VALUE1,
            3 => REV15NPR::VALUE2,
            4 => REV15NPR::VALUE3,
            7 => REV15NPR::VALUE4,
            i => REV15NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV15NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV15NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV15NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV15NPR::VALUE4
    }
}
#[doc = "Values that can be written to the field `REV8NP`"]
pub enum REV8NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV8NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV8NPW::VALUE1 => 0,
            REV8NPW::VALUE2 => 3,
            REV8NPW::VALUE3 => 4,
            REV8NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV8NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV8NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV8NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV8NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV8NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV8NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV8NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV9NP`"]
pub enum REV9NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV9NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV9NPW::VALUE1 => 0,
            REV9NPW::VALUE2 => 3,
            REV9NPW::VALUE3 => 4,
            REV9NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV9NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV9NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV9NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV9NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV9NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV9NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV9NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV10NP`"]
pub enum REV10NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV10NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV10NPW::VALUE1 => 0,
            REV10NPW::VALUE2 => 3,
            REV10NPW::VALUE3 => 4,
            REV10NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV10NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV10NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV10NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV10NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV10NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV10NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV10NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV11NP`"]
pub enum REV11NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV11NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV11NPW::VALUE1 => 0,
            REV11NPW::VALUE2 => 3,
            REV11NPW::VALUE3 => 4,
            REV11NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV11NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV11NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV11NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV11NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV11NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV11NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV11NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV12NP`"]
pub enum REV12NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV12NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV12NPW::VALUE1 => 0,
            REV12NPW::VALUE2 => 3,
            REV12NPW::VALUE3 => 4,
            REV12NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV12NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV12NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV12NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV12NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV12NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV12NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV12NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV13NP`"]
pub enum REV13NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV13NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV13NPW::VALUE1 => 0,
            REV13NPW::VALUE2 => 3,
            REV13NPW::VALUE3 => 4,
            REV13NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV13NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV13NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV13NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV13NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV13NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV13NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV13NPW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REV14NP`"]
pub enum REV14NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV14NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV14NPW::VALUE1 => 0,
            REV14NPW::VALUE2 => 3,
            REV14NPW::VALUE3 => 4,
            REV14NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV14NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV14NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV14NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV14NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV14NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV14NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV14NPW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REV15NP`"]
pub enum REV15NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV15NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV15NPW::VALUE1 => 0,
            REV15NPW::VALUE2 => 3,
            REV15NPW::VALUE3 => 4,
            REV15NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV15NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV15NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV15NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV15NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV15NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV15NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV15NPW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev8np(&self) -> REV8NPR {
        REV8NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev9np(&self) -> REV9NPR {
        REV9NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev10np(&self) -> REV10NPR {
        REV10NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev11np(&self) -> REV11NPR {
        REV11NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev12np(&self) -> REV12NPR {
        REV12NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev13np(&self) -> REV13NPR {
        REV13NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev14np(&self) -> REV14NPR {
        REV14NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev15np(&self) -> REV15NPR {
        REV15NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev8np(&mut self) -> _REV8NPW {
        _REV8NPW { w: self }
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev9np(&mut self) -> _REV9NPW {
        _REV9NPW { w: self }
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev10np(&mut self) -> _REV10NPW {
        _REV10NPW { w: self }
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev11np(&mut self) -> _REV11NPW {
        _REV11NPW { w: self }
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev12np(&mut self) -> _REV12NPW {
        _REV12NPW { w: self }
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev13np(&mut self) -> _REV13NPW {
        _REV13NPW { w: self }
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev14np(&mut self) -> _REV14NPW {
        _REV14NPW { w: self }
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev15np(&mut self) -> _REV15NPW {
        _REV15NPW { w: self }
    }
}
