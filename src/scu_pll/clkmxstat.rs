#[doc = "Register `CLKMXSTAT` reader"]
pub type R = crate::R<CLKMXSTAT_SPEC>;
#[doc = "Field `SYSCLKMUX` reader - Status of System Clock Multiplexing Upon Source Switching"]
pub type SYSCLKMUX_R = crate::FieldReader<SYSCLKMUX_A>;
#[doc = "Status of System Clock Multiplexing Upon Source Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCLKMUX_A {
    #[doc = "1: fOFI clock active"]
    CONST_X1 = 1,
    #[doc = "2: fPLL clock active"]
    CONST_1X = 2,
}
impl From<SYSCLKMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLKMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCLKMUX_A {
    type Ux = u8;
}
impl SYSCLKMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCLKMUX_A> {
        match self.bits {
            1 => Some(SYSCLKMUX_A::CONST_X1),
            2 => Some(SYSCLKMUX_A::CONST_1X),
            _ => None,
        }
    }
    #[doc = "fOFI clock active"]
    #[inline(always)]
    pub fn is_const_x1(&self) -> bool {
        *self == SYSCLKMUX_A::CONST_X1
    }
    #[doc = "fPLL clock active"]
    #[inline(always)]
    pub fn is_const_1x(&self) -> bool {
        *self == SYSCLKMUX_A::CONST_1X
    }
}
impl R {
    #[doc = "Bits 0:1 - Status of System Clock Multiplexing Upon Source Switching"]
    #[inline(always)]
    pub fn sysclkmux(&self) -> SYSCLKMUX_R {
        SYSCLKMUX_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Clock Multiplexing Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkmxstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKMXSTAT_SPEC;
impl crate::RegisterSpec for CLKMXSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkmxstat::R`](R) reader structure"]
impl crate::Readable for CLKMXSTAT_SPEC {}
#[doc = "`reset()` method sets CLKMXSTAT to value 0"]
impl crate::Resettable for CLKMXSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
