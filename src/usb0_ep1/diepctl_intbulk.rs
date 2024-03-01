#[doc = "Register `DIEPCTL_INTBULK` reader"]
pub type R = crate::R<DiepctlIntbulkSpec>;
#[doc = "Register `DIEPCTL_INTBULK` writer"]
pub type W = crate::W<DiepctlIntbulkSpec>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBActEP` reader - USB Active Endpoint"]
pub type UsbactEpR = crate::BitReader;
#[doc = "Field `USBActEP` writer - USB Active Endpoint"]
pub type UsbactEpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpid {
    #[doc = "0: DATA0"]
    Value1 = 0,
    #[doc = "1: DATA1"]
    Value2 = 1,
}
impl From<Dpid> for bool {
    #[inline(always)]
    fn from(variant: Dpid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPID` reader - Endpoint Data PID"]
pub type DpidR = crate::BitReader<Dpid>;
impl DpidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpid {
        match self.bits {
            false => Dpid::Value1,
            true => Dpid::Value2,
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dpid::Value1
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dpid::Value2
    }
}
#[doc = "NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Naksts {
    #[doc = "0: The core is transmitting non-NAK handshakes based on the FIFO status."]
    Value1 = 0,
    #[doc = "1: The core is transmitting NAK handshakes on this endpoint."]
    Value2 = 1,
}
impl From<Naksts> for bool {
    #[inline(always)]
    fn from(variant: Naksts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAKSts` reader - NAK Status"]
pub type NakstsR = crate::BitReader<Naksts>;
impl NakstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Naksts {
        match self.bits {
            false => Naksts::Value1,
            true => Naksts::Value2,
        }
    }
    #[doc = "The core is transmitting non-NAK handshakes based on the FIFO status."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Naksts::Value1
    }
    #[doc = "The core is transmitting NAK handshakes on this endpoint."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Naksts::Value2
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
#[doc = "Field `Snp` reader - Snoop Mode"]
pub type SnpR = crate::BitReader;
#[doc = "Field `Snp` writer - Snoop Mode"]
pub type SnpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Stall` reader - STALL Handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `Stall` writer - STALL Handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFNum` reader - TxFIFO Number"]
pub type TxFnumR = crate::FieldReader;
#[doc = "Field `TxFNum` writer - TxFIFO Number"]
pub type TxFnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetD0PID` writer - Set DATA0 PID"]
pub type SetD0pidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetD1PID` writer - 29 Set DATA1 PID"]
pub type SetD1pidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDis` reader - Endpoint Disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDis` writer - Endpoint Disable"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEna` reader - Endpoint Enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPEna` writer - Endpoint Enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbact_ep(&self) -> UsbactEpR {
        UsbactEpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NakstsR {
        NakstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&self) -> SnpR {
        SnpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn tx_fnum(&self) -> TxFnumR {
        TxFnumR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EpenaR {
        EpenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MpsW<DiepctlIntbulkSpec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn usbact_ep(&mut self) -> UsbactEpW<DiepctlIntbulkSpec> {
        UsbactEpW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EptypeW<DiepctlIntbulkSpec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SnpW<DiepctlIntbulkSpec> {
        SnpW::new(self, 20)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<DiepctlIntbulkSpec> {
        StallW::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fnum(&mut self) -> TxFnumW<DiepctlIntbulkSpec> {
        TxFnumW::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CnakW<DiepctlIntbulkSpec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SnakW<DiepctlIntbulkSpec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - Set DATA0 PID"]
    #[inline(always)]
    #[must_use]
    pub fn set_d0pid(&mut self) -> SetD0pidW<DiepctlIntbulkSpec> {
        SetD0pidW::new(self, 28)
    }
    #[doc = "Bit 29 - 29 Set DATA1 PID"]
    #[inline(always)]
    #[must_use]
    pub fn set_d1pid(&mut self) -> SetD1pidW<DiepctlIntbulkSpec> {
        SetD1pidW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EpdisW<DiepctlIntbulkSpec> {
        EpdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EpenaW<DiepctlIntbulkSpec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "Device Endpoint Control Register \\[INTBULK\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl_intbulk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl_intbulk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepctlIntbulkSpec;
impl crate::RegisterSpec for DiepctlIntbulkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl_intbulk::R`](R) reader structure"]
impl crate::Readable for DiepctlIntbulkSpec {}
#[doc = "`write(|w| ..)` method takes [`diepctl_intbulk::W`](W) writer structure"]
impl crate::Writable for DiepctlIntbulkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPCTL_INTBULK to value 0"]
impl crate::Resettable for DiepctlIntbulkSpec {
    const RESET_VALUE: u32 = 0;
}
