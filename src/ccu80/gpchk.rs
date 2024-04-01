#[doc = "Register `GPCHK` reader"]
pub type R = crate::R<GpchkSpec>;
#[doc = "Register `GPCHK` writer"]
pub type W = crate::W<GpchkSpec>;
#[doc = "Field `PASE` reader - Parity Checker Automatic start/stop"]
pub type PaseR = crate::BitReader;
#[doc = "Field `PASE` writer - Parity Checker Automatic start/stop"]
pub type PaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Parity Checker Automatic start/stop selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pacs {
    #[doc = "0: CC80"]
    Value1 = 0,
    #[doc = "1: CC81"]
    Value2 = 1,
    #[doc = "2: CC82"]
    Value3 = 2,
    #[doc = "3: CC83"]
    Value4 = 3,
}
impl From<Pacs> for u8 {
    #[inline(always)]
    fn from(variant: Pacs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pacs {
    type Ux = u8;
}
impl crate::IsEnum for Pacs {}
#[doc = "Field `PACS` reader - Parity Checker Automatic start/stop selector"]
pub type PacsR = crate::FieldReader<Pacs>;
impl PacsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pacs {
        match self.bits {
            0 => Pacs::Value1,
            1 => Pacs::Value2,
            2 => Pacs::Value3,
            3 => Pacs::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CC80"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pacs::Value1
    }
    #[doc = "CC81"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pacs::Value2
    }
    #[doc = "CC82"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pacs::Value3
    }
    #[doc = "CC83"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pacs::Value4
    }
}
#[doc = "Field `PACS` writer - Parity Checker Automatic start/stop selector"]
pub type PacsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pacs, crate::Safe>;
impl<'a, REG> PacsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC80"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pacs::Value1)
    }
    #[doc = "CC81"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pacs::Value2)
    }
    #[doc = "CC82"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pacs::Value3)
    }
    #[doc = "CC83"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pacs::Value4)
    }
}
#[doc = "Driver Input signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pisel {
    #[doc = "0: CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    Value1 = 0,
    #[doc = "1: CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    Value2 = 1,
    #[doc = "2: CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    Value3 = 2,
    #[doc = "3: CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    Value4 = 3,
}
impl From<Pisel> for u8 {
    #[inline(always)]
    fn from(variant: Pisel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pisel {
    type Ux = u8;
}
impl crate::IsEnum for Pisel {}
#[doc = "Field `PISEL` reader - Driver Input signal selector"]
pub type PiselR = crate::FieldReader<Pisel>;
impl PiselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pisel {
        match self.bits {
            0 => Pisel::Value1,
            1 => Pisel::Value2,
            2 => Pisel::Value3,
            3 => Pisel::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pisel::Value1
    }
    #[doc = "CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pisel::Value2
    }
    #[doc = "CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pisel::Value3
    }
    #[doc = "CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pisel::Value4
    }
}
#[doc = "Field `PISEL` writer - Driver Input signal selector"]
pub type PiselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pisel, crate::Safe>;
impl<'a, REG> PiselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pisel::Value1)
    }
    #[doc = "CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pisel::Value2)
    }
    #[doc = "CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pisel::Value3)
    }
    #[doc = "CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pisel::Value4)
    }
}
#[doc = "Parity Checker Delay Input Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcds {
    #[doc = "0: CCU8x.IGBTA"]
    Value1 = 0,
    #[doc = "1: CCU8x.IGBTB"]
    Value2 = 1,
    #[doc = "2: CCU8x.IGBTC"]
    Value3 = 2,
    #[doc = "3: CCU8x.IGBTD"]
    Value4 = 3,
}
impl From<Pcds> for u8 {
    #[inline(always)]
    fn from(variant: Pcds) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcds {
    type Ux = u8;
}
impl crate::IsEnum for Pcds {}
#[doc = "Field `PCDS` reader - Parity Checker Delay Input Selector"]
pub type PcdsR = crate::FieldReader<Pcds>;
impl PcdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcds {
        match self.bits {
            0 => Pcds::Value1,
            1 => Pcds::Value2,
            2 => Pcds::Value3,
            3 => Pcds::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CCU8x.IGBTA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pcds::Value1
    }
    #[doc = "CCU8x.IGBTB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pcds::Value2
    }
    #[doc = "CCU8x.IGBTC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pcds::Value3
    }
    #[doc = "CCU8x.IGBTD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pcds::Value4
    }
}
#[doc = "Field `PCDS` writer - Parity Checker Delay Input Selector"]
pub type PcdsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pcds, crate::Safe>;
impl<'a, REG> PcdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCU8x.IGBTA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcds::Value1)
    }
    #[doc = "CCU8x.IGBTB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pcds::Value2)
    }
    #[doc = "CCU8x.IGBTC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pcds::Value3)
    }
    #[doc = "CCU8x.IGBTD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pcds::Value4)
    }
}
#[doc = "Parity Checker type selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcts {
    #[doc = "0: Even parity enabled"]
    Value1 = 0,
    #[doc = "1: Odd parity enabled"]
    Value2 = 1,
}
impl From<Pcts> for bool {
    #[inline(always)]
    fn from(variant: Pcts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCTS` reader - Parity Checker type selector"]
pub type PctsR = crate::BitReader<Pcts>;
impl PctsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcts {
        match self.bits {
            false => Pcts::Value1,
            true => Pcts::Value2,
        }
    }
    #[doc = "Even parity enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pcts::Value1
    }
    #[doc = "Odd parity enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pcts::Value2
    }
}
#[doc = "Field `PCTS` writer - Parity Checker type selector"]
pub type PctsW<'a, REG> = crate::BitWriter<'a, REG, Pcts>;
impl<'a, REG> PctsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcts::Value1)
    }
    #[doc = "Odd parity enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pcts::Value2)
    }
}
#[doc = "Field `PCST` reader - Parity Checker XOR status"]
pub type PcstR = crate::BitReader;
#[doc = "Field `PCSEL0` reader - Parity Checker Slice 0 output selection"]
pub type Pcsel0R = crate::FieldReader;
#[doc = "Field `PCSEL0` writer - Parity Checker Slice 0 output selection"]
pub type Pcsel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCSEL1` reader - Parity Checker Slice 1 output selection"]
pub type Pcsel1R = crate::FieldReader;
#[doc = "Field `PCSEL1` writer - Parity Checker Slice 1 output selection"]
pub type Pcsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCSEL2` reader - Parity Checker Slice 2 output selection"]
pub type Pcsel2R = crate::FieldReader;
#[doc = "Field `PCSEL2` writer - Parity Checker Slice 2 output selection"]
pub type Pcsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCSEL3` reader - Parity Checker Slice 3 output selection"]
pub type Pcsel3R = crate::FieldReader;
#[doc = "Field `PCSEL3` writer - Parity Checker Slice 3 output selection"]
pub type Pcsel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Parity Checker Automatic start/stop"]
    #[inline(always)]
    pub fn pase(&self) -> PaseR {
        PaseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Parity Checker Automatic start/stop selector"]
    #[inline(always)]
    pub fn pacs(&self) -> PacsR {
        PacsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Driver Input signal selector"]
    #[inline(always)]
    pub fn pisel(&self) -> PiselR {
        PiselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Parity Checker Delay Input Selector"]
    #[inline(always)]
    pub fn pcds(&self) -> PcdsR {
        PcdsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Parity Checker type selector"]
    #[inline(always)]
    pub fn pcts(&self) -> PctsR {
        PctsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Parity Checker XOR status"]
    #[inline(always)]
    pub fn pcst(&self) -> PcstR {
        PcstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Parity Checker Slice 0 output selection"]
    #[inline(always)]
    pub fn pcsel0(&self) -> Pcsel0R {
        Pcsel0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Parity Checker Slice 1 output selection"]
    #[inline(always)]
    pub fn pcsel1(&self) -> Pcsel1R {
        Pcsel1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Parity Checker Slice 2 output selection"]
    #[inline(always)]
    pub fn pcsel2(&self) -> Pcsel2R {
        Pcsel2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Parity Checker Slice 3 output selection"]
    #[inline(always)]
    pub fn pcsel3(&self) -> Pcsel3R {
        Pcsel3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Checker Automatic start/stop"]
    #[inline(always)]
    #[must_use]
    pub fn pase(&mut self) -> PaseW<GpchkSpec> {
        PaseW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Parity Checker Automatic start/stop selector"]
    #[inline(always)]
    #[must_use]
    pub fn pacs(&mut self) -> PacsW<GpchkSpec> {
        PacsW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Driver Input signal selector"]
    #[inline(always)]
    #[must_use]
    pub fn pisel(&mut self) -> PiselW<GpchkSpec> {
        PiselW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Parity Checker Delay Input Selector"]
    #[inline(always)]
    #[must_use]
    pub fn pcds(&mut self) -> PcdsW<GpchkSpec> {
        PcdsW::new(self, 5)
    }
    #[doc = "Bit 7 - Parity Checker type selector"]
    #[inline(always)]
    #[must_use]
    pub fn pcts(&mut self) -> PctsW<GpchkSpec> {
        PctsW::new(self, 7)
    }
    #[doc = "Bits 16:19 - Parity Checker Slice 0 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel0(&mut self) -> Pcsel0W<GpchkSpec> {
        Pcsel0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Parity Checker Slice 1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel1(&mut self) -> Pcsel1W<GpchkSpec> {
        Pcsel1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Parity Checker Slice 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel2(&mut self) -> Pcsel2W<GpchkSpec> {
        Pcsel2W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Parity Checker Slice 3 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel3(&mut self) -> Pcsel3W<GpchkSpec> {
        Pcsel3W::new(self, 28)
    }
}
#[doc = "Parity Checker Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpchk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpchk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpchkSpec;
impl crate::RegisterSpec for GpchkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpchk::R`](R) reader structure"]
impl crate::Readable for GpchkSpec {}
#[doc = "`write(|w| ..)` method takes [`gpchk::W`](W) writer structure"]
impl crate::Writable for GpchkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPCHK to value 0"]
impl crate::Resettable for GpchkSpec {
    const RESET_VALUE: u32 = 0;
}
