#[doc = "Register `DTC` reader"]
pub type R = crate::R<DtcSpec>;
#[doc = "Register `DTC` writer"]
pub type W = crate::W<DtcSpec>;
#[doc = "Dead Time Enable for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dte1 {
    #[doc = "0: Dead Time for channel 1 is disabled"]
    Value1 = 0,
    #[doc = "1: Dead Time for channel 1 is enabled"]
    Value2 = 1,
}
impl From<Dte1> for bool {
    #[inline(always)]
    fn from(variant: Dte1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE1` reader - Dead Time Enable for Channel 1"]
pub type Dte1R = crate::BitReader<Dte1>;
impl Dte1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dte1 {
        match self.bits {
            false => Dte1::Value1,
            true => Dte1::Value2,
        }
    }
    #[doc = "Dead Time for channel 1 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dte1::Value1
    }
    #[doc = "Dead Time for channel 1 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dte1::Value2
    }
}
#[doc = "Field `DTE1` writer - Dead Time Enable for Channel 1"]
pub type Dte1W<'a, REG> = crate::BitWriter<'a, REG, Dte1>;
impl<'a, REG> Dte1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for channel 1 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dte1::Value1)
    }
    #[doc = "Dead Time for channel 1 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dte1::Value2)
    }
}
#[doc = "Dead Time Enable for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dte2 {
    #[doc = "0: Dead Time for channel 2 is disabled"]
    Value1 = 0,
    #[doc = "1: Dead Time for channel 2 is enabled"]
    Value2 = 1,
}
impl From<Dte2> for bool {
    #[inline(always)]
    fn from(variant: Dte2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE2` reader - Dead Time Enable for Channel 2"]
pub type Dte2R = crate::BitReader<Dte2>;
impl Dte2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dte2 {
        match self.bits {
            false => Dte2::Value1,
            true => Dte2::Value2,
        }
    }
    #[doc = "Dead Time for channel 2 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dte2::Value1
    }
    #[doc = "Dead Time for channel 2 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dte2::Value2
    }
}
#[doc = "Field `DTE2` writer - Dead Time Enable for Channel 2"]
pub type Dte2W<'a, REG> = crate::BitWriter<'a, REG, Dte2>;
impl<'a, REG> Dte2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for channel 2 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dte2::Value1)
    }
    #[doc = "Dead Time for channel 2 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dte2::Value2)
    }
}
#[doc = "Dead Time Enable for CC8yST1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcen1 {
    #[doc = "0: Dead Time for CC8yST1 path is disabled"]
    Value1 = 0,
    #[doc = "1: Dead Time for CC8yST1 path is enabled"]
    Value2 = 1,
}
impl From<Dcen1> for bool {
    #[inline(always)]
    fn from(variant: Dcen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN1` reader - Dead Time Enable for CC8yST1"]
pub type Dcen1R = crate::BitReader<Dcen1>;
impl Dcen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcen1 {
        match self.bits {
            false => Dcen1::Value1,
            true => Dcen1::Value2,
        }
    }
    #[doc = "Dead Time for CC8yST1 path is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dcen1::Value1
    }
    #[doc = "Dead Time for CC8yST1 path is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dcen1::Value2
    }
}
#[doc = "Field `DCEN1` writer - Dead Time Enable for CC8yST1"]
pub type Dcen1W<'a, REG> = crate::BitWriter<'a, REG, Dcen1>;
impl<'a, REG> Dcen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for CC8yST1 path is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen1::Value1)
    }
    #[doc = "Dead Time for CC8yST1 path is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen1::Value2)
    }
}
#[doc = "Dead Time Enable for inverted CC8yST1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcen2 {
    #[doc = "0: Dead Time for inverted CC8yST1 path is disabled"]
    Value1 = 0,
    #[doc = "1: Dead Time for inverted CC8yST1 path is enabled"]
    Value2 = 1,
}
impl From<Dcen2> for bool {
    #[inline(always)]
    fn from(variant: Dcen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN2` reader - Dead Time Enable for inverted CC8yST1"]
pub type Dcen2R = crate::BitReader<Dcen2>;
impl Dcen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcen2 {
        match self.bits {
            false => Dcen2::Value1,
            true => Dcen2::Value2,
        }
    }
    #[doc = "Dead Time for inverted CC8yST1 path is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dcen2::Value1
    }
    #[doc = "Dead Time for inverted CC8yST1 path is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dcen2::Value2
    }
}
#[doc = "Field `DCEN2` writer - Dead Time Enable for inverted CC8yST1"]
pub type Dcen2W<'a, REG> = crate::BitWriter<'a, REG, Dcen2>;
impl<'a, REG> Dcen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for inverted CC8yST1 path is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen2::Value1)
    }
    #[doc = "Dead Time for inverted CC8yST1 path is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen2::Value2)
    }
}
#[doc = "Dead Time Enable for CC8yST2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcen3 {
    #[doc = "0: Dead Time for CC8yST2 path is disabled"]
    Value1 = 0,
    #[doc = "1: Dead Time for CC8yST2 path is enabled"]
    Value2 = 1,
}
impl From<Dcen3> for bool {
    #[inline(always)]
    fn from(variant: Dcen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN3` reader - Dead Time Enable for CC8yST2"]
pub type Dcen3R = crate::BitReader<Dcen3>;
impl Dcen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcen3 {
        match self.bits {
            false => Dcen3::Value1,
            true => Dcen3::Value2,
        }
    }
    #[doc = "Dead Time for CC8yST2 path is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dcen3::Value1
    }
    #[doc = "Dead Time for CC8yST2 path is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dcen3::Value2
    }
}
#[doc = "Field `DCEN3` writer - Dead Time Enable for CC8yST2"]
pub type Dcen3W<'a, REG> = crate::BitWriter<'a, REG, Dcen3>;
impl<'a, REG> Dcen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for CC8yST2 path is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen3::Value1)
    }
    #[doc = "Dead Time for CC8yST2 path is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen3::Value2)
    }
}
#[doc = "Dead Time Enable for inverted CC8yST2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcen4 {
    #[doc = "0: Dead Time for inverted CC8yST2 path is disabled"]
    Value1 = 0,
    #[doc = "1: Dead Time for inverted CC8yST2 path is enabled"]
    Value2 = 1,
}
impl From<Dcen4> for bool {
    #[inline(always)]
    fn from(variant: Dcen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN4` reader - Dead Time Enable for inverted CC8yST2"]
pub type Dcen4R = crate::BitReader<Dcen4>;
impl Dcen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcen4 {
        match self.bits {
            false => Dcen4::Value1,
            true => Dcen4::Value2,
        }
    }
    #[doc = "Dead Time for inverted CC8yST2 path is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dcen4::Value1
    }
    #[doc = "Dead Time for inverted CC8yST2 path is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dcen4::Value2
    }
}
#[doc = "Field `DCEN4` writer - Dead Time Enable for inverted CC8yST2"]
pub type Dcen4W<'a, REG> = crate::BitWriter<'a, REG, Dcen4>;
impl<'a, REG> Dcen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Time for inverted CC8yST2 path is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen4::Value1)
    }
    #[doc = "Dead Time for inverted CC8yST2 path is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen4::Value2)
    }
}
#[doc = "Dead Time clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtcc {
    #[doc = "0: ftclk"]
    Value1 = 0,
    #[doc = "1: ftclk/2"]
    Value2 = 1,
    #[doc = "2: ftclk/4"]
    Value3 = 2,
    #[doc = "3: ftclk/8"]
    Value4 = 3,
}
impl From<Dtcc> for u8 {
    #[inline(always)]
    fn from(variant: Dtcc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtcc {
    type Ux = u8;
}
impl crate::IsEnum for Dtcc {}
#[doc = "Field `DTCC` reader - Dead Time clock control"]
pub type DtccR = crate::FieldReader<Dtcc>;
impl DtccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtcc {
        match self.bits {
            0 => Dtcc::Value1,
            1 => Dtcc::Value2,
            2 => Dtcc::Value3,
            3 => Dtcc::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "ftclk"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dtcc::Value1
    }
    #[doc = "ftclk/2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dtcc::Value2
    }
    #[doc = "ftclk/4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Dtcc::Value3
    }
    #[doc = "ftclk/8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Dtcc::Value4
    }
}
#[doc = "Field `DTCC` writer - Dead Time clock control"]
pub type DtccW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dtcc, crate::Safe>;
impl<'a, REG> DtccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ftclk"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcc::Value1)
    }
    #[doc = "ftclk/2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcc::Value2)
    }
    #[doc = "ftclk/4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcc::Value3)
    }
    #[doc = "ftclk/8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcc::Value4)
    }
}
impl R {
    #[doc = "Bit 0 - Dead Time Enable for Channel 1"]
    #[inline(always)]
    pub fn dte1(&self) -> Dte1R {
        Dte1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dead Time Enable for Channel 2"]
    #[inline(always)]
    pub fn dte2(&self) -> Dte2R {
        Dte2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dead Time Enable for CC8yST1"]
    #[inline(always)]
    pub fn dcen1(&self) -> Dcen1R {
        Dcen1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dead Time Enable for inverted CC8yST1"]
    #[inline(always)]
    pub fn dcen2(&self) -> Dcen2R {
        Dcen2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Dead Time Enable for CC8yST2"]
    #[inline(always)]
    pub fn dcen3(&self) -> Dcen3R {
        Dcen3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dead Time Enable for inverted CC8yST2"]
    #[inline(always)]
    pub fn dcen4(&self) -> Dcen4R {
        Dcen4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Dead Time clock control"]
    #[inline(always)]
    pub fn dtcc(&self) -> DtccR {
        DtccR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dead Time Enable for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dte1(&mut self) -> Dte1W<DtcSpec> {
        Dte1W::new(self, 0)
    }
    #[doc = "Bit 1 - Dead Time Enable for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn dte2(&mut self) -> Dte2W<DtcSpec> {
        Dte2W::new(self, 1)
    }
    #[doc = "Bit 2 - Dead Time Enable for CC8yST1"]
    #[inline(always)]
    #[must_use]
    pub fn dcen1(&mut self) -> Dcen1W<DtcSpec> {
        Dcen1W::new(self, 2)
    }
    #[doc = "Bit 3 - Dead Time Enable for inverted CC8yST1"]
    #[inline(always)]
    #[must_use]
    pub fn dcen2(&mut self) -> Dcen2W<DtcSpec> {
        Dcen2W::new(self, 3)
    }
    #[doc = "Bit 4 - Dead Time Enable for CC8yST2"]
    #[inline(always)]
    #[must_use]
    pub fn dcen3(&mut self) -> Dcen3W<DtcSpec> {
        Dcen3W::new(self, 4)
    }
    #[doc = "Bit 5 - Dead Time Enable for inverted CC8yST2"]
    #[inline(always)]
    #[must_use]
    pub fn dcen4(&mut self) -> Dcen4W<DtcSpec> {
        Dcen4W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Dead Time clock control"]
    #[inline(always)]
    #[must_use]
    pub fn dtcc(&mut self) -> DtccW<DtcSpec> {
        DtccW::new(self, 6)
    }
}
#[doc = "Dead Time Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtcSpec;
impl crate::RegisterSpec for DtcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtc::R`](R) reader structure"]
impl crate::Readable for DtcSpec {}
#[doc = "`write(|w| ..)` method takes [`dtc::W`](W) writer structure"]
impl crate::Writable for DtcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTC to value 0"]
impl crate::Resettable for DtcSpec {
    const RESET_VALUE: u32 = 0;
}
