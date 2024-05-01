#[doc = "Register `NCR` reader"]
pub type R = crate::R<NcrSpec>;
#[doc = "Register `NCR` writer"]
pub type W = crate::W<NcrSpec>;
#[doc = "Node Initialization\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Init {
    #[doc = "0: Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    Value1 = 0,
    #[doc = "1: Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    Value2 = 1,
}
impl From<Init> for bool {
    #[inline(always)]
    fn from(variant: Init) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Node Initialization"]
pub type InitR = crate::BitReader<Init>;
impl InitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Init {
        match self.bits {
            false => Init::Value1,
            true => Init::Value2,
        }
    }
    #[doc = "Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Init::Value1
    }
    #[doc = "Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Init::Value2
    }
}
#[doc = "Field `INIT` writer - Node Initialization"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG, Init>;
impl<'a, REG> InitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resetting bit INIT enables the participation of the node in the CAN traffic. If the CAN node is in the bus-off state, the ongoing bus-off recovery (which does not depend on the INIT bit) is continued. With the end of the bus-off recovery sequence the CAN node is allowed to take part in the CAN traffic. If the CAN node is not in the bus-off state, a sequence of 11 consecutive recessive bits must be detected before the node is allowed to take part in the CAN traffic."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Init::Value1)
    }
    #[doc = "Setting this bit terminates the participation of this node in the CAN traffic. Any ongoing frame transfer is cancelled and the transmit line goes recessive. If the CAN node is in the bus-off state, then the running bus-off recovery sequence is continued. If the INIT bit is still set after the successful completion of the bus-off recovery sequence, i.e. after detecting 128 sequences of 11 consecutive recessive bits (11 1), then the CAN node leaves the bus-off state but remains inactive as long as INIT remains set."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Init::Value2)
    }
}
#[doc = "Transfer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trie {
    #[doc = "0: Transfer interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: Transfer interrupt is enabled."]
    Value2 = 1,
}
impl From<Trie> for bool {
    #[inline(always)]
    fn from(variant: Trie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIE` reader - Transfer Interrupt Enable"]
pub type TrieR = crate::BitReader<Trie>;
impl TrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trie {
        match self.bits {
            false => Trie::Value1,
            true => Trie::Value2,
        }
    }
    #[doc = "Transfer interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trie::Value1
    }
    #[doc = "Transfer interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trie::Value2
    }
}
#[doc = "Field `TRIE` writer - Transfer Interrupt Enable"]
pub type TrieW<'a, REG> = crate::BitWriter<'a, REG, Trie>;
impl<'a, REG> TrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trie::Value1)
    }
    #[doc = "Transfer interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trie::Value2)
    }
}
#[doc = "LEC Indicated Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lecie {
    #[doc = "0: Last error code interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: Last error code interrupt is enabled."]
    Value2 = 1,
}
impl From<Lecie> for bool {
    #[inline(always)]
    fn from(variant: Lecie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LECIE` reader - LEC Indicated Error Interrupt Enable"]
pub type LecieR = crate::BitReader<Lecie>;
impl LecieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lecie {
        match self.bits {
            false => Lecie::Value1,
            true => Lecie::Value2,
        }
    }
    #[doc = "Last error code interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lecie::Value1
    }
    #[doc = "Last error code interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lecie::Value2
    }
}
#[doc = "Field `LECIE` writer - LEC Indicated Error Interrupt Enable"]
pub type LecieW<'a, REG> = crate::BitWriter<'a, REG, Lecie>;
impl<'a, REG> LecieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Last error code interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lecie::Value1)
    }
    #[doc = "Last error code interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lecie::Value2)
    }
}
#[doc = "Alert Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alie {
    #[doc = "0: Alert interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: Alert interrupt is enabled."]
    Value2 = 1,
}
impl From<Alie> for bool {
    #[inline(always)]
    fn from(variant: Alie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIE` reader - Alert Interrupt Enable"]
pub type AlieR = crate::BitReader<Alie>;
impl AlieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alie {
        match self.bits {
            false => Alie::Value1,
            true => Alie::Value2,
        }
    }
    #[doc = "Alert interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Alie::Value1
    }
    #[doc = "Alert interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Alie::Value2
    }
}
#[doc = "Field `ALIE` writer - Alert Interrupt Enable"]
pub type AlieW<'a, REG> = crate::BitWriter<'a, REG, Alie>;
impl<'a, REG> AlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alert interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alie::Value1)
    }
    #[doc = "Alert interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alie::Value2)
    }
}
#[doc = "Field `CANDIS` reader - CAN Disable"]
pub type CandisR = crate::BitReader;
#[doc = "Field `CANDIS` writer - CAN Disable"]
pub type CandisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` reader - Transmit Disable"]
pub type TxdisR = crate::BitReader;
#[doc = "Field `TXDIS` writer - Transmit Disable"]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configuration Change Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cce {
    #[doc = "0: The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    Value1 = 0,
    #[doc = "1: The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    Value2 = 1,
}
impl From<Cce> for bool {
    #[inline(always)]
    fn from(variant: Cce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - Configuration Change Enable"]
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
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cce::Value1
    }
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cce::Value2
    }
}
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG, Cce>;
impl<'a, REG> CceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may only be read. All attempts to modify them are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cce::Value1)
    }
    #[doc = "The Bit Timing Register, the Port Control Register, Error Counter Register may be read and written."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cce::Value2)
    }
}
#[doc = "Field `CALM` reader - CAN Analyzer Mode"]
pub type CalmR = crate::BitReader;
#[doc = "Field `CALM` writer - CAN Analyzer Mode"]
pub type CalmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn trie(&self) -> TrieR {
        TrieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    pub fn lecie(&self) -> LecieR {
        LecieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> AlieR {
        AlieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    pub fn candis(&self) -> CandisR {
        CandisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline(always)]
    pub fn txdis(&self) -> TxdisR {
        TxdisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    pub fn calm(&self) -> CalmR {
        CalmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<NcrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trie(&mut self) -> TrieW<NcrSpec> {
        TrieW::new(self, 1)
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lecie(&mut self) -> LecieW<NcrSpec> {
        LecieW::new(self, 2)
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> AlieW<NcrSpec> {
        AlieW::new(self, 3)
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    #[must_use]
    pub fn candis(&mut self) -> CandisW<NcrSpec> {
        CandisW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TxdisW<NcrSpec> {
        TxdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CceW<NcrSpec> {
        CceW::new(self, 6)
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CalmW<NcrSpec> {
        CalmW::new(self, 7)
    }
}
#[doc = "Node Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NcrSpec;
impl crate::RegisterSpec for NcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr::R`](R) reader structure"]
impl crate::Readable for NcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ncr::W`](W) writer structure"]
impl crate::Writable for NcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCR to value 0x41"]
impl crate::Resettable for NcrSpec {
    const RESET_VALUE: u32 = 0x41;
}
