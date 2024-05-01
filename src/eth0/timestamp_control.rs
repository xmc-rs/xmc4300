#[doc = "Register `TIMESTAMP_CONTROL` reader"]
pub type R = crate::R<TimestampControlSpec>;
#[doc = "Register `TIMESTAMP_CONTROL` writer"]
pub type W = crate::W<TimestampControlSpec>;
#[doc = "Field `TSENA` reader - Timestamp Enable"]
pub type TsenaR = crate::BitReader;
#[doc = "Field `TSENA` writer - Timestamp Enable"]
pub type TsenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCFUPDT` reader - Timestamp Fine or Coarse Update"]
pub type TscfupdtR = crate::BitReader;
#[doc = "Field `TSCFUPDT` writer - Timestamp Fine or Coarse Update"]
pub type TscfupdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSINIT` reader - Timestamp Initialize"]
pub type TsinitR = crate::BitReader;
#[doc = "Field `TSINIT` writer - Timestamp Initialize"]
pub type TsinitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUPDT` reader - Timestamp Update"]
pub type TsupdtR = crate::BitReader;
#[doc = "Field `TSUPDT` writer - Timestamp Update"]
pub type TsupdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTRIG` reader - Timestamp Interrupt Trigger Enable"]
pub type TstrigR = crate::BitReader;
#[doc = "Field `TSTRIG` writer - Timestamp Interrupt Trigger Enable"]
pub type TstrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSADDREG` reader - Addend Reg Update"]
pub type TsaddregR = crate::BitReader;
#[doc = "Field `TSADDREG` writer - Addend Reg Update"]
pub type TsaddregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENALL` reader - Enable Timestamp for All Frames"]
pub type TsenallR = crate::BitReader;
#[doc = "Field `TSENALL` writer - Enable Timestamp for All Frames"]
pub type TsenallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCTRLSSR` reader - Timestamp Digital or Binary Rollover Control"]
pub type TsctrlssrR = crate::BitReader;
#[doc = "Field `TSCTRLSSR` writer - Timestamp Digital or Binary Rollover Control"]
pub type TsctrlssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVER2ENA` reader - Enable PTP packet Processing for Version 2 Format"]
pub type Tsver2enaR = crate::BitReader;
#[doc = "Field `TSVER2ENA` writer - Enable PTP packet Processing for Version 2 Format"]
pub type Tsver2enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPENA` reader - Enable Processing of PTP over Ethernet Frames"]
pub type TsipenaR = crate::BitReader;
#[doc = "Field `TSIPENA` writer - Enable Processing of PTP over Ethernet Frames"]
pub type TsipenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV6ENA` reader - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
pub type Tsipv6enaR = crate::BitReader;
#[doc = "Field `TSIPV6ENA` writer - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
pub type Tsipv6enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV4ENA` reader - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub type Tsipv4enaR = crate::BitReader;
#[doc = "Field `TSIPV4ENA` writer - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub type Tsipv4enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEVNTENA` reader - Enable Timestamp Snapshot for Event Messages"]
pub type TsevntenaR = crate::BitReader;
#[doc = "Field `TSEVNTENA` writer - Enable Timestamp Snapshot for Event Messages"]
pub type TsevntenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMSTRENA` reader - Enable Snapshot for Messages Relevant to Master"]
pub type TsmstrenaR = crate::BitReader;
#[doc = "Field `TSMSTRENA` writer - Enable Snapshot for Messages Relevant to Master"]
pub type TsmstrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAPTYPSEL` reader - Select PTP packets for Taking Snapshots"]
pub type SnaptypselR = crate::FieldReader;
#[doc = "Field `SNAPTYPSEL` writer - Select PTP packets for Taking Snapshots"]
pub type SnaptypselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSENMACADDR` reader - Enable MAC address for PTP Frame Filtering"]
pub type TsenmacaddrR = crate::BitReader;
#[doc = "Field `TSENMACADDR` writer - Enable MAC address for PTP Frame Filtering"]
pub type TsenmacaddrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsena(&self) -> TsenaR {
        TsenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TscfupdtR {
        TscfupdtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&self) -> TsinitR {
        TsinitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TsupdtR {
        TsupdtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn tstrig(&self) -> TstrigR {
        TstrigR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TsaddregR {
        TsaddregR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    pub fn tsenall(&self) -> TsenallR {
        TsenallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TsctrlssrR {
        TsctrlssrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> Tsver2enaR {
        Tsver2enaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    pub fn tsipena(&self) -> TsipenaR {
        TsipenaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> Tsipv6enaR {
        Tsipv6enaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> Tsipv4enaR {
        Tsipv4enaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TsevntenaR {
        TsevntenaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TsmstrenaR {
        TsmstrenaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SnaptypselR {
        SnaptypselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TsenmacaddrR {
        TsenmacaddrR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TsenaW<TimestampControlSpec> {
        TsenaW::new(self, 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    #[must_use]
    pub fn tscfupdt(&mut self) -> TscfupdtW<TimestampControlSpec> {
        TscfupdtW::new(self, 1)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    #[must_use]
    pub fn tsinit(&mut self) -> TsinitW<TimestampControlSpec> {
        TsinitW::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    #[must_use]
    pub fn tsupdt(&mut self) -> TsupdtW<TimestampControlSpec> {
        TsupdtW::new(self, 3)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstrig(&mut self) -> TstrigW<TimestampControlSpec> {
        TstrigW::new(self, 4)
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    #[must_use]
    pub fn tsaddreg(&mut self) -> TsaddregW<TimestampControlSpec> {
        TsaddregW::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn tsenall(&mut self) -> TsenallW<TimestampControlSpec> {
        TsenallW::new(self, 8)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrlssr(&mut self) -> TsctrlssrW<TimestampControlSpec> {
        TsctrlssrW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    #[must_use]
    pub fn tsver2ena(&mut self) -> Tsver2enaW<TimestampControlSpec> {
        Tsver2enaW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    #[must_use]
    pub fn tsipena(&mut self) -> TsipenaW<TimestampControlSpec> {
        TsipenaW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv6ena(&mut self) -> Tsipv6enaW<TimestampControlSpec> {
        Tsipv6enaW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv4ena(&mut self) -> Tsipv4enaW<TimestampControlSpec> {
        Tsipv4enaW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    #[must_use]
    pub fn tsevntena(&mut self) -> TsevntenaW<TimestampControlSpec> {
        TsevntenaW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    #[must_use]
    pub fn tsmstrena(&mut self) -> TsmstrenaW<TimestampControlSpec> {
        TsmstrenaW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    #[must_use]
    pub fn snaptypsel(&mut self) -> SnaptypselW<TimestampControlSpec> {
        SnaptypselW::new(self, 16)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn tsenmacaddr(&mut self) -> TsenmacaddrW<TimestampControlSpec> {
        TsenmacaddrW::new(self, 18)
    }
}
#[doc = "Timestamp Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampControlSpec;
impl crate::RegisterSpec for TimestampControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_control::R`](R) reader structure"]
impl crate::Readable for TimestampControlSpec {}
#[doc = "`write(|w| ..)` method takes [`timestamp_control::W`](W) writer structure"]
impl crate::Writable for TimestampControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMESTAMP_CONTROL to value 0x2000"]
impl crate::Resettable for TimestampControlSpec {
    const RESET_VALUE: u32 = 0x2000;
}
