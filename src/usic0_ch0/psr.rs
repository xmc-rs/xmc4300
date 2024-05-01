#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSR_SPEC>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PSR_SPEC>;
#[doc = "Field `ST0` reader - Protocol Status Flag 0"]
pub type ST0_R = crate::BitReader;
#[doc = "Field `ST0` writer - Protocol Status Flag 0"]
pub type ST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST1` reader - Protocol Status Flag 1"]
pub type ST1_R = crate::BitReader;
#[doc = "Field `ST1` writer - Protocol Status Flag 1"]
pub type ST1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST2` reader - Protocol Status Flag 2"]
pub type ST2_R = crate::BitReader;
#[doc = "Field `ST2` writer - Protocol Status Flag 2"]
pub type ST2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST3` reader - Protocol Status Flag 3"]
pub type ST3_R = crate::BitReader;
#[doc = "Field `ST3` writer - Protocol Status Flag 3"]
pub type ST3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST4` reader - Protocol Status Flag 4"]
pub type ST4_R = crate::BitReader;
#[doc = "Field `ST4` writer - Protocol Status Flag 4"]
pub type ST4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST5` reader - Protocol Status Flag 5"]
pub type ST5_R = crate::BitReader;
#[doc = "Field `ST5` writer - Protocol Status Flag 5"]
pub type ST5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST6` reader - Protocol Status Flag 6"]
pub type ST6_R = crate::BitReader;
#[doc = "Field `ST6` writer - Protocol Status Flag 6"]
pub type ST6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST7` reader - Protocol Status Flag 7"]
pub type ST7_R = crate::BitReader;
#[doc = "Field `ST7` writer - Protocol Status Flag 7"]
pub type ST7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST8` reader - Protocol Status Flag 8"]
pub type ST8_R = crate::BitReader;
#[doc = "Field `ST8` writer - Protocol Status Flag 8"]
pub type ST8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST9` reader - Protocol Status Flag 9"]
pub type ST9_R = crate::BitReader;
#[doc = "Field `ST9` writer - Protocol Status Flag 9"]
pub type ST9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receiver Start Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSIF_A {
    #[doc = "0: A receiver start event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A receiver start event has occurred."]
    VALUE2 = 1,
}
impl From<RSIF_A> for bool {
    #[inline(always)]
    fn from(variant: RSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSIF` reader - Receiver Start Indication Flag"]
pub type RSIF_R = crate::BitReader<RSIF_A>;
impl RSIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSIF_A {
        match self.bits {
            false => RSIF_A::VALUE1,
            true => RSIF_A::VALUE2,
        }
    }
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSIF_A::VALUE1
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSIF_A::VALUE2
    }
}
#[doc = "Field `RSIF` writer - Receiver Start Indication Flag"]
pub type RSIF_W<'a, REG> = crate::BitWriter<'a, REG, RSIF_A>;
impl<'a, REG> RSIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receiver start event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RSIF_A::VALUE1)
    }
    #[doc = "A receiver start event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RSIF_A::VALUE2)
    }
}
#[doc = "Data Lost Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLIF_A {
    #[doc = "0: A data lost event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A data lost event has occurred."]
    VALUE2 = 1,
}
impl From<DLIF_A> for bool {
    #[inline(always)]
    fn from(variant: DLIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLIF` reader - Data Lost Indication Flag"]
pub type DLIF_R = crate::BitReader<DLIF_A>;
impl DLIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLIF_A {
        match self.bits {
            false => DLIF_A::VALUE1,
            true => DLIF_A::VALUE2,
        }
    }
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLIF_A::VALUE1
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLIF_A::VALUE2
    }
}
#[doc = "Field `DLIF` writer - Data Lost Indication Flag"]
pub type DLIF_W<'a, REG> = crate::BitWriter<'a, REG, DLIF_A>;
impl<'a, REG> DLIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A data lost event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DLIF_A::VALUE1)
    }
    #[doc = "A data lost event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DLIF_A::VALUE2)
    }
}
#[doc = "Transmit Shift Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIF_A {
    #[doc = "0: A transmit shift event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A transmit shift event has occurred."]
    VALUE2 = 1,
}
impl From<TSIF_A> for bool {
    #[inline(always)]
    fn from(variant: TSIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIF` reader - Transmit Shift Indication Flag"]
pub type TSIF_R = crate::BitReader<TSIF_A>;
impl TSIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSIF_A {
        match self.bits {
            false => TSIF_A::VALUE1,
            true => TSIF_A::VALUE2,
        }
    }
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSIF_A::VALUE1
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSIF_A::VALUE2
    }
}
#[doc = "Field `TSIF` writer - Transmit Shift Indication Flag"]
pub type TSIF_W<'a, REG> = crate::BitWriter<'a, REG, TSIF_A>;
impl<'a, REG> TSIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit shift event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSIF_A::VALUE1)
    }
    #[doc = "A transmit shift event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSIF_A::VALUE2)
    }
}
#[doc = "Transmit Buffer Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBIF_A {
    #[doc = "0: A transmit buffer event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A transmit buffer event has occurred."]
    VALUE2 = 1,
}
impl From<TBIF_A> for bool {
    #[inline(always)]
    fn from(variant: TBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBIF` reader - Transmit Buffer Indication Flag"]
pub type TBIF_R = crate::BitReader<TBIF_A>;
impl TBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBIF_A {
        match self.bits {
            false => TBIF_A::VALUE1,
            true => TBIF_A::VALUE2,
        }
    }
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBIF_A::VALUE1
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBIF_A::VALUE2
    }
}
#[doc = "Field `TBIF` writer - Transmit Buffer Indication Flag"]
pub type TBIF_W<'a, REG> = crate::BitWriter<'a, REG, TBIF_A>;
impl<'a, REG> TBIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A transmit buffer event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TBIF_A::VALUE1)
    }
    #[doc = "A transmit buffer event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TBIF_A::VALUE2)
    }
}
#[doc = "Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIF_A {
    #[doc = "0: A receive event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A receive event has occurred."]
    VALUE2 = 1,
}
impl From<RIF_A> for bool {
    #[inline(always)]
    fn from(variant: RIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIF` reader - Receive Indication Flag"]
pub type RIF_R = crate::BitReader<RIF_A>;
impl RIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RIF_A {
        match self.bits {
            false => RIF_A::VALUE1,
            true => RIF_A::VALUE2,
        }
    }
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RIF_A::VALUE1
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RIF_A::VALUE2
    }
}
#[doc = "Field `RIF` writer - Receive Indication Flag"]
pub type RIF_W<'a, REG> = crate::BitWriter<'a, REG, RIF_A>;
impl<'a, REG> RIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RIF_A::VALUE1)
    }
    #[doc = "A receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RIF_A::VALUE2)
    }
}
#[doc = "Alternative Receive Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIF_A {
    #[doc = "0: An alternative receive event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: An alternative receive event has occurred."]
    VALUE2 = 1,
}
impl From<AIF_A> for bool {
    #[inline(always)]
    fn from(variant: AIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIF` reader - Alternative Receive Indication Flag"]
pub type AIF_R = crate::BitReader<AIF_A>;
impl AIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AIF_A {
        match self.bits {
            false => AIF_A::VALUE1,
            true => AIF_A::VALUE2,
        }
    }
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AIF_A::VALUE1
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AIF_A::VALUE2
    }
}
#[doc = "Field `AIF` writer - Alternative Receive Indication Flag"]
pub type AIF_W<'a, REG> = crate::BitWriter<'a, REG, AIF_A>;
impl<'a, REG> AIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An alternative receive event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AIF_A::VALUE1)
    }
    #[doc = "An alternative receive event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AIF_A::VALUE2)
    }
}
#[doc = "Baud Rate Generator Indication Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGIF_A {
    #[doc = "0: A baud rate generator event has not occurred."]
    VALUE1 = 0,
    #[doc = "1: A baud rate generator event has occurred."]
    VALUE2 = 1,
}
impl From<BRGIF_A> for bool {
    #[inline(always)]
    fn from(variant: BRGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGIF` reader - Baud Rate Generator Indication Flag"]
pub type BRGIF_R = crate::BitReader<BRGIF_A>;
impl BRGIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRGIF_A {
        match self.bits {
            false => BRGIF_A::VALUE1,
            true => BRGIF_A::VALUE2,
        }
    }
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BRGIF_A::VALUE1
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BRGIF_A::VALUE2
    }
}
#[doc = "Field `BRGIF` writer - Baud Rate Generator Indication Flag"]
pub type BRGIF_W<'a, REG> = crate::BitWriter<'a, REG, BRGIF_A>;
impl<'a, REG> BRGIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A baud rate generator event has not occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BRGIF_A::VALUE1)
    }
    #[doc = "A baud rate generator event has occurred."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BRGIF_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Protocol Status Flag 0"]
    #[inline(always)]
    pub fn st0(&self) -> ST0_R {
        ST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protocol Status Flag 1"]
    #[inline(always)]
    pub fn st1(&self) -> ST1_R {
        ST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protocol Status Flag 2"]
    #[inline(always)]
    pub fn st2(&self) -> ST2_R {
        ST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protocol Status Flag 3"]
    #[inline(always)]
    pub fn st3(&self) -> ST3_R {
        ST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protocol Status Flag 4"]
    #[inline(always)]
    pub fn st4(&self) -> ST4_R {
        ST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protocol Status Flag 5"]
    #[inline(always)]
    pub fn st5(&self) -> ST5_R {
        ST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protocol Status Flag 6"]
    #[inline(always)]
    pub fn st6(&self) -> ST6_R {
        ST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protocol Status Flag 7"]
    #[inline(always)]
    pub fn st7(&self) -> ST7_R {
        ST7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol Status Flag 8"]
    #[inline(always)]
    pub fn st8(&self) -> ST8_R {
        ST8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protocol Status Flag 9"]
    #[inline(always)]
    pub fn st9(&self) -> ST9_R {
        ST9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    pub fn rsif(&self) -> RSIF_R {
        RSIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    pub fn dlif(&self) -> DLIF_R {
        DLIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    pub fn tsif(&self) -> TSIF_R {
        TSIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    pub fn tbif(&self) -> TBIF_R {
        TBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    pub fn rif(&self) -> RIF_R {
        RIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    pub fn aif(&self) -> AIF_R {
        AIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    pub fn brgif(&self) -> BRGIF_R {
        BRGIF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protocol Status Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> ST0_W<PSR_SPEC> {
        ST0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Protocol Status Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn st1(&mut self) -> ST1_W<PSR_SPEC> {
        ST1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Protocol Status Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn st2(&mut self) -> ST2_W<PSR_SPEC> {
        ST2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Protocol Status Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn st3(&mut self) -> ST3_W<PSR_SPEC> {
        ST3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Protocol Status Flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn st4(&mut self) -> ST4_W<PSR_SPEC> {
        ST4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Protocol Status Flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn st5(&mut self) -> ST5_W<PSR_SPEC> {
        ST5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Protocol Status Flag 6"]
    #[inline(always)]
    #[must_use]
    pub fn st6(&mut self) -> ST6_W<PSR_SPEC> {
        ST6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Protocol Status Flag 7"]
    #[inline(always)]
    #[must_use]
    pub fn st7(&mut self) -> ST7_W<PSR_SPEC> {
        ST7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Protocol Status Flag 8"]
    #[inline(always)]
    #[must_use]
    pub fn st8(&mut self) -> ST8_W<PSR_SPEC> {
        ST8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Protocol Status Flag 9"]
    #[inline(always)]
    #[must_use]
    pub fn st9(&mut self) -> ST9_W<PSR_SPEC> {
        ST9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Receiver Start Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsif(&mut self) -> RSIF_W<PSR_SPEC> {
        RSIF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Data Lost Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlif(&mut self) -> DLIF_W<PSR_SPEC> {
        DLIF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Shift Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsif(&mut self) -> TSIF_W<PSR_SPEC> {
        TSIF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Buffer Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tbif(&mut self) -> TBIF_W<PSR_SPEC> {
        TBIF_W::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rif(&mut self) -> RIF_W<PSR_SPEC> {
        RIF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Alternative Receive Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aif(&mut self) -> AIF_W<PSR_SPEC> {
        AIF_W::new(self, 15)
    }
    #[doc = "Bit 16 - Baud Rate Generator Indication Flag"]
    #[inline(always)]
    #[must_use]
    pub fn brgif(&mut self) -> BRGIF_W<PSR_SPEC> {
        BRGIF_W::new(self, 16)
    }
}
#[doc = "Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
