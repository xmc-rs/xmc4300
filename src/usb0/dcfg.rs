#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DcfgSpec>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DcfgSpec>;
#[doc = "Device Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DevSpd {
    #[doc = "3: Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    Value4 = 3,
}
impl From<DevSpd> for u8 {
    #[inline(always)]
    fn from(variant: DevSpd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DevSpd {
    type Ux = u8;
}
impl crate::IsEnum for DevSpd {}
#[doc = "Field `DevSpd` reader - Device Speed"]
pub type DevSpdR = crate::FieldReader<DevSpd>;
impl DevSpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DevSpd> {
        match self.bits {
            3 => Some(DevSpd::Value4),
            _ => None,
        }
    }
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DevSpd::Value4
    }
}
#[doc = "Field `DevSpd` writer - Device Speed"]
pub type DevSpdW<'a, REG> = crate::FieldWriter<'a, REG, 2, DevSpd>;
impl<'a, REG> DevSpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DevSpd::Value4)
    }
}
#[doc = "Non-Zero-Length Status OUT Handshake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NzstsOuthshk {
    #[doc = "1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    Value1 = 1,
    #[doc = "0: Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    Value2 = 0,
}
impl From<NzstsOuthshk> for bool {
    #[inline(always)]
    fn from(variant: NzstsOuthshk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NZStsOUTHShk` reader - Non-Zero-Length Status OUT Handshake"]
pub type NzstsOuthshkR = crate::BitReader<NzstsOuthshk>;
impl NzstsOuthshkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NzstsOuthshk {
        match self.bits {
            true => NzstsOuthshk::Value1,
            false => NzstsOuthshk::Value2,
        }
    }
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NzstsOuthshk::Value1
    }
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NzstsOuthshk::Value2
    }
}
#[doc = "Field `NZStsOUTHShk` writer - Non-Zero-Length Status OUT Handshake"]
pub type NzstsOuthshkW<'a, REG> = crate::BitWriter<'a, REG, NzstsOuthshk>;
impl<'a, REG> NzstsOuthshkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NzstsOuthshk::Value1)
    }
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NzstsOuthshk::Value2)
    }
}
#[doc = "Field `DevAddr` reader - Device Address"]
pub type DevAddrR = crate::FieldReader;
#[doc = "Field `DevAddr` writer - Device Address"]
pub type DevAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Periodic Frame Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PerFrInt {
    #[doc = "0: 80% of the frame interval"]
    Value1 = 0,
    #[doc = "1: 85%"]
    Value2 = 1,
    #[doc = "2: 90%"]
    Value3 = 2,
    #[doc = "3: 95%"]
    Value4 = 3,
}
impl From<PerFrInt> for u8 {
    #[inline(always)]
    fn from(variant: PerFrInt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PerFrInt {
    type Ux = u8;
}
impl crate::IsEnum for PerFrInt {}
#[doc = "Field `PerFrInt` reader - Periodic Frame Interval"]
pub type PerFrIntR = crate::FieldReader<PerFrInt>;
impl PerFrIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerFrInt {
        match self.bits {
            0 => PerFrInt::Value1,
            1 => PerFrInt::Value2,
            2 => PerFrInt::Value3,
            3 => PerFrInt::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "80% of the frame interval"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PerFrInt::Value1
    }
    #[doc = "85%"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PerFrInt::Value2
    }
    #[doc = "90%"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PerFrInt::Value3
    }
    #[doc = "95%"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PerFrInt::Value4
    }
}
#[doc = "Field `PerFrInt` writer - Periodic Frame Interval"]
pub type PerFrIntW<'a, REG> = crate::FieldWriter<'a, REG, 2, PerFrInt, crate::Safe>;
impl<'a, REG> PerFrIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "80% of the frame interval"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PerFrInt::Value1)
    }
    #[doc = "85%"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PerFrInt::Value2)
    }
    #[doc = "90%"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PerFrInt::Value3)
    }
    #[doc = "95%"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PerFrInt::Value4)
    }
}
#[doc = "Field `DescDMA` reader - Enable Scatter/Gather DMA in Device mode."]
pub type DescDmaR = crate::BitReader;
#[doc = "Field `DescDMA` writer - Enable Scatter/Gather DMA in Device mode."]
pub type DescDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Periodic Scheduling Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PerSchIntvl {
    #[doc = "0: 25% of frame."]
    Value1 = 0,
    #[doc = "1: 50% of frame."]
    Value2 = 1,
    #[doc = "2: 75% of frame."]
    Value3 = 2,
}
impl From<PerSchIntvl> for u8 {
    #[inline(always)]
    fn from(variant: PerSchIntvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PerSchIntvl {
    type Ux = u8;
}
impl crate::IsEnum for PerSchIntvl {}
#[doc = "Field `PerSchIntvl` reader - Periodic Scheduling Interval"]
pub type PerSchIntvlR = crate::FieldReader<PerSchIntvl>;
impl PerSchIntvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PerSchIntvl> {
        match self.bits {
            0 => Some(PerSchIntvl::Value1),
            1 => Some(PerSchIntvl::Value2),
            2 => Some(PerSchIntvl::Value3),
            _ => None,
        }
    }
    #[doc = "25% of frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PerSchIntvl::Value1
    }
    #[doc = "50% of frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PerSchIntvl::Value2
    }
    #[doc = "75% of frame."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PerSchIntvl::Value3
    }
}
#[doc = "Field `PerSchIntvl` writer - Periodic Scheduling Interval"]
pub type PerSchIntvlW<'a, REG> = crate::FieldWriter<'a, REG, 2, PerSchIntvl>;
impl<'a, REG> PerSchIntvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "25% of frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PerSchIntvl::Value1)
    }
    #[doc = "50% of frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PerSchIntvl::Value2)
    }
    #[doc = "75% of frame."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PerSchIntvl::Value3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn dev_spd(&self) -> DevSpdR {
        DevSpdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzsts_outhshk(&self) -> NzstsOuthshkR {
        NzstsOuthshkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DevAddrR {
        DevAddrR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn per_fr_int(&self) -> PerFrIntR {
        PerFrIntR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    pub fn desc_dma(&self) -> DescDmaR {
        DescDmaR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn per_sch_intvl(&self) -> PerSchIntvlR {
        PerSchIntvlR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    #[must_use]
    pub fn dev_spd(&mut self) -> DevSpdW<DcfgSpec> {
        DevSpdW::new(self, 0)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn nzsts_outhshk(&mut self) -> NzstsOuthshkW<DcfgSpec> {
        NzstsOuthshkW::new(self, 2)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn dev_addr(&mut self) -> DevAddrW<DcfgSpec> {
        DevAddrW::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    #[must_use]
    pub fn per_fr_int(&mut self) -> PerFrIntW<DcfgSpec> {
        PerFrIntW::new(self, 11)
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    #[must_use]
    pub fn desc_dma(&mut self) -> DescDmaW<DcfgSpec> {
        DescDmaW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    #[must_use]
    pub fn per_sch_intvl(&mut self) -> PerSchIntvlW<DcfgSpec> {
        PerSchIntvlW::new(self, 24)
    }
}
#[doc = "Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfgSpec;
impl crate::RegisterSpec for DcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCFG to value 0x0820_0000"]
impl crate::Resettable for DcfgSpec {
    const RESET_VALUE: u32 = 0x0820_0000;
}
