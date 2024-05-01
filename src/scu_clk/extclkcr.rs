#[doc = "Register `EXTCLKCR` reader"]
pub type R = crate::R<ExtclkcrSpec>;
#[doc = "Register `EXTCLKCR` writer"]
pub type W = crate::W<ExtclkcrSpec>;
#[doc = "External Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ecksel {
    #[doc = "0: fSYS clock"]
    Const00 = 0,
    #[doc = "2: fUSB clock divided according to ECKDIV bit field configuration"]
    Const10 = 2,
    #[doc = "3: fPLL clock divided according to ECKDIV bit field configuration"]
    Const11 = 3,
}
impl From<Ecksel> for u8 {
    #[inline(always)]
    fn from(variant: Ecksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ecksel {
    type Ux = u8;
}
impl crate::IsEnum for Ecksel {}
#[doc = "Field `ECKSEL` reader - External Clock Selection Value"]
pub type EckselR = crate::FieldReader<Ecksel>;
impl EckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ecksel> {
        match self.bits {
            0 => Some(Ecksel::Const00),
            2 => Some(Ecksel::Const10),
            3 => Some(Ecksel::Const11),
            _ => None,
        }
    }
    #[doc = "fSYS clock"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == Ecksel::Const00
    }
    #[doc = "fUSB clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == Ecksel::Const10
    }
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == Ecksel::Const11
    }
}
#[doc = "Field `ECKSEL` writer - External Clock Selection Value"]
pub type EckselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ecksel>;
impl<'a, REG> EckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fSYS clock"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(Ecksel::Const00)
    }
    #[doc = "fUSB clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ecksel::Const10)
    }
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ecksel::Const11)
    }
}
#[doc = "Field `ECKDIV` reader - External Clock Divider Value"]
pub type EckdivR = crate::FieldReader<u16>;
#[doc = "Field `ECKDIV` writer - External Clock Divider Value"]
pub type EckdivW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline(always)]
    pub fn ecksel(&self) -> EckselR {
        EckselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    pub fn eckdiv(&self) -> EckdivR {
        EckdivR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecksel(&mut self) -> EckselW<ExtclkcrSpec> {
        EckselW::new(self, 0)
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn eckdiv(&mut self) -> EckdivW<ExtclkcrSpec> {
        EckdivW::new(self, 16)
    }
}
#[doc = "External Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtclkcrSpec;
impl crate::RegisterSpec for ExtclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extclkcr::R`](R) reader structure"]
impl crate::Readable for ExtclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`extclkcr::W`](W) writer structure"]
impl crate::Writable for ExtclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTCLKCR to value 0"]
impl crate::Resettable for ExtclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
