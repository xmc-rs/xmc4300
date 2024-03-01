#[doc = "Register `PEEN` reader"]
pub type R = crate::R<PeenSpec>;
#[doc = "Register `PEEN` writer"]
pub type W = crate::W<PeenSpec>;
#[doc = "Parity Error Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peenps {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peenps> for bool {
    #[inline(always)]
    fn from(variant: Peenps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENPS` reader - Parity Error Enable for PSRAM"]
pub type PeenpsR = crate::BitReader<Peenps>;
impl PeenpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peenps {
        match self.bits {
            false => Peenps::Const0,
            true => Peenps::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peenps::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peenps::Const1
    }
}
#[doc = "Field `PEENPS` writer - Parity Error Enable for PSRAM"]
pub type PeenpsW<'a, REG> = crate::BitWriter<'a, REG, Peenps>;
impl<'a, REG> PeenpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peenps::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peenps::Const1)
    }
}
#[doc = "Parity Error Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peends1 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peends1> for bool {
    #[inline(always)]
    fn from(variant: Peends1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENDS1` reader - Parity Error Enable for DSRAM1"]
pub type Peends1R = crate::BitReader<Peends1>;
impl Peends1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peends1 {
        match self.bits {
            false => Peends1::Const0,
            true => Peends1::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peends1::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peends1::Const1
    }
}
#[doc = "Field `PEENDS1` writer - Parity Error Enable for DSRAM1"]
pub type Peends1W<'a, REG> = crate::BitWriter<'a, REG, Peends1>;
impl<'a, REG> Peends1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peends1::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peends1::Const1)
    }
}
#[doc = "Parity Error Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peenu0 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peenu0> for bool {
    #[inline(always)]
    fn from(variant: Peenu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENU0` reader - Parity Error Enable for USIC0 Memory"]
pub type Peenu0R = crate::BitReader<Peenu0>;
impl Peenu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peenu0 {
        match self.bits {
            false => Peenu0::Const0,
            true => Peenu0::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peenu0::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peenu0::Const1
    }
}
#[doc = "Field `PEENU0` writer - Parity Error Enable for USIC0 Memory"]
pub type Peenu0W<'a, REG> = crate::BitWriter<'a, REG, Peenu0>;
impl<'a, REG> Peenu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peenu0::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peenu0::Const1)
    }
}
#[doc = "Parity Error Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peenu1 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peenu1> for bool {
    #[inline(always)]
    fn from(variant: Peenu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENU1` reader - Parity Error Enable for USIC1 Memory"]
pub type Peenu1R = crate::BitReader<Peenu1>;
impl Peenu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peenu1 {
        match self.bits {
            false => Peenu1::Const0,
            true => Peenu1::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peenu1::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peenu1::Const1
    }
}
#[doc = "Field `PEENU1` writer - Parity Error Enable for USIC1 Memory"]
pub type Peenu1W<'a, REG> = crate::BitWriter<'a, REG, Peenu1>;
impl<'a, REG> Peenu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peenu1::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peenu1::Const1)
    }
}
#[doc = "Parity Error Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peenmc {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peenmc> for bool {
    #[inline(always)]
    fn from(variant: Peenmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENMC` reader - Parity Error Enable for MultiCAN Memory"]
pub type PeenmcR = crate::BitReader<Peenmc>;
impl PeenmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peenmc {
        match self.bits {
            false => Peenmc::Const0,
            true => Peenmc::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peenmc::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peenmc::Const1
    }
}
#[doc = "Field `PEENMC` writer - Parity Error Enable for MultiCAN Memory"]
pub type PeenmcW<'a, REG> = crate::BitWriter<'a, REG, Peenmc>;
impl<'a, REG> PeenmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peenmc::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peenmc::Const1)
    }
}
#[doc = "Parity Error Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peenpprf {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peenpprf> for bool {
    #[inline(always)]
    fn from(variant: Peenpprf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENPPRF` reader - Parity Error Enable for PMU Prefetch Memory"]
pub type PeenpprfR = crate::BitReader<Peenpprf>;
impl PeenpprfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peenpprf {
        match self.bits {
            false => Peenpprf::Const0,
            true => Peenpprf::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peenpprf::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peenpprf::Const1
    }
}
#[doc = "Field `PEENPPRF` writer - Parity Error Enable for PMU Prefetch Memory"]
pub type PeenpprfW<'a, REG> = crate::BitWriter<'a, REG, Peenpprf>;
impl<'a, REG> PeenpprfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peenpprf::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peenpprf::Const1)
    }
}
#[doc = "Parity Error Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peenusb {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peenusb> for bool {
    #[inline(always)]
    fn from(variant: Peenusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENUSB` reader - Parity Error Enable for USB Memory"]
pub type PeenusbR = crate::BitReader<Peenusb>;
impl PeenusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peenusb {
        match self.bits {
            false => Peenusb::Const0,
            true => Peenusb::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peenusb::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peenusb::Const1
    }
}
#[doc = "Field `PEENUSB` writer - Parity Error Enable for USB Memory"]
pub type PeenusbW<'a, REG> = crate::BitWriter<'a, REG, Peenusb>;
impl<'a, REG> PeenusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peenusb::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peenusb::Const1)
    }
}
#[doc = "Parity Error Enable for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peeneth0tx {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peeneth0tx> for bool {
    #[inline(always)]
    fn from(variant: Peeneth0tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENETH0TX` reader - Parity Error Enable for ETH TX Memory"]
pub type Peeneth0txR = crate::BitReader<Peeneth0tx>;
impl Peeneth0txR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peeneth0tx {
        match self.bits {
            false => Peeneth0tx::Const0,
            true => Peeneth0tx::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peeneth0tx::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peeneth0tx::Const1
    }
}
#[doc = "Field `PEENETH0TX` writer - Parity Error Enable for ETH TX Memory"]
pub type Peeneth0txW<'a, REG> = crate::BitWriter<'a, REG, Peeneth0tx>;
impl<'a, REG> Peeneth0txW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peeneth0tx::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peeneth0tx::Const1)
    }
}
#[doc = "Parity Error Enable for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peeneth0rx {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peeneth0rx> for bool {
    #[inline(always)]
    fn from(variant: Peeneth0rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENETH0RX` reader - Parity Error Enable for ETH RX Memory"]
pub type Peeneth0rxR = crate::BitReader<Peeneth0rx>;
impl Peeneth0rxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peeneth0rx {
        match self.bits {
            false => Peeneth0rx::Const0,
            true => Peeneth0rx::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peeneth0rx::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peeneth0rx::Const1
    }
}
#[doc = "Field `PEENETH0RX` writer - Parity Error Enable for ETH RX Memory"]
pub type Peeneth0rxW<'a, REG> = crate::BitWriter<'a, REG, Peeneth0rx>;
impl<'a, REG> Peeneth0rxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peeneth0rx::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peeneth0rx::Const1)
    }
}
#[doc = "Parity Error Enable for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peensd0 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peensd0> for bool {
    #[inline(always)]
    fn from(variant: Peensd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENSD0` reader - Parity Error Enable for SDMMC Memory 0"]
pub type Peensd0R = crate::BitReader<Peensd0>;
impl Peensd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peensd0 {
        match self.bits {
            false => Peensd0::Const0,
            true => Peensd0::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peensd0::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peensd0::Const1
    }
}
#[doc = "Field `PEENSD0` writer - Parity Error Enable for SDMMC Memory 0"]
pub type Peensd0W<'a, REG> = crate::BitWriter<'a, REG, Peensd0>;
impl<'a, REG> Peensd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peensd0::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peensd0::Const1)
    }
}
#[doc = "Parity Error Enable for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peensd1 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peensd1> for bool {
    #[inline(always)]
    fn from(variant: Peensd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENSD1` reader - Parity Error Enable for SDMMC Memory 1"]
pub type Peensd1R = crate::BitReader<Peensd1>;
impl Peensd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peensd1 {
        match self.bits {
            false => Peensd1::Const0,
            true => Peensd1::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peensd1::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peensd1::Const1
    }
}
#[doc = "Field `PEENSD1` writer - Parity Error Enable for SDMMC Memory 1"]
pub type Peensd1W<'a, REG> = crate::BitWriter<'a, REG, Peensd1>;
impl<'a, REG> Peensd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peensd1::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peensd1::Const1)
    }
}
#[doc = "Parity Error Enable for ECAT0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peenecat0 {
    #[doc = "0: Disabled"]
    Const0 = 0,
    #[doc = "1: Enabled"]
    Const1 = 1,
}
impl From<Peenecat0> for bool {
    #[inline(always)]
    fn from(variant: Peenecat0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEENECAT0` reader - Parity Error Enable for ECAT0 Memory"]
pub type Peenecat0R = crate::BitReader<Peenecat0>;
impl Peenecat0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peenecat0 {
        match self.bits {
            false => Peenecat0::Const0,
            true => Peenecat0::Const1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Peenecat0::Const0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Peenecat0::Const1
    }
}
#[doc = "Field `PEENECAT0` writer - Parity Error Enable for ECAT0 Memory"]
pub type Peenecat0W<'a, REG> = crate::BitWriter<'a, REG, Peenecat0>;
impl<'a, REG> Peenecat0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Peenecat0::Const0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Peenecat0::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    pub fn peenps(&self) -> PeenpsR {
        PeenpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    pub fn peends1(&self) -> Peends1R {
        Peends1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peenu0(&self) -> Peenu0R {
        Peenu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peenu1(&self) -> Peenu1R {
        Peenu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn peenmc(&self) -> PeenmcR {
        PeenmcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn peenpprf(&self) -> PeenpprfR {
        PeenpprfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    pub fn peenusb(&self) -> PeenusbR {
        PeenusbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline(always)]
    pub fn peeneth0tx(&self) -> Peeneth0txR {
        Peeneth0txR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline(always)]
    pub fn peeneth0rx(&self) -> Peeneth0rxR {
        Peeneth0rxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline(always)]
    pub fn peensd0(&self) -> Peensd0R {
        Peensd0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline(always)]
    pub fn peensd1(&self) -> Peensd1R {
        Peensd1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Parity Error Enable for ECAT0 Memory"]
    #[inline(always)]
    pub fn peenecat0(&self) -> Peenecat0R {
        Peenecat0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn peenps(&mut self) -> PeenpsW<PeenSpec> {
        PeenpsW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn peends1(&mut self) -> Peends1W<PeenSpec> {
        Peends1W::new(self, 1)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenu0(&mut self) -> Peenu0W<PeenSpec> {
        Peenu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenu1(&mut self) -> Peenu1W<PeenSpec> {
        Peenu1W::new(self, 9)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenmc(&mut self) -> PeenmcW<PeenSpec> {
        PeenmcW::new(self, 12)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenpprf(&mut self) -> PeenpprfW<PeenSpec> {
        PeenpprfW::new(self, 13)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenusb(&mut self) -> PeenusbW<PeenSpec> {
        PeenusbW::new(self, 16)
    }
    #[doc = "Bit 17 - Parity Error Enable for ETH TX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peeneth0tx(&mut self) -> Peeneth0txW<PeenSpec> {
        Peeneth0txW::new(self, 17)
    }
    #[doc = "Bit 18 - Parity Error Enable for ETH RX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peeneth0rx(&mut self) -> Peeneth0rxW<PeenSpec> {
        Peeneth0rxW::new(self, 18)
    }
    #[doc = "Bit 19 - Parity Error Enable for SDMMC Memory 0"]
    #[inline(always)]
    #[must_use]
    pub fn peensd0(&mut self) -> Peensd0W<PeenSpec> {
        Peensd0W::new(self, 19)
    }
    #[doc = "Bit 20 - Parity Error Enable for SDMMC Memory 1"]
    #[inline(always)]
    #[must_use]
    pub fn peensd1(&mut self) -> Peensd1W<PeenSpec> {
        Peensd1W::new(self, 20)
    }
    #[doc = "Bit 24 - Parity Error Enable for ECAT0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenecat0(&mut self) -> Peenecat0W<PeenSpec> {
        Peenecat0W::new(self, 24)
    }
}
#[doc = "Parity Error Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeenSpec;
impl crate::RegisterSpec for PeenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peen::R`](R) reader structure"]
impl crate::Readable for PeenSpec {}
#[doc = "`write(|w| ..)` method takes [`peen::W`](W) writer structure"]
impl crate::Writable for PeenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PEEN to value 0"]
impl crate::Resettable for PeenSpec {
    const RESET_VALUE: u32 = 0;
}
