#[doc = "Register `DOEPTSIZ0` reader"]
pub type R = crate::R<Doeptsiz0Spec>;
#[doc = "Register `DOEPTSIZ0` writer"]
pub type W = crate::W<Doeptsiz0Spec>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XferSizeR = crate::FieldReader;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PktCntR = crate::FieldReader;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PktCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "SETUP Packet Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Supcnt {
    #[doc = "1: 1 packet"]
    Value1 = 1,
    #[doc = "2: 2 packets"]
    Value2 = 2,
    #[doc = "3: 3 packets"]
    Value3 = 3,
}
impl From<Supcnt> for u8 {
    #[inline(always)]
    fn from(variant: Supcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Supcnt {
    type Ux = u8;
}
impl crate::IsEnum for Supcnt {}
#[doc = "Field `SUPCnt` reader - SETUP Packet Count"]
pub type SupcntR = crate::FieldReader<Supcnt>;
impl SupcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Supcnt> {
        match self.bits {
            1 => Some(Supcnt::Value1),
            2 => Some(Supcnt::Value2),
            3 => Some(Supcnt::Value3),
            _ => None,
        }
    }
    #[doc = "1 packet"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Supcnt::Value1
    }
    #[doc = "2 packets"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Supcnt::Value2
    }
    #[doc = "3 packets"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Supcnt::Value3
    }
}
#[doc = "Field `SUPCnt` writer - SETUP Packet Count"]
pub type SupcntW<'a, REG> = crate::FieldWriter<'a, REG, 2, Supcnt>;
impl<'a, REG> SupcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 packet"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Supcnt::Value1)
    }
    #[doc = "2 packets"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Supcnt::Value2)
    }
    #[doc = "3 packets"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Supcnt::Value3)
    }
}
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XferSizeR {
        XferSizeR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PktCntR {
        PktCntR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&self) -> SupcntR {
        SupcntR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XferSizeW<Doeptsiz0Spec> {
        XferSizeW::new(self, 0)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PktCntW<Doeptsiz0Spec> {
        PktCntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt(&mut self) -> SupcntW<Doeptsiz0Spec> {
        SupcntW::new(self, 29)
    }
}
#[doc = "Device OUT Endpoint Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doeptsiz0Spec;
impl crate::RegisterSpec for Doeptsiz0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz0::R`](R) reader structure"]
impl crate::Readable for Doeptsiz0Spec {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz0::W`](W) writer structure"]
impl crate::Writable for Doeptsiz0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ0 to value 0"]
impl crate::Resettable for Doeptsiz0Spec {
    const RESET_VALUE: u32 = 0;
}
