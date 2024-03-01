#[doc = "Register `BRSMR` reader"]
pub type R = crate::R<BrsmrSpec>;
#[doc = "Register `BRSMR` writer"]
pub type W = crate::W<BrsmrSpec>;
#[doc = "Enable Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Engt {
    #[doc = "0: No conversion requests are issued"]
    Value1 = 0,
    #[doc = "1: Conversion requests are issued if at least one pending bit is set"]
    Value2 = 1,
    #[doc = "2: Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    Value3 = 2,
    #[doc = "3: Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    Value4 = 3,
}
impl From<Engt> for u8 {
    #[inline(always)]
    fn from(variant: Engt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Engt {
    type Ux = u8;
}
#[doc = "Field `ENGT` reader - Enable Gate"]
pub type EngtR = crate::FieldReader<Engt>;
impl EngtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Engt {
        match self.bits {
            0 => Engt::Value1,
            1 => Engt::Value2,
            2 => Engt::Value3,
            3 => Engt::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No conversion requests are issued"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Engt::Value1
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Engt::Value2
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Engt::Value3
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Engt::Value4
    }
}
#[doc = "Field `ENGT` writer - Enable Gate"]
pub type EngtW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Engt>;
impl<'a, REG> EngtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No conversion requests are issued"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Engt::Value1)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Engt::Value2)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Engt::Value3)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Engt::Value4)
    }
}
#[doc = "Enable External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entr {
    #[doc = "0: External trigger disabled"]
    Value1 = 0,
    #[doc = "1: The selected edge at the selected trigger input signal REQTR generates the load event"]
    Value2 = 1,
}
impl From<Entr> for bool {
    #[inline(always)]
    fn from(variant: Entr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENTR` reader - Enable External Trigger"]
pub type EntrR = crate::BitReader<Entr>;
impl EntrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Entr {
        match self.bits {
            false => Entr::Value1,
            true => Entr::Value2,
        }
    }
    #[doc = "External trigger disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Entr::Value1
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the load event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Entr::Value2
    }
}
#[doc = "Field `ENTR` writer - Enable External Trigger"]
pub type EntrW<'a, REG> = crate::BitWriter<'a, REG, Entr>;
impl<'a, REG> EntrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Entr::Value1)
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the load event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Entr::Value2)
    }
}
#[doc = "Enable Source Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ensi {
    #[doc = "0: No request source interrupt"]
    Value1 = 0,
    #[doc = "1: A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    Value2 = 1,
}
impl From<Ensi> for bool {
    #[inline(always)]
    fn from(variant: Ensi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSI` reader - Enable Source Interrupt"]
pub type EnsiR = crate::BitReader<Ensi>;
impl EnsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ensi {
        match self.bits {
            false => Ensi::Value1,
            true => Ensi::Value2,
        }
    }
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ensi::Value1
    }
    #[doc = "A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ensi::Value2
    }
}
#[doc = "Field `ENSI` writer - Enable Source Interrupt"]
pub type EnsiW<'a, REG> = crate::BitWriter<'a, REG, Ensi>;
impl<'a, REG> EnsiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ensi::Value1)
    }
    #[doc = "A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ensi::Value2)
    }
}
#[doc = "Autoscan Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scan {
    #[doc = "0: No autoscan"]
    Value1 = 0,
    #[doc = "1: Autoscan functionality enabled: a request source event automatically generates a load event"]
    Value2 = 1,
}
impl From<Scan> for bool {
    #[inline(always)]
    fn from(variant: Scan) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCAN` reader - Autoscan Enable"]
pub type ScanR = crate::BitReader<Scan>;
impl ScanR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scan {
        match self.bits {
            false => Scan::Value1,
            true => Scan::Value2,
        }
    }
    #[doc = "No autoscan"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Scan::Value1
    }
    #[doc = "Autoscan functionality enabled: a request source event automatically generates a load event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Scan::Value2
    }
}
#[doc = "Field `SCAN` writer - Autoscan Enable"]
pub type ScanW<'a, REG> = crate::BitWriter<'a, REG, Scan>;
impl<'a, REG> ScanW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No autoscan"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Scan::Value1)
    }
    #[doc = "Autoscan functionality enabled: a request source event automatically generates a load event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Scan::Value2)
    }
}
#[doc = "Autoscan Source Load Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldm {
    #[doc = "0: Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    Value1 = 0,
    #[doc = "1: Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    Value2 = 1,
}
impl From<Ldm> for bool {
    #[inline(always)]
    fn from(variant: Ldm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDM` reader - Autoscan Source Load Event Mode"]
pub type LdmR = crate::BitReader<Ldm>;
impl LdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldm {
        match self.bits {
            false => Ldm::Value1,
            true => Ldm::Value2,
        }
    }
    #[doc = "Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ldm::Value1
    }
    #[doc = "Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ldm::Value2
    }
}
#[doc = "Field `LDM` writer - Autoscan Source Load Event Mode"]
pub type LdmW<'a, REG> = crate::BitWriter<'a, REG, Ldm>;
impl<'a, REG> LdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ldm::Value1)
    }
    #[doc = "Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ldm::Value2)
    }
}
#[doc = "Request Gate Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reqgt {
    #[doc = "0: The gate input is low"]
    Value1 = 0,
    #[doc = "1: The gate input is high"]
    Value2 = 1,
}
impl From<Reqgt> for bool {
    #[inline(always)]
    fn from(variant: Reqgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQGT` reader - Request Gate Level"]
pub type ReqgtR = crate::BitReader<Reqgt>;
impl ReqgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reqgt {
        match self.bits {
            false => Reqgt::Value1,
            true => Reqgt::Value2,
        }
    }
    #[doc = "The gate input is low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Reqgt::Value1
    }
    #[doc = "The gate input is high"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Reqgt::Value2
    }
}
#[doc = "Clear Pending Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrpnd {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: The bits in registers BRSPNDx are cleared"]
    Value2 = 1,
}
impl From<Clrpnd> for bool {
    #[inline(always)]
    fn from(variant: Clrpnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRPND` writer - Clear Pending Bits"]
pub type ClrpndW<'a, REG> = crate::BitWriter<'a, REG, Clrpnd>;
impl<'a, REG> ClrpndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Clrpnd::Value1)
    }
    #[doc = "The bits in registers BRSPNDx are cleared"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Clrpnd::Value2)
    }
}
#[doc = "Generate Load Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldev {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: A load event is generated"]
    Value2 = 1,
}
impl From<Ldev> for bool {
    #[inline(always)]
    fn from(variant: Ldev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDEV` writer - Generate Load Event"]
pub type LdevW<'a, REG> = crate::BitWriter<'a, REG, Ldev>;
impl<'a, REG> LdevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ldev::Value1)
    }
    #[doc = "A load event is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ldev::Value2)
    }
}
#[doc = "Repeat Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rptdis {
    #[doc = "0: A cancelled conversion is repeated"]
    Value1 = 0,
    #[doc = "1: A cancelled conversion is discarded"]
    Value2 = 1,
}
impl From<Rptdis> for bool {
    #[inline(always)]
    fn from(variant: Rptdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPTDIS` reader - Repeat Disable"]
pub type RptdisR = crate::BitReader<Rptdis>;
impl RptdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rptdis {
        match self.bits {
            false => Rptdis::Value1,
            true => Rptdis::Value2,
        }
    }
    #[doc = "A cancelled conversion is repeated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rptdis::Value1
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rptdis::Value2
    }
}
#[doc = "Field `RPTDIS` writer - Repeat Disable"]
pub type RptdisW<'a, REG> = crate::BitWriter<'a, REG, Rptdis>;
impl<'a, REG> RptdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A cancelled conversion is repeated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rptdis::Value1)
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rptdis::Value2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    pub fn engt(&self) -> EngtR {
        EngtR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    pub fn entr(&self) -> EntrR {
        EntrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&self) -> EnsiR {
        EnsiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline(always)]
    pub fn scan(&self) -> ScanR {
        ScanR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline(always)]
    pub fn ldm(&self) -> LdmR {
        LdmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline(always)]
    pub fn reqgt(&self) -> ReqgtR {
        ReqgtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    pub fn rptdis(&self) -> RptdisR {
        RptdisR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    #[must_use]
    pub fn engt(&mut self) -> EngtW<BrsmrSpec> {
        EngtW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn entr(&mut self) -> EntrW<BrsmrSpec> {
        EntrW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ensi(&mut self) -> EnsiW<BrsmrSpec> {
        EnsiW::new(self, 3)
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> ScanW<BrsmrSpec> {
        ScanW::new(self, 4)
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldm(&mut self) -> LdmW<BrsmrSpec> {
        LdmW::new(self, 5)
    }
    #[doc = "Bit 8 - Clear Pending Bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrpnd(&mut self) -> ClrpndW<BrsmrSpec> {
        ClrpndW::new(self, 8)
    }
    #[doc = "Bit 9 - Generate Load Event"]
    #[inline(always)]
    #[must_use]
    pub fn ldev(&mut self) -> LdevW<BrsmrSpec> {
        LdevW::new(self, 9)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rptdis(&mut self) -> RptdisW<BrsmrSpec> {
        RptdisW::new(self, 16)
    }
}
#[doc = "Background Request Source Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brsmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brsmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrsmrSpec;
impl crate::RegisterSpec for BrsmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brsmr::R`](R) reader structure"]
impl crate::Readable for BrsmrSpec {}
#[doc = "`write(|w| ..)` method takes [`brsmr::W`](W) writer structure"]
impl crate::Writable for BrsmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRSMR to value 0"]
impl crate::Resettable for BrsmrSpec {
    const RESET_VALUE: u32 = 0;
}
