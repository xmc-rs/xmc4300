#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "CRC Mismatch Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmi {
    #[doc = "0: CRC Mismatch Interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: CRC Mismatch Interrupt is enabled"]
    Value2 = 1,
}
impl From<Cmi> for bool {
    #[inline(always)]
    fn from(variant: Cmi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMI` reader - CRC Mismatch Interrupt"]
pub type CmiR = crate::BitReader<Cmi>;
impl CmiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmi {
        match self.bits {
            false => Cmi::Value1,
            true => Cmi::Value2,
        }
    }
    #[doc = "CRC Mismatch Interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cmi::Value1
    }
    #[doc = "CRC Mismatch Interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cmi::Value2
    }
}
#[doc = "Field `CMI` writer - CRC Mismatch Interrupt"]
pub type CmiW<'a, REG> = crate::BitWriter<'a, REG, Cmi>;
impl<'a, REG> CmiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC Mismatch Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmi::Value1)
    }
    #[doc = "CRC Mismatch Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmi::Value2)
    }
}
#[doc = "Configuration Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cei {
    #[doc = "0: Configuration Error Interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Configuration Error Interrupt is enabled"]
    Value2 = 1,
}
impl From<Cei> for bool {
    #[inline(always)]
    fn from(variant: Cei) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEI` reader - Configuration Error Interrupt"]
pub type CeiR = crate::BitReader<Cei>;
impl CeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cei {
        match self.bits {
            false => Cei::Value1,
            true => Cei::Value2,
        }
    }
    #[doc = "Configuration Error Interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cei::Value1
    }
    #[doc = "Configuration Error Interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cei::Value2
    }
}
#[doc = "Field `CEI` writer - Configuration Error Interrupt"]
pub type CeiW<'a, REG> = crate::BitWriter<'a, REG, Cei>;
impl<'a, REG> CeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configuration Error Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cei::Value1)
    }
    #[doc = "Configuration Error Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cei::Value2)
    }
}
#[doc = "Length Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lei {
    #[doc = "0: Length Error Interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Length Error Interrupt is enabled"]
    Value2 = 1,
}
impl From<Lei> for bool {
    #[inline(always)]
    fn from(variant: Lei) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEI` reader - Length Error Interrupt"]
pub type LeiR = crate::BitReader<Lei>;
impl LeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lei {
        match self.bits {
            false => Lei::Value1,
            true => Lei::Value2,
        }
    }
    #[doc = "Length Error Interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lei::Value1
    }
    #[doc = "Length Error Interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lei::Value2
    }
}
#[doc = "Field `LEI` writer - Length Error Interrupt"]
pub type LeiW<'a, REG> = crate::BitWriter<'a, REG, Lei>;
impl<'a, REG> LeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Length Error Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lei::Value1)
    }
    #[doc = "Length Error Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lei::Value2)
    }
}
#[doc = "Bus Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bei {
    #[doc = "0: Bus Error Interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Bus Error Interrupt is enabled"]
    Value2 = 1,
}
impl From<Bei> for bool {
    #[inline(always)]
    fn from(variant: Bei) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEI` reader - Bus Error Interrupt"]
pub type BeiR = crate::BitReader<Bei>;
impl BeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bei {
        match self.bits {
            false => Bei::Value1,
            true => Bei::Value2,
        }
    }
    #[doc = "Bus Error Interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bei::Value1
    }
    #[doc = "Bus Error Interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bei::Value2
    }
}
#[doc = "Field `BEI` writer - Bus Error Interrupt"]
pub type BeiW<'a, REG> = crate::BitWriter<'a, REG, Bei>;
impl<'a, REG> BeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus Error Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bei::Value1)
    }
    #[doc = "Bus Error Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bei::Value2)
    }
}
#[doc = "CRC Check Comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cce {
    #[doc = "0: CRC check comparison at the end of a message is disabled"]
    Value1 = 0,
    #[doc = "1: CRC check comparison at the end of a message is enabled"]
    Value2 = 1,
}
impl From<Cce> for bool {
    #[inline(always)]
    fn from(variant: Cce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - CRC Check Comparison"]
pub type CceR = crate::BitReader<Cce>;
impl CceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cce {
        match self.bits {
            false => Cce::Value1,
            true => Cce::Value2,
        }
    }
    #[doc = "CRC check comparison at the end of a message is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cce::Value1
    }
    #[doc = "CRC check comparison at the end of a message is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cce::Value2
    }
}
#[doc = "Field `CCE` writer - CRC Check Comparison"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG, Cce>;
impl<'a, REG> CceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC check comparison at the end of a message is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cce::Value1)
    }
    #[doc = "CRC check comparison at the end of a message is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cce::Value2)
    }
}
#[doc = "Automatic Length Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alr {
    #[doc = "0: Disables automatic reload of the LENGTH field."]
    Value1 = 0,
    #[doc = "1: Enables automatic reload of the LENGTH field at the end of a message."]
    Value2 = 1,
}
impl From<Alr> for bool {
    #[inline(always)]
    fn from(variant: Alr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALR` reader - Automatic Length Reload"]
pub type AlrR = crate::BitReader<Alr>;
impl AlrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alr {
        match self.bits {
            false => Alr::Value1,
            true => Alr::Value2,
        }
    }
    #[doc = "Disables automatic reload of the LENGTH field."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Alr::Value1
    }
    #[doc = "Enables automatic reload of the LENGTH field at the end of a message."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Alr::Value2
    }
}
#[doc = "Field `ALR` writer - Automatic Length Reload"]
pub type AlrW<'a, REG> = crate::BitWriter<'a, REG, Alr>;
impl<'a, REG> AlrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables automatic reload of the LENGTH field."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alr::Value1)
    }
    #[doc = "Enables automatic reload of the LENGTH field at the end of a message."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alr::Value2)
    }
}
#[doc = "IR Byte Wise Reflection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refin {
    #[doc = "0: IR Byte Wise Reflection is disabled"]
    Value1 = 0,
    #[doc = "1: IR Byte Wise Reflection is enabled"]
    Value2 = 1,
}
impl From<Refin> for bool {
    #[inline(always)]
    fn from(variant: Refin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFIN` reader - IR Byte Wise Reflection"]
pub type RefinR = crate::BitReader<Refin>;
impl RefinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refin {
        match self.bits {
            false => Refin::Value1,
            true => Refin::Value2,
        }
    }
    #[doc = "IR Byte Wise Reflection is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Refin::Value1
    }
    #[doc = "IR Byte Wise Reflection is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Refin::Value2
    }
}
#[doc = "Field `REFIN` writer - IR Byte Wise Reflection"]
pub type RefinW<'a, REG> = crate::BitWriter<'a, REG, Refin>;
impl<'a, REG> RefinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IR Byte Wise Reflection is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Refin::Value1)
    }
    #[doc = "IR Byte Wise Reflection is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Refin::Value2)
    }
}
#[doc = "CRC 32-Bit Wise Reflection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refout {
    #[doc = "0: CRC 32-bit wise is disabled"]
    Value1 = 0,
    #[doc = "1: CRC 32-bit wise is enabled"]
    Value2 = 1,
}
impl From<Refout> for bool {
    #[inline(always)]
    fn from(variant: Refout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFOUT` reader - CRC 32-Bit Wise Reflection"]
pub type RefoutR = crate::BitReader<Refout>;
impl RefoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refout {
        match self.bits {
            false => Refout::Value1,
            true => Refout::Value2,
        }
    }
    #[doc = "CRC 32-bit wise is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Refout::Value1
    }
    #[doc = "CRC 32-bit wise is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Refout::Value2
    }
}
#[doc = "Field `REFOUT` writer - CRC 32-Bit Wise Reflection"]
pub type RefoutW<'a, REG> = crate::BitWriter<'a, REG, Refout>;
impl<'a, REG> RefoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC 32-bit wise is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Refout::Value1)
    }
    #[doc = "CRC 32-bit wise is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Refout::Value2)
    }
}
#[doc = "Selects the value to be xored with the final CRC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xsel {
    #[doc = "0: 0x00000000"]
    Value1 = 0,
    #[doc = "1: 0xFFFFFFFF"]
    Value2 = 1,
}
impl From<Xsel> for bool {
    #[inline(always)]
    fn from(variant: Xsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XSEL` reader - Selects the value to be xored with the final CRC"]
pub type XselR = crate::BitReader<Xsel>;
impl XselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xsel {
        match self.bits {
            false => Xsel::Value1,
            true => Xsel::Value2,
        }
    }
    #[doc = "0x00000000"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Xsel::Value1
    }
    #[doc = "0xFFFFFFFF"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Xsel::Value2
    }
}
#[doc = "Field `XSEL` writer - Selects the value to be xored with the final CRC"]
pub type XselW<'a, REG> = crate::BitWriter<'a, REG, Xsel>;
impl<'a, REG> XselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0x00000000"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Xsel::Value1)
    }
    #[doc = "0xFFFFFFFF"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Xsel::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - CRC Mismatch Interrupt"]
    #[inline(always)]
    pub fn cmi(&self) -> CmiR {
        CmiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Error Interrupt"]
    #[inline(always)]
    pub fn cei(&self) -> CeiR {
        CeiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Length Error Interrupt"]
    #[inline(always)]
    pub fn lei(&self) -> LeiR {
        LeiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Error Interrupt"]
    #[inline(always)]
    pub fn bei(&self) -> BeiR {
        BeiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC Check Comparison"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Length Reload"]
    #[inline(always)]
    pub fn alr(&self) -> AlrR {
        AlrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - IR Byte Wise Reflection"]
    #[inline(always)]
    pub fn refin(&self) -> RefinR {
        RefinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC 32-Bit Wise Reflection"]
    #[inline(always)]
    pub fn refout(&self) -> RefoutR {
        RefoutR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects the value to be xored with the final CRC"]
    #[inline(always)]
    pub fn xsel(&self) -> XselR {
        XselR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Mismatch Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmi(&mut self) -> CmiW<CfgSpec> {
        CmiW::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cei(&mut self) -> CeiW<CfgSpec> {
        CeiW::new(self, 1)
    }
    #[doc = "Bit 2 - Length Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lei(&mut self) -> LeiW<CfgSpec> {
        LeiW::new(self, 2)
    }
    #[doc = "Bit 3 - Bus Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bei(&mut self) -> BeiW<CfgSpec> {
        BeiW::new(self, 3)
    }
    #[doc = "Bit 4 - CRC Check Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CceW<CfgSpec> {
        CceW::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic Length Reload"]
    #[inline(always)]
    #[must_use]
    pub fn alr(&mut self) -> AlrW<CfgSpec> {
        AlrW::new(self, 5)
    }
    #[doc = "Bit 8 - IR Byte Wise Reflection"]
    #[inline(always)]
    #[must_use]
    pub fn refin(&mut self) -> RefinW<CfgSpec> {
        RefinW::new(self, 8)
    }
    #[doc = "Bit 9 - CRC 32-Bit Wise Reflection"]
    #[inline(always)]
    #[must_use]
    pub fn refout(&mut self) -> RefoutW<CfgSpec> {
        RefoutW::new(self, 9)
    }
    #[doc = "Bit 10 - Selects the value to be xored with the final CRC"]
    #[inline(always)]
    #[must_use]
    pub fn xsel(&mut self) -> XselW<CfgSpec> {
        XselW::new(self, 10)
    }
}
#[doc = "CRC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0x0700"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x0700;
}
