#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DCFG_SPEC>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DCFG_SPEC>;
#[doc = "Device Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEV_SPD_A {
    #[doc = "3: Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    VALUE4 = 3,
}
impl From<DEV_SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DEV_SPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DEV_SPD_A {
    type Ux = u8;
}
impl crate::IsEnum for DEV_SPD_A {}
#[doc = "Field `DevSpd` reader - Device Speed"]
pub type DEV_SPD_R = crate::FieldReader<DEV_SPD_A>;
impl DEV_SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DEV_SPD_A> {
        match self.bits {
            3 => Some(DEV_SPD_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DEV_SPD_A::VALUE4
    }
}
#[doc = "Field `DevSpd` writer - Device Speed"]
pub type DEV_SPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DEV_SPD_A>;
impl<'a, REG> DEV_SPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DEV_SPD_A::VALUE4)
    }
}
#[doc = "Non-Zero-Length Status OUT Handshake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NZSTS_OUTHSHK_A {
    #[doc = "1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    VALUE1 = 1,
    #[doc = "0: Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    VALUE2 = 0,
}
impl From<NZSTS_OUTHSHK_A> for bool {
    #[inline(always)]
    fn from(variant: NZSTS_OUTHSHK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NZStsOUTHShk` reader - Non-Zero-Length Status OUT Handshake"]
pub type NZSTS_OUTHSHK_R = crate::BitReader<NZSTS_OUTHSHK_A>;
impl NZSTS_OUTHSHK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NZSTS_OUTHSHK_A {
        match self.bits {
            true => NZSTS_OUTHSHK_A::VALUE1,
            false => NZSTS_OUTHSHK_A::VALUE2,
        }
    }
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NZSTS_OUTHSHK_A::VALUE1
    }
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NZSTS_OUTHSHK_A::VALUE2
    }
}
#[doc = "Field `NZStsOUTHShk` writer - Non-Zero-Length Status OUT Handshake"]
pub type NZSTS_OUTHSHK_W<'a, REG> = crate::BitWriter<'a, REG, NZSTS_OUTHSHK_A>;
impl<'a, REG> NZSTS_OUTHSHK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NZSTS_OUTHSHK_A::VALUE1)
    }
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NZSTS_OUTHSHK_A::VALUE2)
    }
}
#[doc = "Field `DevAddr` reader - Device Address"]
pub type DEV_ADDR_R = crate::FieldReader;
#[doc = "Field `DevAddr` writer - Device Address"]
pub type DEV_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Periodic Frame Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PER_FR_INT_A {
    #[doc = "0: 80% of the frame interval"]
    VALUE1 = 0,
    #[doc = "1: 85%"]
    VALUE2 = 1,
    #[doc = "2: 90%"]
    VALUE3 = 2,
    #[doc = "3: 95%"]
    VALUE4 = 3,
}
impl From<PER_FR_INT_A> for u8 {
    #[inline(always)]
    fn from(variant: PER_FR_INT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PER_FR_INT_A {
    type Ux = u8;
}
impl crate::IsEnum for PER_FR_INT_A {}
#[doc = "Field `PerFrInt` reader - Periodic Frame Interval"]
pub type PER_FR_INT_R = crate::FieldReader<PER_FR_INT_A>;
impl PER_FR_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PER_FR_INT_A {
        match self.bits {
            0 => PER_FR_INT_A::VALUE1,
            1 => PER_FR_INT_A::VALUE2,
            2 => PER_FR_INT_A::VALUE3,
            3 => PER_FR_INT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "80% of the frame interval"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PER_FR_INT_A::VALUE1
    }
    #[doc = "85%"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PER_FR_INT_A::VALUE2
    }
    #[doc = "90%"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PER_FR_INT_A::VALUE3
    }
    #[doc = "95%"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PER_FR_INT_A::VALUE4
    }
}
#[doc = "Field `PerFrInt` writer - Periodic Frame Interval"]
pub type PER_FR_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PER_FR_INT_A, crate::Safe>;
impl<'a, REG> PER_FR_INT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "80% of the frame interval"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PER_FR_INT_A::VALUE1)
    }
    #[doc = "85%"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PER_FR_INT_A::VALUE2)
    }
    #[doc = "90%"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PER_FR_INT_A::VALUE3)
    }
    #[doc = "95%"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PER_FR_INT_A::VALUE4)
    }
}
#[doc = "Field `DescDMA` reader - Enable Scatter/Gather DMA in Device mode."]
pub type DESC_DMA_R = crate::BitReader;
#[doc = "Field `DescDMA` writer - Enable Scatter/Gather DMA in Device mode."]
pub type DESC_DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Periodic Scheduling Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PER_SCH_INTVL_A {
    #[doc = "0: 25% of frame."]
    VALUE1 = 0,
    #[doc = "1: 50% of frame."]
    VALUE2 = 1,
    #[doc = "2: 75% of frame."]
    VALUE3 = 2,
}
impl From<PER_SCH_INTVL_A> for u8 {
    #[inline(always)]
    fn from(variant: PER_SCH_INTVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PER_SCH_INTVL_A {
    type Ux = u8;
}
impl crate::IsEnum for PER_SCH_INTVL_A {}
#[doc = "Field `PerSchIntvl` reader - Periodic Scheduling Interval"]
pub type PER_SCH_INTVL_R = crate::FieldReader<PER_SCH_INTVL_A>;
impl PER_SCH_INTVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PER_SCH_INTVL_A> {
        match self.bits {
            0 => Some(PER_SCH_INTVL_A::VALUE1),
            1 => Some(PER_SCH_INTVL_A::VALUE2),
            2 => Some(PER_SCH_INTVL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "25% of frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PER_SCH_INTVL_A::VALUE1
    }
    #[doc = "50% of frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PER_SCH_INTVL_A::VALUE2
    }
    #[doc = "75% of frame."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PER_SCH_INTVL_A::VALUE3
    }
}
#[doc = "Field `PerSchIntvl` writer - Periodic Scheduling Interval"]
pub type PER_SCH_INTVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PER_SCH_INTVL_A>;
impl<'a, REG> PER_SCH_INTVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "25% of frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PER_SCH_INTVL_A::VALUE1)
    }
    #[doc = "50% of frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PER_SCH_INTVL_A::VALUE2)
    }
    #[doc = "75% of frame."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PER_SCH_INTVL_A::VALUE3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn dev_spd(&self) -> DEV_SPD_R {
        DEV_SPD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzsts_outhshk(&self) -> NZSTS_OUTHSHK_R {
        NZSTS_OUTHSHK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn per_fr_int(&self) -> PER_FR_INT_R {
        PER_FR_INT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    pub fn desc_dma(&self) -> DESC_DMA_R {
        DESC_DMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn per_sch_intvl(&self) -> PER_SCH_INTVL_R {
        PER_SCH_INTVL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn dev_spd(&mut self) -> DEV_SPD_W<DCFG_SPEC> {
        DEV_SPD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzsts_outhshk(&mut self) -> NZSTS_OUTHSHK_W<DCFG_SPEC> {
        NZSTS_OUTHSHK_W::new(self, 2)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W<DCFG_SPEC> {
        DEV_ADDR_W::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn per_fr_int(&mut self) -> PER_FR_INT_W<DCFG_SPEC> {
        PER_FR_INT_W::new(self, 11)
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    pub fn desc_dma(&mut self) -> DESC_DMA_W<DCFG_SPEC> {
        DESC_DMA_W::new(self, 23)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn per_sch_intvl(&mut self) -> PER_SCH_INTVL_W<DCFG_SPEC> {
        PER_SCH_INTVL_W::new(self, 24)
    }
}
#[doc = "Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCFG to value 0x0820_0000"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: u32 = 0x0820_0000;
}
