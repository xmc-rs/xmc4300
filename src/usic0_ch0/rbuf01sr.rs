#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RBUF01SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct WLEN0R {
    bits: u8,
}
impl WLEN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SOF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF0R {
    #[doc = "The data in RBUF0 has not been the first data word of a data frame."]
    VALUE1,
    #[doc = "The data in RBUF0 has been the first data word of a data frame."]
    VALUE2,
}
impl SOF0R {
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
            SOF0R::VALUE1 => false,
            SOF0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOF0R {
        match value {
            false => SOF0R::VALUE1,
            true => SOF0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SOF0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SOF0R::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PAR0R {
    bits: bool,
}
impl PAR0R {
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
#[doc = "Possible values of the field `PERR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERR0R {
    #[doc = "The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    VALUE1,
    #[doc = "The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    VALUE2,
}
impl PERR0R {
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
            PERR0R::VALUE1 => false,
            PERR0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERR0R {
        match value {
            false => PERR0R::VALUE1,
            true => PERR0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PERR0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PERR0R::VALUE2
    }
}
#[doc = "Possible values of the field `RDV00`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDV00R {
    #[doc = "Register RBUF0 does not contain data that has not yet been read out."]
    VALUE1,
    #[doc = "Register RBUF0 contains data that has not yet been read out."]
    VALUE2,
}
impl RDV00R {
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
            RDV00R::VALUE1 => false,
            RDV00R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDV00R {
        match value {
            false => RDV00R::VALUE1,
            true => RDV00R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RDV00R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RDV00R::VALUE2
    }
}
#[doc = "Possible values of the field `RDV01`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDV01R {
    #[doc = "Register RBUF1 does not contain data that has not yet been read out."]
    VALUE1,
    #[doc = "Register RBUF1 contains data that has not yet been read out."]
    VALUE2,
}
impl RDV01R {
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
            RDV01R::VALUE1 => false,
            RDV01R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDV01R {
        match value {
            false => RDV01R::VALUE1,
            true => RDV01R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RDV01R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RDV01R::VALUE2
    }
}
#[doc = "Possible values of the field `DS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DS0R {
    #[doc = "The register RBUF contains the data of RBUF0 (same for associated status information)."]
    VALUE1,
    #[doc = "The register RBUF contains the data of RBUF1 (same for associated status information)."]
    VALUE2,
}
impl DS0R {
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
            DS0R::VALUE1 => false,
            DS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DS0R {
        match value {
            false => DS0R::VALUE1,
            true => DS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DS0R::VALUE2
    }
}
#[doc = "Possible values of the field `WLEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLEN1R {
    #[doc = "One bit has been received."]
    VALUE1,
    #[doc = "Sixteen bits have been received."]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WLEN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WLEN1R::VALUE1 => 0,
            WLEN1R::VALUE2 => 15,
            WLEN1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WLEN1R {
        match value {
            0 => WLEN1R::VALUE1,
            15 => WLEN1R::VALUE2,
            i => WLEN1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WLEN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WLEN1R::VALUE2
    }
}
#[doc = "Possible values of the field `SOF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF1R {
    #[doc = "The data in RBUF1 has not been the first data word of a data frame."]
    VALUE1,
    #[doc = "The data in RBUF1 has been the first data word of a data frame."]
    VALUE2,
}
impl SOF1R {
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
            SOF1R::VALUE1 => false,
            SOF1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOF1R {
        match value {
            false => SOF1R::VALUE1,
            true => SOF1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SOF1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SOF1R::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PAR1R {
    bits: bool,
}
impl PAR1R {
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
#[doc = "Possible values of the field `PERR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERR1R {
    #[doc = "The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    VALUE1,
    #[doc = "The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    VALUE2,
}
impl PERR1R {
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
            PERR1R::VALUE1 => false,
            PERR1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERR1R {
        match value {
            false => PERR1R::VALUE1,
            true => PERR1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PERR1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PERR1R::VALUE2
    }
}
#[doc = "Possible values of the field `RDV10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDV10R {
    #[doc = "Register RBUF0 does not contain data that has not yet been read out."]
    VALUE1,
    #[doc = "Register RBUF0 contains data that has not yet been read out."]
    VALUE2,
}
impl RDV10R {
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
            RDV10R::VALUE1 => false,
            RDV10R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDV10R {
        match value {
            false => RDV10R::VALUE1,
            true => RDV10R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RDV10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RDV10R::VALUE2
    }
}
#[doc = "Possible values of the field `RDV11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDV11R {
    #[doc = "Register RBUF1 does not contain data that has not yet been read out."]
    VALUE1,
    #[doc = "Register RBUF1 contains data that has not yet been read out."]
    VALUE2,
}
impl RDV11R {
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
            RDV11R::VALUE1 => false,
            RDV11R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDV11R {
        match value {
            false => RDV11R::VALUE1,
            true => RDV11R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RDV11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RDV11R::VALUE2
    }
}
#[doc = "Possible values of the field `DS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DS1R {
    #[doc = "The register RBUF contains the data of RBUF0 (same for associated status information)."]
    VALUE1,
    #[doc = "The register RBUF contains the data of RBUF1 (same for associated status information)."]
    VALUE2,
}
impl DS1R {
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
            DS1R::VALUE1 => false,
            DS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DS1R {
        match value {
            false => DS1R::VALUE1,
            true => DS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DS1R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Received Data Word Length in RBUF0"]
    #[inline]
    pub fn wlen0(&self) -> WLEN0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WLEN0R { bits }
    }
    #[doc = "Bit 6 - Start of Frame in RBUF0"]
    #[inline]
    pub fn sof0(&self) -> SOF0R {
        SOF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Protocol-Related Argument in RBUF0"]
    #[inline]
    pub fn par0(&self) -> PAR0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAR0R { bits }
    }
    #[doc = "Bit 9 - Protocol-related Error in RBUF0"]
    #[inline]
    pub fn perr0(&self) -> PERR0R {
        PERR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Receive Data Valid in RBUF0"]
    #[inline]
    pub fn rdv00(&self) -> RDV00R {
        RDV00R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Receive Data Valid in RBUF1"]
    #[inline]
    pub fn rdv01(&self) -> RDV01R {
        RDV01R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Data Source"]
    #[inline]
    pub fn ds0(&self) -> DS0R {
        DS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Received Data Word Length in RBUF1"]
    #[inline]
    pub fn wlen1(&self) -> WLEN1R {
        WLEN1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Start of Frame in RBUF1"]
    #[inline]
    pub fn sof1(&self) -> SOF1R {
        SOF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Protocol-Related Argument in RBUF1"]
    #[inline]
    pub fn par1(&self) -> PAR1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAR1R { bits }
    }
    #[doc = "Bit 25 - Protocol-related Error in RBUF1"]
    #[inline]
    pub fn perr1(&self) -> PERR1R {
        PERR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Receive Data Valid in RBUF0"]
    #[inline]
    pub fn rdv10(&self) -> RDV10R {
        RDV10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Receive Data Valid in RBUF1"]
    #[inline]
    pub fn rdv11(&self) -> RDV11R {
        RDV11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Data Source"]
    #[inline]
    pub fn ds1(&self) -> DS1R {
        DS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
