#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DCTL_SPEC>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DCTL_SPEC>;
#[doc = "Field `RmtWkUpSig` reader - Remote Wakeup Signaling"]
pub type RMT_WK_UP_SIG_R = crate::BitReader;
#[doc = "Field `RmtWkUpSig` writer - Remote Wakeup Signaling"]
pub type RMT_WK_UP_SIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SftDiscon` reader - Soft Disconnect"]
pub type SFT_DISCON_R = crate::BitReader<SFT_DISCON_A>;
#[doc = "Soft Disconnect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFT_DISCON_A {
    #[doc = "0: Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    VALUE1 = 0,
    #[doc = "1: The core drives a device disconnect event to the USB host."]
    VALUE2 = 1,
}
impl From<SFT_DISCON_A> for bool {
    #[inline(always)]
    fn from(variant: SFT_DISCON_A) -> Self {
        variant as u8 != 0
    }
}
impl SFT_DISCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SFT_DISCON_A {
        match self.bits {
            false => SFT_DISCON_A::VALUE1,
            true => SFT_DISCON_A::VALUE2,
        }
    }
    #[doc = "Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SFT_DISCON_A::VALUE1
    }
    #[doc = "The core drives a device disconnect event to the USB host."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SFT_DISCON_A::VALUE2
    }
}
#[doc = "Field `SftDiscon` writer - Soft Disconnect"]
pub type SFT_DISCON_W<'a, REG> = crate::BitWriter<'a, REG, SFT_DISCON_A>;
impl<'a, REG> SFT_DISCON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation. When this bit is cleared after a soft disconnect, the core drives a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SFT_DISCON_A::VALUE1)
    }
    #[doc = "The core drives a device disconnect event to the USB host."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SFT_DISCON_A::VALUE2)
    }
}
#[doc = "Field `GNPINNakSts` reader - Global Non-periodic IN NAK Status"]
pub type GNPINNAK_STS_R = crate::BitReader<GNPINNAK_STS_A>;
#[doc = "Global Non-periodic IN NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GNPINNAK_STS_A {
    #[doc = "0: A handshake is sent out based on the data availability in the transmit FIFO."]
    VALUE1 = 0,
    #[doc = "1: A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data availability in the transmit FIFO."]
    VALUE2 = 1,
}
impl From<GNPINNAK_STS_A> for bool {
    #[inline(always)]
    fn from(variant: GNPINNAK_STS_A) -> Self {
        variant as u8 != 0
    }
}
impl GNPINNAK_STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GNPINNAK_STS_A {
        match self.bits {
            false => GNPINNAK_STS_A::VALUE1,
            true => GNPINNAK_STS_A::VALUE2,
        }
    }
    #[doc = "A handshake is sent out based on the data availability in the transmit FIFO."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GNPINNAK_STS_A::VALUE1
    }
    #[doc = "A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data availability in the transmit FIFO."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GNPINNAK_STS_A::VALUE2
    }
}
#[doc = "Field `GOUTNakSts` reader - Global OUT NAK Status"]
pub type GOUTNAK_STS_R = crate::BitReader<GOUTNAK_STS_A>;
#[doc = "Global OUT NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GOUTNAK_STS_A {
    #[doc = "0: A handshake is sent based on the FIFO Status and the NAK and STALL bit settings."]
    VALUE1 = 0,
    #[doc = "1: No data is written to the RxFIFO, irrespective of space availability. Sends a NAK handshake on all packets, except on SETUP transactions. All isochronous OUT packets are dropped."]
    VALUE2 = 1,
}
impl From<GOUTNAK_STS_A> for bool {
    #[inline(always)]
    fn from(variant: GOUTNAK_STS_A) -> Self {
        variant as u8 != 0
    }
}
impl GOUTNAK_STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GOUTNAK_STS_A {
        match self.bits {
            false => GOUTNAK_STS_A::VALUE1,
            true => GOUTNAK_STS_A::VALUE2,
        }
    }
    #[doc = "A handshake is sent based on the FIFO Status and the NAK and STALL bit settings."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GOUTNAK_STS_A::VALUE1
    }
    #[doc = "No data is written to the RxFIFO, irrespective of space availability. Sends a NAK handshake on all packets, except on SETUP transactions. All isochronous OUT packets are dropped."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GOUTNAK_STS_A::VALUE2
    }
}
#[doc = "Field `SGNPInNak` writer - Set Global Non-periodic IN NAK"]
pub type SGNPIN_NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGNPInNak` writer - Clear Global Non-periodic IN NAK"]
pub type CGNPIN_NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGOUTNak` writer - Set Global OUT NAK"]
pub type SGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGOUTNak` writer - Clear Global OUT NAK"]
pub type CGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMC` reader - Global Multi Count"]
pub type GMC_R = crate::FieldReader<GMC_A>;
#[doc = "Global Multi Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GMC_A {
    #[doc = "0: Invalid."]
    VALUE1 = 0,
    #[doc = "1: 1 packet."]
    VALUE2 = 1,
    #[doc = "2: 2 packets."]
    VALUE3 = 2,
    #[doc = "3: 3 packets."]
    VALUE4 = 3,
}
impl From<GMC_A> for u8 {
    #[inline(always)]
    fn from(variant: GMC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GMC_A {
    type Ux = u8;
}
impl GMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GMC_A {
        match self.bits {
            0 => GMC_A::VALUE1,
            1 => GMC_A::VALUE2,
            2 => GMC_A::VALUE3,
            3 => GMC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Invalid."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GMC_A::VALUE1
    }
    #[doc = "1 packet."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GMC_A::VALUE2
    }
    #[doc = "2 packets."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GMC_A::VALUE3
    }
    #[doc = "3 packets."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GMC_A::VALUE4
    }
}
#[doc = "Field `GMC` writer - Global Multi Count"]
pub type GMC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, GMC_A>;
impl<'a, REG> GMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Invalid."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GMC_A::VALUE1)
    }
    #[doc = "1 packet."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GMC_A::VALUE2)
    }
    #[doc = "2 packets."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(GMC_A::VALUE3)
    }
    #[doc = "3 packets."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(GMC_A::VALUE4)
    }
}
#[doc = "Field `IgnrFrmNum` reader - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
pub type IGNR_FRM_NUM_R = crate::BitReader<IGNR_FRM_NUM_A>;
#[doc = "Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IGNR_FRM_NUM_A {
    #[doc = "0: Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    VALUE1 = 0,
    #[doc = "1: Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    VALUE2 = 1,
}
impl From<IGNR_FRM_NUM_A> for bool {
    #[inline(always)]
    fn from(variant: IGNR_FRM_NUM_A) -> Self {
        variant as u8 != 0
    }
}
impl IGNR_FRM_NUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IGNR_FRM_NUM_A {
        match self.bits {
            false => IGNR_FRM_NUM_A::VALUE1,
            true => IGNR_FRM_NUM_A::VALUE2,
        }
    }
    #[doc = "Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IGNR_FRM_NUM_A::VALUE1
    }
    #[doc = "Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IGNR_FRM_NUM_A::VALUE2
    }
}
#[doc = "Field `IgnrFrmNum` writer - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
pub type IGNR_FRM_NUM_W<'a, REG> = crate::BitWriter<'a, REG, IGNR_FRM_NUM_A>;
impl<'a, REG> IGNR_FRM_NUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scatter/Gather enabled: The core transmits the packets only in the frame number in which they are intended to be transmitted. Scatter/Gather disabled: Periodic transfer interrupt feature is disabled; the application must program transfers for periodic endpoints every frame"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IGNR_FRM_NUM_A::VALUE1)
    }
    #[doc = "Scatter/Gather enabled: The core ignores the frame number, sending packets immediately as the packets are ready. Scatter/Gather disabled: Periodic transfer interrupt feature is enabled; the application can program transfers for multiple frames for periodic endpoints."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IGNR_FRM_NUM_A::VALUE2)
    }
}
#[doc = "Field `NakOnBble` reader - Set NAK automatically on babble"]
pub type NAK_ON_BBLE_R = crate::BitReader;
#[doc = "Field `NakOnBble` writer - Set NAK automatically on babble"]
pub type NAK_ON_BBLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnContOnBNA` reader - Enable continue on BNA"]
pub type EN_CONT_ON_BNA_R = crate::BitReader<EN_CONT_ON_BNA_A>;
#[doc = "Enable continue on BNA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CONT_ON_BNA_A {
    #[doc = "0: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    VALUE1 = 0,
    #[doc = "1: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    VALUE2 = 1,
}
impl From<EN_CONT_ON_BNA_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CONT_ON_BNA_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CONT_ON_BNA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_CONT_ON_BNA_A {
        match self.bits {
            false => EN_CONT_ON_BNA_A::VALUE1,
            true => EN_CONT_ON_BNA_A::VALUE2,
        }
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EN_CONT_ON_BNA_A::VALUE1
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EN_CONT_ON_BNA_A::VALUE2
    }
}
#[doc = "Field `EnContOnBNA` writer - Enable continue on BNA"]
pub type EN_CONT_ON_BNA_W<'a, REG> = crate::BitWriter<'a, REG, EN_CONT_ON_BNA_A>;
impl<'a, REG> EN_CONT_ON_BNA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_CONT_ON_BNA_A::VALUE1)
    }
    #[doc = "After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EN_CONT_ON_BNA_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmt_wk_up_sig(&self) -> RMT_WK_UP_SIG_R {
        RMT_WK_UP_SIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sft_discon(&self) -> SFT_DISCON_R {
        SFT_DISCON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline(always)]
    pub fn gnpinnak_sts(&self) -> GNPINNAK_STS_R {
        GNPINNAK_STS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline(always)]
    pub fn goutnak_sts(&self) -> GOUTNAK_STS_R {
        GOUTNAK_STS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Global Multi Count"]
    #[inline(always)]
    pub fn gmc(&self) -> GMC_R {
        GMC_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
    #[inline(always)]
    pub fn ignr_frm_num(&self) -> IGNR_FRM_NUM_R {
        IGNR_FRM_NUM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set NAK automatically on babble"]
    #[inline(always)]
    pub fn nak_on_bble(&self) -> NAK_ON_BBLE_R {
        NAK_ON_BBLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable continue on BNA"]
    #[inline(always)]
    pub fn en_cont_on_bna(&self) -> EN_CONT_ON_BNA_R {
        EN_CONT_ON_BNA_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_wk_up_sig(&mut self) -> RMT_WK_UP_SIG_W<DCTL_SPEC> {
        RMT_WK_UP_SIG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sft_discon(&mut self) -> SFT_DISCON_W<DCTL_SPEC> {
        SFT_DISCON_W::new(self, 1)
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgnpin_nak(&mut self) -> SGNPIN_NAK_W<DCTL_SPEC> {
        SGNPIN_NAK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgnpin_nak(&mut self) -> CGNPIN_NAK_W<DCTL_SPEC> {
        CGNPIN_NAK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W<DCTL_SPEC> {
        SGOUTNAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W<DCTL_SPEC> {
        CGOUTNAK_W::new(self, 10)
    }
    #[doc = "Bits 13:14 - Global Multi Count"]
    #[inline(always)]
    #[must_use]
    pub fn gmc(&mut self) -> GMC_W<DCTL_SPEC> {
        GMC_W::new(self, 13)
    }
    #[doc = "Bit 15 - Ignore frame number for isochronous endpoints in case of Scatter/Gather DMA"]
    #[inline(always)]
    #[must_use]
    pub fn ignr_frm_num(&mut self) -> IGNR_FRM_NUM_W<DCTL_SPEC> {
        IGNR_FRM_NUM_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set NAK automatically on babble"]
    #[inline(always)]
    #[must_use]
    pub fn nak_on_bble(&mut self) -> NAK_ON_BBLE_W<DCTL_SPEC> {
        NAK_ON_BBLE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable continue on BNA"]
    #[inline(always)]
    #[must_use]
    pub fn en_cont_on_bna(&mut self) -> EN_CONT_ON_BNA_W<DCTL_SPEC> {
        EN_CONT_ON_BNA_W::new(self, 17)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCTL to value 0x02"]
impl crate::Resettable for DCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
