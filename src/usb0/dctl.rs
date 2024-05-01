#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DctlSpec>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DctlSpec>;
#[doc = "Field `RmtWkUpSig` reader - Remote Wakeup Signaling"]
pub type RmtWkUpSigR = crate::BitReader;
#[doc = "Field `RmtWkUpSig` writer - Remote Wakeup Signaling"]
pub type RmtWkUpSigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Soft Disconnect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SftDiscon {
    #[doc = "0: Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    Value1 = 0,
    #[doc = "1: The core drives a device disconnect event to the USB host."]
    Value2 = 1,
}
impl From<SftDiscon> for bool {
    #[inline(always)]
    fn from(variant: SftDiscon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SftDiscon` reader - Soft Disconnect"]
pub type SftDisconR = crate::BitReader<SftDiscon>;
impl SftDisconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SftDiscon {
        match self.bits {
            false => SftDiscon::Value1,
            true => SftDiscon::Value2,
        }
    }
    #[doc = "Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SftDiscon::Value1
    }
    #[doc = "The core drives a device disconnect event to the USB host."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SftDiscon::Value2
    }
}
#[doc = "Field `SftDiscon` writer - Soft Disconnect"]
pub type SftDisconW<'a, REG> = crate::BitWriter<'a, REG, SftDiscon>;
impl<'a, REG> SftDisconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SftDiscon::Value1)
    }
    #[doc = "The core drives a device disconnect event to the USB host."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SftDiscon::Value2)
    }
}
#[doc = "Global Non-periodic IN NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GnpinnakSts {
    #[doc = "0: A handshake is sent out based on the data availability in the transmit FIFO."]
    Value1 = 0,
    #[doc = "1: A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data availability in the transmit FIFO."]
    Value2 = 1,
}
impl From<GnpinnakSts> for bool {
    #[inline(always)]
    fn from(variant: GnpinnakSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GNPINNakSts` reader - Global Non-periodic IN NAK Status"]
pub type GnpinnakStsR = crate::BitReader<GnpinnakSts>;
impl GnpinnakStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GnpinnakSts {
        match self.bits {
            false => GnpinnakSts::Value1,
            true => GnpinnakSts::Value2,
        }
    }
    #[doc = "A handshake is sent out based on the data availability in the transmit FIFO."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GnpinnakSts::Value1
    }
    #[doc = "A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data availability in the transmit FIFO."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GnpinnakSts::Value2
    }
}
#[doc = "Global OUT NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoutnakSts {
    #[doc = "0: A handshake is sent based on the FIFO Status and the NAK and STALL bit settings."]
    Value1 = 0,
    #[doc = "1: No data is written to the RxFIFO, irrespective of space availability. Sends a NAK handshake on all packets, except on SETUP transactions. All isochronous OUT packets are dropped."]
    Value2 = 1,
}
impl From<GoutnakSts> for bool {
    #[inline(always)]
    fn from(variant: GoutnakSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GOUTNakSts` reader - Global OUT NAK Status"]
pub type GoutnakStsR = crate::BitReader<GoutnakSts>;
impl GoutnakStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GoutnakSts {
        match self.bits {
            false => GoutnakSts::Value1,
            true => GoutnakSts::Value2,
        }
    }
    #[doc = "A handshake is sent based on the FIFO Status and the NAK and STALL bit settings."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GoutnakSts::Value1
    }
    #[doc = "No data is written to the RxFIFO, irrespective of space availability. Sends a NAK handshake on all packets, except on SETUP transactions. All isochronous OUT packets are dropped."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GoutnakSts::Value2
    }
}
#[doc = "Field `SGNPInNak` writer - Set Global Non-periodic IN NAK"]
pub type SgnpinNakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGNPInNak` writer - Clear Global Non-periodic IN NAK"]
pub type CgnpinNakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGOUTNak` writer - Set Global OUT NAK"]
pub type SgoutnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGOUTNak` writer - Clear Global OUT NAK"]
pub type CgoutnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Global Multi Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gmc {
    #[doc = "0: Invalid."]
    Value1 = 0,
    #[doc = "1: 1 packet."]
    Value2 = 1,
    #[doc = "2: 2 packets."]
    Value3 = 2,
    #[doc = "3: 3 packets."]
    Value4 = 3,
}
impl From<Gmc> for u8 {
    #[inline(always)]
    fn from(variant: Gmc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gmc {
    type Ux = u8;
}
impl crate::IsEnum for Gmc {}
#[doc = "Field `GMC` reader - Global Multi Count"]
pub type GmcR = crate::FieldReader<Gmc>;
impl GmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gmc {
        match self.bits {
            0 => Gmc::Value1,
            1 => Gmc::Value2,
            2 => Gmc::Value3,
            3 => Gmc::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Invalid."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Gmc::Value1
    }
    #[doc = "1 packet."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gmc::Value2
    }
    #[doc = "2 packets."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Gmc::Value3
    }
    #[doc = "3 packets."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Gmc::Value4
    }
}
#[doc = "Field `GMC` writer - Global Multi Count"]
pub type GmcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gmc, crate::Safe>;
impl<'a, REG> GmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Invalid."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gmc::Value1)
    }
    #[doc = "1 packet."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gmc::Value2)
    }
    #[doc = "2 packets."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Gmc::Value3)
    }
    #[doc = "3 packets."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Gmc::Value4)
    }
}
#[doc = "Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgnrFrmNum {
    #[doc = "0: Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    Value1 = 0,
    #[doc = "1: Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    Value2 = 1,
}
impl From<IgnrFrmNum> for bool {
    #[inline(always)]
    fn from(variant: IgnrFrmNum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IgnrFrmNum` reader - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
pub type IgnrFrmNumR = crate::BitReader<IgnrFrmNum>;
impl IgnrFrmNumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IgnrFrmNum {
        match self.bits {
            false => IgnrFrmNum::Value1,
            true => IgnrFrmNum::Value2,
        }
    }
    #[doc = "Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IgnrFrmNum::Value1
    }
    #[doc = "Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IgnrFrmNum::Value2
    }
}
#[doc = "Field `IgnrFrmNum` writer - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
pub type IgnrFrmNumW<'a, REG> = crate::BitWriter<'a, REG, IgnrFrmNum>;
impl<'a, REG> IgnrFrmNumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IgnrFrmNum::Value1)
    }
    #[doc = "Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IgnrFrmNum::Value2)
    }
}
#[doc = "Field `NakOnBble` reader - Set NAK automatically on babble"]
pub type NakOnBbleR = crate::BitReader;
#[doc = "Field `NakOnBble` writer - Set NAK automatically on babble"]
pub type NakOnBbleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable continue on BNA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnContOnBna {
    #[doc = "0: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    Value1 = 0,
    #[doc = "1: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    Value2 = 1,
}
impl From<EnContOnBna> for bool {
    #[inline(always)]
    fn from(variant: EnContOnBna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnContOnBNA` reader - Enable continue on BNA"]
pub type EnContOnBnaR = crate::BitReader<EnContOnBna>;
impl EnContOnBnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnContOnBna {
        match self.bits {
            false => EnContOnBna::Value1,
            true => EnContOnBna::Value2,
        }
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EnContOnBna::Value1
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EnContOnBna::Value2
    }
}
#[doc = "Field `EnContOnBNA` writer - Enable continue on BNA"]
pub type EnContOnBnaW<'a, REG> = crate::BitWriter<'a, REG, EnContOnBna>;
impl<'a, REG> EnContOnBnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EnContOnBna::Value1)
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EnContOnBna::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmt_wk_up_sig(&self) -> RmtWkUpSigR {
        RmtWkUpSigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sft_discon(&self) -> SftDisconR {
        SftDisconR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline(always)]
    pub fn gnpinnak_sts(&self) -> GnpinnakStsR {
        GnpinnakStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline(always)]
    pub fn goutnak_sts(&self) -> GoutnakStsR {
        GoutnakStsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Global Multi Count"]
    #[inline(always)]
    pub fn gmc(&self) -> GmcR {
        GmcR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
    #[inline(always)]
    pub fn ignr_frm_num(&self) -> IgnrFrmNumR {
        IgnrFrmNumR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set NAK automatically on babble"]
    #[inline(always)]
    pub fn nak_on_bble(&self) -> NakOnBbleR {
        NakOnBbleR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable continue on BNA"]
    #[inline(always)]
    pub fn en_cont_on_bna(&self) -> EnContOnBnaR {
        EnContOnBnaR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_wk_up_sig(&mut self) -> RmtWkUpSigW<DctlSpec> {
        RmtWkUpSigW::new(self, 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sft_discon(&mut self) -> SftDisconW<DctlSpec> {
        SftDisconW::new(self, 1)
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgnpin_nak(&mut self) -> SgnpinNakW<DctlSpec> {
        SgnpinNakW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgnpin_nak(&mut self) -> CgnpinNakW<DctlSpec> {
        CgnpinNakW::new(self, 8)
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgoutnak(&mut self) -> SgoutnakW<DctlSpec> {
        SgoutnakW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgoutnak(&mut self) -> CgoutnakW<DctlSpec> {
        CgoutnakW::new(self, 10)
    }
    #[doc = "Bits 13:14 - Global Multi Count"]
    #[inline(always)]
    #[must_use]
    pub fn gmc(&mut self) -> GmcW<DctlSpec> {
        GmcW::new(self, 13)
    }
    #[doc = "Bit 15 - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
    #[inline(always)]
    #[must_use]
    pub fn ignr_frm_num(&mut self) -> IgnrFrmNumW<DctlSpec> {
        IgnrFrmNumW::new(self, 15)
    }
    #[doc = "Bit 16 - Set NAK automatically on babble"]
    #[inline(always)]
    #[must_use]
    pub fn nak_on_bble(&mut self) -> NakOnBbleW<DctlSpec> {
        NakOnBbleW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable continue on BNA"]
    #[inline(always)]
    #[must_use]
    pub fn en_cont_on_bna(&mut self) -> EnContOnBnaW<DctlSpec> {
        EnContOnBnaW::new(self, 17)
    }
}
#[doc = "Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctlSpec;
impl crate::RegisterSpec for DctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTL to value 0x02"]
impl crate::Resettable for DctlSpec {
    const RESET_VALUE: u32 = 0x02;
}
