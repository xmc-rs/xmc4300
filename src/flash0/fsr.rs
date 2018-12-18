#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PBUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBUSYR {
    #[doc = "PFLASH ready, not busy; PFLASH in read mode."]
    VALUE1,
    #[doc = "PFLASH busy; PFLASH not in read mode."]
    VALUE2,
}
impl PBUSYR {
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
            PBUSYR::VALUE1 => false,
            PBUSYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PBUSYR {
        match value {
            false => PBUSYR::VALUE1,
            true => PBUSYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PBUSYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PBUSYR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct FABUSYR {
    bits: bool,
}
impl FABUSYR {
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
#[doc = "Possible values of the field `PROG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGR {
    #[doc = "There is no program operation requested or in progress or just finished."]
    VALUE1,
    #[doc = "Programming operation (write page) requested (from FIM) or in action or finished."]
    VALUE2,
}
impl PROGR {
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
            PROGR::VALUE1 => false,
            PROGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROGR {
        match value {
            false => PROGR::VALUE1,
            true => PROGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PROGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PROGR::VALUE2
    }
}
#[doc = "Possible values of the field `ERASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASER {
    #[doc = "There is no erase operation requested or in progress or just finished"]
    VALUE1,
    #[doc = "Erase operation requested (from FIM) or in action or finished."]
    VALUE2,
}
impl ERASER {
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
            ERASER::VALUE1 => false,
            ERASER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERASER {
        match value {
            false => ERASER::VALUE1,
            true => ERASER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERASER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERASER::VALUE2
    }
}
#[doc = "Possible values of the field `PFPAGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFPAGER {
    #[doc = "Program Flash not in page mode"]
    VALUE1,
    #[doc = "Program Flash in page mode; assembly buffer of PFLASH (256 byte) is in use (being filled up)"]
    VALUE2,
}
impl PFPAGER {
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
            PFPAGER::VALUE1 => false,
            PFPAGER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFPAGER {
        match value {
            false => PFPAGER::VALUE1,
            true => PFPAGER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PFPAGER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PFPAGER::VALUE2
    }
}
#[doc = "Possible values of the field `PFOPER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFOPERR {
    #[doc = "No operation error reported by Program Flash"]
    VALUE1,
    #[doc = "Flash array operation aborted, because of a Flash array failure, e.g. an ECC error in microcode."]
    VALUE2,
}
impl PFOPERR {
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
            PFOPERR::VALUE1 => false,
            PFOPERR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFOPERR {
        match value {
            false => PFOPERR::VALUE1,
            true => PFOPERR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PFOPERR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PFOPERR::VALUE2
    }
}
#[doc = "Possible values of the field `SQER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SQERR {
    #[doc = "No sequence error"]
    VALUE1,
    #[doc = "Command state machine operation unsuccessful because of improper address or command sequence."]
    VALUE2,
}
impl SQERR {
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
            SQERR::VALUE1 => false,
            SQERR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SQERR {
        match value {
            false => SQERR::VALUE1,
            true => SQERR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SQERR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SQERR::VALUE2
    }
}
#[doc = "Possible values of the field `PROER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROERR {
    #[doc = "No protection error"]
    VALUE1,
    #[doc = "Protection error."]
    VALUE2,
}
impl PROERR {
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
            PROERR::VALUE1 => false,
            PROERR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROERR {
        match value {
            false => PROERR::VALUE1,
            true => PROERR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PROERR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PROERR::VALUE2
    }
}
#[doc = "Possible values of the field `PFSBER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSBERR {
    #[doc = "No Single-Bit Error detected during read access to PFLASH"]
    VALUE1,
    #[doc = "Single-Bit Error detected and corrected"]
    VALUE2,
}
impl PFSBERR {
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
            PFSBERR::VALUE1 => false,
            PFSBERR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFSBERR {
        match value {
            false => PFSBERR::VALUE1,
            true => PFSBERR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PFSBERR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PFSBERR::VALUE2
    }
}
#[doc = "Possible values of the field `PFDBER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFDBERR {
    #[doc = "No Double-Bit Error detected during read access to PFLASH"]
    VALUE1,
    #[doc = "Double-Bit Error detected in PFLASH"]
    VALUE2,
}
impl PFDBERR {
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
            PFDBERR::VALUE1 => false,
            PFDBERR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFDBERR {
        match value {
            false => PFDBERR::VALUE1,
            true => PFDBERR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PFDBERR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PFDBERR::VALUE2
    }
}
#[doc = "Possible values of the field `PROIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROINR {
    #[doc = "No protection is installed"]
    VALUE1,
    #[doc = "Read or/and write protection for one or more users is configured and correctly confirmed in the User Configuration Block(s)."]
    VALUE2,
}
impl PROINR {
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
            PROINR::VALUE1 => false,
            PROINR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROINR {
        match value {
            false => PROINR::VALUE1,
            true => PROINR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PROINR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PROINR::VALUE2
    }
}
#[doc = "Possible values of the field `RPROIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPROINR {
    #[doc = "No read protection installed"]
    VALUE1,
    #[doc = "Read protection and global write protection is configured and correctly confirmed in the User Configuration Block 0."]
    VALUE2,
}
impl RPROINR {
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
            RPROINR::VALUE1 => false,
            RPROINR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RPROINR {
        match value {
            false => RPROINR::VALUE1,
            true => RPROINR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RPROINR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RPROINR::VALUE2
    }
}
#[doc = "Possible values of the field `RPRODIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPRODISR {
    #[doc = "Read protection (if installed) is not disabled"]
    VALUE1,
    #[doc = "Read and global write protection is temporarily disabled."]
    VALUE2,
}
impl RPRODISR {
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
            RPRODISR::VALUE1 => false,
            RPRODISR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RPRODISR {
        match value {
            false => RPRODISR::VALUE1,
            true => RPRODISR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RPRODISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RPRODISR::VALUE2
    }
}
#[doc = "Possible values of the field `WPROIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROIN0R {
    #[doc = "No write protection installed for user 0"]
    VALUE1,
    #[doc = "Sector write protection for user 0 is configured and correctly confirmed in the User Configuration Block 0."]
    VALUE2,
}
impl WPROIN0R {
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
            WPROIN0R::VALUE1 => false,
            WPROIN0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPROIN0R {
        match value {
            false => WPROIN0R::VALUE1,
            true => WPROIN0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WPROIN0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WPROIN0R::VALUE2
    }
}
#[doc = "Possible values of the field `WPROIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROIN1R {
    #[doc = "No write protection installed for user 1"]
    VALUE1,
    #[doc = "Sector write protection for user 1 is configured and correctly confirmed in the User Configuration Block 1."]
    VALUE2,
}
impl WPROIN1R {
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
            WPROIN1R::VALUE1 => false,
            WPROIN1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPROIN1R {
        match value {
            false => WPROIN1R::VALUE1,
            true => WPROIN1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WPROIN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WPROIN1R::VALUE2
    }
}
#[doc = "Possible values of the field `WPROIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROIN2R {
    #[doc = "No OTP write protection installed for user 2"]
    VALUE1,
    #[doc = "Sector OTP write protection with ROM functionality is configured and correctly confirmed in the UCB2. The protection is locked for ever."]
    VALUE2,
}
impl WPROIN2R {
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
            WPROIN2R::VALUE1 => false,
            WPROIN2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPROIN2R {
        match value {
            false => WPROIN2R::VALUE1,
            true => WPROIN2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WPROIN2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WPROIN2R::VALUE2
    }
}
#[doc = "Possible values of the field `WPRODIS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPRODIS0R {
    #[doc = "All protected sectors of user 0 are locked if write protection is installed"]
    VALUE1,
    #[doc = "All write-protected sectors of user 0 are temporarily unlocked, if not coincidently locked by user 2 or via read protection."]
    VALUE2,
}
impl WPRODIS0R {
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
            WPRODIS0R::VALUE1 => false,
            WPRODIS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPRODIS0R {
        match value {
            false => WPRODIS0R::VALUE1,
            true => WPRODIS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WPRODIS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WPRODIS0R::VALUE2
    }
}
#[doc = "Possible values of the field `WPRODIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPRODIS1R {
    #[doc = "All protected sectors of user 1 are locked if write protection is installed"]
    VALUE1,
    #[doc = "All write-protected sectors of user 1 are temporarily unlocked, if not coincidently locked by user 0 or user 2 or via read protection."]
    VALUE2,
}
impl WPRODIS1R {
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
            WPRODIS1R::VALUE1 => false,
            WPRODIS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPRODIS1R {
        match value {
            false => WPRODIS1R::VALUE1,
            true => WPRODIS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WPRODIS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WPRODIS1R::VALUE2
    }
}
#[doc = "Possible values of the field `SLM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLMR {
    #[doc = "Flash not in sleep mode"]
    VALUE1,
    #[doc = "Flash is in sleep or shut down mode"]
    VALUE2,
}
impl SLMR {
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
            SLMR::VALUE1 => false,
            SLMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLMR {
        match value {
            false => SLMR::VALUE1,
            true => SLMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SLMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SLMR::VALUE2
    }
}
#[doc = "Possible values of the field `VER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VERR {
    #[doc = "The page is correctly programmed or the sector correctly erased. All programmed or erased bits have full expected quality."]
    VALUE1,
    #[doc = "A program verify error or an erase verify error has been detected. Full quality (retention time) of all programmed (\"1\") or erased (\"0\") bits cannot be guaranteed."]
    VALUE2,
}
impl VERR {
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
            VERR::VALUE1 => false,
            VERR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VERR {
        match value {
            false => VERR::VALUE1,
            true => VERR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VERR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VERR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Program Flash Busy"]
    #[inline]
    pub fn pbusy(&self) -> PBUSYR {
        PBUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Flash Array Busy"]
    #[inline]
    pub fn fabusy(&self) -> FABUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FABUSYR { bits }
    }
    #[doc = "Bit 4 - Programming State"]
    #[inline]
    pub fn prog(&self) -> PROGR {
        PROGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Erase State"]
    #[inline]
    pub fn erase(&self) -> ERASER {
        ERASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Program Flash in Page Mode"]
    #[inline]
    pub fn pfpage(&self) -> PFPAGER {
        PFPAGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Program Flash Operation Error"]
    #[inline]
    pub fn pfoper(&self) -> PFOPERR {
        PFOPERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Command Sequence Error"]
    #[inline]
    pub fn sqer(&self) -> SQERR {
        SQERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Protection Error"]
    #[inline]
    pub fn proer(&self) -> PROERR {
        PROERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PFLASH Single-Bit Error and Correction"]
    #[inline]
    pub fn pfsber(&self) -> PFSBERR {
        PFSBERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - PFLASH Double-Bit Error"]
    #[inline]
    pub fn pfdber(&self) -> PFDBERR {
        PFDBERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Protection Installed"]
    #[inline]
    pub fn proin(&self) -> PROINR {
        PROINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Read Protection Installed"]
    #[inline]
    pub fn rproin(&self) -> RPROINR {
        RPROINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Read Protection Disable State"]
    #[inline]
    pub fn rprodis(&self) -> RPRODISR {
        RPRODISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Sector Write Protection Installed for User 0"]
    #[inline]
    pub fn wproin0(&self) -> WPROIN0R {
        WPROIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Sector Write Protection Installed for User 1"]
    #[inline]
    pub fn wproin1(&self) -> WPROIN1R {
        WPROIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Sector OTP Protection Installed for User 2"]
    #[inline]
    pub fn wproin2(&self) -> WPROIN2R {
        WPROIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Sector Write Protection Disabled for User 0"]
    #[inline]
    pub fn wprodis0(&self) -> WPRODIS0R {
        WPRODIS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Sector Write Protection Disabled for User 1"]
    #[inline]
    pub fn wprodis1(&self) -> WPRODIS1R {
        WPRODIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Flash Sleep Mode"]
    #[inline]
    pub fn slm(&self) -> SLMR {
        SLMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Verify Error"]
    #[inline]
    pub fn ver(&self) -> VERR {
        VERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
