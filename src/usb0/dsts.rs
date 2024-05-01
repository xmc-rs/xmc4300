#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DstsSpec>;
#[doc = "Field `SuspSts` reader - Suspend Status"]
pub type SuspStsR = crate::BitReader;
#[doc = "Enumerated Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnumSpd {
    #[doc = "3: Full speed (PHY clock is running at 48 MHz)"]
    Value4 = 3,
}
impl From<EnumSpd> for u8 {
    #[inline(always)]
    fn from(variant: EnumSpd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EnumSpd {
    type Ux = u8;
}
impl crate::IsEnum for EnumSpd {}
#[doc = "Field `EnumSpd` reader - Enumerated Speed"]
pub type EnumSpdR = crate::FieldReader<EnumSpd>;
impl EnumSpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EnumSpd> {
        match self.bits {
            3 => Some(EnumSpd::Value4),
            _ => None,
        }
    }
    #[doc = "Full speed (PHY clock is running at 48 MHz)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EnumSpd::Value4
    }
}
#[doc = "Field `ErrticErr` reader - Erratic Error"]
pub type ErrticErrR = crate::BitReader;
#[doc = "Field `SOFFN` reader - Frame Number of the Received SOF"]
pub type SoffnR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn susp_sts(&self) -> SuspStsR {
        SuspStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enum_spd(&self) -> EnumSpdR {
        EnumSpdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errtic_err(&self) -> ErrticErrR {
        ErrticErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame Number of the Received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SoffnR {
        SoffnR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
#[doc = "Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstsSpec;
impl crate::RegisterSpec for DstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DstsSpec {}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DstsSpec {
    const RESET_VALUE: u32 = 0x02;
}
