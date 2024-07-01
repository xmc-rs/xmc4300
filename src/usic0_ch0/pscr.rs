#[doc = "Register `PSCR` writer"]
pub type W = crate::W<PSCR_SPEC>;
#[doc = "Clear Status Flag 0 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST0_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST0_A> for bool {
    #[inline(always)]
    fn from(variant: CST0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST0` writer - Clear Status Flag 0 in PSR"]
pub type CST0_W<'a, REG> = crate::BitWriter<'a, REG, CST0_A>;
impl<'a, REG> CST0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST0_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST0_A::VALUE2)
    }
}
#[doc = "Clear Status Flag 1 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST1_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST1_A> for bool {
    #[inline(always)]
    fn from(variant: CST1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST1` writer - Clear Status Flag 1 in PSR"]
pub type CST1_W<'a, REG> = crate::BitWriter<'a, REG, CST1_A>;
impl<'a, REG> CST1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST1_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST1_A::VALUE2)
    }
}
#[doc = "Clear Status Flag 2 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST2_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST2_A> for bool {
    #[inline(always)]
    fn from(variant: CST2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST2` writer - Clear Status Flag 2 in PSR"]
pub type CST2_W<'a, REG> = crate::BitWriter<'a, REG, CST2_A>;
impl<'a, REG> CST2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST2_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST2_A::VALUE2)
    }
}
#[doc = "Clear Status Flag 3 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST3_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST3_A> for bool {
    #[inline(always)]
    fn from(variant: CST3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST3` writer - Clear Status Flag 3 in PSR"]
pub type CST3_W<'a, REG> = crate::BitWriter<'a, REG, CST3_A>;
impl<'a, REG> CST3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST3_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST3_A::VALUE2)
    }
}
#[doc = "Clear Status Flag 4 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST4_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST4_A> for bool {
    #[inline(always)]
    fn from(variant: CST4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST4` writer - Clear Status Flag 4 in PSR"]
pub type CST4_W<'a, REG> = crate::BitWriter<'a, REG, CST4_A>;
impl<'a, REG> CST4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST4_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST4_A::VALUE2)
    }
}
#[doc = "Clear Status Flag 5 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST5_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST5_A> for bool {
    #[inline(always)]
    fn from(variant: CST5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST5` writer - Clear Status Flag 5 in PSR"]
pub type CST5_W<'a, REG> = crate::BitWriter<'a, REG, CST5_A>;
impl<'a, REG> CST5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST5_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST5_A::VALUE2)
    }
}
#[doc = "Clear Status Flag 6 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST6_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST6_A> for bool {
    #[inline(always)]
    fn from(variant: CST6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST6` writer - Clear Status Flag 6 in PSR"]
pub type CST6_W<'a, REG> = crate::BitWriter<'a, REG, CST6_A>;
impl<'a, REG> CST6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST6_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST6_A::VALUE2)
    }
}
#[doc = "Clear Status Flag 7 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST7_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST7_A> for bool {
    #[inline(always)]
    fn from(variant: CST7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST7` writer - Clear Status Flag 7 in PSR"]
pub type CST7_W<'a, REG> = crate::BitWriter<'a, REG, CST7_A>;
impl<'a, REG> CST7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST7_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST7_A::VALUE2)
    }
}
#[doc = "Clear Status Flag 8 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST8_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST8_A> for bool {
    #[inline(always)]
    fn from(variant: CST8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST8` writer - Clear Status Flag 8 in PSR"]
pub type CST8_W<'a, REG> = crate::BitWriter<'a, REG, CST8_A>;
impl<'a, REG> CST8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST8_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST8_A::VALUE2)
    }
}
#[doc = "Clear Status Flag 9 in PSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST9_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.STx is cleared."]
    VALUE2 = 1,
}
impl From<CST9_A> for bool {
    #[inline(always)]
    fn from(variant: CST9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST9` writer - Clear Status Flag 9 in PSR"]
pub type CST9_W<'a, REG> = crate::BitWriter<'a, REG, CST9_A>;
impl<'a, REG> CST9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CST9_A::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CST9_A::VALUE2)
    }
}
#[doc = "Clear Receiver Start Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSIF_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.RSIF is cleared."]
    VALUE2 = 1,
}
impl From<CRSIF_A> for bool {
    #[inline(always)]
    fn from(variant: CRSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSIF` writer - Clear Receiver Start Indication Flag"]
pub type CRSIF_W<'a, REG> = crate::BitWriter<'a, REG, CRSIF_A>;
impl<'a, REG> CRSIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CRSIF_A::VALUE1)
    }
    #[doc = "Flag PSR.RSIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CRSIF_A::VALUE2)
    }
}
#[doc = "Clear Data Lost Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDLIF_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.DLIF is cleared."]
    VALUE2 = 1,
}
impl From<CDLIF_A> for bool {
    #[inline(always)]
    fn from(variant: CDLIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDLIF` writer - Clear Data Lost Indication Flag"]
pub type CDLIF_W<'a, REG> = crate::BitWriter<'a, REG, CDLIF_A>;
impl<'a, REG> CDLIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CDLIF_A::VALUE1)
    }
    #[doc = "Flag PSR.DLIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CDLIF_A::VALUE2)
    }
}
#[doc = "Clear Transmit Shift Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIF_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.TSIF is cleared."]
    VALUE2 = 1,
}
impl From<CTSIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIF` writer - Clear Transmit Shift Indication Flag"]
pub type CTSIF_W<'a, REG> = crate::BitWriter<'a, REG, CTSIF_A>;
impl<'a, REG> CTSIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIF_A::VALUE1)
    }
    #[doc = "Flag PSR.TSIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIF_A::VALUE2)
    }
}
#[doc = "Clear Transmit Buffer Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTBIF_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.TBIF is cleared."]
    VALUE2 = 1,
}
impl From<CTBIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTBIF` writer - Clear Transmit Buffer Indication Flag"]
pub type CTBIF_W<'a, REG> = crate::BitWriter<'a, REG, CTBIF_A>;
impl<'a, REG> CTBIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTBIF_A::VALUE1)
    }
    #[doc = "Flag PSR.TBIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTBIF_A::VALUE2)
    }
}
#[doc = "Clear Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRIF_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.RIF is cleared."]
    VALUE2 = 1,
}
impl From<CRIF_A> for bool {
    #[inline(always)]
    fn from(variant: CRIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRIF` writer - Clear Receive Indication Flag"]
pub type CRIF_W<'a, REG> = crate::BitWriter<'a, REG, CRIF_A>;
impl<'a, REG> CRIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CRIF_A::VALUE1)
    }
    #[doc = "Flag PSR.RIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CRIF_A::VALUE2)
    }
}
#[doc = "Clear Alternative Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAIF_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.AIF is cleared."]
    VALUE2 = 1,
}
impl From<CAIF_A> for bool {
    #[inline(always)]
    fn from(variant: CAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAIF` writer - Clear Alternative Receive Indication Flag"]
pub type CAIF_W<'a, REG> = crate::BitWriter<'a, REG, CAIF_A>;
impl<'a, REG> CAIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CAIF_A::VALUE1)
    }
    #[doc = "Flag PSR.AIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CAIF_A::VALUE2)
    }
}
#[doc = "Clear Baud Rate Generator Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBRGIF_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Flag PSR.BRGIF is cleared."]
    VALUE2 = 1,
}
impl From<CBRGIF_A> for bool {
    #[inline(always)]
    fn from(variant: CBRGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBRGIF` writer - Clear Baud Rate Generator Indication Flag"]
pub type CBRGIF_W<'a, REG> = crate::BitWriter<'a, REG, CBRGIF_A>;
impl<'a, REG> CBRGIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CBRGIF_A::VALUE1)
    }
    #[doc = "Flag PSR.BRGIF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CBRGIF_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Status Flag 0 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst0(&mut self) -> CST0_W<PSCR_SPEC> {
        CST0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Status Flag 1 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst1(&mut self) -> CST1_W<PSCR_SPEC> {
        CST1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Status Flag 2 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst2(&mut self) -> CST2_W<PSCR_SPEC> {
        CST2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Status Flag 3 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst3(&mut self) -> CST3_W<PSCR_SPEC> {
        CST3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Status Flag 4 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst4(&mut self) -> CST4_W<PSCR_SPEC> {
        CST4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Status Flag 5 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst5(&mut self) -> CST5_W<PSCR_SPEC> {
        CST5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Status Flag 6 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst6(&mut self) -> CST6_W<PSCR_SPEC> {
        CST6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Status Flag 7 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst7(&mut self) -> CST7_W<PSCR_SPEC> {
        CST7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Status Flag 8 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst8(&mut self) -> CST8_W<PSCR_SPEC> {
        CST8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Status Flag 9 in PSR"]
    #[inline(always)]
    #[must_use]
    pub fn cst9(&mut self) -> CST9_W<PSCR_SPEC> {
        CST9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn crsif(&mut self) -> CRSIF_W<PSCR_SPEC> {
        CRSIF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cdlif(&mut self) -> CDLIF_W<PSCR_SPEC> {
        CDLIF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsif(&mut self) -> CTSIF_W<PSCR_SPEC> {
        CTSIF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctbif(&mut self) -> CTBIF_W<PSCR_SPEC> {
        CTBIF_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn crif(&mut self) -> CRIF_W<PSCR_SPEC> {
        CRIF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn caif(&mut self) -> CAIF_W<PSCR_SPEC> {
        CAIF_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cbrgif(&mut self) -> CBRGIF_W<PSCR_SPEC> {
        CBRGIF_W::new(self, 16)
    }
}
#[doc = "Protocol Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSCR_SPEC;
impl crate::RegisterSpec for PSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscr::W`](W) writer structure"]
impl crate::Writable for PSCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCR to value 0"]
impl crate::Resettable for PSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
