#[doc = "Register `WDTCLKCR` reader"]
pub type R = crate::R<WDTCLKCR_SPEC>;
#[doc = "Register `WDTCLKCR` writer"]
pub type W = crate::W<WDTCLKCR_SPEC>;
#[doc = "Field `WDTDIV` reader - WDT Clock Divider Value"]
pub type WDTDIV_R = crate::FieldReader;
#[doc = "Field `WDTDIV` writer - WDT Clock Divider Value"]
pub type WDTDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WDTSEL` reader - WDT Clock Selection Value"]
pub type WDTSEL_R = crate::FieldReader<WDTSEL_A>;
#[doc = "WDT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTSEL_A {
    #[doc = "0: fOFI clock"]
    CONST_00 = 0,
    #[doc = "1: fSTDBY clock"]
    CONST_01 = 1,
    #[doc = "2: fPLL clock"]
    CONST_10 = 2,
}
impl From<WDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDTSEL_A {
    type Ux = u8;
}
impl WDTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDTSEL_A> {
        match self.bits {
            0 => Some(WDTSEL_A::CONST_00),
            1 => Some(WDTSEL_A::CONST_01),
            2 => Some(WDTSEL_A::CONST_10),
            _ => None,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == WDTSEL_A::CONST_00
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == WDTSEL_A::CONST_01
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == WDTSEL_A::CONST_10
    }
}
#[doc = "Field `WDTSEL` writer - WDT Clock Selection Value"]
pub type WDTSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, WDTSEL_A>;
impl<'a, REG, const O: u8> WDTSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSEL_A::CONST_00)
    }
    #[doc = "fSTDBY clock"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSEL_A::CONST_01)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSEL_A::CONST_10)
    }
}
impl R {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WDTDIV_R {
        WDTDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDT Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtdiv(&mut self) -> WDTDIV_W<WDTCLKCR_SPEC, 0> {
        WDTDIV_W::new(self)
    }
    #[doc = "Bits 16:17 - WDT Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdtsel(&mut self) -> WDTSEL_W<WDTCLKCR_SPEC, 16> {
        WDTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "WDT Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCLKCR_SPEC;
impl crate::RegisterSpec for WDTCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtclkcr::R`](R) reader structure"]
impl crate::Readable for WDTCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtclkcr::W`](W) writer structure"]
impl crate::Writable for WDTCLKCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCLKCR to value 0"]
impl crate::Resettable for WDTCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
