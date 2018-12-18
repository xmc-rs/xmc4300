#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFSR {
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
#[doc = "Possible values of the field `IACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IACCVIOLR {
    #[doc = "no instruction access violation fault"]
    VALUE1,
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution."]
    VALUE2,
}
impl IACCVIOLR {
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
            IACCVIOLR::VALUE1 => false,
            IACCVIOLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IACCVIOLR {
        match value {
            false => IACCVIOLR::VALUE1,
            true => IACCVIOLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IACCVIOLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IACCVIOLR::VALUE2
    }
}
#[doc = "Possible values of the field `DACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCVIOLR {
    #[doc = "no data access violation fault"]
    VALUE1,
    #[doc = "the processor attempted a load or store at a location that does not permit the operation."]
    VALUE2,
}
impl DACCVIOLR {
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
            DACCVIOLR::VALUE1 => false,
            DACCVIOLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACCVIOLR {
        match value {
            false => DACCVIOLR::VALUE1,
            true => DACCVIOLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DACCVIOLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DACCVIOLR::VALUE2
    }
}
#[doc = "Possible values of the field `MUNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUNSTKERRR {
    #[doc = "no unstacking fault"]
    VALUE1,
    #[doc = "unstack for an exception return has caused one or more access violations."]
    VALUE2,
}
impl MUNSTKERRR {
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
            MUNSTKERRR::VALUE1 => false,
            MUNSTKERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MUNSTKERRR {
        match value {
            false => MUNSTKERRR::VALUE1,
            true => MUNSTKERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MUNSTKERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MUNSTKERRR::VALUE2
    }
}
#[doc = "Possible values of the field `MSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTKERRR {
    #[doc = "no stacking fault"]
    VALUE1,
    #[doc = "stacking for an exception entry has caused one or more access violations."]
    VALUE2,
}
impl MSTKERRR {
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
            MSTKERRR::VALUE1 => false,
            MSTKERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTKERRR {
        match value {
            false => MSTKERRR::VALUE1,
            true => MSTKERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSTKERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSTKERRR::VALUE2
    }
}
#[doc = "Possible values of the field `MLSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MLSPERRR {
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    VALUE1,
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    VALUE2,
}
impl MLSPERRR {
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
            MLSPERRR::VALUE1 => false,
            MLSPERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MLSPERRR {
        match value {
            false => MLSPERRR::VALUE1,
            true => MLSPERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MLSPERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MLSPERRR::VALUE2
    }
}
#[doc = "Possible values of the field `MMARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMARVALIDR {
    #[doc = "value in MMAR is not a valid fault address"]
    VALUE1,
    #[doc = "MMAR holds a valid fault address."]
    VALUE2,
}
impl MMARVALIDR {
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
            MMARVALIDR::VALUE1 => false,
            MMARVALIDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMARVALIDR {
        match value {
            false => MMARVALIDR::VALUE1,
            true => MMARVALIDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MMARVALIDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MMARVALIDR::VALUE2
    }
}
#[doc = "Possible values of the field `IBUSERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBUSERRR {
    #[doc = "no instruction bus error"]
    VALUE1,
    #[doc = "instruction bus error."]
    VALUE2,
}
impl IBUSERRR {
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
            IBUSERRR::VALUE1 => false,
            IBUSERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBUSERRR {
        match value {
            false => IBUSERRR::VALUE1,
            true => IBUSERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IBUSERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IBUSERRR::VALUE2
    }
}
#[doc = "Possible values of the field `PRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRECISERRR {
    #[doc = "no precise data bus error"]
    VALUE1,
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault."]
    VALUE2,
}
impl PRECISERRR {
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
            PRECISERRR::VALUE1 => false,
            PRECISERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRECISERRR {
        match value {
            false => PRECISERRR::VALUE1,
            true => PRECISERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRECISERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRECISERRR::VALUE2
    }
}
#[doc = "Possible values of the field `IMPRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPRECISERRR {
    #[doc = "no imprecise data bus error"]
    VALUE1,
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error."]
    VALUE2,
}
impl IMPRECISERRR {
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
            IMPRECISERRR::VALUE1 => false,
            IMPRECISERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMPRECISERRR {
        match value {
            false => IMPRECISERRR::VALUE1,
            true => IMPRECISERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IMPRECISERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IMPRECISERRR::VALUE2
    }
}
#[doc = "Possible values of the field `UNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNSTKERRR {
    #[doc = "no unstacking fault"]
    VALUE1,
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    VALUE2,
}
impl UNSTKERRR {
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
            UNSTKERRR::VALUE1 => false,
            UNSTKERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNSTKERRR {
        match value {
            false => UNSTKERRR::VALUE1,
            true => UNSTKERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == UNSTKERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == UNSTKERRR::VALUE2
    }
}
#[doc = "Possible values of the field `STKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKERRR {
    #[doc = "no stacking fault"]
    VALUE1,
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    VALUE2,
}
impl STKERRR {
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
            STKERRR::VALUE1 => false,
            STKERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STKERRR {
        match value {
            false => STKERRR::VALUE1,
            true => STKERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STKERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STKERRR::VALUE2
    }
}
#[doc = "Possible values of the field `LSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPERRR {
    #[doc = "No bus fault occurred during floating-point lazy state preservation."]
    VALUE1,
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    VALUE2,
}
impl LSPERRR {
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
            LSPERRR::VALUE1 => false,
            LSPERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSPERRR {
        match value {
            false => LSPERRR::VALUE1,
            true => LSPERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LSPERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LSPERRR::VALUE2
    }
}
#[doc = "Possible values of the field `BFARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFARVALIDR {
    #[doc = "value in BFAR is not a valid fault address"]
    VALUE1,
    #[doc = "BFAR holds a valid fault address."]
    VALUE2,
}
impl BFARVALIDR {
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
            BFARVALIDR::VALUE1 => false,
            BFARVALIDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFARVALIDR {
        match value {
            false => BFARVALIDR::VALUE1,
            true => BFARVALIDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFARVALIDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFARVALIDR::VALUE2
    }
}
#[doc = "Possible values of the field `UNDEFINSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDEFINSTRR {
    #[doc = "no undefined instruction UsageFault"]
    VALUE1,
    #[doc = "the processor has attempted to execute an undefined instruction."]
    VALUE2,
}
impl UNDEFINSTRR {
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
            UNDEFINSTRR::VALUE1 => false,
            UNDEFINSTRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNDEFINSTRR {
        match value {
            false => UNDEFINSTRR::VALUE1,
            true => UNDEFINSTRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == UNDEFINSTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == UNDEFINSTRR::VALUE2
    }
}
#[doc = "Possible values of the field `INVSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVSTATER {
    #[doc = "no invalid state UsageFault"]
    VALUE1,
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR."]
    VALUE2,
}
impl INVSTATER {
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
            INVSTATER::VALUE1 => false,
            INVSTATER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVSTATER {
        match value {
            false => INVSTATER::VALUE1,
            true => INVSTATER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INVSTATER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INVSTATER::VALUE2
    }
}
#[doc = "Possible values of the field `INVPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVPCR {
    #[doc = "no invalid PC load UsageFault"]
    VALUE1,
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value."]
    VALUE2,
}
impl INVPCR {
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
            INVPCR::VALUE1 => false,
            INVPCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVPCR {
        match value {
            false => INVPCR::VALUE1,
            true => INVPCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INVPCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INVPCR::VALUE2
    }
}
#[doc = "Possible values of the field `NOCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCPR {
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    VALUE1,
    #[doc = "the processor has attempted to access a coprocessor."]
    VALUE2,
}
impl NOCPR {
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
            NOCPR::VALUE1 => false,
            NOCPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOCPR {
        match value {
            false => NOCPR::VALUE1,
            true => NOCPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NOCPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NOCPR::VALUE2
    }
}
#[doc = "Possible values of the field `UNALIGNED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGNEDR {
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    VALUE1,
    #[doc = "the processor has made an unaligned memory access."]
    VALUE2,
}
impl UNALIGNEDR {
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
            UNALIGNEDR::VALUE1 => false,
            UNALIGNEDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNALIGNEDR {
        match value {
            false => UNALIGNEDR::VALUE1,
            true => UNALIGNEDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == UNALIGNEDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == UNALIGNEDR::VALUE2
    }
}
#[doc = "Possible values of the field `DIVBYZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBYZEROR {
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    VALUE1,
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    VALUE2,
}
impl DIVBYZEROR {
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
            DIVBYZEROR::VALUE1 => false,
            DIVBYZEROR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIVBYZEROR {
        match value {
            false => DIVBYZEROR::VALUE1,
            true => DIVBYZEROR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVBYZEROR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVBYZEROR::VALUE2
    }
}
#[doc = "Values that can be written to the field `IACCVIOL`"]
pub enum IACCVIOLW {
    #[doc = "no instruction access violation fault"]
    VALUE1,
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution."]
    VALUE2,
}
impl IACCVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IACCVIOLW::VALUE1 => false,
            IACCVIOLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IACCVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _IACCVIOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IACCVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no instruction access violation fault"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IACCVIOLW::VALUE1)
    }
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IACCVIOLW::VALUE2)
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
#[doc = "Values that can be written to the field `DACCVIOL`"]
pub enum DACCVIOLW {
    #[doc = "no data access violation fault"]
    VALUE1,
    #[doc = "the processor attempted a load or store at a location that does not permit the operation."]
    VALUE2,
}
impl DACCVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACCVIOLW::VALUE1 => false,
            DACCVIOLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACCVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCVIOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACCVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no data access violation fault"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DACCVIOLW::VALUE1)
    }
    #[doc = "the processor attempted a load or store at a location that does not permit the operation."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DACCVIOLW::VALUE2)
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
#[doc = "Values that can be written to the field `MUNSTKERR`"]
pub enum MUNSTKERRW {
    #[doc = "no unstacking fault"]
    VALUE1,
    #[doc = "unstack for an exception return has caused one or more access violations."]
    VALUE2,
}
impl MUNSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MUNSTKERRW::VALUE1 => false,
            MUNSTKERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUNSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MUNSTKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUNSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MUNSTKERRW::VALUE1)
    }
    #[doc = "unstack for an exception return has caused one or more access violations."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MUNSTKERRW::VALUE2)
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
#[doc = "Values that can be written to the field `MSTKERR`"]
pub enum MSTKERRW {
    #[doc = "no stacking fault"]
    VALUE1,
    #[doc = "stacking for an exception entry has caused one or more access violations."]
    VALUE2,
}
impl MSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTKERRW::VALUE1 => false,
            MSTKERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no stacking fault"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSTKERRW::VALUE1)
    }
    #[doc = "stacking for an exception entry has caused one or more access violations."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSTKERRW::VALUE2)
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
#[doc = "Values that can be written to the field `MLSPERR`"]
pub enum MLSPERRW {
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    VALUE1,
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    VALUE2,
}
impl MLSPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MLSPERRW::VALUE1 => false,
            MLSPERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MLSPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MLSPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MLSPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MLSPERRW::VALUE1)
    }
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MLSPERRW::VALUE2)
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
#[doc = "Values that can be written to the field `MMARVALID`"]
pub enum MMARVALIDW {
    #[doc = "value in MMAR is not a valid fault address"]
    VALUE1,
    #[doc = "MMAR holds a valid fault address."]
    VALUE2,
}
impl MMARVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMARVALIDW::VALUE1 => false,
            MMARVALIDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMARVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _MMARVALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMARVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "value in MMAR is not a valid fault address"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMARVALIDW::VALUE1)
    }
    #[doc = "MMAR holds a valid fault address."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMARVALIDW::VALUE2)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IBUSERR`"]
pub enum IBUSERRW {
    #[doc = "no instruction bus error"]
    VALUE1,
    #[doc = "instruction bus error."]
    VALUE2,
}
impl IBUSERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IBUSERRW::VALUE1 => false,
            IBUSERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IBUSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _IBUSERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IBUSERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no instruction bus error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IBUSERRW::VALUE1)
    }
    #[doc = "instruction bus error."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IBUSERRW::VALUE2)
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
#[doc = "Values that can be written to the field `PRECISERR`"]
pub enum PRECISERRW {
    #[doc = "no precise data bus error"]
    VALUE1,
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault."]
    VALUE2,
}
impl PRECISERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRECISERRW::VALUE1 => false,
            PRECISERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRECISERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRECISERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRECISERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no precise data bus error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRECISERRW::VALUE1)
    }
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRECISERRW::VALUE2)
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
#[doc = "Values that can be written to the field `IMPRECISERR`"]
pub enum IMPRECISERRW {
    #[doc = "no imprecise data bus error"]
    VALUE1,
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error."]
    VALUE2,
}
impl IMPRECISERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IMPRECISERRW::VALUE1 => false,
            IMPRECISERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMPRECISERRW<'a> {
    w: &'a mut W,
}
impl<'a> _IMPRECISERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMPRECISERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no imprecise data bus error"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IMPRECISERRW::VALUE1)
    }
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IMPRECISERRW::VALUE2)
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
#[doc = "Values that can be written to the field `UNSTKERR`"]
pub enum UNSTKERRW {
    #[doc = "no unstacking fault"]
    VALUE1,
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    VALUE2,
}
impl UNSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNSTKERRW::VALUE1 => false,
            UNSTKERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _UNSTKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(UNSTKERRW::VALUE1)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(UNSTKERRW::VALUE2)
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
#[doc = "Values that can be written to the field `STKERR`"]
pub enum STKERRW {
    #[doc = "no stacking fault"]
    VALUE1,
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    VALUE2,
}
impl STKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STKERRW::VALUE1 => false,
            STKERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _STKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no stacking fault"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STKERRW::VALUE1)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STKERRW::VALUE2)
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
#[doc = "Values that can be written to the field `LSPERR`"]
pub enum LSPERRW {
    #[doc = "No bus fault occurred during floating-point lazy state preservation."]
    VALUE1,
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    VALUE2,
}
impl LSPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPERRW::VALUE1 => false,
            LSPERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No bus fault occurred during floating-point lazy state preservation."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LSPERRW::VALUE1)
    }
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LSPERRW::VALUE2)
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
#[doc = "Values that can be written to the field `BFARVALID`"]
pub enum BFARVALIDW {
    #[doc = "value in BFAR is not a valid fault address"]
    VALUE1,
    #[doc = "BFAR holds a valid fault address."]
    VALUE2,
}
impl BFARVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFARVALIDW::VALUE1 => false,
            BFARVALIDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFARVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _BFARVALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFARVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "value in BFAR is not a valid fault address"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFARVALIDW::VALUE1)
    }
    #[doc = "BFAR holds a valid fault address."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFARVALIDW::VALUE2)
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
#[doc = "Values that can be written to the field `UNDEFINSTR`"]
pub enum UNDEFINSTRW {
    #[doc = "no undefined instruction UsageFault"]
    VALUE1,
    #[doc = "the processor has attempted to execute an undefined instruction."]
    VALUE2,
}
impl UNDEFINSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNDEFINSTRW::VALUE1 => false,
            UNDEFINSTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNDEFINSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDEFINSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNDEFINSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no undefined instruction UsageFault"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(UNDEFINSTRW::VALUE1)
    }
    #[doc = "the processor has attempted to execute an undefined instruction."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(UNDEFINSTRW::VALUE2)
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
#[doc = "Values that can be written to the field `INVSTATE`"]
pub enum INVSTATEW {
    #[doc = "no invalid state UsageFault"]
    VALUE1,
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR."]
    VALUE2,
}
impl INVSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVSTATEW::VALUE1 => false,
            INVSTATEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _INVSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVSTATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no invalid state UsageFault"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INVSTATEW::VALUE1)
    }
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INVSTATEW::VALUE2)
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
#[doc = "Values that can be written to the field `INVPC`"]
pub enum INVPCW {
    #[doc = "no invalid PC load UsageFault"]
    VALUE1,
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value."]
    VALUE2,
}
impl INVPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVPCW::VALUE1 => false,
            INVPCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVPCW<'a> {
    w: &'a mut W,
}
impl<'a> _INVPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no invalid PC load UsageFault"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INVPCW::VALUE1)
    }
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INVPCW::VALUE2)
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
#[doc = "Values that can be written to the field `NOCP`"]
pub enum NOCPW {
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    VALUE1,
    #[doc = "the processor has attempted to access a coprocessor."]
    VALUE2,
}
impl NOCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOCPW::VALUE1 => false,
            NOCPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOCPW<'a> {
    w: &'a mut W,
}
impl<'a> _NOCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NOCPW::VALUE1)
    }
    #[doc = "the processor has attempted to access a coprocessor."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NOCPW::VALUE2)
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
#[doc = "Values that can be written to the field `UNALIGNED`"]
pub enum UNALIGNEDW {
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    VALUE1,
    #[doc = "the processor has made an unaligned memory access."]
    VALUE2,
}
impl UNALIGNEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALIGNEDW::VALUE1 => false,
            UNALIGNEDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNALIGNEDW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGNEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNALIGNEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(UNALIGNEDW::VALUE1)
    }
    #[doc = "the processor has made an unaligned memory access."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(UNALIGNEDW::VALUE2)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIVBYZERO`"]
pub enum DIVBYZEROW {
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    VALUE1,
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    VALUE2,
}
impl DIVBYZEROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVBYZEROW::VALUE1 => false,
            DIVBYZEROW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVBYZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVBYZEROW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVBYZEROW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVBYZEROW::VALUE1)
    }
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVBYZEROW::VALUE2)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline]
    pub fn iaccviol(&self) -> IACCVIOLR {
        IACCVIOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline]
    pub fn daccviol(&self) -> DACCVIOLR {
        DACCVIOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline]
    pub fn munstkerr(&self) -> MUNSTKERRR {
        MUNSTKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline]
    pub fn mstkerr(&self) -> MSTKERRR {
        MSTKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - MemManage fault during floating point lazy state preservation"]
    #[inline]
    pub fn mlsperr(&self) -> MLSPERRR {
        MLSPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline]
    pub fn mmarvalid(&self) -> MMARVALIDR {
        MMARVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline]
    pub fn ibuserr(&self) -> IBUSERRR {
        IBUSERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline]
    pub fn preciserr(&self) -> PRECISERRR {
        PRECISERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline]
    pub fn impreciserr(&self) -> IMPRECISERRR {
        IMPRECISERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline]
    pub fn unstkerr(&self) -> UNSTKERRR {
        UNSTKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline]
    pub fn stkerr(&self) -> STKERRR {
        STKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - BusFault during floating point lazy state preservation"]
    #[inline]
    pub fn lsperr(&self) -> LSPERRR {
        LSPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline]
    pub fn bfarvalid(&self) -> BFARVALIDR {
        BFARVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline]
    pub fn undefinstr(&self) -> UNDEFINSTRR {
        UNDEFINSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline]
    pub fn invstate(&self) -> INVSTATER {
        INVSTATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline]
    pub fn invpc(&self) -> INVPCR {
        INVPCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline]
    pub fn nocp(&self) -> NOCPR {
        NOCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline]
    pub fn unaligned(&self) -> UNALIGNEDR {
        UNALIGNEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline]
    pub fn divbyzero(&self) -> DIVBYZEROR {
        DIVBYZEROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline]
    pub fn iaccviol(&mut self) -> _IACCVIOLW {
        _IACCVIOLW { w: self }
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline]
    pub fn daccviol(&mut self) -> _DACCVIOLW {
        _DACCVIOLW { w: self }
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline]
    pub fn munstkerr(&mut self) -> _MUNSTKERRW {
        _MUNSTKERRW { w: self }
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline]
    pub fn mstkerr(&mut self) -> _MSTKERRW {
        _MSTKERRW { w: self }
    }
    #[doc = "Bit 5 - MemManage fault during floating point lazy state preservation"]
    #[inline]
    pub fn mlsperr(&mut self) -> _MLSPERRW {
        _MLSPERRW { w: self }
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline]
    pub fn mmarvalid(&mut self) -> _MMARVALIDW {
        _MMARVALIDW { w: self }
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline]
    pub fn ibuserr(&mut self) -> _IBUSERRW {
        _IBUSERRW { w: self }
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline]
    pub fn preciserr(&mut self) -> _PRECISERRW {
        _PRECISERRW { w: self }
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline]
    pub fn impreciserr(&mut self) -> _IMPRECISERRW {
        _IMPRECISERRW { w: self }
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline]
    pub fn unstkerr(&mut self) -> _UNSTKERRW {
        _UNSTKERRW { w: self }
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline]
    pub fn stkerr(&mut self) -> _STKERRW {
        _STKERRW { w: self }
    }
    #[doc = "Bit 13 - BusFault during floating point lazy state preservation"]
    #[inline]
    pub fn lsperr(&mut self) -> _LSPERRW {
        _LSPERRW { w: self }
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline]
    pub fn bfarvalid(&mut self) -> _BFARVALIDW {
        _BFARVALIDW { w: self }
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline]
    pub fn undefinstr(&mut self) -> _UNDEFINSTRW {
        _UNDEFINSTRW { w: self }
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline]
    pub fn invstate(&mut self) -> _INVSTATEW {
        _INVSTATEW { w: self }
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline]
    pub fn invpc(&mut self) -> _INVPCW {
        _INVPCW { w: self }
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline]
    pub fn nocp(&mut self) -> _NOCPW {
        _NOCPW { w: self }
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline]
    pub fn unaligned(&mut self) -> _UNALIGNEDW {
        _UNALIGNEDW { w: self }
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline]
    pub fn divbyzero(&mut self) -> _DIVBYZEROW {
        _DIVBYZEROW { w: self }
    }
}
