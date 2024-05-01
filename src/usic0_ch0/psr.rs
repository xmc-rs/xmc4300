#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PsrSpec>;
#[doc = "Field `ST0` reader - Protocol Status Flag 0"]
pub type St0R = crate::BitReader;
#[doc = "Field `ST0` writer - Protocol Status Flag 0"]
pub type St0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST1` reader - Protocol Status Flag 1"]
pub type St1R = crate::BitReader;
#[doc = "Field `ST1` writer - Protocol Status Flag 1"]
pub type St1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST2` reader - Protocol Status Flag 2"]
pub type St2R = crate::BitReader;
#[doc = "Field `ST2` writer - Protocol Status Flag 2"]
pub type St2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST3` reader - Protocol Status Flag 3"]
pub type St3R = crate::BitReader;
#[doc = "Field `ST3` writer - Protocol Status Flag 3"]
pub type St3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST4` reader - Protocol Status Flag 4"]
pub type St4R = crate::BitReader;
#[doc = "Field `ST4` writer - Protocol Status Flag 4"]
pub type St4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST5` reader - Protocol Status Flag 5"]
pub type St5R = crate::BitReader;
#[doc = "Field `ST5` writer - Protocol Status Flag 5"]
pub type St5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST6` reader - Protocol Status Flag 6"]
pub type St6R = crate::BitReader;
#[doc = "Field `ST6` writer - Protocol Status Flag 6"]
pub type St6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST7` reader - Protocol Status Flag 7"]
pub type St7R = crate::BitReader;
#[doc = "Field `ST7` writer - Protocol Status Flag 7"]
pub type St7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST8` reader - Protocol Status Flag 8"]
pub type St8R = crate::BitReader;
#[doc = "Field `ST8` writer - Protocol Status Flag 8"]
pub type St8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST9` reader - Protocol Status Flag 9"]
pub type St9R = crate::BitReader;
#[doc = "Field `ST9` writer - Protocol Status Flag 9"]
pub type St9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receiver Start Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsif {
    #[doc = "0: A receiver start event has not occurred."]
    Value1 = 0,
    #[doc = "1: A receiver start event has occurred."]
    Value2 = 1,
}
impl From<Rsif> for bool {
    #[inline(always)]
    fn from(variant: Rsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSIF` reader - Receiver Start Indication Flag"]
pub type RsifR = crate::BitReader<Rsif>;
impl RsifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsif {
        match self.bits {
            false => Rsif::Value1,
            true => Rsif::Value2,
        }
    }
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rsif::Value1
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rsif::Value2
    }
}
#[doc = "Field `RSIF` writer - Receiver Start Indication Flag"]
pub type RsifW<'a, REG> = crate::BitWriter<'a, REG, Rsif>;
impl<'a, REG> RsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsif::Value1)
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rsif::Value2)
    }
}
#[doc = "Data Lost Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlif {
    #[doc = "0: A data lost event has not occurred."]
    Value1 = 0,
    #[doc = "1: A data lost event has occurred."]
    Value2 = 1,
}
impl From<Dlif> for bool {
    #[inline(always)]
    fn from(variant: Dlif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLIF` reader - Data Lost Indication Flag"]
pub type DlifR = crate::BitReader<Dlif>;
impl DlifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlif {
        match self.bits {
            false => Dlif::Value1,
            true => Dlif::Value2,
        }
    }
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dlif::Value1
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dlif::Value2
    }
}
#[doc = "Field `DLIF` writer - Data Lost Indication Flag"]
pub type DlifW<'a, REG> = crate::BitWriter<'a, REG, Dlif>;
impl<'a, REG> DlifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlif::Value1)
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dlif::Value2)
    }
}
#[doc = "Transmit Shift Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsif {
    #[doc = "0: A transmit shift event has not occurred."]
    Value1 = 0,
    #[doc = "1: A transmit shift event has occurred."]
    Value2 = 1,
}
impl From<Tsif> for bool {
    #[inline(always)]
    fn from(variant: Tsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIF` reader - Transmit Shift Indication Flag"]
pub type TsifR = crate::BitReader<Tsif>;
impl TsifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsif {
        match self.bits {
            false => Tsif::Value1,
            true => Tsif::Value2,
        }
    }
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsif::Value1
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsif::Value2
    }
}
#[doc = "Field `TSIF` writer - Transmit Shift Indication Flag"]
pub type TsifW<'a, REG> = crate::BitWriter<'a, REG, Tsif>;
impl<'a, REG> TsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsif::Value1)
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsif::Value2)
    }
}
#[doc = "Transmit Buffer Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbif {
    #[doc = "0: A transmit buffer event has not occurred."]
    Value1 = 0,
    #[doc = "1: A transmit buffer event has occurred."]
    Value2 = 1,
}
impl From<Tbif> for bool {
    #[inline(always)]
    fn from(variant: Tbif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBIF` reader - Transmit Buffer Indication Flag"]
pub type TbifR = crate::BitReader<Tbif>;
impl TbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbif {
        match self.bits {
            false => Tbif::Value1,
            true => Tbif::Value2,
        }
    }
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tbif::Value1
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tbif::Value2
    }
}
#[doc = "Field `TBIF` writer - Transmit Buffer Indication Flag"]
pub type TbifW<'a, REG> = crate::BitWriter<'a, REG, Tbif>;
impl<'a, REG> TbifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbif::Value1)
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tbif::Value2)
    }
}
#[doc = "Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rif {
    #[doc = "0: A receive event has not occurred."]
    Value1 = 0,
    #[doc = "1: A receive event has occurred."]
    Value2 = 1,
}
impl From<Rif> for bool {
    #[inline(always)]
    fn from(variant: Rif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIF` reader - Receive Indication Flag"]
pub type RifR = crate::BitReader<Rif>;
impl RifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rif {
        match self.bits {
            false => Rif::Value1,
            true => Rif::Value2,
        }
    }
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rif::Value1
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rif::Value2
    }
}
#[doc = "Field `RIF` writer - Receive Indication Flag"]
pub type RifW<'a, REG> = crate::BitWriter<'a, REG, Rif>;
impl<'a, REG> RifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rif::Value1)
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rif::Value2)
    }
}
#[doc = "Alternative Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aif {
    #[doc = "0: An alternative receive event has not occurred."]
    Value1 = 0,
    #[doc = "1: An alternative receive event has occurred."]
    Value2 = 1,
}
impl From<Aif> for bool {
    #[inline(always)]
    fn from(variant: Aif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIF` reader - Alternative Receive Indication Flag"]
pub type AifR = crate::BitReader<Aif>;
impl AifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aif {
        match self.bits {
            false => Aif::Value1,
            true => Aif::Value2,
        }
    }
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Aif::Value1
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Aif::Value2
    }
}
#[doc = "Field `AIF` writer - Alternative Receive Indication Flag"]
pub type AifW<'a, REG> = crate::BitWriter<'a, REG, Aif>;
impl<'a, REG> AifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Aif::Value1)
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Aif::Value2)
    }
}
#[doc = "Baud Rate Generator Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brgif {
    #[doc = "0: A baud rate generator event has not occurred."]
    Value1 = 0,
    #[doc = "1: A baud rate generator event has occurred."]
    Value2 = 1,
}
impl From<Brgif> for bool {
    #[inline(always)]
    fn from(variant: Brgif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGIF` reader - Baud Rate Generator Indication Flag"]
pub type BrgifR = crate::BitReader<Brgif>;
impl BrgifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brgif {
        match self.bits {
            false => Brgif::Value1,
            true => Brgif::Value2,
        }
    }
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Brgif::Value1
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Brgif::Value2
    }
}
#[doc = "Field `BRGIF` writer - Baud Rate Generator Indication Flag"]
pub type BrgifW<'a, REG> = crate::BitWriter<'a, REG, Brgif>;
impl<'a, REG> BrgifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Brgif::Value1)
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Brgif::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Protocol Status Flag 0"]
    #[inline(always)]
    pub fn st0(&self) -> St0R {
        St0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protocol Status Flag 1"]
    #[inline(always)]
    pub fn st1(&self) -> St1R {
        St1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protocol Status Flag 2"]
    #[inline(always)]
    pub fn st2(&self) -> St2R {
        St2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protocol Status Flag 3"]
    #[inline(always)]
    pub fn st3(&self) -> St3R {
        St3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protocol Status Flag 4"]
    #[inline(always)]
    pub fn st4(&self) -> St4R {
        St4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protocol Status Flag 5"]
    #[inline(always)]
    pub fn st5(&self) -> St5R {
        St5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protocol Status Flag 6"]
    #[inline(always)]
    pub fn st6(&self) -> St6R {
        St6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protocol Status Flag 7"]
    #[inline(always)]
    pub fn st7(&self) -> St7R {
        St7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol Status Flag 8"]
    #[inline(always)]
    pub fn st8(&self) -> St8R {
        St8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protocol Status Flag 9"]
    #[inline(always)]
    pub fn st9(&self) -> St9R {
        St9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    pub fn rsif(&self) -> RsifR {
        RsifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    pub fn dlif(&self) -> DlifR {
        DlifR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    pub fn tsif(&self) -> TsifR {
        TsifR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    pub fn tbif(&self) -> TbifR {
        TbifR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    pub fn rif(&self) -> RifR {
        RifR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    pub fn aif(&self) -> AifR {
        AifR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    pub fn brgif(&self) -> BrgifR {
        BrgifR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protocol Status Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> St0W<PsrSpec> {
        St0W::new(self, 0)
    }
    #[doc = "Bit 1 - Protocol Status Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn st1(&mut self) -> St1W<PsrSpec> {
        St1W::new(self, 1)
    }
    #[doc = "Bit 2 - Protocol Status Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn st2(&mut self) -> St2W<PsrSpec> {
        St2W::new(self, 2)
    }
    #[doc = "Bit 3 - Protocol Status Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn st3(&mut self) -> St3W<PsrSpec> {
        St3W::new(self, 3)
    }
    #[doc = "Bit 4 - Protocol Status Flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn st4(&mut self) -> St4W<PsrSpec> {
        St4W::new(self, 4)
    }
    #[doc = "Bit 5 - Protocol Status Flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn st5(&mut self) -> St5W<PsrSpec> {
        St5W::new(self, 5)
    }
    #[doc = "Bit 6 - Protocol Status Flag 6"]
    #[inline(always)]
    #[must_use]
    pub fn st6(&mut self) -> St6W<PsrSpec> {
        St6W::new(self, 6)
    }
    #[doc = "Bit 7 - Protocol Status Flag 7"]
    #[inline(always)]
    #[must_use]
    pub fn st7(&mut self) -> St7W<PsrSpec> {
        St7W::new(self, 7)
    }
    #[doc = "Bit 8 - Protocol Status Flag 8"]
    #[inline(always)]
    #[must_use]
    pub fn st8(&mut self) -> St8W<PsrSpec> {
        St8W::new(self, 8)
    }
    #[doc = "Bit 9 - Protocol Status Flag 9"]
    #[inline(always)]
    #[must_use]
    pub fn st9(&mut self) -> St9W<PsrSpec> {
        St9W::new(self, 9)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsif(&mut self) -> RsifW<PsrSpec> {
        RsifW::new(self, 10)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlif(&mut self) -> DlifW<PsrSpec> {
        DlifW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsif(&mut self) -> TsifW<PsrSpec> {
        TsifW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbif(&mut self) -> TbifW<PsrSpec> {
        TbifW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RifW<PsrSpec> {
        RifW::new(self, 14)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aif(&mut self) -> AifW<PsrSpec> {
        AifW::new(self, 15)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn brgif(&mut self) -> BrgifW<PsrSpec> {
        BrgifW::new(self, 16)
    }
}
#[doc = "Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0;
}
