#[doc = "Register `OPERATION_MODE` reader"]
pub type R = crate::R<OPERATION_MODE_SPEC>;
#[doc = "Register `OPERATION_MODE` writer"]
pub type W = crate::W<OPERATION_MODE_SPEC>;
#[doc = "Field `SR` reader - Start or Stop Receive"]
pub type SR_R = crate::BitReader;
#[doc = "Field `SR` writer - Start or Stop Receive"]
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on Second Frame"]
pub type OSF_R = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on Second Frame"]
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - Receive Threshold Control"]
pub type RTC_R = crate::FieldReader;
#[doc = "Field `RTC` writer - Receive Threshold Control"]
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUF` reader - Forward Undersized Good Frames"]
pub type FUF_R = crate::BitReader;
#[doc = "Field `FUF` writer - Forward Undersized Good Frames"]
pub type FUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEF` reader - Forward Error Frames"]
pub type FEF_R = crate::BitReader;
#[doc = "Field `FEF` writer - Forward Error Frames"]
pub type FEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST` reader - Start or Stop Transmission Command"]
pub type ST_R = crate::BitReader;
#[doc = "Field `ST` writer - Start or Stop Transmission Command"]
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTC` reader - Transmit Threshold Control"]
pub type TTC_R = crate::FieldReader;
#[doc = "Field `TTC` writer - Transmit Threshold Control"]
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FTF` reader - Flush Transmit FIFO"]
pub type FTF_R = crate::BitReader;
#[doc = "Field `FTF` writer - Flush Transmit FIFO"]
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Transmit Store and Forward"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - Transmit Store and Forward"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFF` reader - Disable Flushing of Received Frames"]
pub type DFF_R = crate::BitReader;
#[doc = "Field `DFF` writer - Disable Flushing of Received Frames"]
pub type DFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Receive Store and Forward"]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - Receive Store and Forward"]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Disable Dropping of TCP/IP Checksum Error Frames"]
pub type DT_R = crate::BitReader;
#[doc = "Field `DT` writer - Disable Dropping of TCP/IP Checksum Error Frames"]
pub type DT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    pub fn fuf(&self) -> FUF_R {
        FUF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    pub fn dff(&self) -> DFF_R {
        DFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<OPERATION_MODE_SPEC> {
        SR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<OPERATION_MODE_SPEC> {
        OSF_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<OPERATION_MODE_SPEC> {
        RTC_W::new(self, 3)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    #[must_use]
    pub fn fuf(&mut self) -> FUF_W<OPERATION_MODE_SPEC> {
        FUF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<OPERATION_MODE_SPEC> {
        FEF_W::new(self, 7)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<OPERATION_MODE_SPEC> {
        ST_W::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<OPERATION_MODE_SPEC> {
        TTC_W::new(self, 14)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<OPERATION_MODE_SPEC> {
        FTF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<OPERATION_MODE_SPEC> {
        TSF_W::new(self, 21)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    #[must_use]
    pub fn dff(&mut self) -> DFF_W<OPERATION_MODE_SPEC> {
        DFF_W::new(self, 24)
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<OPERATION_MODE_SPEC> {
        RSF_W::new(self, 25)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<OPERATION_MODE_SPEC> {
        DT_W::new(self, 26)
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
#[doc = "Operation Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`operation_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operation_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPERATION_MODE_SPEC;
impl crate::RegisterSpec for OPERATION_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`operation_mode::R`](R) reader structure"]
impl crate::Readable for OPERATION_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`operation_mode::W`](W) writer structure"]
impl crate::Writable for OPERATION_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPERATION_MODE to value 0"]
impl crate::Resettable for OPERATION_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
