#[doc = "Register `EMUXCTR` reader"]
pub type R = crate::R<EmuxctrSpec>;
#[doc = "Register `EMUXCTR` writer"]
pub type W = crate::W<EmuxctrSpec>;
#[doc = "Field `EMUXSET` reader - External Multiplexer Start Selection"]
pub type EmuxsetR = crate::FieldReader;
#[doc = "Field `EMUXSET` writer - External Multiplexer Start Selection"]
pub type EmuxsetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EMUXACT` reader - External Multiplexer Actual Selection"]
pub type EmuxactR = crate::FieldReader;
#[doc = "Field `EMUXCH` reader - External Multiplexer Channel Select"]
pub type EmuxchR = crate::FieldReader<u16>;
#[doc = "Field `EMUXCH` writer - External Multiplexer Channel Select"]
pub type EmuxchW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "External Multiplexer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emuxmode {
    #[doc = "0: Software control (no hardware action)"]
    Value1 = 0,
    #[doc = "1: Steady mode (use EMUXSET value)"]
    Value2 = 1,
    #[doc = "2: Single-step mode"]
    Value3 = 2,
    #[doc = "3: Sequence mode"]
    Value4 = 3,
}
impl From<Emuxmode> for u8 {
    #[inline(always)]
    fn from(variant: Emuxmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emuxmode {
    type Ux = u8;
}
impl crate::IsEnum for Emuxmode {}
#[doc = "Field `EMUXMODE` reader - External Multiplexer Mode"]
pub type EmuxmodeR = crate::FieldReader<Emuxmode>;
impl EmuxmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emuxmode {
        match self.bits {
            0 => Emuxmode::Value1,
            1 => Emuxmode::Value2,
            2 => Emuxmode::Value3,
            3 => Emuxmode::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Software control (no hardware action)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Emuxmode::Value1
    }
    #[doc = "Steady mode (use EMUXSET value)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Emuxmode::Value2
    }
    #[doc = "Single-step mode"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Emuxmode::Value3
    }
    #[doc = "Sequence mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Emuxmode::Value4
    }
}
#[doc = "Field `EMUXMODE` writer - External Multiplexer Mode"]
pub type EmuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Emuxmode, crate::Safe>;
impl<'a, REG> EmuxmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software control (no hardware action)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Emuxmode::Value1)
    }
    #[doc = "Steady mode (use EMUXSET value)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Emuxmode::Value2)
    }
    #[doc = "Single-step mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Emuxmode::Value3)
    }
    #[doc = "Sequence mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Emuxmode::Value4)
    }
}
#[doc = "External Multiplexer Coding Scheme\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emxcod {
    #[doc = "0: Output the channel number in binary code"]
    Value1 = 0,
    #[doc = "1: Output the channel number in Gray code"]
    Value2 = 1,
}
impl From<Emxcod> for bool {
    #[inline(always)]
    fn from(variant: Emxcod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMXCOD` reader - External Multiplexer Coding Scheme"]
pub type EmxcodR = crate::BitReader<Emxcod>;
impl EmxcodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emxcod {
        match self.bits {
            false => Emxcod::Value1,
            true => Emxcod::Value2,
        }
    }
    #[doc = "Output the channel number in binary code"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Emxcod::Value1
    }
    #[doc = "Output the channel number in Gray code"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Emxcod::Value2
    }
}
#[doc = "Field `EMXCOD` writer - External Multiplexer Coding Scheme"]
pub type EmxcodW<'a, REG> = crate::BitWriter<'a, REG, Emxcod>;
impl<'a, REG> EmxcodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output the channel number in binary code"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Emxcod::Value1)
    }
    #[doc = "Output the channel number in Gray code"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Emxcod::Value2)
    }
}
#[doc = "External Multiplexer Sample Time Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emxst {
    #[doc = "0: Use STCE whenever the setting changes"]
    Value1 = 0,
    #[doc = "1: Use STCE for each conversion of an external channel"]
    Value2 = 1,
}
impl From<Emxst> for bool {
    #[inline(always)]
    fn from(variant: Emxst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMXST` reader - External Multiplexer Sample Time Control"]
pub type EmxstR = crate::BitReader<Emxst>;
impl EmxstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emxst {
        match self.bits {
            false => Emxst::Value1,
            true => Emxst::Value2,
        }
    }
    #[doc = "Use STCE whenever the setting changes"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Emxst::Value1
    }
    #[doc = "Use STCE for each conversion of an external channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Emxst::Value2
    }
}
#[doc = "Field `EMXST` writer - External Multiplexer Sample Time Control"]
pub type EmxstW<'a, REG> = crate::BitWriter<'a, REG, Emxst>;
impl<'a, REG> EmxstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use STCE whenever the setting changes"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Emxst::Value1)
    }
    #[doc = "Use STCE for each conversion of an external channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Emxst::Value2)
    }
}
#[doc = "External Multiplexer Channel Selection Style\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emxcss {
    #[doc = "0: Channel number: Bitfield EMUXCH selects an arbitrary channel"]
    Value1 = 0,
    #[doc = "1: Channel enable: Each bit of bitfield EMUXCH selects the associated channel for EMUX control"]
    Value2 = 1,
}
impl From<Emxcss> for bool {
    #[inline(always)]
    fn from(variant: Emxcss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMXCSS` reader - External Multiplexer Channel Selection Style"]
pub type EmxcssR = crate::BitReader<Emxcss>;
impl EmxcssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emxcss {
        match self.bits {
            false => Emxcss::Value1,
            true => Emxcss::Value2,
        }
    }
    #[doc = "Channel number: Bitfield EMUXCH selects an arbitrary channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Emxcss::Value1
    }
    #[doc = "Channel enable: Each bit of bitfield EMUXCH selects the associated channel for EMUX control"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Emxcss::Value2
    }
}
#[doc = "Write Control for EMUX Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emxwc {
    #[doc = "0: No write access to EMUX cfg."]
    Value1 = 0,
    #[doc = "1: Bitfields EMXMODE, EMXCOD, EMXST, EMXCSS can be written"]
    Value2 = 1,
}
impl From<Emxwc> for bool {
    #[inline(always)]
    fn from(variant: Emxwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMXWC` writer - Write Control for EMUX Configuration"]
pub type EmxwcW<'a, REG> = crate::BitWriter<'a, REG, Emxwc>;
impl<'a, REG> EmxwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to EMUX cfg."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Emxwc::Value1)
    }
    #[doc = "Bitfields EMXMODE, EMXCOD, EMXST, EMXCSS can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Emxwc::Value2)
    }
}
impl R {
    #[doc = "Bits 0:2 - External Multiplexer Start Selection"]
    #[inline(always)]
    pub fn emuxset(&self) -> EmuxsetR {
        EmuxsetR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - External Multiplexer Actual Selection"]
    #[inline(always)]
    pub fn emuxact(&self) -> EmuxactR {
        EmuxactR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - External Multiplexer Channel Select"]
    #[inline(always)]
    pub fn emuxch(&self) -> EmuxchR {
        EmuxchR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:27 - External Multiplexer Mode"]
    #[inline(always)]
    pub fn emuxmode(&self) -> EmuxmodeR {
        EmuxmodeR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - External Multiplexer Coding Scheme"]
    #[inline(always)]
    pub fn emxcod(&self) -> EmxcodR {
        EmxcodR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - External Multiplexer Sample Time Control"]
    #[inline(always)]
    pub fn emxst(&self) -> EmxstR {
        EmxstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - External Multiplexer Channel Selection Style"]
    #[inline(always)]
    pub fn emxcss(&self) -> EmxcssR {
        EmxcssR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Multiplexer Start Selection"]
    #[inline(always)]
    #[must_use]
    pub fn emuxset(&mut self) -> EmuxsetW<EmuxctrSpec> {
        EmuxsetW::new(self, 0)
    }
    #[doc = "Bits 16:25 - External Multiplexer Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn emuxch(&mut self) -> EmuxchW<EmuxctrSpec> {
        EmuxchW::new(self, 16)
    }
    #[doc = "Bits 26:27 - External Multiplexer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn emuxmode(&mut self) -> EmuxmodeW<EmuxctrSpec> {
        EmuxmodeW::new(self, 26)
    }
    #[doc = "Bit 28 - External Multiplexer Coding Scheme"]
    #[inline(always)]
    #[must_use]
    pub fn emxcod(&mut self) -> EmxcodW<EmuxctrSpec> {
        EmxcodW::new(self, 28)
    }
    #[doc = "Bit 29 - External Multiplexer Sample Time Control"]
    #[inline(always)]
    #[must_use]
    pub fn emxst(&mut self) -> EmxstW<EmuxctrSpec> {
        EmxstW::new(self, 29)
    }
    #[doc = "Bit 31 - Write Control for EMUX Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn emxwc(&mut self) -> EmxwcW<EmuxctrSpec> {
        EmxwcW::new(self, 31)
    }
}
#[doc = "E0ternal Multiplexer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emuxctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emuxctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmuxctrSpec;
impl crate::RegisterSpec for EmuxctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emuxctr::R`](R) reader structure"]
impl crate::Readable for EmuxctrSpec {}
#[doc = "`write(|w| ..)` method takes [`emuxctr::W`](W) writer structure"]
impl crate::Writable for EmuxctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMUXCTR to value 0"]
impl crate::Resettable for EmuxctrSpec {
    const RESET_VALUE: u32 = 0;
}
