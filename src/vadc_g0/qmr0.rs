#[doc = "Register `QMR0` reader"]
pub type R = crate::R<Qmr0Spec>;
#[doc = "Register `QMR0` writer"]
pub type W = crate::W<Qmr0Spec>;
#[doc = "Enable Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Engt {
    #[doc = "0: No conversion requests are issued"]
    Value1 = 0,
    #[doc = "1: Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    Value2 = 1,
    #[doc = "2: Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    Value3 = 2,
    #[doc = "3: Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
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
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Engt::Value2
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Engt::Value3
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
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
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Engt::Value2)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Engt::Value3)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
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
    #[doc = "1: The selected edge at the selected trigger input signal REQTR generates the trigger event"]
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
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the trigger event"]
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
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the trigger event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Entr::Value2)
    }
}
#[doc = "Clear Valid Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrv {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: The next pending valid queue entry in the sequence and the event flag EV are cleared. If there is a valid entry in the queue backup register (QBUR.V = 1), this entry is cleared, otherwise the entry in queue register 0 is cleared."]
    Value2 = 1,
}
impl From<Clrv> for bool {
    #[inline(always)]
    fn from(variant: Clrv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRV` writer - Clear Valid Bit"]
pub type ClrvW<'a, REG> = crate::BitWriter<'a, REG, Clrv>;
impl<'a, REG> ClrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Clrv::Value1)
    }
    #[doc = "The next pending valid queue entry in the sequence and the event flag EV are cleared. If there is a valid entry in the queue backup register (QBUR.V = 1), this entry is cleared, otherwise the entry in queue register 0 is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Clrv::Value2)
    }
}
#[doc = "Trigger Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trev {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Generate a trigger event by software"]
    Value2 = 1,
}
impl From<Trev> for bool {
    #[inline(always)]
    fn from(variant: Trev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TREV` writer - Trigger Event"]
pub type TrevW<'a, REG> = crate::BitWriter<'a, REG, Trev>;
impl<'a, REG> TrevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trev::Value1)
    }
    #[doc = "Generate a trigger event by software"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trev::Value2)
    }
}
#[doc = "Flush Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flush {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear all queue entries (including backup stage) and the event flag EV. The queue contains no more valid entry."]
    Value2 = 1,
}
impl From<Flush> for bool {
    #[inline(always)]
    fn from(variant: Flush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH` writer - Flush Queue"]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG, Flush>;
impl<'a, REG> FlushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Value1)
    }
    #[doc = "Clear all queue entries (including backup stage) and the event flag EV. The queue contains no more valid entry."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Value2)
    }
}
#[doc = "Clear Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cev {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit EV"]
    Value2 = 1,
}
impl From<Cev> for bool {
    #[inline(always)]
    fn from(variant: Cev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV` writer - Clear Event Flag"]
pub type CevW<'a, REG> = crate::BitWriter<'a, REG, Cev>;
impl<'a, REG> CevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cev::Value1)
    }
    #[doc = "Clear bit EV"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cev::Value2)
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
    pub fn engt(&mut self) -> EngtW<Qmr0Spec> {
        EngtW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn entr(&mut self) -> EntrW<Qmr0Spec> {
        EntrW::new(self, 2)
    }
    #[doc = "Bit 8 - Clear Valid Bit"]
    #[inline(always)]
    #[must_use]
    pub fn clrv(&mut self) -> ClrvW<Qmr0Spec> {
        ClrvW::new(self, 8)
    }
    #[doc = "Bit 9 - Trigger Event"]
    #[inline(always)]
    #[must_use]
    pub fn trev(&mut self) -> TrevW<Qmr0Spec> {
        TrevW::new(self, 9)
    }
    #[doc = "Bit 10 - Flush Queue"]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FlushW<Qmr0Spec> {
        FlushW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cev(&mut self) -> CevW<Qmr0Spec> {
        CevW::new(self, 11)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rptdis(&mut self) -> RptdisW<Qmr0Spec> {
        RptdisW::new(self, 16)
    }
}
#[doc = "Queue 0 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qmr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qmr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qmr0Spec;
impl crate::RegisterSpec for Qmr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qmr0::R`](R) reader structure"]
impl crate::Readable for Qmr0Spec {}
#[doc = "`write(|w| ..)` method takes [`qmr0::W`](W) writer structure"]
impl crate::Writable for Qmr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QMR0 to value 0"]
impl crate::Resettable for Qmr0Spec {
    const RESET_VALUE: u32 = 0;
}
