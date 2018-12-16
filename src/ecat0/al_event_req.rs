#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AL_EVENT_REQ {
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
#[doc = "Possible values of the field `AL_CE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AL_CER {
    #[doc = "No AL Control Register change"]
    VALUE1,
    #[doc = "AL Control Register has been written"]
    VALUE2,
}
impl AL_CER {
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
            AL_CER::VALUE1 => false,
            AL_CER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AL_CER {
        match value {
            false => AL_CER::VALUE1,
            true => AL_CER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AL_CER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AL_CER::VALUE2
    }
}
#[doc = "Possible values of the field `DC_LE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_LER {
    #[doc = "No change on DC Latch Inputs"]
    VALUE1,
    #[doc = "At least one change on DC Latch Inputs"]
    VALUE2,
}
impl DC_LER {
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
            DC_LER::VALUE1 => false,
            DC_LER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DC_LER {
        match value {
            false => DC_LER::VALUE1,
            true => DC_LER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DC_LER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DC_LER::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct ST_S0R {
    bits: bool,
}
impl ST_S0R {
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
pub struct ST_S1R {
    bits: bool,
}
impl ST_S1R {
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
#[doc = "Possible values of the field `SM_A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM_AR {
    #[doc = "No change in any SyncManager"]
    VALUE1,
    #[doc = "At least one change on DC Latch Inputs"]
    VALUE2,
}
impl SM_AR {
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
            SM_AR::VALUE1 => false,
            SM_AR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SM_AR {
        match value {
            false => SM_AR::VALUE1,
            true => SM_AR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SM_AR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SM_AR::VALUE2
    }
}
#[doc = "Possible values of the field `EEP_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEP_ER {
    #[doc = "No command pending"]
    VALUE1,
    #[doc = "EEPROM command pending"]
    VALUE2,
}
impl EEP_ER {
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
            EEP_ER::VALUE1 => false,
            EEP_ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEP_ER {
        match value {
            false => EEP_ER::VALUE1,
            true => EEP_ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EEP_ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EEP_ER::VALUE2
    }
}
#[doc = "Possible values of the field `WP_D`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_DR {
    #[doc = "Has not expired"]
    VALUE1,
    #[doc = "Has expired"]
    VALUE2,
}
impl WP_DR {
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
            WP_DR::VALUE1 => false,
            WP_DR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP_DR {
        match value {
            false => WP_DR::VALUE1,
            true => WP_DR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WP_DR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WP_DR::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_0R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_0R {
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
            SMI_0R::VALUE1 => false,
            SMI_0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_0R {
        match value {
            false => SMI_0R::VALUE1,
            true => SMI_0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_0R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_1R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_1R {
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
            SMI_1R::VALUE1 => false,
            SMI_1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_1R {
        match value {
            false => SMI_1R::VALUE1,
            true => SMI_1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_1R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_2R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_2R {
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
            SMI_2R::VALUE1 => false,
            SMI_2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_2R {
        match value {
            false => SMI_2R::VALUE1,
            true => SMI_2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_2R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_3R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_3R {
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
            SMI_3R::VALUE1 => false,
            SMI_3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_3R {
        match value {
            false => SMI_3R::VALUE1,
            true => SMI_3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_3R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_4R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_4R {
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
            SMI_4R::VALUE1 => false,
            SMI_4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_4R {
        match value {
            false => SMI_4R::VALUE1,
            true => SMI_4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_4R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_5R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_5R {
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
            SMI_5R::VALUE1 => false,
            SMI_5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_5R {
        match value {
            false => SMI_5R::VALUE1,
            true => SMI_5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_5R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_6R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_6R {
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
            SMI_6R::VALUE1 => false,
            SMI_6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_6R {
        match value {
            false => SMI_6R::VALUE1,
            true => SMI_6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_6R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_7R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_7R {
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
            SMI_7R::VALUE1 => false,
            SMI_7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_7R {
        match value {
            false => SMI_7R::VALUE1,
            true => SMI_7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_7R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_8R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_8R {
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
            SMI_8R::VALUE1 => false,
            SMI_8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_8R {
        match value {
            false => SMI_8R::VALUE1,
            true => SMI_8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_8R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_9R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_9R {
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
            SMI_9R::VALUE1 => false,
            SMI_9R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_9R {
        match value {
            false => SMI_9R::VALUE1,
            true => SMI_9R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_9R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_9R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_10R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_10R {
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
            SMI_10R::VALUE1 => false,
            SMI_10R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_10R {
        match value {
            false => SMI_10R::VALUE1,
            true => SMI_10R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_10R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_11R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_11R {
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
            SMI_11R::VALUE1 => false,
            SMI_11R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_11R {
        match value {
            false => SMI_11R::VALUE1,
            true => SMI_11R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_11R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_12R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_12R {
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
            SMI_12R::VALUE1 => false,
            SMI_12R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_12R {
        match value {
            false => SMI_12R::VALUE1,
            true => SMI_12R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_12R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_12R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_13R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_13R {
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
            SMI_13R::VALUE1 => false,
            SMI_13R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_13R {
        match value {
            false => SMI_13R::VALUE1,
            true => SMI_13R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_13R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_13R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_14R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_14R {
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
            SMI_14R::VALUE1 => false,
            SMI_14R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_14R {
        match value {
            false => SMI_14R::VALUE1,
            true => SMI_14R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_14R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_14R::VALUE2
    }
}
#[doc = "Possible values of the field `SMI_15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMI_15R {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_15R {
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
            SMI_15R::VALUE1 => false,
            SMI_15R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMI_15R {
        match value {
            false => SMI_15R::VALUE1,
            true => SMI_15R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SMI_15R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SMI_15R::VALUE2
    }
}
#[doc = "Values that can be written to the field `SMI_2`"]
pub enum SMI_2W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_2W::VALUE1 => false,
            SMI_2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_2W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_2W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_2W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_3`"]
pub enum SMI_3W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_3W::VALUE1 => false,
            SMI_3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_3W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_3W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_3W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_4`"]
pub enum SMI_4W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_4W::VALUE1 => false,
            SMI_4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_4W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_4W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_4W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_5`"]
pub enum SMI_5W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_5W::VALUE1 => false,
            SMI_5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_5W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_5W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_5W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_6`"]
pub enum SMI_6W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_6W::VALUE1 => false,
            SMI_6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_6W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_6W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_6W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_7`"]
pub enum SMI_7W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_7W::VALUE1 => false,
            SMI_7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_7W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_7W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_7W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_8`"]
pub enum SMI_8W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_8W::VALUE1 => false,
            SMI_8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_8W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_8W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_8W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_9`"]
pub enum SMI_9W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_9W::VALUE1 => false,
            SMI_9W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_9W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_9W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_9W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_10`"]
pub enum SMI_10W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_10W::VALUE1 => false,
            SMI_10W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_10W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_10W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_10W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_11`"]
pub enum SMI_11W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_11W::VALUE1 => false,
            SMI_11W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_11W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_11W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_11W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_12`"]
pub enum SMI_12W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_12W::VALUE1 => false,
            SMI_12W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_12W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_12W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_12W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_13`"]
pub enum SMI_13W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_13W::VALUE1 => false,
            SMI_13W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_13W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_13W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_13W::VALUE2)
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
#[doc = "Values that can be written to the field `SMI_14`"]
pub enum SMI_14W {
    #[doc = "No SyncManager 0 interrupt"]
    VALUE1,
    #[doc = "SyncManager 0 interrupt pending"]
    VALUE2,
}
impl SMI_14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMI_14W::VALUE1 => false,
            SMI_14W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMI_14W<'a> {
    w: &'a mut W,
}
impl<'a> _SMI_14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMI_14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SyncManager 0 interrupt"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SMI_14W::VALUE1)
    }
    #[doc = "SyncManager 0 interrupt pending"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SMI_14W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - AL Control event"]
    #[inline]
    pub fn al_ce(&self) -> AL_CER {
        AL_CER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DC Latch event"]
    #[inline]
    pub fn dc_le(&self) -> DC_LER {
        DC_LER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - State of DC SYNC0"]
    #[inline]
    pub fn st_s0(&self) -> ST_S0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST_S0R { bits }
    }
    #[doc = "Bit 3 - State of DC SYNC1"]
    #[inline]
    pub fn st_s1(&self) -> ST_S1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ST_S1R { bits }
    }
    #[doc = "Bit 4 - SyncManager activation register changed"]
    #[inline]
    pub fn sm_a(&self) -> SM_AR {
        SM_AR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - EEPROM Emulation"]
    #[inline]
    pub fn eep_e(&self) -> EEP_ER {
        EEP_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Watchdog Process Data"]
    #[inline]
    pub fn wp_d(&self) -> WP_DR {
        WP_DR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - SyncManager interrupt"]
    #[inline]
    pub fn smi_0(&self) -> SMI_0R {
        SMI_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - SyncManager interrupt"]
    #[inline]
    pub fn smi_1(&self) -> SMI_1R {
        SMI_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline]
    pub fn smi_2(&self) -> SMI_2R {
        SMI_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline]
    pub fn smi_3(&self) -> SMI_3R {
        SMI_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline]
    pub fn smi_4(&self) -> SMI_4R {
        SMI_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline]
    pub fn smi_5(&self) -> SMI_5R {
        SMI_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline]
    pub fn smi_6(&self) -> SMI_6R {
        SMI_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline]
    pub fn smi_7(&self) -> SMI_7R {
        SMI_7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline]
    pub fn smi_8(&self) -> SMI_8R {
        SMI_8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline]
    pub fn smi_9(&self) -> SMI_9R {
        SMI_9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline]
    pub fn smi_10(&self) -> SMI_10R {
        SMI_10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline]
    pub fn smi_11(&self) -> SMI_11R {
        SMI_11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline]
    pub fn smi_12(&self) -> SMI_12R {
        SMI_12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline]
    pub fn smi_13(&self) -> SMI_13R {
        SMI_13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline]
    pub fn smi_14(&self) -> SMI_14R {
        SMI_14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - SyncManager interrupt"]
    #[inline]
    pub fn smi_15(&self) -> SMI_15R {
        SMI_15R::_from({
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
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 10 - SyncManager interrupt"]
    #[inline]
    pub fn smi_2(&mut self) -> _SMI_2W {
        _SMI_2W { w: self }
    }
    #[doc = "Bit 11 - SyncManager interrupt"]
    #[inline]
    pub fn smi_3(&mut self) -> _SMI_3W {
        _SMI_3W { w: self }
    }
    #[doc = "Bit 12 - SyncManager interrupt"]
    #[inline]
    pub fn smi_4(&mut self) -> _SMI_4W {
        _SMI_4W { w: self }
    }
    #[doc = "Bit 13 - SyncManager interrupt"]
    #[inline]
    pub fn smi_5(&mut self) -> _SMI_5W {
        _SMI_5W { w: self }
    }
    #[doc = "Bit 14 - SyncManager interrupt"]
    #[inline]
    pub fn smi_6(&mut self) -> _SMI_6W {
        _SMI_6W { w: self }
    }
    #[doc = "Bit 15 - SyncManager interrupt"]
    #[inline]
    pub fn smi_7(&mut self) -> _SMI_7W {
        _SMI_7W { w: self }
    }
    #[doc = "Bit 16 - SyncManager interrupt"]
    #[inline]
    pub fn smi_8(&mut self) -> _SMI_8W {
        _SMI_8W { w: self }
    }
    #[doc = "Bit 17 - SyncManager interrupt"]
    #[inline]
    pub fn smi_9(&mut self) -> _SMI_9W {
        _SMI_9W { w: self }
    }
    #[doc = "Bit 18 - SyncManager interrupt"]
    #[inline]
    pub fn smi_10(&mut self) -> _SMI_10W {
        _SMI_10W { w: self }
    }
    #[doc = "Bit 19 - SyncManager interrupt"]
    #[inline]
    pub fn smi_11(&mut self) -> _SMI_11W {
        _SMI_11W { w: self }
    }
    #[doc = "Bit 20 - SyncManager interrupt"]
    #[inline]
    pub fn smi_12(&mut self) -> _SMI_12W {
        _SMI_12W { w: self }
    }
    #[doc = "Bit 21 - SyncManager interrupt"]
    #[inline]
    pub fn smi_13(&mut self) -> _SMI_13W {
        _SMI_13W { w: self }
    }
    #[doc = "Bit 22 - SyncManager interrupt"]
    #[inline]
    pub fn smi_14(&mut self) -> _SMI_14W {
        _SMI_14W { w: self }
    }
}
