#[doc = "Register `GLOBCFG` reader"]
pub type R = crate::R<GlobcfgSpec>;
#[doc = "Register `GLOBCFG` writer"]
pub type W = crate::W<GlobcfgSpec>;
#[doc = "Divider Factor for the Analog Internal Clock\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diva {
    #[doc = "0: fADCI = fADC / 2"]
    Value1 = 0,
    #[doc = "1: fADCI = fADC / 2"]
    Value2 = 1,
    #[doc = "2: fADCI = fADC / 3"]
    Value3 = 2,
    #[doc = "31: fADCI = fADC / 32"]
    Value4 = 31,
}
impl From<Diva> for u8 {
    #[inline(always)]
    fn from(variant: Diva) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diva {
    type Ux = u8;
}
impl crate::IsEnum for Diva {}
#[doc = "Field `DIVA` reader - Divider Factor for the Analog Internal Clock"]
pub type DivaR = crate::FieldReader<Diva>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Diva> {
        match self.bits {
            0 => Some(Diva::Value1),
            1 => Some(Diva::Value2),
            2 => Some(Diva::Value3),
            31 => Some(Diva::Value4),
            _ => None,
        }
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Diva::Value1
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Diva::Value2
    }
    #[doc = "fADCI = fADC / 3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Diva::Value3
    }
    #[doc = "fADCI = fADC / 32"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Diva::Value4
    }
}
#[doc = "Field `DIVA` writer - Divider Factor for the Analog Internal Clock"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 5, Diva>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Value1)
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Value2)
    }
    #[doc = "fADCI = fADC / 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Value3)
    }
    #[doc = "fADCI = fADC / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Value4)
    }
}
#[doc = "Double Clock for the MSB Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcmsb {
    #[doc = "0: 1 clock cycles for the MSB (standard)"]
    Value1 = 0,
    #[doc = "1: 2 clock cycles for the MSB (fADCI > 20 MHz)"]
    Value2 = 1,
}
impl From<Dcmsb> for bool {
    #[inline(always)]
    fn from(variant: Dcmsb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMSB` reader - Double Clock for the MSB Conversion"]
pub type DcmsbR = crate::BitReader<Dcmsb>;
impl DcmsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcmsb {
        match self.bits {
            false => Dcmsb::Value1,
            true => Dcmsb::Value2,
        }
    }
    #[doc = "1 clock cycles for the MSB (standard)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dcmsb::Value1
    }
    #[doc = "2 clock cycles for the MSB (fADCI > 20 MHz)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dcmsb::Value2
    }
}
#[doc = "Field `DCMSB` writer - Double Clock for the MSB Conversion"]
pub type DcmsbW<'a, REG> = crate::BitWriter<'a, REG, Dcmsb>;
impl<'a, REG> DcmsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 clock cycles for the MSB (standard)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmsb::Value1)
    }
    #[doc = "2 clock cycles for the MSB (fADCI > 20 MHz)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmsb::Value2)
    }
}
#[doc = "Divider Factor for the Arbiter Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divd {
    #[doc = "0: fADCD = fADC"]
    Value1 = 0,
    #[doc = "1: fADCD = fADC / 2"]
    Value2 = 1,
    #[doc = "2: fADCD = fADC / 3"]
    Value3 = 2,
    #[doc = "3: fADCD = fADC / 4"]
    Value4 = 3,
}
impl From<Divd> for u8 {
    #[inline(always)]
    fn from(variant: Divd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divd {
    type Ux = u8;
}
impl crate::IsEnum for Divd {}
#[doc = "Field `DIVD` reader - Divider Factor for the Arbiter Clock"]
pub type DivdR = crate::FieldReader<Divd>;
impl DivdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divd {
        match self.bits {
            0 => Divd::Value1,
            1 => Divd::Value2,
            2 => Divd::Value3,
            3 => Divd::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "fADCD = fADC"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Divd::Value1
    }
    #[doc = "fADCD = fADC / 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Divd::Value2
    }
    #[doc = "fADCD = fADC / 3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Divd::Value3
    }
    #[doc = "fADCD = fADC / 4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Divd::Value4
    }
}
#[doc = "Field `DIVD` writer - Divider Factor for the Arbiter Clock"]
pub type DivdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Divd, crate::Safe>;
impl<'a, REG> DivdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fADCD = fADC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Divd::Value1)
    }
    #[doc = "fADCD = fADC / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Divd::Value2)
    }
    #[doc = "fADCD = fADC / 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Divd::Value3)
    }
    #[doc = "fADCD = fADC / 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Divd::Value4)
    }
}
#[doc = "Write Control for Divider Parameters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Divwc {
    #[doc = "0: No write access to divider parameters"]
    Value1 = 0,
    #[doc = "1: Bitfields DIVA, DCMSB, DIVD can be written"]
    Value2 = 1,
}
impl From<Divwc> for bool {
    #[inline(always)]
    fn from(variant: Divwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVWC` writer - Write Control for Divider Parameters"]
pub type DivwcW<'a, REG> = crate::BitWriter<'a, REG, Divwc>;
impl<'a, REG> DivwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to divider parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Divwc::Value1)
    }
    #[doc = "Bitfields DIVA, DCMSB, DIVD can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Divwc::Value2)
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpcal0 {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    Value1 = 0,
    #[doc = "1: No post-calibration"]
    Value2 = 1,
}
impl From<Dpcal0> for bool {
    #[inline(always)]
    fn from(variant: Dpcal0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPCAL0` reader - Disable Post-Calibration"]
pub type Dpcal0R = crate::BitReader<Dpcal0>;
impl Dpcal0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpcal0 {
        match self.bits {
            false => Dpcal0::Value1,
            true => Dpcal0::Value2,
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dpcal0::Value1
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dpcal0::Value2
    }
}
#[doc = "Field `DPCAL0` writer - Disable Post-Calibration"]
pub type Dpcal0W<'a, REG> = crate::BitWriter<'a, REG, Dpcal0>;
impl<'a, REG> Dpcal0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpcal0::Value1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dpcal0::Value2)
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpcal1 {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    Value1 = 0,
    #[doc = "1: No post-calibration"]
    Value2 = 1,
}
impl From<Dpcal1> for bool {
    #[inline(always)]
    fn from(variant: Dpcal1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPCAL1` reader - Disable Post-Calibration"]
pub type Dpcal1R = crate::BitReader<Dpcal1>;
impl Dpcal1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpcal1 {
        match self.bits {
            false => Dpcal1::Value1,
            true => Dpcal1::Value2,
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dpcal1::Value1
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dpcal1::Value2
    }
}
#[doc = "Field `DPCAL1` writer - Disable Post-Calibration"]
pub type Dpcal1W<'a, REG> = crate::BitWriter<'a, REG, Dpcal1>;
impl<'a, REG> Dpcal1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpcal1::Value1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dpcal1::Value2)
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpcal2 {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    Value1 = 0,
    #[doc = "1: No post-calibration"]
    Value2 = 1,
}
impl From<Dpcal2> for bool {
    #[inline(always)]
    fn from(variant: Dpcal2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPCAL2` reader - Disable Post-Calibration"]
pub type Dpcal2R = crate::BitReader<Dpcal2>;
impl Dpcal2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpcal2 {
        match self.bits {
            false => Dpcal2::Value1,
            true => Dpcal2::Value2,
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dpcal2::Value1
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dpcal2::Value2
    }
}
#[doc = "Field `DPCAL2` writer - Disable Post-Calibration"]
pub type Dpcal2W<'a, REG> = crate::BitWriter<'a, REG, Dpcal2>;
impl<'a, REG> Dpcal2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpcal2::Value1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dpcal2::Value2)
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpcal3 {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    Value1 = 0,
    #[doc = "1: No post-calibration"]
    Value2 = 1,
}
impl From<Dpcal3> for bool {
    #[inline(always)]
    fn from(variant: Dpcal3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPCAL3` reader - Disable Post-Calibration"]
pub type Dpcal3R = crate::BitReader<Dpcal3>;
impl Dpcal3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpcal3 {
        match self.bits {
            false => Dpcal3::Value1,
            true => Dpcal3::Value2,
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dpcal3::Value1
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dpcal3::Value2
    }
}
#[doc = "Field `DPCAL3` writer - Disable Post-Calibration"]
pub type Dpcal3W<'a, REG> = crate::BitWriter<'a, REG, Dpcal3>;
impl<'a, REG> Dpcal3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpcal3::Value1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dpcal3::Value2)
    }
}
#[doc = "Start-Up Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sucal {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    Value2 = 1,
}
impl From<Sucal> for bool {
    #[inline(always)]
    fn from(variant: Sucal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUCAL` writer - Start-Up Calibration"]
pub type SucalW<'a, REG> = crate::BitWriter<'a, REG, Sucal>;
impl<'a, REG> SucalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sucal::Value1)
    }
    #[doc = "Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sucal::Value2)
    }
}
impl R {
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline(always)]
    pub fn dcmsb(&self) -> DcmsbR {
        DcmsbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline(always)]
    pub fn divd(&self) -> DivdR {
        DivdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal0(&self) -> Dpcal0R {
        Dpcal0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal1(&self) -> Dpcal1R {
        Dpcal1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal2(&self) -> Dpcal2R {
        Dpcal2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal3(&self) -> Dpcal3R {
        Dpcal3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DivaW<GlobcfgSpec> {
        DivaW::new(self, 0)
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn dcmsb(&mut self) -> DcmsbW<GlobcfgSpec> {
        DcmsbW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline(always)]
    #[must_use]
    pub fn divd(&mut self) -> DivdW<GlobcfgSpec> {
        DivdW::new(self, 8)
    }
    #[doc = "Bit 15 - Write Control for Divider Parameters"]
    #[inline(always)]
    #[must_use]
    pub fn divwc(&mut self) -> DivwcW<GlobcfgSpec> {
        DivwcW::new(self, 15)
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal0(&mut self) -> Dpcal0W<GlobcfgSpec> {
        Dpcal0W::new(self, 16)
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal1(&mut self) -> Dpcal1W<GlobcfgSpec> {
        Dpcal1W::new(self, 17)
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal2(&mut self) -> Dpcal2W<GlobcfgSpec> {
        Dpcal2W::new(self, 18)
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal3(&mut self) -> Dpcal3W<GlobcfgSpec> {
        Dpcal3W::new(self, 19)
    }
    #[doc = "Bit 31 - Start-Up Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn sucal(&mut self) -> SucalW<GlobcfgSpec> {
        SucalW::new(self, 31)
    }
}
#[doc = "Global Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobcfgSpec;
impl crate::RegisterSpec for GlobcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globcfg::R`](R) reader structure"]
impl crate::Readable for GlobcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`globcfg::W`](W) writer structure"]
impl crate::Writable for GlobcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBCFG to value 0x0f"]
impl crate::Resettable for GlobcfgSpec {
    const RESET_VALUE: u32 = 0x0f;
}
