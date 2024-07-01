#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DSTS_SPEC>;
#[doc = "Field `SuspSts` reader - Suspend Status"]
pub type SUSP_STS_R = crate::BitReader;
#[doc = "Enumerated Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENUM_SPD_A {
    #[doc = "3: Full speed (PHY clock is running at 48 MHz)"]
    VALUE4 = 3,
}
impl From<ENUM_SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUM_SPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ENUM_SPD_A {
    type Ux = u8;
}
impl crate::IsEnum for ENUM_SPD_A {}
#[doc = "Field `EnumSpd` reader - Enumerated Speed"]
pub type ENUM_SPD_R = crate::FieldReader<ENUM_SPD_A>;
impl ENUM_SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ENUM_SPD_A> {
        match self.bits {
            3 => Some(ENUM_SPD_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Full speed (PHY clock is running at 48 MHz)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ENUM_SPD_A::VALUE4
    }
}
#[doc = "Field `ErrticErr` reader - Erratic Error"]
pub type ERRTIC_ERR_R = crate::BitReader;
#[doc = "Field `SOFFN` reader - Frame Number of the Received SOF"]
pub type SOFFN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn susp_sts(&self) -> SUSP_STS_R {
        SUSP_STS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enum_spd(&self) -> ENUM_SPD_R {
        ENUM_SPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errtic_err(&self) -> ERRTIC_ERR_R {
        ERRTIC_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame Number of the Received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
#[doc = "Device Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DSTS_SPEC {}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DSTS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
