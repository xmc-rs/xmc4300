#[doc = "Register `HCTSIZ_BUFFERMODE` reader"]
pub type R = crate::R<HctsizBuffermodeSpec>;
#[doc = "Register `HCTSIZ_BUFFERMODE` writer"]
pub type W = crate::W<HctsizBuffermodeSpec>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XferSizeR = crate::FieldReader<u32>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PktCntR = crate::FieldReader<u16>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PktCntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid {
    #[doc = "0: DATA0"]
    Value1 = 0,
    #[doc = "1: DATA2"]
    Value2 = 1,
    #[doc = "2: DATA1"]
    Value3 = 2,
    #[doc = "3: MDATA (non-control)/SETUP (control)"]
    Value4 = 3,
}
impl From<Pid> for u8 {
    #[inline(always)]
    fn from(variant: Pid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pid {
    type Ux = u8;
}
impl crate::IsEnum for Pid {}
#[doc = "Field `Pid` reader - PID"]
pub type PidR = crate::FieldReader<Pid>;
impl PidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pid {
        match self.bits {
            0 => Pid::Value1,
            1 => Pid::Value2,
            2 => Pid::Value3,
            3 => Pid::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pid::Value1
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pid::Value2
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pid::Value3
    }
    #[doc = "MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pid::Value4
    }
}
#[doc = "Field `Pid` writer - PID"]
pub type PidW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pid, crate::Safe>;
impl<'a, REG> PidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Value1)
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Value2)
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Value3)
    }
    #[doc = "MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Value4)
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
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XferSizeW<HctsizBuffermodeSpec> {
        XferSizeW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PktCntW<HctsizBuffermodeSpec> {
        PktCntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PidW<HctsizBuffermodeSpec> {
        PidW::new(self, 29)
    }
}
#[doc = "Host Channel Transfer Size Register \\[BUFFERMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz_buffermode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz_buffermode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HctsizBuffermodeSpec;
impl crate::RegisterSpec for HctsizBuffermodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz_buffermode::R`](R) reader structure"]
impl crate::Readable for HctsizBuffermodeSpec {}
#[doc = "`write(|w| ..)` method takes [`hctsiz_buffermode::W`](W) writer structure"]
impl crate::Writable for HctsizBuffermodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCTSIZ_BUFFERMODE to value 0"]
impl crate::Resettable for HctsizBuffermodeSpec {
    const RESET_VALUE: u32 = 0;
}
