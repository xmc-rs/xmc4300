#[doc = "Register `CFGH` reader"]
pub type R = crate::R<CFGH_SPEC>;
#[doc = "Register `CFGH` writer"]
pub type W = crate::W<CFGH_SPEC>;
#[doc = "Flow Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCMODE_A {
    #[doc = "0: Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    VALUE1 = 0,
    #[doc = "1: Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    VALUE2 = 1,
}
impl From<FCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FCMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCMODE` reader - Flow Control Mode"]
pub type FCMODE_R = crate::BitReader<FCMODE_A>;
impl FCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCMODE_A {
        match self.bits {
            false => FCMODE_A::VALUE1,
            true => FCMODE_A::VALUE2,
        }
    }
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCMODE_A::VALUE1
    }
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCMODE_A::VALUE2
    }
}
#[doc = "Field `FCMODE` writer - Flow Control Mode"]
pub type FCMODE_W<'a, REG> = crate::BitWriter<'a, REG, FCMODE_A>;
impl<'a, REG> FCMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FCMODE_A::VALUE1)
    }
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FCMODE_A::VALUE2)
    }
}
#[doc = "FIFO Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_MODE_A {
    #[doc = "0: Space/data available for single AHB transfer of the specified transfer width."]
    VALUE1 = 0,
    #[doc = "1: Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    VALUE2 = 1,
}
impl From<FIFO_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_MODE` reader - FIFO Mode Select"]
pub type FIFO_MODE_R = crate::BitReader<FIFO_MODE_A>;
impl FIFO_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_MODE_A {
        match self.bits {
            false => FIFO_MODE_A::VALUE1,
            true => FIFO_MODE_A::VALUE2,
        }
    }
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFO_MODE_A::VALUE1
    }
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFO_MODE_A::VALUE2
    }
}
#[doc = "Field `FIFO_MODE` writer - FIFO Mode Select"]
pub type FIFO_MODE_W<'a, REG> = crate::BitWriter<'a, REG, FIFO_MODE_A>;
impl<'a, REG> FIFO_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_MODE_A::VALUE1)
    }
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_MODE_A::VALUE2)
    }
}
#[doc = "Field `PROTCTL` reader - Protection Control"]
pub type PROTCTL_R = crate::FieldReader;
#[doc = "Field `PROTCTL` writer - Protection Control"]
pub type PROTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DS_UPD_EN` reader - Destination Status Update Enable"]
pub type DS_UPD_EN_R = crate::BitReader;
#[doc = "Field `DS_UPD_EN` writer - Destination Status Update Enable"]
pub type DS_UPD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS_UPD_EN` reader - Source Status Update Enable"]
pub type SS_UPD_EN_R = crate::BitReader;
#[doc = "Field `SS_UPD_EN` writer - Source Status Update Enable"]
pub type SS_UPD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_PER` reader - Source Peripheral"]
pub type SRC_PER_R = crate::FieldReader;
#[doc = "Field `SRC_PER` writer - Source Peripheral"]
pub type SRC_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEST_PER` reader - Destination Peripheral"]
pub type DEST_PER_R = crate::FieldReader;
#[doc = "Field `DEST_PER` writer - Destination Peripheral"]
pub type DEST_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    pub fn fcmode(&self) -> FCMODE_R {
        FCMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FIFO_MODE_R {
        FIFO_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&self) -> PROTCTL_R {
        PROTCTL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Destination Status Update Enable"]
    #[inline(always)]
    pub fn ds_upd_en(&self) -> DS_UPD_EN_R {
        DS_UPD_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Status Update Enable"]
    #[inline(always)]
    pub fn ss_upd_en(&self) -> SS_UPD_EN_R {
        SS_UPD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    pub fn src_per(&self) -> SRC_PER_R {
        SRC_PER_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    pub fn dest_per(&self) -> DEST_PER_R {
        DEST_PER_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    pub fn fcmode(&mut self) -> FCMODE_W<CFGH_SPEC> {
        FCMODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&mut self) -> FIFO_MODE_W<CFGH_SPEC> {
        FIFO_MODE_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&mut self) -> PROTCTL_W<CFGH_SPEC> {
        PROTCTL_W::new(self, 2)
    }
    #[doc = "Bit 5 - Destination Status Update Enable"]
    #[inline(always)]
    pub fn ds_upd_en(&mut self) -> DS_UPD_EN_W<CFGH_SPEC> {
        DS_UPD_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Source Status Update Enable"]
    #[inline(always)]
    pub fn ss_upd_en(&mut self) -> SS_UPD_EN_W<CFGH_SPEC> {
        SS_UPD_EN_W::new(self, 6)
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    pub fn src_per(&mut self) -> SRC_PER_W<CFGH_SPEC> {
        SRC_PER_W::new(self, 7)
    }
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    pub fn dest_per(&mut self) -> DEST_PER_W<CFGH_SPEC> {
        DEST_PER_W::new(self, 11)
    }
}
#[doc = "Configuration Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGH_SPEC;
impl crate::RegisterSpec for CFGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgh::R`](R) reader structure"]
impl crate::Readable for CFGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgh::W`](W) writer structure"]
impl crate::Writable for CFGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGH to value 0x04"]
impl crate::Resettable for CFGH_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
