#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AL_EVENT_MASK {
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
#[doc = "Possible values of the field `AL_CE_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AL_CE_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl AL_CE_MASKR {
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
            AL_CE_MASKR::VALUE1 => false,
            AL_CE_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AL_CE_MASKR {
        match value {
            false => AL_CE_MASKR::VALUE1,
            true => AL_CE_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AL_CE_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AL_CE_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `DC_LE_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_LE_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl DC_LE_MASKR {
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
            DC_LE_MASKR::VALUE1 => false,
            DC_LE_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DC_LE_MASKR {
        match value {
            false => DC_LE_MASKR::VALUE1,
            true => DC_LE_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DC_LE_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DC_LE_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `ST_S0_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST_S0_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl ST_S0_MASKR {
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
            ST_S0_MASKR::VALUE1 => false,
            ST_S0_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ST_S0_MASKR {
        match value {
            false => ST_S0_MASKR::VALUE1,
            true => ST_S0_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ST_S0_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ST_S0_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `ST_S1_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST_S1_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl ST_S1_MASKR {
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
            ST_S1_MASKR::VALUE1 => false,
            ST_S1_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ST_S1_MASKR {
        match value {
            false => ST_S1_MASKR::VALUE1,
            true => ST_S1_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ST_S1_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ST_S1_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SM_A_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM_A_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SM_A_MASKR {
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
            SM_A_MASKR::VALUE1 => false,
            SM_A_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM_A_MASKR {
        match value {
            false => SM_A_MASKR::VALUE1,
            true => SM_A_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SM_A_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SM_A_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `EEP_E_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEP_E_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl EEP_E_MASKR {
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
            EEP_E_MASKR::VALUE1 => false,
            EEP_E_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEP_E_MASKR {
        match value {
            false => EEP_E_MASKR::VALUE1,
            true => EEP_E_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EEP_E_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EEP_E_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `WP_D_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_D_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl WP_D_MASKR {
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
            WP_D_MASKR::VALUE1 => false,
            WP_D_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP_D_MASKR {
        match value {
            false => WP_D_MASKR::VALUE1,
            true => WP_D_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WP_D_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WP_D_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_0_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_0_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_0_MASKR {
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
            SMI_0_MASKR::VALUE1 => false,
            SMI_0_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_0_MASKR {
        match value {
            false => SMI_0_MASKR::VALUE1,
            true => SMI_0_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_0_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_0_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_1_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_1_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_1_MASKR {
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
            SMI_1_MASKR::VALUE1 => false,
            SMI_1_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_1_MASKR {
        match value {
            false => SMI_1_MASKR::VALUE1,
            true => SMI_1_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_1_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_1_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_2_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_2_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_2_MASKR {
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
            SMI_2_MASKR::VALUE1 => false,
            SMI_2_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_2_MASKR {
        match value {
            false => SMI_2_MASKR::VALUE1,
            true => SMI_2_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_2_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_2_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_3_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_3_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_3_MASKR {
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
            SMI_3_MASKR::VALUE1 => false,
            SMI_3_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_3_MASKR {
        match value {
            false => SMI_3_MASKR::VALUE1,
            true => SMI_3_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_3_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_3_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_4_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_4_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_4_MASKR {
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
            SMI_4_MASKR::VALUE1 => false,
            SMI_4_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_4_MASKR {
        match value {
            false => SMI_4_MASKR::VALUE1,
            true => SMI_4_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_4_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_4_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_5_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_5_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_5_MASKR {
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
            SMI_5_MASKR::VALUE1 => false,
            SMI_5_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_5_MASKR {
        match value {
            false => SMI_5_MASKR::VALUE1,
            true => SMI_5_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_5_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_5_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_6_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_6_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_6_MASKR {
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
            SMI_6_MASKR::VALUE1 => false,
            SMI_6_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_6_MASKR {
        match value {
            false => SMI_6_MASKR::VALUE1,
            true => SMI_6_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_6_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_6_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_7_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_7_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_7_MASKR {
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
            SMI_7_MASKR::VALUE1 => false,
            SMI_7_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_7_MASKR {
        match value {
            false => SMI_7_MASKR::VALUE1,
            true => SMI_7_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_7_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_7_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_8_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_8_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_8_MASKR {
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
            SMI_8_MASKR::VALUE1 => false,
            SMI_8_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_8_MASKR {
        match value {
            false => SMI_8_MASKR::VALUE1,
            true => SMI_8_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_8_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_8_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_9_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_9_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_9_MASKR {
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
            SMI_9_MASKR::VALUE1 => false,
            SMI_9_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_9_MASKR {
        match value {
            false => SMI_9_MASKR::VALUE1,
            true => SMI_9_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_9_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_9_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_10_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_10_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_10_MASKR {
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
            SMI_10_MASKR::VALUE1 => false,
            SMI_10_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_10_MASKR {
        match value {
            false => SMI_10_MASKR::VALUE1,
            true => SMI_10_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_10_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_10_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_11_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_11_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_11_MASKR {
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
            SMI_11_MASKR::VALUE1 => false,
            SMI_11_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_11_MASKR {
        match value {
            false => SMI_11_MASKR::VALUE1,
            true => SMI_11_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_11_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_11_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_12_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_12_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_12_MASKR {
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
            SMI_12_MASKR::VALUE1 => false,
            SMI_12_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_12_MASKR {
        match value {
            false => SMI_12_MASKR::VALUE1,
            true => SMI_12_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_12_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_12_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_13_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_13_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_13_MASKR {
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
            SMI_13_MASKR::VALUE1 => false,
            SMI_13_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_13_MASKR {
        match value {
            false => SMI_13_MASKR::VALUE1,
            true => SMI_13_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_13_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_13_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_14_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_14_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_14_MASKR {
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
            SMI_14_MASKR::VALUE1 => false,
            SMI_14_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_14_MASKR {
        match value {
            false => SMI_14_MASKR::VALUE1,
            true => SMI_14_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_14_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_14_MASKR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_15_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_15_MASKR {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_15_MASKR {
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
            SMI_15_MASKR::VALUE1 => false,
            SMI_15_MASKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_15_MASKR {
        match value {
            false => SMI_15_MASKR::VALUE1,
            true => SMI_15_MASKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_15_MASKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_15_MASKR::VALUE2
    }
}
#[doc = "Values that can be written to the field `AL_CE_MASK`"]
pub enum AL_CE_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl AL_CE_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AL_CE_MASKW::VALUE1 => false,
            AL_CE_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AL_CE_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _AL_CE_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AL_CE_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AL_CE_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AL_CE_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DC_LE_MASK`"]
pub enum DC_LE_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl DC_LE_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DC_LE_MASKW::VALUE1 => false,
            DC_LE_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DC_LE_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _DC_LE_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DC_LE_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DC_LE_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DC_LE_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ST_S0_MASK`"]
pub enum ST_S0_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl ST_S0_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ST_S0_MASKW::VALUE1 => false,
            ST_S0_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ST_S0_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _ST_S0_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ST_S0_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ST_S0_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ST_S0_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ST_S1_MASK`"]
pub enum ST_S1_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl ST_S1_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ST_S1_MASKW::VALUE1 => false,
            ST_S1_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ST_S1_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _ST_S1_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ST_S1_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ST_S1_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ST_S1_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM_A_MASK`"]
pub enum SM_A_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SM_A_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SM_A_MASKW::VALUE1 => false,
            SM_A_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SM_A_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SM_A_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SM_A_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SM_A_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SM_A_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EEP_E_MASK`"]
pub enum EEP_E_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl EEP_E_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEP_E_MASKW::VALUE1 => false,
            EEP_E_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEP_E_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _EEP_E_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEP_E_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EEP_E_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EEP_E_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP_D_MASK`"]
pub enum WP_D_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl WP_D_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP_D_MASKW::VALUE1 => false,
            WP_D_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP_D_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _WP_D_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP_D_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WP_D_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WP_D_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_0_MASK`"]
pub enum SMI_0_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_0_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_0_MASKW::VALUE1 => false,
            SMI_0_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_0_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_0_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_0_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_0_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_0_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_1_MASK`"]
pub enum SMI_1_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_1_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_1_MASKW::VALUE1 => false,
            SMI_1_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_1_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_1_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_1_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_1_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_1_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_2_MASK`"]
pub enum SMI_2_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_2_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_2_MASKW::VALUE1 => false,
            SMI_2_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_2_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_2_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_2_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_2_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_2_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_3_MASK`"]
pub enum SMI_3_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_3_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_3_MASKW::VALUE1 => false,
            SMI_3_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_3_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_3_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_3_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_3_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_3_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_4_MASK`"]
pub enum SMI_4_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_4_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_4_MASKW::VALUE1 => false,
            SMI_4_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_4_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_4_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_4_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_4_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_4_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_5_MASK`"]
pub enum SMI_5_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_5_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_5_MASKW::VALUE1 => false,
            SMI_5_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_5_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_5_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_5_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_5_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_5_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_6_MASK`"]
pub enum SMI_6_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_6_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_6_MASKW::VALUE1 => false,
            SMI_6_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_6_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_6_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_6_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_6_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_6_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_7_MASK`"]
pub enum SMI_7_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_7_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_7_MASKW::VALUE1 => false,
            SMI_7_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_7_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_7_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_7_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_7_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_7_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_8_MASK`"]
pub enum SMI_8_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_8_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_8_MASKW::VALUE1 => false,
            SMI_8_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_8_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_8_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_8_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_8_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_8_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_9_MASK`"]
pub enum SMI_9_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_9_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_9_MASKW::VALUE1 => false,
            SMI_9_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_9_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_9_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_9_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_9_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_9_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_10_MASK`"]
pub enum SMI_10_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_10_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_10_MASKW::VALUE1 => false,
            SMI_10_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_10_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_10_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_10_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_10_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_10_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_11_MASK`"]
pub enum SMI_11_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_11_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_11_MASKW::VALUE1 => false,
            SMI_11_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_11_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_11_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_11_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_11_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_11_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_12_MASK`"]
pub enum SMI_12_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_12_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_12_MASKW::VALUE1 => false,
            SMI_12_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_12_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_12_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_12_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_12_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_12_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_13_MASK`"]
pub enum SMI_13_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_13_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_13_MASKW::VALUE1 => false,
            SMI_13_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_13_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_13_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_13_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_13_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_13_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_14_MASK`"]
pub enum SMI_14_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_14_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_14_MASKW::VALUE1 => false,
            SMI_14_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_14_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_14_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_14_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_14_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_14_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMI_15_MASK`"]
pub enum SMI_15_MASKW {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    VALUE1,
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    VALUE2,
}
impl SMI_15_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_15_MASKW::VALUE1 => false,
            SMI_15_MASKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_15_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_15_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_15_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_15_MASKW::VALUE1)
    }
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_15_MASKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - AL Control event"]
    #[inline]
    pub fn al_ce_mask(&self) -> AL_CE_MASKR {
        AL_CE_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline]
    pub fn dc_le_mask(&self) -> DC_LE_MASKR {
        DC_LE_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline]
    pub fn st_s0_mask(&self) -> ST_S0_MASKR {
        ST_S0_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline]
    pub fn st_s1_mask(&self) -> ST_S1_MASKR {
        ST_S1_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline]
    pub fn sm_a_mask(&self) -> SM_A_MASKR {
        SM_A_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline]
    pub fn eep_e_mask(&self) -> EEP_E_MASKR {
        EEP_E_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline]
    pub fn wp_d_mask(&self) -> WP_D_MASKR {
        WP_D_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline]
    pub fn smi_0_mask(&self) -> SMI_0_MASKR {
        SMI_0_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline]
    pub fn smi_1_mask(&self) -> SMI_1_MASKR {
        SMI_1_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline]
    pub fn smi_2_mask(&self) -> SMI_2_MASKR {
        SMI_2_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline]
    pub fn smi_3_mask(&self) -> SMI_3_MASKR {
        SMI_3_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline]
    pub fn smi_4_mask(&self) -> SMI_4_MASKR {
        SMI_4_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline]
    pub fn smi_5_mask(&self) -> SMI_5_MASKR {
        SMI_5_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline]
    pub fn smi_6_mask(&self) -> SMI_6_MASKR {
        SMI_6_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline]
    pub fn smi_7_mask(&self) -> SMI_7_MASKR {
        SMI_7_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline]
    pub fn smi_8_mask(&self) -> SMI_8_MASKR {
        SMI_8_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline]
    pub fn smi_9_mask(&self) -> SMI_9_MASKR {
        SMI_9_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline]
    pub fn smi_10_mask(&self) -> SMI_10_MASKR {
        SMI_10_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline]
    pub fn smi_11_mask(&self) -> SMI_11_MASKR {
        SMI_11_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline]
    pub fn smi_12_mask(&self) -> SMI_12_MASKR {
        SMI_12_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline]
    pub fn smi_13_mask(&self) -> SMI_13_MASKR {
        SMI_13_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline]
    pub fn smi_14_mask(&self) -> SMI_14_MASKR {
        SMI_14_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline]
    pub fn smi_15_mask(&self) -> SMI_15_MASKR {
        SMI_15_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16777007 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - AL Control event"]
    #[inline]
    pub fn al_ce_mask(&mut self) -> _AL_CE_MASKW {
        _AL_CE_MASKW { w: self }
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline]
    pub fn dc_le_mask(&mut self) -> _DC_LE_MASKW {
        _DC_LE_MASKW { w: self }
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline]
    pub fn st_s0_mask(&mut self) -> _ST_S0_MASKW {
        _ST_S0_MASKW { w: self }
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline]
    pub fn st_s1_mask(&mut self) -> _ST_S1_MASKW {
        _ST_S1_MASKW { w: self }
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline]
    pub fn sm_a_mask(&mut self) -> _SM_A_MASKW {
        _SM_A_MASKW { w: self }
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline]
    pub fn eep_e_mask(&mut self) -> _EEP_E_MASKW {
        _EEP_E_MASKW { w: self }
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline]
    pub fn wp_d_mask(&mut self) -> _WP_D_MASKW {
        _WP_D_MASKW { w: self }
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline]
    pub fn smi_0_mask(&mut self) -> _SMI_0_MASKW {
        _SMI_0_MASKW { w: self }
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline]
    pub fn smi_1_mask(&mut self) -> _SMI_1_MASKW {
        _SMI_1_MASKW { w: self }
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline]
    pub fn smi_2_mask(&mut self) -> _SMI_2_MASKW {
        _SMI_2_MASKW { w: self }
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline]
    pub fn smi_3_mask(&mut self) -> _SMI_3_MASKW {
        _SMI_3_MASKW { w: self }
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline]
    pub fn smi_4_mask(&mut self) -> _SMI_4_MASKW {
        _SMI_4_MASKW { w: self }
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline]
    pub fn smi_5_mask(&mut self) -> _SMI_5_MASKW {
        _SMI_5_MASKW { w: self }
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline]
    pub fn smi_6_mask(&mut self) -> _SMI_6_MASKW {
        _SMI_6_MASKW { w: self }
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline]
    pub fn smi_7_mask(&mut self) -> _SMI_7_MASKW {
        _SMI_7_MASKW { w: self }
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline]
    pub fn smi_8_mask(&mut self) -> _SMI_8_MASKW {
        _SMI_8_MASKW { w: self }
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline]
    pub fn smi_9_mask(&mut self) -> _SMI_9_MASKW {
        _SMI_9_MASKW { w: self }
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline]
    pub fn smi_10_mask(&mut self) -> _SMI_10_MASKW {
        _SMI_10_MASKW { w: self }
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline]
    pub fn smi_11_mask(&mut self) -> _SMI_11_MASKW {
        _SMI_11_MASKW { w: self }
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline]
    pub fn smi_12_mask(&mut self) -> _SMI_12_MASKW {
        _SMI_12_MASKW { w: self }
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline]
    pub fn smi_13_mask(&mut self) -> _SMI_13_MASKW {
        _SMI_13_MASKW { w: self }
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline]
    pub fn smi_14_mask(&mut self) -> _SMI_14_MASKW {
        _SMI_14_MASKW { w: self }
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline]
    pub fn smi_15_mask(&mut self) -> _SMI_15_MASKW {
        _SMI_15_MASKW { w: self }
    }
}
