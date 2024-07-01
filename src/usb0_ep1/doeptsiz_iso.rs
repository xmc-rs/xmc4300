#[doc = "Register `DOEPTSIZ_ISO` reader"]
pub type R = crate::R<DOEPTSIZ_ISO_SPEC>;
#[doc = "Register `DOEPTSIZ_ISO` writer"]
pub type W = crate::W<DOEPTSIZ_ISO_SPEC>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Received Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_DPID_A {
    #[doc = "0: DATA0"]
    VALUE1 = 0,
    #[doc = "1: DATA2"]
    VALUE2 = 1,
    #[doc = "2: DATA1"]
    VALUE3 = 2,
    #[doc = "3: MDATA"]
    VALUE4 = 3,
}
impl From<RX_DPID_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_DPID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_DPID_A {
    type Ux = u8;
}
impl crate::IsEnum for RX_DPID_A {}
#[doc = "Field `RxDPID` reader - Received Data PID"]
pub type RX_DPID_R = crate::FieldReader<RX_DPID_A>;
impl RX_DPID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_DPID_A {
        match self.bits {
            0 => RX_DPID_A::VALUE1,
            1 => RX_DPID_A::VALUE2,
            2 => RX_DPID_A::VALUE3,
            3 => RX_DPID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RX_DPID_A::VALUE1
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RX_DPID_A::VALUE2
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RX_DPID_A::VALUE3
    }
    #[doc = "MDATA"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RX_DPID_A::VALUE4
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFER_SIZE_R {
        XFER_SIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKT_CNT_R {
        PKT_CNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Received Data PID"]
    #[inline(always)]
    pub fn rx_dpid(&self) -> RX_DPID_R {
        RX_DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<DOEPTSIZ_ISO_SPEC> {
        XFER_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<DOEPTSIZ_ISO_SPEC> {
        PKT_CNT_W::new(self, 19)
    }
}
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz_iso::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz_iso::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ_ISO_SPEC;
impl crate::RegisterSpec for DOEPTSIZ_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz_iso::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ_ISO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz_iso::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ_ISO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ_ISO to value 0"]
impl crate::Resettable for DOEPTSIZ_ISO_SPEC {
    const RESET_VALUE: u32 = 0;
}
