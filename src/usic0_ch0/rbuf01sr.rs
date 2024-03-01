#[doc = "Register `RBUF01SR` reader"]
pub type R = crate::R<Rbuf01srSpec>;
#[doc = "Field `WLEN0` reader - Received Data Word Length in RBUF0"]
pub type Wlen0R = crate::FieldReader;
#[doc = "Start of Frame in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sof0 {
    #[doc = "0: The data in RBUF0 has not been the first data word of a data frame."]
    Value1 = 0,
    #[doc = "1: The data in RBUF0 has been the first data word of a data frame."]
    Value2 = 1,
}
impl From<Sof0> for bool {
    #[inline(always)]
    fn from(variant: Sof0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF0` reader - Start of Frame in RBUF0"]
pub type Sof0R = crate::BitReader<Sof0>;
impl Sof0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sof0 {
        match self.bits {
            false => Sof0::Value1,
            true => Sof0::Value2,
        }
    }
    #[doc = "The data in RBUF0 has not been the first data word of a data frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sof0::Value1
    }
    #[doc = "The data in RBUF0 has been the first data word of a data frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sof0::Value2
    }
}
#[doc = "Field `PAR0` reader - Protocol-Related Argument in RBUF0"]
pub type Par0R = crate::BitReader;
#[doc = "Protocol-related Error in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perr0 {
    #[doc = "0: The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    Value1 = 0,
    #[doc = "1: The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    Value2 = 1,
}
impl From<Perr0> for bool {
    #[inline(always)]
    fn from(variant: Perr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERR0` reader - Protocol-related Error in RBUF0"]
pub type Perr0R = crate::BitReader<Perr0>;
impl Perr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perr0 {
        match self.bits {
            false => Perr0::Value1,
            true => Perr0::Value2,
        }
    }
    #[doc = "The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Perr0::Value1
    }
    #[doc = "The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Perr0::Value2
    }
}
#[doc = "Receive Data Valid in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdv00 {
    #[doc = "0: Register RBUF0 does not contain data that has not yet been read out."]
    Value1 = 0,
    #[doc = "1: Register RBUF0 contains data that has not yet been read out."]
    Value2 = 1,
}
impl From<Rdv00> for bool {
    #[inline(always)]
    fn from(variant: Rdv00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDV00` reader - Receive Data Valid in RBUF0"]
pub type Rdv00R = crate::BitReader<Rdv00>;
impl Rdv00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdv00 {
        match self.bits {
            false => Rdv00::Value1,
            true => Rdv00::Value2,
        }
    }
    #[doc = "Register RBUF0 does not contain data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rdv00::Value1
    }
    #[doc = "Register RBUF0 contains data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rdv00::Value2
    }
}
#[doc = "Receive Data Valid in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdv01 {
    #[doc = "0: Register RBUF1 does not contain data that has not yet been read out."]
    Value1 = 0,
    #[doc = "1: Register RBUF1 contains data that has not yet been read out."]
    Value2 = 1,
}
impl From<Rdv01> for bool {
    #[inline(always)]
    fn from(variant: Rdv01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDV01` reader - Receive Data Valid in RBUF1"]
pub type Rdv01R = crate::BitReader<Rdv01>;
impl Rdv01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdv01 {
        match self.bits {
            false => Rdv01::Value1,
            true => Rdv01::Value2,
        }
    }
    #[doc = "Register RBUF1 does not contain data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rdv01::Value1
    }
    #[doc = "Register RBUF1 contains data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rdv01::Value2
    }
}
#[doc = "Data Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ds0 {
    #[doc = "0: The register RBUF contains the data of RBUF0 (same for associated status information)."]
    Value1 = 0,
    #[doc = "1: The register RBUF contains the data of RBUF1 (same for associated status information)."]
    Value2 = 1,
}
impl From<Ds0> for bool {
    #[inline(always)]
    fn from(variant: Ds0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS0` reader - Data Source"]
pub type Ds0R = crate::BitReader<Ds0>;
impl Ds0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds0 {
        match self.bits {
            false => Ds0::Value1,
            true => Ds0::Value2,
        }
    }
    #[doc = "The register RBUF contains the data of RBUF0 (same for associated status information)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ds0::Value1
    }
    #[doc = "The register RBUF contains the data of RBUF1 (same for associated status information)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ds0::Value2
    }
}
#[doc = "Received Data Word Length in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wlen1 {
    #[doc = "0: One bit has been received."]
    Value1 = 0,
    #[doc = "15: Sixteen bits have been received."]
    Value2 = 15,
}
impl From<Wlen1> for u8 {
    #[inline(always)]
    fn from(variant: Wlen1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wlen1 {
    type Ux = u8;
}
#[doc = "Field `WLEN1` reader - Received Data Word Length in RBUF1"]
pub type Wlen1R = crate::FieldReader<Wlen1>;
impl Wlen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wlen1> {
        match self.bits {
            0 => Some(Wlen1::Value1),
            15 => Some(Wlen1::Value2),
            _ => None,
        }
    }
    #[doc = "One bit has been received."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wlen1::Value1
    }
    #[doc = "Sixteen bits have been received."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wlen1::Value2
    }
}
#[doc = "Start of Frame in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sof1 {
    #[doc = "0: The data in RBUF1 has not been the first data word of a data frame."]
    Value1 = 0,
    #[doc = "1: The data in RBUF1 has been the first data word of a data frame."]
    Value2 = 1,
}
impl From<Sof1> for bool {
    #[inline(always)]
    fn from(variant: Sof1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF1` reader - Start of Frame in RBUF1"]
pub type Sof1R = crate::BitReader<Sof1>;
impl Sof1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sof1 {
        match self.bits {
            false => Sof1::Value1,
            true => Sof1::Value2,
        }
    }
    #[doc = "The data in RBUF1 has not been the first data word of a data frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sof1::Value1
    }
    #[doc = "The data in RBUF1 has been the first data word of a data frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sof1::Value2
    }
}
#[doc = "Field `PAR1` reader - Protocol-Related Argument in RBUF1"]
pub type Par1R = crate::BitReader;
#[doc = "Protocol-related Error in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perr1 {
    #[doc = "0: The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    Value1 = 0,
    #[doc = "1: The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    Value2 = 1,
}
impl From<Perr1> for bool {
    #[inline(always)]
    fn from(variant: Perr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERR1` reader - Protocol-related Error in RBUF1"]
pub type Perr1R = crate::BitReader<Perr1>;
impl Perr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perr1 {
        match self.bits {
            false => Perr1::Value1,
            true => Perr1::Value2,
        }
    }
    #[doc = "The received protocol-related argument PAR matches the expected value. The reception of the data word sets bit PSR.RIF and can generate a receive interrupt."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Perr1::Value1
    }
    #[doc = "The received protocol-related argument PAR does not match the expected value. The reception of the data word sets bit PSR.AIF and can generate an alternative receive interrupt."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Perr1::Value2
    }
}
#[doc = "Receive Data Valid in RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdv10 {
    #[doc = "0: Register RBUF0 does not contain data that has not yet been read out."]
    Value1 = 0,
    #[doc = "1: Register RBUF0 contains data that has not yet been read out."]
    Value2 = 1,
}
impl From<Rdv10> for bool {
    #[inline(always)]
    fn from(variant: Rdv10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDV10` reader - Receive Data Valid in RBUF0"]
pub type Rdv10R = crate::BitReader<Rdv10>;
impl Rdv10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdv10 {
        match self.bits {
            false => Rdv10::Value1,
            true => Rdv10::Value2,
        }
    }
    #[doc = "Register RBUF0 does not contain data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rdv10::Value1
    }
    #[doc = "Register RBUF0 contains data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rdv10::Value2
    }
}
#[doc = "Receive Data Valid in RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdv11 {
    #[doc = "0: Register RBUF1 does not contain data that has not yet been read out."]
    Value1 = 0,
    #[doc = "1: Register RBUF1 contains data that has not yet been read out."]
    Value2 = 1,
}
impl From<Rdv11> for bool {
    #[inline(always)]
    fn from(variant: Rdv11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDV11` reader - Receive Data Valid in RBUF1"]
pub type Rdv11R = crate::BitReader<Rdv11>;
impl Rdv11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdv11 {
        match self.bits {
            false => Rdv11::Value1,
            true => Rdv11::Value2,
        }
    }
    #[doc = "Register RBUF1 does not contain data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rdv11::Value1
    }
    #[doc = "Register RBUF1 contains data that has not yet been read out."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rdv11::Value2
    }
}
#[doc = "Data Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ds1 {
    #[doc = "0: The register RBUF contains the data of RBUF0 (same for associated status information)."]
    Value1 = 0,
    #[doc = "1: The register RBUF contains the data of RBUF1 (same for associated status information)."]
    Value2 = 1,
}
impl From<Ds1> for bool {
    #[inline(always)]
    fn from(variant: Ds1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS1` reader - Data Source"]
pub type Ds1R = crate::BitReader<Ds1>;
impl Ds1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds1 {
        match self.bits {
            false => Ds1::Value1,
            true => Ds1::Value2,
        }
    }
    #[doc = "The register RBUF contains the data of RBUF0 (same for associated status information)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ds1::Value1
    }
    #[doc = "The register RBUF contains the data of RBUF1 (same for associated status information)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ds1::Value2
    }
}
impl R {
    #[doc = "Bits 0:3 - Received Data Word Length in RBUF0"]
    #[inline(always)]
    pub fn wlen0(&self) -> Wlen0R {
        Wlen0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Start of Frame in RBUF0"]
    #[inline(always)]
    pub fn sof0(&self) -> Sof0R {
        Sof0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol-Related Argument in RBUF0"]
    #[inline(always)]
    pub fn par0(&self) -> Par0R {
        Par0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protocol-related Error in RBUF0"]
    #[inline(always)]
    pub fn perr0(&self) -> Perr0R {
        Perr0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Data Valid in RBUF0"]
    #[inline(always)]
    pub fn rdv00(&self) -> Rdv00R {
        Rdv00R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Data Valid in RBUF1"]
    #[inline(always)]
    pub fn rdv01(&self) -> Rdv01R {
        Rdv01R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Source"]
    #[inline(always)]
    pub fn ds0(&self) -> Ds0R {
        Ds0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Received Data Word Length in RBUF1"]
    #[inline(always)]
    pub fn wlen1(&self) -> Wlen1R {
        Wlen1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Start of Frame in RBUF1"]
    #[inline(always)]
    pub fn sof1(&self) -> Sof1R {
        Sof1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Protocol-Related Argument in RBUF1"]
    #[inline(always)]
    pub fn par1(&self) -> Par1R {
        Par1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protocol-related Error in RBUF1"]
    #[inline(always)]
    pub fn perr1(&self) -> Perr1R {
        Perr1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive Data Valid in RBUF0"]
    #[inline(always)]
    pub fn rdv10(&self) -> Rdv10R {
        Rdv10R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Receive Data Valid in RBUF1"]
    #[inline(always)]
    pub fn rdv11(&self) -> Rdv11R {
        Rdv11R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data Source"]
    #[inline(always)]
    pub fn ds1(&self) -> Ds1R {
        Ds1R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Receiver Buffer 01 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf01sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rbuf01srSpec;
impl crate::RegisterSpec for Rbuf01srSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbuf01sr::R`](R) reader structure"]
impl crate::Readable for Rbuf01srSpec {}
#[doc = "`reset()` method sets RBUF01SR to value 0"]
impl crate::Resettable for Rbuf01srSpec {
    const RESET_VALUE: u32 = 0;
}
