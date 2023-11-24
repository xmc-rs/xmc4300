#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mo0: MO,
    mo1: MO,
    mo2: MO,
    mo3: MO,
    mo4: MO,
    mo5: MO,
    mo6: MO,
    mo7: MO,
    mo8: MO,
    mo9: MO,
    mo10: MO,
    mo11: MO,
    mo12: MO,
    mo13: MO,
    mo14: MO,
    mo15: MO,
    mo16: MO,
    mo17: MO,
    mo18: MO,
    mo19: MO,
    mo20: MO,
    mo21: MO,
    mo22: MO,
    mo23: MO,
    mo24: MO,
    mo25: MO,
    mo26: MO,
    mo27: MO,
    mo28: MO,
    mo29: MO,
    mo30: MO,
    mo31: MO,
    mo32: MO,
    mo33: MO,
    mo34: MO,
    mo35: MO,
    mo36: MO,
    mo37: MO,
    mo38: MO,
    mo39: MO,
    mo40: MO,
    mo41: MO,
    mo42: MO,
    mo43: MO,
    mo44: MO,
    mo45: MO,
    mo46: MO,
    mo47: MO,
    mo48: MO,
    mo49: MO,
    mo50: MO,
    mo51: MO,
    mo52: MO,
    mo53: MO,
    mo54: MO,
    mo55: MO,
    mo56: MO,
    mo57: MO,
    mo58: MO,
    mo59: MO,
    mo60: MO,
    mo61: MO,
    mo62: MO,
    mo63: MO,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo0(&self) -> &MO {
        &self.mo0
    }
    #[doc = "0x20..0x40 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo1(&self) -> &MO {
        &self.mo1
    }
    #[doc = "0x40..0x60 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo2(&self) -> &MO {
        &self.mo2
    }
    #[doc = "0x60..0x80 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo3(&self) -> &MO {
        &self.mo3
    }
    #[doc = "0x80..0xa0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo4(&self) -> &MO {
        &self.mo4
    }
    #[doc = "0xa0..0xc0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo5(&self) -> &MO {
        &self.mo5
    }
    #[doc = "0xc0..0xe0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo6(&self) -> &MO {
        &self.mo6
    }
    #[doc = "0xe0..0x100 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo7(&self) -> &MO {
        &self.mo7
    }
    #[doc = "0x100..0x120 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo8(&self) -> &MO {
        &self.mo8
    }
    #[doc = "0x120..0x140 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo9(&self) -> &MO {
        &self.mo9
    }
    #[doc = "0x140..0x160 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo10(&self) -> &MO {
        &self.mo10
    }
    #[doc = "0x160..0x180 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo11(&self) -> &MO {
        &self.mo11
    }
    #[doc = "0x180..0x1a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo12(&self) -> &MO {
        &self.mo12
    }
    #[doc = "0x1a0..0x1c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo13(&self) -> &MO {
        &self.mo13
    }
    #[doc = "0x1c0..0x1e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo14(&self) -> &MO {
        &self.mo14
    }
    #[doc = "0x1e0..0x200 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo15(&self) -> &MO {
        &self.mo15
    }
    #[doc = "0x200..0x220 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo16(&self) -> &MO {
        &self.mo16
    }
    #[doc = "0x220..0x240 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo17(&self) -> &MO {
        &self.mo17
    }
    #[doc = "0x240..0x260 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo18(&self) -> &MO {
        &self.mo18
    }
    #[doc = "0x260..0x280 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo19(&self) -> &MO {
        &self.mo19
    }
    #[doc = "0x280..0x2a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo20(&self) -> &MO {
        &self.mo20
    }
    #[doc = "0x2a0..0x2c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo21(&self) -> &MO {
        &self.mo21
    }
    #[doc = "0x2c0..0x2e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo22(&self) -> &MO {
        &self.mo22
    }
    #[doc = "0x2e0..0x300 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo23(&self) -> &MO {
        &self.mo23
    }
    #[doc = "0x300..0x320 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo24(&self) -> &MO {
        &self.mo24
    }
    #[doc = "0x320..0x340 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo25(&self) -> &MO {
        &self.mo25
    }
    #[doc = "0x340..0x360 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo26(&self) -> &MO {
        &self.mo26
    }
    #[doc = "0x360..0x380 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo27(&self) -> &MO {
        &self.mo27
    }
    #[doc = "0x380..0x3a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo28(&self) -> &MO {
        &self.mo28
    }
    #[doc = "0x3a0..0x3c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo29(&self) -> &MO {
        &self.mo29
    }
    #[doc = "0x3c0..0x3e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo30(&self) -> &MO {
        &self.mo30
    }
    #[doc = "0x3e0..0x400 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo31(&self) -> &MO {
        &self.mo31
    }
    #[doc = "0x400..0x420 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo32(&self) -> &MO {
        &self.mo32
    }
    #[doc = "0x420..0x440 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo33(&self) -> &MO {
        &self.mo33
    }
    #[doc = "0x440..0x460 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo34(&self) -> &MO {
        &self.mo34
    }
    #[doc = "0x460..0x480 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo35(&self) -> &MO {
        &self.mo35
    }
    #[doc = "0x480..0x4a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo36(&self) -> &MO {
        &self.mo36
    }
    #[doc = "0x4a0..0x4c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo37(&self) -> &MO {
        &self.mo37
    }
    #[doc = "0x4c0..0x4e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo38(&self) -> &MO {
        &self.mo38
    }
    #[doc = "0x4e0..0x500 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo39(&self) -> &MO {
        &self.mo39
    }
    #[doc = "0x500..0x520 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo40(&self) -> &MO {
        &self.mo40
    }
    #[doc = "0x520..0x540 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo41(&self) -> &MO {
        &self.mo41
    }
    #[doc = "0x540..0x560 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo42(&self) -> &MO {
        &self.mo42
    }
    #[doc = "0x560..0x580 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo43(&self) -> &MO {
        &self.mo43
    }
    #[doc = "0x580..0x5a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo44(&self) -> &MO {
        &self.mo44
    }
    #[doc = "0x5a0..0x5c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo45(&self) -> &MO {
        &self.mo45
    }
    #[doc = "0x5c0..0x5e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo46(&self) -> &MO {
        &self.mo46
    }
    #[doc = "0x5e0..0x600 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo47(&self) -> &MO {
        &self.mo47
    }
    #[doc = "0x600..0x620 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo48(&self) -> &MO {
        &self.mo48
    }
    #[doc = "0x620..0x640 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo49(&self) -> &MO {
        &self.mo49
    }
    #[doc = "0x640..0x660 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo50(&self) -> &MO {
        &self.mo50
    }
    #[doc = "0x660..0x680 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo51(&self) -> &MO {
        &self.mo51
    }
    #[doc = "0x680..0x6a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo52(&self) -> &MO {
        &self.mo52
    }
    #[doc = "0x6a0..0x6c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo53(&self) -> &MO {
        &self.mo53
    }
    #[doc = "0x6c0..0x6e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo54(&self) -> &MO {
        &self.mo54
    }
    #[doc = "0x6e0..0x700 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo55(&self) -> &MO {
        &self.mo55
    }
    #[doc = "0x700..0x720 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo56(&self) -> &MO {
        &self.mo56
    }
    #[doc = "0x720..0x740 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo57(&self) -> &MO {
        &self.mo57
    }
    #[doc = "0x740..0x760 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo58(&self) -> &MO {
        &self.mo58
    }
    #[doc = "0x760..0x780 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo59(&self) -> &MO {
        &self.mo59
    }
    #[doc = "0x780..0x7a0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo60(&self) -> &MO {
        &self.mo60
    }
    #[doc = "0x7a0..0x7c0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo61(&self) -> &MO {
        &self.mo61
    }
    #[doc = "0x7c0..0x7e0 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo62(&self) -> &MO {
        &self.mo62
    }
    #[doc = "0x7e0..0x800 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo63(&self) -> &MO {
        &self.mo63
    }
}
#[doc = "Message Object Registers"]
pub use self::mo::MO;
#[doc = r"Cluster"]
#[doc = "Message Object Registers"]
pub mod mo;
