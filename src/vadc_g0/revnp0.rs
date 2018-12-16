#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REVNP0 {
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
#[doc = "Possible values of the field `REV0NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV0NPR {
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
#[doc = "Possible values of the field `REV1NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV1NPR {
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
impl REV1NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV1NPR::VALUE1 => 0,
            REV1NPR::VALUE2 => 3,
            REV1NPR::VALUE3 => 4,
            REV1NPR::VALUE4 => 7,
            REV1NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV1NPR {
        match value {
            0 => REV1NPR::VALUE1,
            3 => REV1NPR::VALUE2,
            4 => REV1NPR::VALUE3,
            7 => REV1NPR::VALUE4,
            i => REV1NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV1NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV1NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV1NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV1NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV2NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV2NPR {
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
impl REV2NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV2NPR::VALUE1 => 0,
            REV2NPR::VALUE2 => 3,
            REV2NPR::VALUE3 => 4,
            REV2NPR::VALUE4 => 7,
            REV2NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV2NPR {
        match value {
            0 => REV2NPR::VALUE1,
            3 => REV2NPR::VALUE2,
            4 => REV2NPR::VALUE3,
            7 => REV2NPR::VALUE4,
            i => REV2NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV2NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV2NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV2NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV2NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV3NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV3NPR {
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
impl REV3NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV3NPR::VALUE1 => 0,
            REV3NPR::VALUE2 => 3,
            REV3NPR::VALUE3 => 4,
            REV3NPR::VALUE4 => 7,
            REV3NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV3NPR {
        match value {
            0 => REV3NPR::VALUE1,
            3 => REV3NPR::VALUE2,
            4 => REV3NPR::VALUE3,
            7 => REV3NPR::VALUE4,
            i => REV3NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV3NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV3NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV3NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV3NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV4NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV4NPR {
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
impl REV4NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV4NPR::VALUE1 => 0,
            REV4NPR::VALUE2 => 3,
            REV4NPR::VALUE3 => 4,
            REV4NPR::VALUE4 => 7,
            REV4NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV4NPR {
        match value {
            0 => REV4NPR::VALUE1,
            3 => REV4NPR::VALUE2,
            4 => REV4NPR::VALUE3,
            7 => REV4NPR::VALUE4,
            i => REV4NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV4NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV4NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV4NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV4NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV5NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV5NPR {
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
impl REV5NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV5NPR::VALUE1 => 0,
            REV5NPR::VALUE2 => 3,
            REV5NPR::VALUE3 => 4,
            REV5NPR::VALUE4 => 7,
            REV5NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV5NPR {
        match value {
            0 => REV5NPR::VALUE1,
            3 => REV5NPR::VALUE2,
            4 => REV5NPR::VALUE3,
            7 => REV5NPR::VALUE4,
            i => REV5NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV5NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV5NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV5NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV5NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV6NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV6NPR {
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
impl REV6NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV6NPR::VALUE1 => 0,
            REV6NPR::VALUE2 => 3,
            REV6NPR::VALUE3 => 4,
            REV6NPR::VALUE4 => 7,
            REV6NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV6NPR {
        match value {
            0 => REV6NPR::VALUE1,
            3 => REV6NPR::VALUE2,
            4 => REV6NPR::VALUE3,
            7 => REV6NPR::VALUE4,
            i => REV6NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV6NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV6NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV6NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV6NPR::VALUE4
    }
}
#[doc = "Possible values of the field `REV7NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV7NPR {
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
impl REV7NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV7NPR::VALUE1 => 0,
            REV7NPR::VALUE2 => 3,
            REV7NPR::VALUE3 => 4,
            REV7NPR::VALUE4 => 7,
            REV7NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV7NPR {
        match value {
            0 => REV7NPR::VALUE1,
            3 => REV7NPR::VALUE2,
            4 => REV7NPR::VALUE3,
            7 => REV7NPR::VALUE4,
            i => REV7NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REV7NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REV7NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == REV7NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == REV7NPR::VALUE4
    }
}
#[doc = "Values that can be written to the field `REV0NP`"]
pub enum REV0NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
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
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV0NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV0NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV0NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV0NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV1NP`"]
pub enum REV1NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV1NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV1NPW::VALUE1 => 0,
            REV1NPW::VALUE2 => 3,
            REV1NPW::VALUE3 => 4,
            REV1NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV1NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV1NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV1NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV1NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV1NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV1NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV1NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV2NP`"]
pub enum REV2NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV2NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV2NPW::VALUE1 => 0,
            REV2NPW::VALUE2 => 3,
            REV2NPW::VALUE3 => 4,
            REV2NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV2NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV2NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV2NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV2NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV2NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV2NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV2NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV3NP`"]
pub enum REV3NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV3NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV3NPW::VALUE1 => 0,
            REV3NPW::VALUE2 => 3,
            REV3NPW::VALUE3 => 4,
            REV3NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV3NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV3NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV3NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV3NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV3NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV3NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV3NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV4NP`"]
pub enum REV4NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV4NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV4NPW::VALUE1 => 0,
            REV4NPW::VALUE2 => 3,
            REV4NPW::VALUE3 => 4,
            REV4NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV4NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV4NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV4NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV4NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV4NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV4NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV4NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV5NP`"]
pub enum REV5NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV5NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV5NPW::VALUE1 => 0,
            REV5NPW::VALUE2 => 3,
            REV5NPW::VALUE3 => 4,
            REV5NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV5NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV5NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV5NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV5NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV5NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV5NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV5NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV6NP`"]
pub enum REV6NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV6NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV6NPW::VALUE1 => 0,
            REV6NPW::VALUE2 => 3,
            REV6NPW::VALUE3 => 4,
            REV6NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV6NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV6NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV6NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV6NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV6NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV6NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV6NPW::VALUE4)
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
#[doc = "Values that can be written to the field `REV7NP`"]
pub enum REV7NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl REV7NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV7NPW::VALUE1 => 0,
            REV7NPW::VALUE2 => 3,
            REV7NPW::VALUE3 => 4,
            REV7NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV7NPW<'a> {
    w: &'a mut W,
}
impl<'a> _REV7NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV7NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV7NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV7NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(REV7NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(REV7NPW::VALUE4)
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
    pub fn rev0np(&self) -> REV0NPR {
        REV0NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev1np(&self) -> REV1NPR {
        REV1NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev2np(&self) -> REV2NPR {
        REV2NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev3np(&self) -> REV3NPR {
        REV3NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev4np(&self) -> REV4NPR {
        REV4NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev5np(&self) -> REV5NPR {
        REV5NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev6np(&self) -> REV6NPR {
        REV6NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev7np(&self) -> REV7NPR {
        REV7NPR::_from({
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
    pub fn rev0np(&mut self) -> _REV0NPW {
        _REV0NPW { w: self }
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev1np(&mut self) -> _REV1NPW {
        _REV1NPW { w: self }
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev2np(&mut self) -> _REV2NPW {
        _REV2NPW { w: self }
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev3np(&mut self) -> _REV3NPW {
        _REV3NPW { w: self }
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev4np(&mut self) -> _REV4NPW {
        _REV4NPW { w: self }
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev5np(&mut self) -> _REV5NPW {
        _REV5NPW { w: self }
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev6np(&mut self) -> _REV6NPW {
        _REV6NPW { w: self }
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Result Event i"]
    #[inline]
    pub fn rev7np(&mut self) -> _REV7NPW {
        _REV7NPW { w: self }
    }
}
