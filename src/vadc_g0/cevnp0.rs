#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CEVNP0 {
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
#[doc = "Possible values of the field `CEV0NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV0NPR {
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
impl CEV0NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CEV0NPR::VALUE1 => 0,
            CEV0NPR::VALUE2 => 3,
            CEV0NPR::VALUE3 => 4,
            CEV0NPR::VALUE4 => 7,
            CEV0NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CEV0NPR {
        match value {
            0 => CEV0NPR::VALUE1,
            3 => CEV0NPR::VALUE2,
            4 => CEV0NPR::VALUE3,
            7 => CEV0NPR::VALUE4,
            i => CEV0NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV0NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV0NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CEV0NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CEV0NPR::VALUE4
    }
}
#[doc = "Possible values of the field `CEV1NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV1NPR {
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
impl CEV1NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CEV1NPR::VALUE1 => 0,
            CEV1NPR::VALUE2 => 3,
            CEV1NPR::VALUE3 => 4,
            CEV1NPR::VALUE4 => 7,
            CEV1NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CEV1NPR {
        match value {
            0 => CEV1NPR::VALUE1,
            3 => CEV1NPR::VALUE2,
            4 => CEV1NPR::VALUE3,
            7 => CEV1NPR::VALUE4,
            i => CEV1NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV1NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV1NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CEV1NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CEV1NPR::VALUE4
    }
}
#[doc = "Possible values of the field `CEV2NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV2NPR {
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
impl CEV2NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CEV2NPR::VALUE1 => 0,
            CEV2NPR::VALUE2 => 3,
            CEV2NPR::VALUE3 => 4,
            CEV2NPR::VALUE4 => 7,
            CEV2NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CEV2NPR {
        match value {
            0 => CEV2NPR::VALUE1,
            3 => CEV2NPR::VALUE2,
            4 => CEV2NPR::VALUE3,
            7 => CEV2NPR::VALUE4,
            i => CEV2NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV2NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV2NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CEV2NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CEV2NPR::VALUE4
    }
}
#[doc = "Possible values of the field `CEV3NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV3NPR {
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
impl CEV3NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CEV3NPR::VALUE1 => 0,
            CEV3NPR::VALUE2 => 3,
            CEV3NPR::VALUE3 => 4,
            CEV3NPR::VALUE4 => 7,
            CEV3NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CEV3NPR {
        match value {
            0 => CEV3NPR::VALUE1,
            3 => CEV3NPR::VALUE2,
            4 => CEV3NPR::VALUE3,
            7 => CEV3NPR::VALUE4,
            i => CEV3NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV3NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV3NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CEV3NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CEV3NPR::VALUE4
    }
}
#[doc = "Possible values of the field `CEV4NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV4NPR {
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
impl CEV4NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CEV4NPR::VALUE1 => 0,
            CEV4NPR::VALUE2 => 3,
            CEV4NPR::VALUE3 => 4,
            CEV4NPR::VALUE4 => 7,
            CEV4NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CEV4NPR {
        match value {
            0 => CEV4NPR::VALUE1,
            3 => CEV4NPR::VALUE2,
            4 => CEV4NPR::VALUE3,
            7 => CEV4NPR::VALUE4,
            i => CEV4NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV4NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV4NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CEV4NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CEV4NPR::VALUE4
    }
}
#[doc = "Possible values of the field `CEV5NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV5NPR {
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
impl CEV5NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CEV5NPR::VALUE1 => 0,
            CEV5NPR::VALUE2 => 3,
            CEV5NPR::VALUE3 => 4,
            CEV5NPR::VALUE4 => 7,
            CEV5NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CEV5NPR {
        match value {
            0 => CEV5NPR::VALUE1,
            3 => CEV5NPR::VALUE2,
            4 => CEV5NPR::VALUE3,
            7 => CEV5NPR::VALUE4,
            i => CEV5NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV5NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV5NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CEV5NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CEV5NPR::VALUE4
    }
}
#[doc = "Possible values of the field `CEV6NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV6NPR {
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
impl CEV6NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CEV6NPR::VALUE1 => 0,
            CEV6NPR::VALUE2 => 3,
            CEV6NPR::VALUE3 => 4,
            CEV6NPR::VALUE4 => 7,
            CEV6NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CEV6NPR {
        match value {
            0 => CEV6NPR::VALUE1,
            3 => CEV6NPR::VALUE2,
            4 => CEV6NPR::VALUE3,
            7 => CEV6NPR::VALUE4,
            i => CEV6NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV6NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV6NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CEV6NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CEV6NPR::VALUE4
    }
}
#[doc = "Possible values of the field `CEV7NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEV7NPR {
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
impl CEV7NPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CEV7NPR::VALUE1 => 0,
            CEV7NPR::VALUE2 => 3,
            CEV7NPR::VALUE3 => 4,
            CEV7NPR::VALUE4 => 7,
            CEV7NPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CEV7NPR {
        match value {
            0 => CEV7NPR::VALUE1,
            3 => CEV7NPR::VALUE2,
            4 => CEV7NPR::VALUE3,
            7 => CEV7NPR::VALUE4,
            i => CEV7NPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CEV7NPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CEV7NPR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CEV7NPR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CEV7NPR::VALUE4
    }
}
#[doc = "Values that can be written to the field `CEV0NP`"]
pub enum CEV0NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl CEV0NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CEV0NPW::VALUE1 => 0,
            CEV0NPW::VALUE2 => 3,
            CEV0NPW::VALUE3 => 4,
            CEV0NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV0NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CEV0NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV0NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV0NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV0NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV0NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV0NPW::VALUE4)
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
#[doc = "Values that can be written to the field `CEV1NP`"]
pub enum CEV1NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl CEV1NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CEV1NPW::VALUE1 => 0,
            CEV1NPW::VALUE2 => 3,
            CEV1NPW::VALUE3 => 4,
            CEV1NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV1NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CEV1NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV1NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV1NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV1NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV1NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV1NPW::VALUE4)
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
#[doc = "Values that can be written to the field `CEV2NP`"]
pub enum CEV2NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl CEV2NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CEV2NPW::VALUE1 => 0,
            CEV2NPW::VALUE2 => 3,
            CEV2NPW::VALUE3 => 4,
            CEV2NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV2NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CEV2NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV2NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV2NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV2NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV2NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV2NPW::VALUE4)
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
#[doc = "Values that can be written to the field `CEV3NP`"]
pub enum CEV3NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl CEV3NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CEV3NPW::VALUE1 => 0,
            CEV3NPW::VALUE2 => 3,
            CEV3NPW::VALUE3 => 4,
            CEV3NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV3NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CEV3NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV3NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV3NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV3NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV3NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV3NPW::VALUE4)
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
#[doc = "Values that can be written to the field `CEV4NP`"]
pub enum CEV4NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl CEV4NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CEV4NPW::VALUE1 => 0,
            CEV4NPW::VALUE2 => 3,
            CEV4NPW::VALUE3 => 4,
            CEV4NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV4NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CEV4NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV4NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV4NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV4NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV4NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV4NPW::VALUE4)
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
#[doc = "Values that can be written to the field `CEV5NP`"]
pub enum CEV5NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl CEV5NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CEV5NPW::VALUE1 => 0,
            CEV5NPW::VALUE2 => 3,
            CEV5NPW::VALUE3 => 4,
            CEV5NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV5NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CEV5NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV5NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV5NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV5NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV5NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV5NPW::VALUE4)
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
#[doc = "Values that can be written to the field `CEV6NP`"]
pub enum CEV6NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl CEV6NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CEV6NPW::VALUE1 => 0,
            CEV6NPW::VALUE2 => 3,
            CEV6NPW::VALUE3 => 4,
            CEV6NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV6NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CEV6NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV6NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV6NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV6NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV6NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV6NPW::VALUE4)
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
#[doc = "Values that can be written to the field `CEV7NP`"]
pub enum CEV7NPW {
    #[doc = "Select service request line 0 of group x"]
    VALUE1,
    #[doc = "Select service request line 3 of group x"]
    VALUE2,
    #[doc = "Select shared service request line 0"]
    VALUE3,
    #[doc = "Select shared service request line 3"]
    VALUE4,
}
impl CEV7NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CEV7NPW::VALUE1 => 0,
            CEV7NPW::VALUE2 => 3,
            CEV7NPW::VALUE3 => 4,
            CEV7NPW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV7NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CEV7NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV7NPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select service request line 0 of group x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV7NPW::VALUE1)
    }
    #[doc = "Select service request line 3 of group x"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV7NPW::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CEV7NPW::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CEV7NPW::VALUE4)
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
    #[doc = "Bits 0:3 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev0np(&self) -> CEV0NPR {
        CEV0NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev1np(&self) -> CEV1NPR {
        CEV1NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev2np(&self) -> CEV2NPR {
        CEV2NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev3np(&self) -> CEV3NPR {
        CEV3NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev4np(&self) -> CEV4NPR {
        CEV4NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev5np(&self) -> CEV5NPR {
        CEV5NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev6np(&self) -> CEV6NPR {
        CEV6NPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev7np(&self) -> CEV7NPR {
        CEV7NPR::_from({
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
    #[doc = "Bits 0:3 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev0np(&mut self) -> _CEV0NPW {
        _CEV0NPW { w: self }
    }
    #[doc = "Bits 4:7 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev1np(&mut self) -> _CEV1NPW {
        _CEV1NPW { w: self }
    }
    #[doc = "Bits 8:11 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev2np(&mut self) -> _CEV2NPW {
        _CEV2NPW { w: self }
    }
    #[doc = "Bits 12:15 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev3np(&mut self) -> _CEV3NPW {
        _CEV3NPW { w: self }
    }
    #[doc = "Bits 16:19 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev4np(&mut self) -> _CEV4NPW {
        _CEV4NPW { w: self }
    }
    #[doc = "Bits 20:23 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev5np(&mut self) -> _CEV5NPW {
        _CEV5NPW { w: self }
    }
    #[doc = "Bits 24:27 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev6np(&mut self) -> _CEV6NPW {
        _CEV6NPW { w: self }
    }
    #[doc = "Bits 28:31 - Service Request Node Pointer Channel Event i"]
    #[inline]
    pub fn cev7np(&mut self) -> _CEV7NPW {
        _CEV7NPW { w: self }
    }
}
