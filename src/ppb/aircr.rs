#[doc = "Register `AIRCR` reader"]
pub type R = crate::R<AircrSpec>;
#[doc = "Register `AIRCR` writer"]
pub type W = crate::W<AircrSpec>;
#[doc = "Field `VECTRESET` writer - Reserved for Debug use."]
pub type VectresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTCLRACTIVE` writer - Reserved for Debug use."]
pub type VectclractiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "System reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysresetreq {
    #[doc = "0: no system reset request"]
    Value1 = 0,
    #[doc = "1: asserts a signal to the outer system that requests a reset."]
    Value2 = 1,
}
impl From<Sysresetreq> for bool {
    #[inline(always)]
    fn from(variant: Sysresetreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRESETREQ` writer - System reset request"]
pub type SysresetreqW<'a, REG> = crate::BitWriter<'a, REG, Sysresetreq>;
impl<'a, REG> SysresetreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no system reset request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sysresetreq::Value1)
    }
    #[doc = "asserts a signal to the outer system that requests a reset."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sysresetreq::Value2)
    }
}
#[doc = "Field `PRIGROUP` reader - Interrupt priority grouping field"]
pub type PrigroupR = crate::FieldReader;
#[doc = "Field `PRIGROUP` writer - Interrupt priority grouping field"]
pub type PrigroupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Data endianness bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endianness {
    #[doc = "0: Little-endian"]
    Value1 = 0,
    #[doc = "1: Big-endian."]
    Value2 = 1,
}
impl From<Endianness> for bool {
    #[inline(always)]
    fn from(variant: Endianness) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIANNESS` reader - Data endianness bit"]
pub type EndiannessR = crate::BitReader<Endianness>;
impl EndiannessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endianness {
        match self.bits {
            false => Endianness::Value1,
            true => Endianness::Value2,
        }
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Endianness::Value1
    }
    #[doc = "Big-endian."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Endianness::Value2
    }
}
#[doc = "Field `VECTKEY` reader - Register key"]
pub type VectkeyR = crate::FieldReader<u16>;
#[doc = "Field `VECTKEY` writer - Register key"]
pub type VectkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:10 - Interrupt priority grouping field"]
    #[inline(always)]
    pub fn prigroup(&self) -> PrigroupR {
        PrigroupR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Data endianness bit"]
    #[inline(always)]
    pub fn endianness(&self) -> EndiannessR {
        EndiannessR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&self) -> VectkeyR {
        VectkeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved for Debug use."]
    #[inline(always)]
    #[must_use]
    pub fn vectreset(&mut self) -> VectresetW<AircrSpec> {
        VectresetW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved for Debug use."]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VectclractiveW<AircrSpec> {
        VectclractiveW::new(self, 1)
    }
    #[doc = "Bit 2 - System reset request"]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SysresetreqW<AircrSpec> {
        SysresetreqW::new(self, 2)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field"]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PrigroupW<AircrSpec> {
        PrigroupW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VectkeyW<AircrSpec> {
        VectkeyW::new(self, 16)
    }
}
#[doc = "Application Interrupt and Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aircr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aircr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AircrSpec;
impl crate::RegisterSpec for AircrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aircr::R`](R) reader structure"]
impl crate::Readable for AircrSpec {}
#[doc = "`write(|w| ..)` method takes [`aircr::W`](W) writer structure"]
impl crate::Writable for AircrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0xfa05_0000"]
impl crate::Resettable for AircrSpec {
    const RESET_VALUE: u32 = 0xfa05_0000;
}
