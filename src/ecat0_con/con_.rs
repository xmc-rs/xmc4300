#[doc = "Register `CON` reader"]
pub type R = crate::R<ConSpec>;
#[doc = "Register `CON` writer"]
pub type W = crate::W<ConSpec>;
#[doc = "Enable EtherCAT Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecatrsten {
    #[doc = "0: Reset request by EtherCAT Master disabled"]
    Value1 = 0,
    #[doc = "1: Reset request by EtherCAT Master enabled"]
    Value2 = 1,
}
impl From<Ecatrsten> for bool {
    #[inline(always)]
    fn from(variant: Ecatrsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECATRSTEN` reader - Enable EtherCAT Reset Request"]
pub type EcatrstenR = crate::BitReader<Ecatrsten>;
impl EcatrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecatrsten {
        match self.bits {
            false => Ecatrsten::Value1,
            true => Ecatrsten::Value2,
        }
    }
    #[doc = "Reset request by EtherCAT Master disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ecatrsten::Value1
    }
    #[doc = "Reset request by EtherCAT Master enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ecatrsten::Value2
    }
}
#[doc = "Field `ECATRSTEN` writer - Enable EtherCAT Reset Request"]
pub type EcatrstenW<'a, REG> = crate::BitWriter<'a, REG, Ecatrsten>;
impl<'a, REG> EcatrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset request by EtherCAT Master disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecatrsten::Value1)
    }
    #[doc = "Reset request by EtherCAT Master enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ecatrsten::Value2)
    }
}
#[doc = "LATCHIN0 Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Latchin0sel {
    #[doc = "0: Data input LATCHIN0A is selected"]
    Value1 = 0,
    #[doc = "1: Data input LATCHIN0B is selected"]
    Value2 = 1,
    #[doc = "2: Data input LATCHIN0C is selected"]
    Value3 = 2,
    #[doc = "3: Data input LATCHIN0D is selected"]
    Value4 = 3,
}
impl From<Latchin0sel> for u8 {
    #[inline(always)]
    fn from(variant: Latchin0sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Latchin0sel {
    type Ux = u8;
}
impl crate::IsEnum for Latchin0sel {}
#[doc = "Field `LATCHIN0SEL` reader - LATCHIN0 Input Select"]
pub type Latchin0selR = crate::FieldReader<Latchin0sel>;
impl Latchin0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Latchin0sel {
        match self.bits {
            0 => Latchin0sel::Value1,
            1 => Latchin0sel::Value2,
            2 => Latchin0sel::Value3,
            3 => Latchin0sel::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input LATCHIN0A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Latchin0sel::Value1
    }
    #[doc = "Data input LATCHIN0B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Latchin0sel::Value2
    }
    #[doc = "Data input LATCHIN0C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Latchin0sel::Value3
    }
    #[doc = "Data input LATCHIN0D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Latchin0sel::Value4
    }
}
#[doc = "Field `LATCHIN0SEL` writer - LATCHIN0 Input Select"]
pub type Latchin0selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Latchin0sel, crate::Safe>;
impl<'a, REG> Latchin0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input LATCHIN0A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Latchin0sel::Value1)
    }
    #[doc = "Data input LATCHIN0B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Latchin0sel::Value2)
    }
    #[doc = "Data input LATCHIN0C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Latchin0sel::Value3)
    }
    #[doc = "Data input LATCHIN0D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Latchin0sel::Value4)
    }
}
#[doc = "Field `LATCHIN0` reader - EtherCAT LATCH_IN0 Input Signal"]
pub type Latchin0R = crate::BitReader;
#[doc = "LATCHIN1 Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Latchin1sel {
    #[doc = "0: Data input LATCHIN1A is selected"]
    Value1 = 0,
    #[doc = "1: Data input LATCHIN1B is selected"]
    Value2 = 1,
    #[doc = "2: Data input LATCHIN1C is selected"]
    Value3 = 2,
    #[doc = "3: Data input LATCHIN1D is selected"]
    Value4 = 3,
}
impl From<Latchin1sel> for u8 {
    #[inline(always)]
    fn from(variant: Latchin1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Latchin1sel {
    type Ux = u8;
}
impl crate::IsEnum for Latchin1sel {}
#[doc = "Field `LATCHIN1SEL` reader - LATCHIN1 Input Select"]
pub type Latchin1selR = crate::FieldReader<Latchin1sel>;
impl Latchin1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Latchin1sel {
        match self.bits {
            0 => Latchin1sel::Value1,
            1 => Latchin1sel::Value2,
            2 => Latchin1sel::Value3,
            3 => Latchin1sel::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input LATCHIN1A is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Latchin1sel::Value1
    }
    #[doc = "Data input LATCHIN1B is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Latchin1sel::Value2
    }
    #[doc = "Data input LATCHIN1C is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Latchin1sel::Value3
    }
    #[doc = "Data input LATCHIN1D is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Latchin1sel::Value4
    }
}
#[doc = "Field `LATCHIN1SEL` writer - LATCHIN1 Input Select"]
pub type Latchin1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Latchin1sel, crate::Safe>;
impl<'a, REG> Latchin1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input LATCHIN1A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Latchin1sel::Value1)
    }
    #[doc = "Data input LATCHIN1B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Latchin1sel::Value2)
    }
    #[doc = "Data input LATCHIN1C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Latchin1sel::Value3)
    }
    #[doc = "Data input LATCHIN1D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Latchin1sel::Value4)
    }
}
#[doc = "Field `LATCHIN1` reader - EtherCAT LATCH_IN1 Input Signal"]
pub type Latchin1R = crate::BitReader;
#[doc = "Field `PHYOFFSET` reader - Ethernet PHY Address Offset"]
pub type PhyoffsetR = crate::FieldReader;
#[doc = "Field `PHYOFFSET` writer - Ethernet PHY Address Offset"]
pub type PhyoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "MDIO Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mdio {
    #[doc = "0: Data input MDIA is selected"]
    Value1 = 0,
    #[doc = "1: Data input MDIB is selected"]
    Value2 = 1,
    #[doc = "2: Data input MDIC is selected"]
    Value3 = 2,
    #[doc = "3: Data input MDID is selected"]
    Value4 = 3,
}
impl From<Mdio> for u8 {
    #[inline(always)]
    fn from(variant: Mdio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mdio {
    type Ux = u8;
}
impl crate::IsEnum for Mdio {}
#[doc = "Field `MDIO` reader - MDIO Input Select"]
pub type MdioR = crate::FieldReader<Mdio>;
impl MdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdio {
        match self.bits {
            0 => Mdio::Value1,
            1 => Mdio::Value2,
            2 => Mdio::Value3,
            3 => Mdio::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Data input MDIA is selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mdio::Value1
    }
    #[doc = "Data input MDIB is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mdio::Value2
    }
    #[doc = "Data input MDIC is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Mdio::Value3
    }
    #[doc = "Data input MDID is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Mdio::Value4
    }
}
#[doc = "Field `MDIO` writer - MDIO Input Select"]
pub type MdioW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mdio, crate::Safe>;
impl<'a, REG> MdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data input MDIA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mdio::Value1)
    }
    #[doc = "Data input MDIB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mdio::Value2)
    }
    #[doc = "Data input MDIC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Mdio::Value3)
    }
    #[doc = "Data input MDID is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Mdio::Value4)
    }
}
impl R {
    #[doc = "Bit 0 - Enable EtherCAT Reset Request"]
    #[inline(always)]
    pub fn ecatrsten(&self) -> EcatrstenR {
        EcatrstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - LATCHIN0 Input Select"]
    #[inline(always)]
    pub fn latchin0sel(&self) -> Latchin0selR {
        Latchin0selR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - EtherCAT LATCH_IN0 Input Signal"]
    #[inline(always)]
    pub fn latchin0(&self) -> Latchin0R {
        Latchin0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - LATCHIN1 Input Select"]
    #[inline(always)]
    pub fn latchin1sel(&self) -> Latchin1selR {
        Latchin1selR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - EtherCAT LATCH_IN1 Input Signal"]
    #[inline(always)]
    pub fn latchin1(&self) -> Latchin1R {
        Latchin1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Ethernet PHY Address Offset"]
    #[inline(always)]
    pub fn phyoffset(&self) -> PhyoffsetR {
        PhyoffsetR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    pub fn mdio(&self) -> MdioR {
        MdioR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable EtherCAT Reset Request"]
    #[inline(always)]
    #[must_use]
    pub fn ecatrsten(&mut self) -> EcatrstenW<ConSpec> {
        EcatrstenW::new(self, 0)
    }
    #[doc = "Bits 8:9 - LATCHIN0 Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn latchin0sel(&mut self) -> Latchin0selW<ConSpec> {
        Latchin0selW::new(self, 8)
    }
    #[doc = "Bits 12:13 - LATCHIN1 Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn latchin1sel(&mut self) -> Latchin1selW<ConSpec> {
        Latchin1selW::new(self, 12)
    }
    #[doc = "Bits 16:20 - Ethernet PHY Address Offset"]
    #[inline(always)]
    #[must_use]
    pub fn phyoffset(&mut self) -> PhyoffsetW<ConSpec> {
        PhyoffsetW::new(self, 16)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn mdio(&mut self) -> MdioW<ConSpec> {
        MdioW::new(self, 22)
    }
}
#[doc = "EtherCAT 0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConSpec;
impl crate::RegisterSpec for ConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`con::R`](R) reader structure"]
impl crate::Readable for ConSpec {}
#[doc = "`write(|w| ..)` method takes [`con::W`](W) writer structure"]
impl crate::Writable for ConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for ConSpec {
    const RESET_VALUE: u32 = 0;
}
