#[doc = "Register `TIMESTAMP_CONTROL` reader"]
pub type R = crate::R<TIMESTAMP_CONTROL_SPEC>;
#[doc = "Register `TIMESTAMP_CONTROL` writer"]
pub type W = crate::W<TIMESTAMP_CONTROL_SPEC>;
#[doc = "Field `TSENA` reader - Timestamp Enable"]
pub type TSENA_R = crate::BitReader;
#[doc = "Field `TSENA` writer - Timestamp Enable"]
pub type TSENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCFUPDT` reader - Timestamp Fine or Coarse Update"]
pub type TSCFUPDT_R = crate::BitReader;
#[doc = "Field `TSCFUPDT` writer - Timestamp Fine or Coarse Update"]
pub type TSCFUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSINIT` reader - Timestamp Initialize"]
pub type TSINIT_R = crate::BitReader;
#[doc = "Field `TSINIT` writer - Timestamp Initialize"]
pub type TSINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUPDT` reader - Timestamp Update"]
pub type TSUPDT_R = crate::BitReader;
#[doc = "Field `TSUPDT` writer - Timestamp Update"]
pub type TSUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTRIG` reader - Timestamp Interrupt Trigger Enable"]
pub type TSTRIG_R = crate::BitReader;
#[doc = "Field `TSTRIG` writer - Timestamp Interrupt Trigger Enable"]
pub type TSTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSADDREG` reader - Addend Reg Update"]
pub type TSADDREG_R = crate::BitReader;
#[doc = "Field `TSADDREG` writer - Addend Reg Update"]
pub type TSADDREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENALL` reader - Enable Timestamp for All Frames"]
pub type TSENALL_R = crate::BitReader;
#[doc = "Field `TSENALL` writer - Enable Timestamp for All Frames"]
pub type TSENALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCTRLSSR` reader - Timestamp Digital or Binary Rollover Control"]
pub type TSCTRLSSR_R = crate::BitReader;
#[doc = "Field `TSCTRLSSR` writer - Timestamp Digital or Binary Rollover Control"]
pub type TSCTRLSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVER2ENA` reader - Enable PTP packet Processing for Version 2 Format"]
pub type TSVER2ENA_R = crate::BitReader;
#[doc = "Field `TSVER2ENA` writer - Enable PTP packet Processing for Version 2 Format"]
pub type TSVER2ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPENA` reader - Enable Processing of PTP over Ethernet Frames"]
pub type TSIPENA_R = crate::BitReader;
#[doc = "Field `TSIPENA` writer - Enable Processing of PTP over Ethernet Frames"]
pub type TSIPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV6ENA` reader - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
pub type TSIPV6ENA_R = crate::BitReader;
#[doc = "Field `TSIPV6ENA` writer - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
pub type TSIPV6ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV4ENA` reader - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub type TSIPV4ENA_R = crate::BitReader;
#[doc = "Field `TSIPV4ENA` writer - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub type TSIPV4ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEVNTENA` reader - Enable Timestamp Snapshot for Event Messages"]
pub type TSEVNTENA_R = crate::BitReader;
#[doc = "Field `TSEVNTENA` writer - Enable Timestamp Snapshot for Event Messages"]
pub type TSEVNTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMSTRENA` reader - Enable Snapshot for Messages Relevant to Master"]
pub type TSMSTRENA_R = crate::BitReader;
#[doc = "Field `TSMSTRENA` writer - Enable Snapshot for Messages Relevant to Master"]
pub type TSMSTRENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAPTYPSEL` reader - Select PTP packets for Taking Snapshots"]
pub type SNAPTYPSEL_R = crate::FieldReader;
#[doc = "Field `SNAPTYPSEL` writer - Select PTP packets for Taking Snapshots"]
pub type SNAPTYPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSENMACADDR` reader - Enable MAC address for PTP Frame Filtering"]
pub type TSENMACADDR_R = crate::BitReader;
#[doc = "Field `TSENMACADDR` writer - Enable MAC address for PTP Frame Filtering"]
pub type TSENMACADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn tstrig(&self) -> TSTRIG_R {
        TSTRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TSADDREG_R {
        TSADDREG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TSENA_W<TIMESTAMP_CONTROL_SPEC> {
        TSENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    #[must_use]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W<TIMESTAMP_CONTROL_SPEC> {
        TSCFUPDT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    #[must_use]
    pub fn tsinit(&mut self) -> TSINIT_W<TIMESTAMP_CONTROL_SPEC> {
        TSINIT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    #[must_use]
    pub fn tsupdt(&mut self) -> TSUPDT_W<TIMESTAMP_CONTROL_SPEC> {
        TSUPDT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstrig(&mut self) -> TSTRIG_W<TIMESTAMP_CONTROL_SPEC> {
        TSTRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    #[must_use]
    pub fn tsaddreg(&mut self) -> TSADDREG_W<TIMESTAMP_CONTROL_SPEC> {
        TSADDREG_W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn tsenall(&mut self) -> TSENALL_W<TIMESTAMP_CONTROL_SPEC> {
        TSENALL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W<TIMESTAMP_CONTROL_SPEC> {
        TSCTRLSSR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    #[must_use]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W<TIMESTAMP_CONTROL_SPEC> {
        TSVER2ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    #[must_use]
    pub fn tsipena(&mut self) -> TSIPENA_W<TIMESTAMP_CONTROL_SPEC> {
        TSIPENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W<TIMESTAMP_CONTROL_SPEC> {
        TSIPV6ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W<TIMESTAMP_CONTROL_SPEC> {
        TSIPV4ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    #[must_use]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W<TIMESTAMP_CONTROL_SPEC> {
        TSEVNTENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    #[must_use]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W<TIMESTAMP_CONTROL_SPEC> {
        TSMSTRENA_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    #[must_use]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W<TIMESTAMP_CONTROL_SPEC> {
        SNAPTYPSEL_W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W<TIMESTAMP_CONTROL_SPEC> {
        TSENMACADDR_W::new(self, 18)
    }
}
#[doc = "Timestamp Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTAMP_CONTROL_SPEC;
impl crate::RegisterSpec for TIMESTAMP_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_control::R`](R) reader structure"]
impl crate::Readable for TIMESTAMP_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timestamp_control::W`](W) writer structure"]
impl crate::Writable for TIMESTAMP_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMESTAMP_CONTROL to value 0x2000"]
impl crate::Resettable for TIMESTAMP_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
