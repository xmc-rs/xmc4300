#[doc = "Register `HCTSIZ_BUFFERMODE` reader"]
pub type R = crate::R<HCTSIZ_BUFFERMODE_SPEC>;
#[doc = "Register `HCTSIZ_BUFFERMODE` writer"]
pub type W = crate::W<HCTSIZ_BUFFERMODE_SPEC>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PID_A {
    #[doc = "0: DATA0"]
    VALUE1 = 0,
    #[doc = "1: DATA2"]
    VALUE2 = 1,
    #[doc = "2: DATA1"]
    VALUE3 = 2,
    #[doc = "3: MDATA (non-control)/SETUP (control)"]
    VALUE4 = 3,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PID_A {
    type Ux = u8;
}
impl crate::IsEnum for PID_A {}
#[doc = "Field `Pid` reader - PID"]
pub type PID_R = crate::FieldReader<PID_A>;
impl PID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::VALUE1,
            1 => PID_A::VALUE2,
            2 => PID_A::VALUE3,
            3 => PID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PID_A::VALUE1
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PID_A::VALUE2
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PID_A::VALUE3
    }
    #[doc = "MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PID_A::VALUE4
    }
}
#[doc = "Field `Pid` writer - PID"]
pub type PID_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PID_A, crate::Safe>;
impl<'a, REG> PID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::VALUE1)
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::VALUE2)
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::VALUE3)
    }
    #[doc = "MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::VALUE4)
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
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<HCTSIZ_BUFFERMODE_SPEC> {
        XFER_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<HCTSIZ_BUFFERMODE_SPEC> {
        PKT_CNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<HCTSIZ_BUFFERMODE_SPEC> {
        PID_W::new(self, 29)
    }
}
#[doc = "Host Channel Transfer Size Register \\[BUFFERMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz_buffermode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz_buffermode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTSIZ_BUFFERMODE_SPEC;
impl crate::RegisterSpec for HCTSIZ_BUFFERMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz_buffermode::R`](R) reader structure"]
impl crate::Readable for HCTSIZ_BUFFERMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctsiz_buffermode::W`](W) writer structure"]
impl crate::Writable for HCTSIZ_BUFFERMODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCTSIZ_BUFFERMODE to value 0"]
impl crate::Resettable for HCTSIZ_BUFFERMODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
