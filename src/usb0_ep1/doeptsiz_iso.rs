#[doc = "Register `DOEPTSIZ_ISO` reader"]
pub type R = crate::R<DoeptsizIsoSpec>;
#[doc = "Register `DOEPTSIZ_ISO` writer"]
pub type W = crate::W<DoeptsizIsoSpec>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XferSizeR = crate::FieldReader<u32>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PktCntR = crate::FieldReader<u16>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PktCntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Received Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxDpid {
    #[doc = "0: DATA0"]
    Value1 = 0,
    #[doc = "1: DATA2"]
    Value2 = 1,
    #[doc = "2: DATA1"]
    Value3 = 2,
    #[doc = "3: MDATA"]
    Value4 = 3,
}
impl From<RxDpid> for u8 {
    #[inline(always)]
    fn from(variant: RxDpid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxDpid {
    type Ux = u8;
}
#[doc = "Field `RxDPID` reader - Received Data PID"]
pub type RxDpidR = crate::FieldReader<RxDpid>;
impl RxDpidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxDpid {
        match self.bits {
            0 => RxDpid::Value1,
            1 => RxDpid::Value2,
            2 => RxDpid::Value3,
            3 => RxDpid::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RxDpid::Value1
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RxDpid::Value2
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RxDpid::Value3
    }
    #[doc = "MDATA"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RxDpid::Value4
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XferSizeR {
        XferSizeR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PktCntR {
        PktCntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Received Data PID"]
    #[inline(always)]
    pub fn rx_dpid(&self) -> RxDpidR {
        RxDpidR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XferSizeW<DoeptsizIsoSpec> {
        XferSizeW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PktCntW<DoeptsizIsoSpec> {
        PktCntW::new(self, 19)
    }
}
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz_iso::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz_iso::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoeptsizIsoSpec;
impl crate::RegisterSpec for DoeptsizIsoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz_iso::R`](R) reader structure"]
impl crate::Readable for DoeptsizIsoSpec {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz_iso::W`](W) writer structure"]
impl crate::Writable for DoeptsizIsoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ_ISO to value 0"]
impl crate::Resettable for DoeptsizIsoSpec {
    const RESET_VALUE: u32 = 0;
}
