#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GCST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `S0SS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0SSR {
    #[doc = "Shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Shadow transfer has been requested"]
    VALUE2,
}
impl S0SSR {
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
            S0SSR::VALUE1 => false,
            S0SSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0SSR {
        match value {
            false => S0SSR::VALUE1,
            true => S0SSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S0SSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S0SSR::VALUE2
    }
}
#[doc = "Possible values of the field `S0DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0DSSR {
    #[doc = "Dither shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Dither shadow transfer has been requested"]
    VALUE2,
}
impl S0DSSR {
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
            S0DSSR::VALUE1 => false,
            S0DSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0DSSR {
        match value {
            false => S0DSSR::VALUE1,
            true => S0DSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S0DSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S0DSSR::VALUE2
    }
}
#[doc = "Possible values of the field `S0PSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0PSSR {
    #[doc = "Prescaler shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Prescaler shadow transfer has been requested"]
    VALUE2,
}
impl S0PSSR {
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
            S0PSSR::VALUE1 => false,
            S0PSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0PSSR {
        match value {
            false => S0PSSR::VALUE1,
            true => S0PSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S0PSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S0PSSR::VALUE2
    }
}
#[doc = "Possible values of the field `S1SS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1SSR {
    #[doc = "Shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Shadow transfer has been requested"]
    VALUE2,
}
impl S1SSR {
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
            S1SSR::VALUE1 => false,
            S1SSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1SSR {
        match value {
            false => S1SSR::VALUE1,
            true => S1SSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S1SSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S1SSR::VALUE2
    }
}
#[doc = "Possible values of the field `S1DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1DSSR {
    #[doc = "Dither shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Dither shadow transfer has been requested"]
    VALUE2,
}
impl S1DSSR {
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
            S1DSSR::VALUE1 => false,
            S1DSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1DSSR {
        match value {
            false => S1DSSR::VALUE1,
            true => S1DSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S1DSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S1DSSR::VALUE2
    }
}
#[doc = "Possible values of the field `S1PSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1PSSR {
    #[doc = "Prescaler shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Prescaler shadow transfer has been requested"]
    VALUE2,
}
impl S1PSSR {
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
            S1PSSR::VALUE1 => false,
            S1PSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1PSSR {
        match value {
            false => S1PSSR::VALUE1,
            true => S1PSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S1PSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S1PSSR::VALUE2
    }
}
#[doc = "Possible values of the field `S2SS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2SSR {
    #[doc = "Shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Shadow transfer has been requested"]
    VALUE2,
}
impl S2SSR {
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
            S2SSR::VALUE1 => false,
            S2SSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2SSR {
        match value {
            false => S2SSR::VALUE1,
            true => S2SSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S2SSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S2SSR::VALUE2
    }
}
#[doc = "Possible values of the field `S2DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2DSSR {
    #[doc = "Dither shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Dither shadow transfer has been requested"]
    VALUE2,
}
impl S2DSSR {
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
            S2DSSR::VALUE1 => false,
            S2DSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2DSSR {
        match value {
            false => S2DSSR::VALUE1,
            true => S2DSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S2DSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S2DSSR::VALUE2
    }
}
#[doc = "Possible values of the field `S2PSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2PSSR {
    #[doc = "Prescaler shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Prescaler shadow transfer has been requested"]
    VALUE2,
}
impl S2PSSR {
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
            S2PSSR::VALUE1 => false,
            S2PSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2PSSR {
        match value {
            false => S2PSSR::VALUE1,
            true => S2PSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S2PSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S2PSSR::VALUE2
    }
}
#[doc = "Possible values of the field `S3SS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3SSR {
    #[doc = "Shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Shadow transfer has been requested"]
    VALUE2,
}
impl S3SSR {
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
            S3SSR::VALUE1 => false,
            S3SSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3SSR {
        match value {
            false => S3SSR::VALUE1,
            true => S3SSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S3SSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S3SSR::VALUE2
    }
}
#[doc = "Possible values of the field `S3DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3DSSR {
    #[doc = "Dither shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Dither shadow transfer has been requested"]
    VALUE2,
}
impl S3DSSR {
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
            S3DSSR::VALUE1 => false,
            S3DSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3DSSR {
        match value {
            false => S3DSSR::VALUE1,
            true => S3DSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S3DSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S3DSSR::VALUE2
    }
}
#[doc = "Possible values of the field `S3PSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3PSSR {
    #[doc = "Prescaler shadow transfer has not been requested"]
    VALUE1,
    #[doc = "Prescaler shadow transfer has been requested"]
    VALUE2,
}
impl S3PSSR {
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
            S3PSSR::VALUE1 => false,
            S3PSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3PSSR {
        match value {
            false => S3PSSR::VALUE1,
            true => S3PSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S3PSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S3PSSR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CC80ST1R {
    bits: bool,
}
impl CC80ST1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CC81ST1R {
    bits: bool,
}
impl CC81ST1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CC82ST1R {
    bits: bool,
}
impl CC82ST1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CC83ST1R {
    bits: bool,
}
impl CC83ST1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CC80ST2R {
    bits: bool,
}
impl CC80ST2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CC81ST2R {
    bits: bool,
}
impl CC81ST2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CC82ST2R {
    bits: bool,
}
impl CC82ST2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CC83ST2R {
    bits: bool,
}
impl CC83ST2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Slice 0 shadow transfer status"]
    #[inline]
    pub fn s0ss(&self) -> S0SSR {
        S0SSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer status"]
    #[inline]
    pub fn s0dss(&self) -> S0DSSR {
        S0DSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer status"]
    #[inline]
    pub fn s0pss(&self) -> S0PSSR {
        S0PSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer status"]
    #[inline]
    pub fn s1ss(&self) -> S1SSR {
        S1SSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer status"]
    #[inline]
    pub fn s1dss(&self) -> S1DSSR {
        S1DSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer status"]
    #[inline]
    pub fn s1pss(&self) -> S1PSSR {
        S1PSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer status"]
    #[inline]
    pub fn s2ss(&self) -> S2SSR {
        S2SSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer status"]
    #[inline]
    pub fn s2dss(&self) -> S2DSSR {
        S2DSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer status"]
    #[inline]
    pub fn s2pss(&self) -> S2PSSR {
        S2PSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer status"]
    #[inline]
    pub fn s3ss(&self) -> S3SSR {
        S3SSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer status"]
    #[inline]
    pub fn s3dss(&self) -> S3DSSR {
        S3DSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer status"]
    #[inline]
    pub fn s3pss(&self) -> S3PSSR {
        S3PSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Slice 0 compare channel 1 status bit"]
    #[inline]
    pub fn cc80st1(&self) -> CC80ST1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CC80ST1R { bits }
    }
    #[doc = "Bit 17 - Slice 1 compare channel 1 status bit"]
    #[inline]
    pub fn cc81st1(&self) -> CC81ST1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CC81ST1R { bits }
    }
    #[doc = "Bit 18 - Slice 2 compare channel 1 status bit"]
    #[inline]
    pub fn cc82st1(&self) -> CC82ST1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CC82ST1R { bits }
    }
    #[doc = "Bit 19 - Slice 3 compare channel 1 status bit"]
    #[inline]
    pub fn cc83st1(&self) -> CC83ST1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CC83ST1R { bits }
    }
    #[doc = "Bit 20 - Slice 0 compare channel 2 status bit"]
    #[inline]
    pub fn cc80st2(&self) -> CC80ST2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CC80ST2R { bits }
    }
    #[doc = "Bit 21 - Slice 1 compare channel 2 status bit"]
    #[inline]
    pub fn cc81st2(&self) -> CC81ST2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CC81ST2R { bits }
    }
    #[doc = "Bit 22 - Slice 2 compare channel 2 status bit"]
    #[inline]
    pub fn cc82st2(&self) -> CC82ST2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CC82ST2R { bits }
    }
    #[doc = "Bit 23 - Slice 3 compare channel 2 status bit"]
    #[inline]
    pub fn cc83st2(&self) -> CC83ST2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CC83ST2R { bits }
    }
}
