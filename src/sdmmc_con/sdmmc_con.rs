#[doc = "Register `SDMMC_CON` reader"]
pub type R = crate::R<SdmmcConSpec>;
#[doc = "Register `SDMMC_CON` writer"]
pub type W = crate::W<SdmmcConSpec>;
#[doc = "SDMMC Write Protection Input Multiplexer Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wpsel {
    #[doc = "0: P1.1 input pin selected"]
    Value1 = 0,
    #[doc = "1: Software bit WPVAL is selected"]
    Value2 = 1,
}
impl From<Wpsel> for bool {
    #[inline(always)]
    fn from(variant: Wpsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPSEL` reader - SDMMC Write Protection Input Multiplexer Control"]
pub type WpselR = crate::BitReader<Wpsel>;
impl WpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wpsel {
        match self.bits {
            false => Wpsel::Value1,
            true => Wpsel::Value2,
        }
    }
    #[doc = "P1.1 input pin selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wpsel::Value1
    }
    #[doc = "Software bit WPVAL is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wpsel::Value2
    }
}
#[doc = "Field `WPSEL` writer - SDMMC Write Protection Input Multiplexer Control"]
pub type WpselW<'a, REG> = crate::BitWriter<'a, REG, Wpsel>;
impl<'a, REG> WpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "P1.1 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wpsel::Value1)
    }
    #[doc = "Software bit WPVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wpsel::Value2)
    }
}
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wpsval {
    #[doc = "0: No write protection"]
    Value1 = 0,
    #[doc = "1: Write protection active"]
    Value2 = 1,
}
impl From<Wpsval> for bool {
    #[inline(always)]
    fn from(variant: Wpsval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPSVAL` reader - SDMMC Write Protect Software Control"]
pub type WpsvalR = crate::BitReader<Wpsval>;
impl WpsvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wpsval {
        match self.bits {
            false => Wpsval::Value1,
            true => Wpsval::Value2,
        }
    }
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wpsval::Value1
    }
    #[doc = "Write protection active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wpsval::Value2
    }
}
#[doc = "Field `WPSVAL` writer - SDMMC Write Protect Software Control"]
pub type WpsvalW<'a, REG> = crate::BitWriter<'a, REG, Wpsval>;
impl<'a, REG> WpsvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wpsval::Value1)
    }
    #[doc = "Write protection active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wpsval::Value2)
    }
}
#[doc = "SDMMC Card Detection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdsel {
    #[doc = "0: P1.10 input pin selected"]
    Value1 = 0,
    #[doc = "1: Software bit CDSVAL is selected"]
    Value2 = 1,
}
impl From<Cdsel> for bool {
    #[inline(always)]
    fn from(variant: Cdsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDSEL` reader - SDMMC Card Detection Control"]
pub type CdselR = crate::BitReader<Cdsel>;
impl CdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdsel {
        match self.bits {
            false => Cdsel::Value1,
            true => Cdsel::Value2,
        }
    }
    #[doc = "P1.10 input pin selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cdsel::Value1
    }
    #[doc = "Software bit CDSVAL is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cdsel::Value2
    }
}
#[doc = "Field `CDSEL` writer - SDMMC Card Detection Control"]
pub type CdselW<'a, REG> = crate::BitWriter<'a, REG, Cdsel>;
impl<'a, REG> CdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "P1.10 input pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdsel::Value1)
    }
    #[doc = "Software bit CDSVAL is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cdsel::Value2)
    }
}
#[doc = "SDMMC Write Protect Software Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdsval {
    #[doc = "0: No card detected"]
    Value1 = 0,
    #[doc = "1: Card detected"]
    Value2 = 1,
}
impl From<Cdsval> for bool {
    #[inline(always)]
    fn from(variant: Cdsval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDSVAL` reader - SDMMC Write Protect Software Control"]
pub type CdsvalR = crate::BitReader<Cdsval>;
impl CdsvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdsval {
        match self.bits {
            false => Cdsval::Value1,
            true => Cdsval::Value2,
        }
    }
    #[doc = "No card detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cdsval::Value1
    }
    #[doc = "Card detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cdsval::Value2
    }
}
#[doc = "Field `CDSVAL` writer - SDMMC Write Protect Software Control"]
pub type CdsvalW<'a, REG> = crate::BitWriter<'a, REG, Cdsval>;
impl<'a, REG> CdsvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No card detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdsval::Value1)
    }
    #[doc = "Card detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cdsval::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    pub fn wpsel(&self) -> WpselR {
        WpselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn wpsval(&self) -> WpsvalR {
        WpsvalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    pub fn cdsel(&self) -> CdselR {
        CdselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    pub fn cdsval(&self) -> CdsvalR {
        CdsvalR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDMMC Write Protection Input Multiplexer Control"]
    #[inline(always)]
    #[must_use]
    pub fn wpsel(&mut self) -> WpselW<SdmmcConSpec> {
        WpselW::new(self, 0)
    }
    #[doc = "Bit 4 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    #[must_use]
    pub fn wpsval(&mut self) -> WpsvalW<SdmmcConSpec> {
        WpsvalW::new(self, 4)
    }
    #[doc = "Bit 16 - SDMMC Card Detection Control"]
    #[inline(always)]
    #[must_use]
    pub fn cdsel(&mut self) -> CdselW<SdmmcConSpec> {
        CdselW::new(self, 16)
    }
    #[doc = "Bit 20 - SDMMC Write Protect Software Control"]
    #[inline(always)]
    #[must_use]
    pub fn cdsval(&mut self) -> CdsvalW<SdmmcConSpec> {
        CdsvalW::new(self, 20)
    }
}
#[doc = "SDMMC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcConSpec;
impl crate::RegisterSpec for SdmmcConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_con::R`](R) reader structure"]
impl crate::Readable for SdmmcConSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_con::W`](W) writer structure"]
impl crate::Writable for SdmmcConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_CON to value 0"]
impl crate::Resettable for SdmmcConSpec {
    const RESET_VALUE: u32 = 0;
}
