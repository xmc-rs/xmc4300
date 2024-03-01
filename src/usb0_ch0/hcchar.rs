#[doc = "Register `HCCHAR` reader"]
pub type R = crate::R<HccharSpec>;
#[doc = "Register `HCCHAR` writer"]
pub type W = crate::W<HccharSpec>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNum` reader - Endpoint Number"]
pub type EpnumR = crate::FieldReader;
#[doc = "Field `EPNum` writer - Endpoint Number"]
pub type EpnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Endpoint Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epdir {
    #[doc = "0: OUT"]
    Value1 = 0,
    #[doc = "1: IN"]
    Value2 = 1,
}
impl From<Epdir> for bool {
    #[inline(always)]
    fn from(variant: Epdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDir` reader - Endpoint Direction"]
pub type EpdirR = crate::BitReader<Epdir>;
impl EpdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epdir {
        match self.bits {
            false => Epdir::Value1,
            true => Epdir::Value2,
        }
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Epdir::Value1
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Epdir::Value2
    }
}
#[doc = "Field `EPDir` writer - Endpoint Direction"]
pub type EpdirW<'a, REG> = crate::BitWriter<'a, REG, Epdir>;
impl<'a, REG> EpdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OUT"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Epdir::Value1)
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Epdir::Value2)
    }
}
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eptype {
    #[doc = "0: Control"]
    Value1 = 0,
    #[doc = "1: Isochronous"]
    Value2 = 1,
    #[doc = "2: Bulk"]
    Value3 = 2,
    #[doc = "3: Interrupt"]
    Value4 = 3,
}
impl From<Eptype> for u8 {
    #[inline(always)]
    fn from(variant: Eptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eptype {
    type Ux = u8;
}
#[doc = "Field `EPType` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader<Eptype>;
impl EptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eptype {
        match self.bits {
            0 => Eptype::Value1,
            1 => Eptype::Value2,
            2 => Eptype::Value3,
            3 => Eptype::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eptype::Value1
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eptype::Value2
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Eptype::Value3
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Eptype::Value4
    }
}
#[doc = "Field `EPType` writer - Endpoint Type"]
pub type EptypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Eptype>;
impl<'a, REG> EptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Value1)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Value2)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Value3)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Value4)
    }
}
#[doc = "Multi Count / Error Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum McEc {
    #[doc = "1: 1 transaction"]
    Value2 = 1,
    #[doc = "2: 2 transactions to be issued for this endpoint per frame"]
    Value3 = 2,
    #[doc = "3: 3 transactions to be issued for this endpoint per frame"]
    Value4 = 3,
}
impl From<McEc> for u8 {
    #[inline(always)]
    fn from(variant: McEc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for McEc {
    type Ux = u8;
}
#[doc = "Field `MC_EC` reader - Multi Count / Error Count"]
pub type McEcR = crate::FieldReader<McEc>;
impl McEcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<McEc> {
        match self.bits {
            1 => Some(McEc::Value2),
            2 => Some(McEc::Value3),
            3 => Some(McEc::Value4),
            _ => None,
        }
    }
    #[doc = "1 transaction"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == McEc::Value2
    }
    #[doc = "2 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == McEc::Value3
    }
    #[doc = "3 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == McEc::Value4
    }
}
#[doc = "Field `MC_EC` writer - Multi Count / Error Count"]
pub type McEcW<'a, REG> = crate::FieldWriter<'a, REG, 2, McEc>;
impl<'a, REG> McEcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 transaction"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(McEc::Value2)
    }
    #[doc = "2 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(McEc::Value3)
    }
    #[doc = "3 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(McEc::Value4)
    }
}
#[doc = "Field `DevAddr` reader - Device Address"]
pub type DevAddrR = crate::FieldReader;
#[doc = "Field `DevAddr` writer - Device Address"]
pub type DevAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Odd Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OddFrm {
    #[doc = "0: Even frame"]
    Value1 = 0,
    #[doc = "1: Odd frame"]
    Value2 = 1,
}
impl From<OddFrm> for bool {
    #[inline(always)]
    fn from(variant: OddFrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OddFrm` reader - Odd Frame"]
pub type OddFrmR = crate::BitReader<OddFrm>;
impl OddFrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OddFrm {
        match self.bits {
            false => OddFrm::Value1,
            true => OddFrm::Value2,
        }
    }
    #[doc = "Even frame"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OddFrm::Value1
    }
    #[doc = "Odd frame"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OddFrm::Value2
    }
}
#[doc = "Field `OddFrm` writer - Odd Frame"]
pub type OddFrmW<'a, REG> = crate::BitWriter<'a, REG, OddFrm>;
impl<'a, REG> OddFrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even frame"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OddFrm::Value1)
    }
    #[doc = "Odd frame"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OddFrm::Value2)
    }
}
#[doc = "Field `ChDis` reader - Channel Disable"]
pub type ChDisR = crate::BitReader;
#[doc = "Field `ChDis` writer - Channel Disable"]
pub type ChDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChEna {
    #[doc = "0: Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    Value1 = 0,
    #[doc = "1: Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    Value2 = 1,
}
impl From<ChEna> for bool {
    #[inline(always)]
    fn from(variant: ChEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ChEna` reader - Channel Enable"]
pub type ChEnaR = crate::BitReader<ChEna>;
impl ChEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChEna {
        match self.bits {
            false => ChEna::Value1,
            true => ChEna::Value2,
        }
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ChEna::Value1
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ChEna::Value2
    }
}
#[doc = "Field `ChEna` writer - Channel Enable"]
pub type ChEnaW<'a, REG> = crate::BitWriter<'a, REG, ChEna>;
impl<'a, REG> ChEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ChEna::Value1)
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ChEna::Value2)
    }
}
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EpdirR {
        EpdirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count / Error Count"]
    #[inline(always)]
    pub fn mc_ec(&self) -> McEcR {
        McEcR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DevAddrR {
        DevAddrR::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn odd_frm(&self) -> OddFrmR {
        OddFrmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn ch_dis(&self) -> ChDisR {
        ChDisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn ch_ena(&self) -> ChEnaR {
        ChEnaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MpsW<HccharSpec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EpnumW<HccharSpec> {
        EpnumW::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EpdirW<HccharSpec> {
        EpdirW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EptypeW<HccharSpec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multi Count / Error Count"]
    #[inline(always)]
    #[must_use]
    pub fn mc_ec(&mut self) -> McEcW<HccharSpec> {
        McEcW::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn dev_addr(&mut self) -> DevAddrW<HccharSpec> {
        DevAddrW::new(self, 22)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    #[must_use]
    pub fn odd_frm(&mut self) -> OddFrmW<HccharSpec> {
        OddFrmW::new(self, 29)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_dis(&mut self) -> ChDisW<HccharSpec> {
        ChDisW::new(self, 30)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena(&mut self) -> ChEnaW<HccharSpec> {
        ChEnaW::new(self, 31)
    }
}
#[doc = "Host Channel Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccharSpec;
impl crate::RegisterSpec for HccharSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar::R`](R) reader structure"]
impl crate::Readable for HccharSpec {}
#[doc = "`write(|w| ..)` method takes [`hcchar::W`](W) writer structure"]
impl crate::Writable for HccharSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCCHAR to value 0"]
impl crate::Resettable for HccharSpec {
    const RESET_VALUE: u32 = 0;
}
