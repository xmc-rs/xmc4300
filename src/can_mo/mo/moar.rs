#[doc = "Register `MOAR` reader"]
pub type R = crate::R<MoarSpec>;
#[doc = "Register `MOAR` writer"]
pub type W = crate::W<MoarSpec>;
#[doc = "Field `ID` reader - CAN Identifier of Message Object n"]
pub type IdR = crate::FieldReader<u32>;
#[doc = "Field `ID` writer - CAN Identifier of Message Object n"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Identifier Extension Bit of Message Object n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ide {
    #[doc = "0: Message object n handles standard frames with 11-bit identifier."]
    Value1 = 0,
    #[doc = "1: Message object n handles extended frames with 29-bit identifier."]
    Value2 = 1,
}
impl From<Ide> for bool {
    #[inline(always)]
    fn from(variant: Ide) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDE` reader - Identifier Extension Bit of Message Object n"]
pub type IdeR = crate::BitReader<Ide>;
impl IdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ide {
        match self.bits {
            false => Ide::Value1,
            true => Ide::Value2,
        }
    }
    #[doc = "Message object n handles standard frames with 11-bit identifier."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ide::Value1
    }
    #[doc = "Message object n handles extended frames with 29-bit identifier."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ide::Value2
    }
}
#[doc = "Field `IDE` writer - Identifier Extension Bit of Message Object n"]
pub type IdeW<'a, REG> = crate::BitWriter<'a, REG, Ide>;
impl<'a, REG> IdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Message object n handles standard frames with 11-bit identifier."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ide::Value1)
    }
    #[doc = "Message object n handles extended frames with 29-bit identifier."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ide::Value2)
    }
}
#[doc = "Priority Class\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pri {
    #[doc = "1: Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL &amp; TXEN0 &amp; TXEN1 = 1) somewhere before this object in the list."]
    Value2 = 1,
    #[doc = "2: Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    Value3 = 2,
    #[doc = "3: Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    Value4 = 3,
}
impl From<Pri> for u8 {
    #[inline(always)]
    fn from(variant: Pri) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pri {
    type Ux = u8;
}
impl crate::IsEnum for Pri {}
#[doc = "Field `PRI` reader - Priority Class"]
pub type PriR = crate::FieldReader<Pri>;
impl PriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pri> {
        match self.bits {
            1 => Some(Pri::Value2),
            2 => Some(Pri::Value3),
            3 => Some(Pri::Value4),
            _ => None,
        }
    }
    #[doc = "Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL &amp; TXEN0 &amp; TXEN1 = 1) somewhere before this object in the list."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pri::Value2
    }
    #[doc = "Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pri::Value3
    }
    #[doc = "Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pri::Value4
    }
}
#[doc = "Field `PRI` writer - Priority Class"]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pri>;
impl<'a, REG> PriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmit acceptance filtering is based on the list order. This means that message object n is considered for transmission only if there is no other message object with valid transmit request (MSGVAL &amp; TXEN0 &amp; TXEN1 = 1) somewhere before this object in the list."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pri::Value2)
    }
    #[doc = "Transmit acceptance filtering is based on the CAN identifier. This means, message object n is considered for transmission only if there is no other message object with higher priority identifier + IDE + DIR (with respect to CAN arbitration rules) somewhere in the list (see )."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pri::Value3)
    }
    #[doc = "Transmit acceptance filtering is based on the list order (as PRI = 01B)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pri::Value4)
    }
}
impl R {
    #[doc = "Bits 0:28 - CAN Identifier of Message Object n"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Message Object n"]
    #[inline(always)]
    pub fn ide(&self) -> IdeR {
        IdeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:28 - CAN Identifier of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<MoarSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Message Object n"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IdeW<MoarSpec> {
        IdeW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PriW<MoarSpec> {
        PriW::new(self, 30)
    }
}
#[doc = "Message Object Arbitration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoarSpec;
impl crate::RegisterSpec for MoarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moar::R`](R) reader structure"]
impl crate::Readable for MoarSpec {}
#[doc = "`write(|w| ..)` method takes [`moar::W`](W) writer structure"]
impl crate::Writable for MoarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOAR to value 0"]
impl crate::Resettable for MoarSpec {
    const RESET_VALUE: u32 = 0;
}
