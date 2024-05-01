#[doc = "Register `PSCR` writer"]
pub type W = crate::W<PscrSpec>;
#[doc = "Clear Status Flag 0 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst0> for bool {
    #[inline(always)]
    fn from(variant: Cst0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST0` writer - Clear Status Flag 0 in PSR"]
pub type Cst0W<'a, REG> = crate::BitWriter<'a, REG, Cst0>;
impl<'a, REG> Cst0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst0::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst0::Value2)
    }
}
#[doc = "Clear Status Flag 1 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst1> for bool {
    #[inline(always)]
    fn from(variant: Cst1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST1` writer - Clear Status Flag 1 in PSR"]
pub type Cst1W<'a, REG> = crate::BitWriter<'a, REG, Cst1>;
impl<'a, REG> Cst1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst1::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst1::Value2)
    }
}
#[doc = "Clear Status Flag 2 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst2 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst2> for bool {
    #[inline(always)]
    fn from(variant: Cst2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST2` writer - Clear Status Flag 2 in PSR"]
pub type Cst2W<'a, REG> = crate::BitWriter<'a, REG, Cst2>;
impl<'a, REG> Cst2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst2::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst2::Value2)
    }
}
#[doc = "Clear Status Flag 3 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst3 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst3> for bool {
    #[inline(always)]
    fn from(variant: Cst3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST3` writer - Clear Status Flag 3 in PSR"]
pub type Cst3W<'a, REG> = crate::BitWriter<'a, REG, Cst3>;
impl<'a, REG> Cst3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst3::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst3::Value2)
    }
}
#[doc = "Clear Status Flag 4 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst4 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst4> for bool {
    #[inline(always)]
    fn from(variant: Cst4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST4` writer - Clear Status Flag 4 in PSR"]
pub type Cst4W<'a, REG> = crate::BitWriter<'a, REG, Cst4>;
impl<'a, REG> Cst4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst4::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst4::Value2)
    }
}
#[doc = "Clear Status Flag 5 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst5 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst5> for bool {
    #[inline(always)]
    fn from(variant: Cst5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST5` writer - Clear Status Flag 5 in PSR"]
pub type Cst5W<'a, REG> = crate::BitWriter<'a, REG, Cst5>;
impl<'a, REG> Cst5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst5::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst5::Value2)
    }
}
#[doc = "Clear Status Flag 6 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst6 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst6> for bool {
    #[inline(always)]
    fn from(variant: Cst6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST6` writer - Clear Status Flag 6 in PSR"]
pub type Cst6W<'a, REG> = crate::BitWriter<'a, REG, Cst6>;
impl<'a, REG> Cst6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst6::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst6::Value2)
    }
}
#[doc = "Clear Status Flag 7 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst7 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst7> for bool {
    #[inline(always)]
    fn from(variant: Cst7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST7` writer - Clear Status Flag 7 in PSR"]
pub type Cst7W<'a, REG> = crate::BitWriter<'a, REG, Cst7>;
impl<'a, REG> Cst7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst7::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst7::Value2)
    }
}
#[doc = "Clear Status Flag 8 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst8 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst8> for bool {
    #[inline(always)]
    fn from(variant: Cst8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST8` writer - Clear Status Flag 8 in PSR"]
pub type Cst8W<'a, REG> = crate::BitWriter<'a, REG, Cst8>;
impl<'a, REG> Cst8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst8::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst8::Value2)
    }
}
#[doc = "Clear Status Flag 9 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst9 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    Value2 = 1,
}
impl From<Cst9> for bool {
    #[inline(always)]
    fn from(variant: Cst9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST9` writer - Clear Status Flag 9 in PSR"]
pub type Cst9W<'a, REG> = crate::BitWriter<'a, REG, Cst9>;
impl<'a, REG> Cst9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst9::Value1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cst9::Value2)
    }
}
#[doc = "Clear Receiver Start Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crsif {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.RSIF is cleared."]
    Value2 = 1,
}
impl From<Crsif> for bool {
    #[inline(always)]
    fn from(variant: Crsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSIF` writer - Clear Receiver Start Indication Flag"]
pub type CrsifW<'a, REG> = crate::BitWriter<'a, REG, Crsif>;
impl<'a, REG> CrsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Crsif::Value1)
    }
    #[doc = "Flag PSR.RSIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Crsif::Value2)
    }
}
#[doc = "Clear Data Lost Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdlif {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.DLIF is cleared."]
    Value2 = 1,
}
impl From<Cdlif> for bool {
    #[inline(always)]
    fn from(variant: Cdlif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDLIF` writer - Clear Data Lost Indication Flag"]
pub type CdlifW<'a, REG> = crate::BitWriter<'a, REG, Cdlif>;
impl<'a, REG> CdlifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdlif::Value1)
    }
    #[doc = "Flag PSR.DLIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cdlif::Value2)
    }
}
#[doc = "Clear Transmit Shift Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsif {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.TSIF is cleared."]
    Value2 = 1,
}
impl From<Ctsif> for bool {
    #[inline(always)]
    fn from(variant: Ctsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIF` writer - Clear Transmit Shift Indication Flag"]
pub type CtsifW<'a, REG> = crate::BitWriter<'a, REG, Ctsif>;
impl<'a, REG> CtsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsif::Value1)
    }
    #[doc = "Flag PSR.TSIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsif::Value2)
    }
}
#[doc = "Clear Transmit Buffer Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctbif {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.TBIF is cleared."]
    Value2 = 1,
}
impl From<Ctbif> for bool {
    #[inline(always)]
    fn from(variant: Ctbif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTBIF` writer - Clear Transmit Buffer Indication Flag"]
pub type CtbifW<'a, REG> = crate::BitWriter<'a, REG, Ctbif>;
impl<'a, REG> CtbifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctbif::Value1)
    }
    #[doc = "Flag PSR.TBIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctbif::Value2)
    }
}
#[doc = "Clear Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crif {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.RIF is cleared."]
    Value2 = 1,
}
impl From<Crif> for bool {
    #[inline(always)]
    fn from(variant: Crif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRIF` writer - Clear Receive Indication Flag"]
pub type CrifW<'a, REG> = crate::BitWriter<'a, REG, Crif>;
impl<'a, REG> CrifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Crif::Value1)
    }
    #[doc = "Flag PSR.RIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Crif::Value2)
    }
}
#[doc = "Clear Alternative Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caif {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.AIF is cleared."]
    Value2 = 1,
}
impl From<Caif> for bool {
    #[inline(always)]
    fn from(variant: Caif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAIF` writer - Clear Alternative Receive Indication Flag"]
pub type CaifW<'a, REG> = crate::BitWriter<'a, REG, Caif>;
impl<'a, REG> CaifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Caif::Value1)
    }
    #[doc = "Flag PSR.AIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Caif::Value2)
    }
}
#[doc = "Clear Baud Rate Generator Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cbrgif {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Flag PSR.BRGIF is cleared."]
    Value2 = 1,
}
impl From<Cbrgif> for bool {
    #[inline(always)]
    fn from(variant: Cbrgif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBRGIF` writer - Clear Baud Rate Generator Indication Flag"]
pub type CbrgifW<'a, REG> = crate::BitWriter<'a, REG, Cbrgif>;
impl<'a, REG> CbrgifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cbrgif::Value1)
    }
    #[doc = "Flag PSR.BRGIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cbrgif::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Status Flag 0 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst0(&mut self) -> Cst0W<PscrSpec> {
        Cst0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Status Flag 1 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst1(&mut self) -> Cst1W<PscrSpec> {
        Cst1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Status Flag 2 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst2(&mut self) -> Cst2W<PscrSpec> {
        Cst2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Status Flag 3 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst3(&mut self) -> Cst3W<PscrSpec> {
        Cst3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Status Flag 4 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst4(&mut self) -> Cst4W<PscrSpec> {
        Cst4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Status Flag 5 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst5(&mut self) -> Cst5W<PscrSpec> {
        Cst5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Status Flag 6 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst6(&mut self) -> Cst6W<PscrSpec> {
        Cst6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Status Flag 7 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst7(&mut self) -> Cst7W<PscrSpec> {
        Cst7W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Status Flag 8 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst8(&mut self) -> Cst8W<PscrSpec> {
        Cst8W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Status Flag 9 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst9(&mut self) -> Cst9W<PscrSpec> {
        Cst9W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn crsif(&mut self) -> CrsifW<PscrSpec> {
        CrsifW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cdlif(&mut self) -> CdlifW<PscrSpec> {
        CdlifW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsif(&mut self) -> CtsifW<PscrSpec> {
        CtsifW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctbif(&mut self) -> CtbifW<PscrSpec> {
        CtbifW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn crif(&mut self) -> CrifW<PscrSpec> {
        CrifW::new(self, 14)
    }
    #[doc = "Bit 15 - Clear Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn caif(&mut self) -> CaifW<PscrSpec> {
        CaifW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cbrgif(&mut self) -> CbrgifW<PscrSpec> {
        CbrgifW::new(self, 16)
    }
}
#[doc = "Protocol Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscrSpec;
impl crate::RegisterSpec for PscrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscr::W`](W) writer structure"]
impl crate::Writable for PscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCR to value 0"]
impl crate::Resettable for PscrSpec {
    const RESET_VALUE: u32 = 0;
}
