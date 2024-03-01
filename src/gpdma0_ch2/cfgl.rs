#[doc = "Register `CFGL` reader"]
pub type R = crate::R<CfglSpec>;
#[doc = "Register `CFGL` writer"]
pub type W = crate::W<CfglSpec>;
#[doc = "Field `CH_PRIOR` reader - Channel priority"]
pub type ChPriorR = crate::FieldReader;
#[doc = "Field `CH_PRIOR` writer - Channel priority"]
pub type ChPriorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Channel Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChSusp {
    #[doc = "0: Not suspended."]
    Value1 = 0,
    #[doc = "1: Suspend DMA transfer from the source."]
    Value2 = 1,
}
impl From<ChSusp> for bool {
    #[inline(always)]
    fn from(variant: ChSusp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH_SUSP` reader - Channel Suspend"]
pub type ChSuspR = crate::BitReader<ChSusp>;
impl ChSuspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChSusp {
        match self.bits {
            false => ChSusp::Value1,
            true => ChSusp::Value2,
        }
    }
    #[doc = "Not suspended."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ChSusp::Value1
    }
    #[doc = "Suspend DMA transfer from the source."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ChSusp::Value2
    }
}
#[doc = "Field `CH_SUSP` writer - Channel Suspend"]
pub type ChSuspW<'a, REG> = crate::BitWriter<'a, REG, ChSusp>;
impl<'a, REG> ChSuspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not suspended."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ChSusp::Value1)
    }
    #[doc = "Suspend DMA transfer from the source."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ChSusp::Value2)
    }
}
#[doc = "Indicates if there is data left in the channel FIFO\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoEmpty {
    #[doc = "1: Channel FIFO empty"]
    Value1 = 1,
    #[doc = "0: Channel FIFO not empty"]
    Value2 = 0,
}
impl From<FifoEmpty> for bool {
    #[inline(always)]
    fn from(variant: FifoEmpty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_EMPTY` reader - Indicates if there is data left in the channel FIFO"]
pub type FifoEmptyR = crate::BitReader<FifoEmpty>;
impl FifoEmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoEmpty {
        match self.bits {
            true => FifoEmpty::Value1,
            false => FifoEmpty::Value2,
        }
    }
    #[doc = "Channel FIFO empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FifoEmpty::Value1
    }
    #[doc = "Channel FIFO not empty"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FifoEmpty::Value2
    }
}
#[doc = "Destination Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsSelDst {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    Value1 = 0,
    #[doc = "1: Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    Value2 = 1,
}
impl From<HsSelDst> for bool {
    #[inline(always)]
    fn from(variant: HsSelDst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_SEL_DST` reader - Destination Software or Hardware Handshaking Select"]
pub type HsSelDstR = crate::BitReader<HsSelDst>;
impl HsSelDstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsSelDst {
        match self.bits {
            false => HsSelDst::Value1,
            true => HsSelDst::Value2,
        }
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HsSelDst::Value1
    }
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HsSelDst::Value2
    }
}
#[doc = "Field `HS_SEL_DST` writer - Destination Software or Hardware Handshaking Select"]
pub type HsSelDstW<'a, REG> = crate::BitWriter<'a, REG, HsSelDst>;
impl<'a, REG> HsSelDstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HsSelDst::Value1)
    }
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HsSelDst::Value2)
    }
}
#[doc = "Source Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsSelSrc {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    Value1 = 0,
    #[doc = "1: Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    Value2 = 1,
}
impl From<HsSelSrc> for bool {
    #[inline(always)]
    fn from(variant: HsSelSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_SEL_SRC` reader - Source Software or Hardware Handshaking Select"]
pub type HsSelSrcR = crate::BitReader<HsSelSrc>;
impl HsSelSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsSelSrc {
        match self.bits {
            false => HsSelSrc::Value1,
            true => HsSelSrc::Value2,
        }
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HsSelSrc::Value1
    }
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HsSelSrc::Value2
    }
}
#[doc = "Field `HS_SEL_SRC` writer - Source Software or Hardware Handshaking Select"]
pub type HsSelSrcW<'a, REG> = crate::BitWriter<'a, REG, HsSelSrc>;
impl<'a, REG> HsSelSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HsSelSrc::Value1)
    }
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HsSelSrc::Value2)
    }
}
#[doc = "Channel Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockChL {
    #[doc = "0: Over complete DMA transfer"]
    Value1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    Value2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    Value3 = 2,
}
impl From<LockChL> for u8 {
    #[inline(always)]
    fn from(variant: LockChL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockChL {
    type Ux = u8;
}
#[doc = "Field `LOCK_CH_L` reader - Channel Lock Level"]
pub type LockChLR = crate::FieldReader<LockChL>;
impl LockChLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockChL> {
        match self.bits {
            0 => Some(LockChL::Value1),
            1 => Some(LockChL::Value2),
            2 => Some(LockChL::Value3),
            _ => None,
        }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LockChL::Value1
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LockChL::Value2
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LockChL::Value3
    }
}
#[doc = "Field `LOCK_CH_L` writer - Channel Lock Level"]
pub type LockChLW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockChL>;
impl<'a, REG> LockChLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LockChL::Value1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LockChL::Value2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LockChL::Value3)
    }
}
#[doc = "Bus Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockBL {
    #[doc = "0: Over complete DMA transfer"]
    Value1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    Value2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    Value3 = 2,
}
impl From<LockBL> for u8 {
    #[inline(always)]
    fn from(variant: LockBL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockBL {
    type Ux = u8;
}
#[doc = "Field `LOCK_B_L` reader - Bus Lock Level"]
pub type LockBLR = crate::FieldReader<LockBL>;
impl LockBLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockBL> {
        match self.bits {
            0 => Some(LockBL::Value1),
            1 => Some(LockBL::Value2),
            2 => Some(LockBL::Value3),
            _ => None,
        }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LockBL::Value1
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LockBL::Value2
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LockBL::Value3
    }
}
#[doc = "Field `LOCK_B_L` writer - Bus Lock Level"]
pub type LockBLW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockBL>;
impl<'a, REG> LockBLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LockBL::Value1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LockBL::Value2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LockBL::Value3)
    }
}
#[doc = "Field `LOCK_CH` reader - Channel Lock Bit"]
pub type LockChR = crate::BitReader;
#[doc = "Field `LOCK_CH` writer - Channel Lock Bit"]
pub type LockChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK_B` reader - Bus Lock Bit"]
pub type LockBR = crate::BitReader;
#[doc = "Field `LOCK_B` writer - Bus Lock Bit"]
pub type LockBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Destination Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DstHsPol {
    #[doc = "0: Active high"]
    Value1 = 0,
    #[doc = "1: Active low"]
    Value2 = 1,
}
impl From<DstHsPol> for bool {
    #[inline(always)]
    fn from(variant: DstHsPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_HS_POL` reader - Destination Handshaking Interface Polarity"]
pub type DstHsPolR = crate::BitReader<DstHsPol>;
impl DstHsPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DstHsPol {
        match self.bits {
            false => DstHsPol::Value1,
            true => DstHsPol::Value2,
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DstHsPol::Value1
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DstHsPol::Value2
    }
}
#[doc = "Field `DST_HS_POL` writer - Destination Handshaking Interface Polarity"]
pub type DstHsPolW<'a, REG> = crate::BitWriter<'a, REG, DstHsPol>;
impl<'a, REG> DstHsPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DstHsPol::Value1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DstHsPol::Value2)
    }
}
#[doc = "Source Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SrcHsPol {
    #[doc = "0: Active high"]
    Value1 = 0,
    #[doc = "1: Active low"]
    Value2 = 1,
}
impl From<SrcHsPol> for bool {
    #[inline(always)]
    fn from(variant: SrcHsPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC_HS_POL` reader - Source Handshaking Interface Polarity"]
pub type SrcHsPolR = crate::BitReader<SrcHsPol>;
impl SrcHsPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SrcHsPol {
        match self.bits {
            false => SrcHsPol::Value1,
            true => SrcHsPol::Value2,
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SrcHsPol::Value1
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SrcHsPol::Value2
    }
}
#[doc = "Field `SRC_HS_POL` writer - Source Handshaking Interface Polarity"]
pub type SrcHsPolW<'a, REG> = crate::BitWriter<'a, REG, SrcHsPol>;
impl<'a, REG> SrcHsPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SrcHsPol::Value1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SrcHsPol::Value2)
    }
}
#[doc = "Field `MAX_ABRST` reader - Maximum AMBA Burst Length"]
pub type MaxAbrstR = crate::FieldReader<u16>;
#[doc = "Field `MAX_ABRST` writer - Maximum AMBA Burst Length"]
pub type MaxAbrstW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    pub fn ch_prior(&self) -> ChPriorR {
        ChPriorR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    pub fn ch_susp(&self) -> ChSuspR {
        ChSuspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates if there is data left in the channel FIFO"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HsSelDstR {
        HsSelDstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HsSelSrcR {
        HsSelSrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    pub fn lock_ch_l(&self) -> LockChLR {
        LockChLR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    pub fn lock_b_l(&self) -> LockBLR {
        LockBLR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    pub fn lock_ch(&self) -> LockChR {
        LockChR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    pub fn lock_b(&self) -> LockBR {
        LockBR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&self) -> DstHsPolR {
        DstHsPolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&self) -> SrcHsPolR {
        SrcHsPolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    pub fn max_abrst(&self) -> MaxAbrstR {
        MaxAbrstR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    #[must_use]
    pub fn ch_prior(&mut self) -> ChPriorW<CfglSpec> {
        ChPriorW::new(self, 5)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn ch_susp(&mut self) -> ChSuspW<CfglSpec> {
        ChSuspW::new(self, 8)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_dst(&mut self) -> HsSelDstW<CfglSpec> {
        HsSelDstW::new(self, 10)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_src(&mut self) -> HsSelSrcW<CfglSpec> {
        HsSelSrcW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch_l(&mut self) -> LockChLW<CfglSpec> {
        LockChLW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    #[must_use]
    pub fn lock_b_l(&mut self) -> LockBLW<CfglSpec> {
        LockBLW::new(self, 14)
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch(&mut self) -> LockChW<CfglSpec> {
        LockChW::new(self, 16)
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock_b(&mut self) -> LockBW<CfglSpec> {
        LockBW::new(self, 17)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dst_hs_pol(&mut self) -> DstHsPolW<CfglSpec> {
        DstHsPolW::new(self, 18)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn src_hs_pol(&mut self) -> SrcHsPolW<CfglSpec> {
        SrcHsPolW::new(self, 19)
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn max_abrst(&mut self) -> MaxAbrstW<CfglSpec> {
        MaxAbrstW::new(self, 20)
    }
}
#[doc = "Configuration Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfglSpec;
impl crate::RegisterSpec for CfglSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgl::R`](R) reader structure"]
impl crate::Readable for CfglSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgl::W`](W) writer structure"]
impl crate::Writable for CfglSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGL to value 0x0e00"]
impl crate::Resettable for CfglSpec {
    const RESET_VALUE: u32 = 0x0e00;
}
