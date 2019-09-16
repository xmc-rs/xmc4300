#[doc = "Reader of register RBUF01SR"]
pub type R = crate::R<u32, super::RBUF01SR>;
#[doc = "Reader of field `WLEN0`"]
pub type WLEN0_R = crate::R<u8, u8>;
#[doc = "Start of Frame in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF0_A {
    #[doc = "0: The data in RBUF0 has not been the first data word of a data frame."]
    VALUE1,
    #[doc = "1: The data in RBUF0 has been the first data word of a data frame."]
    VALUE2,
}
impl From<SOF0_A> for bool {
    #[inline(always)]
    fn from(variant: SOF0_A) -> Self {
        match variant {
            SOF0_A::VALUE1 => false,
            SOF0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SOF0`"]
pub type SOF0_R = crate::R<bool, SOF0_A>;
impl SOF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF0_A {
        match self.bits {
            false => SOF0_A::VALUE1,
            true => SOF0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SOF0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SOF0_A::VALUE2
    }
}
#[doc = "Reader of field `PAR0`"]
pub type PAR0_R = crate::R<bool, bool>;
#[doc = "Protocol-related Error in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERR0_A {
    #[doc = "0: The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    VALUE1,
    #[doc = "1: The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    VALUE2,
}
impl From<PERR0_A> for bool {
    #[inline(always)]
    fn from(variant: PERR0_A) -> Self {
        match variant {
            PERR0_A::VALUE1 => false,
            PERR0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PERR0`"]
pub type PERR0_R = crate::R<bool, PERR0_A>;
impl PERR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERR0_A {
        match self.bits {
            false => PERR0_A::VALUE1,
            true => PERR0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PERR0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PERR0_A::VALUE2
    }
}
#[doc = "Receive Data Valid in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDV00_A {
    #[doc = "0: Register RBUF0 does not contain data that has not yet been read out."]
    VALUE1,
    #[doc = "1: Register RBUF0 contains data that has not yet been read out."]
    VALUE2,
}
impl From<RDV00_A> for bool {
    #[inline(always)]
    fn from(variant: RDV00_A) -> Self {
        match variant {
            RDV00_A::VALUE1 => false,
            RDV00_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RDV00`"]
pub type RDV00_R = crate::R<bool, RDV00_A>;
impl RDV00_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDV00_A {
        match self.bits {
            false => RDV00_A::VALUE1,
            true => RDV00_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDV00_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDV00_A::VALUE2
    }
}
#[doc = "Receive Data Valid in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDV01_A {
    #[doc = "0: Register RBUF1 does not contain data that has not yet been read out."]
    VALUE1,
    #[doc = "1: Register RBUF1 contains data that has not yet been read out."]
    VALUE2,
}
impl From<RDV01_A> for bool {
    #[inline(always)]
    fn from(variant: RDV01_A) -> Self {
        match variant {
            RDV01_A::VALUE1 => false,
            RDV01_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RDV01`"]
pub type RDV01_R = crate::R<bool, RDV01_A>;
impl RDV01_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDV01_A {
        match self.bits {
            false => RDV01_A::VALUE1,
            true => RDV01_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDV01_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDV01_A::VALUE2
    }
}
#[doc = "Data Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DS0_A {
    #[doc = "0: The register RBUF contains the data of RBUF0 (same for associated status information)."]
    VALUE1,
    #[doc = "1: The register RBUF contains the data of RBUF1 (same for associated status information)."]
    VALUE2,
}
impl From<DS0_A> for bool {
    #[inline(always)]
    fn from(variant: DS0_A) -> Self {
        match variant {
            DS0_A::VALUE1 => false,
            DS0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DS0`"]
pub type DS0_R = crate::R<bool, DS0_A>;
impl DS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DS0_A {
        match self.bits {
            false => DS0_A::VALUE1,
            true => DS0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DS0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DS0_A::VALUE2
    }
}
#[doc = "Received Data Word Length in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLEN1_A {
    #[doc = "0: One bit has been received."]
    VALUE1,
    #[doc = "15: Sixteen bits have been received."]
    VALUE2,
}
impl From<WLEN1_A> for u8 {
    #[inline(always)]
    fn from(variant: WLEN1_A) -> Self {
        match variant {
            WLEN1_A::VALUE1 => 0,
            WLEN1_A::VALUE2 => 15,
        }
    }
}
#[doc = "Reader of field `WLEN1`"]
pub type WLEN1_R = crate::R<u8, WLEN1_A>;
impl WLEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WLEN1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WLEN1_A::VALUE1),
            15 => Val(WLEN1_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WLEN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WLEN1_A::VALUE2
    }
}
#[doc = "Start of Frame in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF1_A {
    #[doc = "0: The data in RBUF1 has not been the first data word of a data frame."]
    VALUE1,
    #[doc = "1: The data in RBUF1 has been the first data word of a data frame."]
    VALUE2,
}
impl From<SOF1_A> for bool {
    #[inline(always)]
    fn from(variant: SOF1_A) -> Self {
        match variant {
            SOF1_A::VALUE1 => false,
            SOF1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SOF1`"]
pub type SOF1_R = crate::R<bool, SOF1_A>;
impl SOF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF1_A {
        match self.bits {
            false => SOF1_A::VALUE1,
            true => SOF1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SOF1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SOF1_A::VALUE2
    }
}
#[doc = "Reader of field `PAR1`"]
pub type PAR1_R = crate::R<bool, bool>;
#[doc = "Protocol-related Error in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERR1_A {
    #[doc = "0: The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    VALUE1,
    #[doc = "1: The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    VALUE2,
}
impl From<PERR1_A> for bool {
    #[inline(always)]
    fn from(variant: PERR1_A) -> Self {
        match variant {
            PERR1_A::VALUE1 => false,
            PERR1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PERR1`"]
pub type PERR1_R = crate::R<bool, PERR1_A>;
impl PERR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERR1_A {
        match self.bits {
            false => PERR1_A::VALUE1,
            true => PERR1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PERR1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PERR1_A::VALUE2
    }
}
#[doc = "Receive Data Valid in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDV10_A {
    #[doc = "0: Register RBUF0 does not contain data that has not yet been read out."]
    VALUE1,
    #[doc = "1: Register RBUF0 contains data that has not yet been read out."]
    VALUE2,
}
impl From<RDV10_A> for bool {
    #[inline(always)]
    fn from(variant: RDV10_A) -> Self {
        match variant {
            RDV10_A::VALUE1 => false,
            RDV10_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RDV10`"]
pub type RDV10_R = crate::R<bool, RDV10_A>;
impl RDV10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDV10_A {
        match self.bits {
            false => RDV10_A::VALUE1,
            true => RDV10_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDV10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDV10_A::VALUE2
    }
}
#[doc = "Receive Data Valid in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDV11_A {
    #[doc = "0: Register RBUF1 does not contain data that has not yet been read out."]
    VALUE1,
    #[doc = "1: Register RBUF1 contains data that has not yet been read out."]
    VALUE2,
}
impl From<RDV11_A> for bool {
    #[inline(always)]
    fn from(variant: RDV11_A) -> Self {
        match variant {
            RDV11_A::VALUE1 => false,
            RDV11_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RDV11`"]
pub type RDV11_R = crate::R<bool, RDV11_A>;
impl RDV11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDV11_A {
        match self.bits {
            false => RDV11_A::VALUE1,
            true => RDV11_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDV11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDV11_A::VALUE2
    }
}
#[doc = "Data Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DS1_A {
    #[doc = "0: The register RBUF contains the data of RBUF0 (same for associated status information)."]
    VALUE1,
    #[doc = "1: The register RBUF contains the data of RBUF1 (same for associated status information)."]
    VALUE2,
}
impl From<DS1_A> for bool {
    #[inline(always)]
    fn from(variant: DS1_A) -> Self {
        match variant {
            DS1_A::VALUE1 => false,
            DS1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DS1`"]
pub type DS1_R = crate::R<bool, DS1_A>;
impl DS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DS1_A {
        match self.bits {
            false => DS1_A::VALUE1,
            true => DS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DS1_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:3 - Received Data Word Length in RBUF0"]
    #[inline(always)]
    pub fn wlen0(&self) -> WLEN0_R {
        WLEN0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Start of Frame in RBUF0"]
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Protocol-Related Argument in RBUF0"]
    #[inline(always)]
    pub fn par0(&self) -> PAR0_R {
        PAR0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protocol-related Error in RBUF0"]
    #[inline(always)]
    pub fn perr0(&self) -> PERR0_R {
        PERR0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Receive Data Valid in RBUF0"]
    #[inline(always)]
    pub fn rdv00(&self) -> RDV00_R {
        RDV00_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Data Valid in RBUF1"]
    #[inline(always)]
    pub fn rdv01(&self) -> RDV01_R {
        RDV01_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Data Source"]
    #[inline(always)]
    pub fn ds0(&self) -> DS0_R {
        DS0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Received Data Word Length in RBUF1"]
    #[inline(always)]
    pub fn wlen1(&self) -> WLEN1_R {
        WLEN1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Start of Frame in RBUF1"]
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Protocol-Related Argument in RBUF1"]
    #[inline(always)]
    pub fn par1(&self) -> PAR1_R {
        PAR1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Protocol-related Error in RBUF1"]
    #[inline(always)]
    pub fn perr1(&self) -> PERR1_R {
        PERR1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Receive Data Valid in RBUF0"]
    #[inline(always)]
    pub fn rdv10(&self) -> RDV10_R {
        RDV10_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Receive Data Valid in RBUF1"]
    #[inline(always)]
    pub fn rdv11(&self) -> RDV11_R {
        RDV11_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Data Source"]
    #[inline(always)]
    pub fn ds1(&self) -> DS1_R {
        DS1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
