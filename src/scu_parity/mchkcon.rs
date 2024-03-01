#[doc = "Register `MCHKCON` reader"]
pub type R = crate::R<MchkconSpec>;
#[doc = "Register `MCHKCON` writer"]
pub type W = crate::W<MchkconSpec>;
#[doc = "Select Memory Check for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selps {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Selps> for bool {
    #[inline(always)]
    fn from(variant: Selps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELPS` reader - Select Memory Check for PSRAM"]
pub type SelpsR = crate::BitReader<Selps>;
impl SelpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selps {
        match self.bits {
            false => Selps::Const0,
            true => Selps::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Selps::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Selps::Const1
    }
}
#[doc = "Field `SELPS` writer - Select Memory Check for PSRAM"]
pub type SelpsW<'a, REG> = crate::BitWriter<'a, REG, Selps>;
impl<'a, REG> SelpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selps::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selps::Const1)
    }
}
#[doc = "Select Memory Check for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selds1 {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Selds1> for bool {
    #[inline(always)]
    fn from(variant: Selds1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELDS1` reader - Select Memory Check for DSRAM1"]
pub type Selds1R = crate::BitReader<Selds1>;
impl Selds1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selds1 {
        match self.bits {
            false => Selds1::Const0,
            true => Selds1::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Selds1::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Selds1::Const1
    }
}
#[doc = "Field `SELDS1` writer - Select Memory Check for DSRAM1"]
pub type Selds1W<'a, REG> = crate::BitWriter<'a, REG, Selds1>;
impl<'a, REG> Selds1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selds1::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selds1::Const1)
    }
}
#[doc = "Select Memory Check for USIC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic0dra {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Usic0dra> for bool {
    #[inline(always)]
    fn from(variant: Usic0dra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0DRA` reader - Select Memory Check for USIC0"]
pub type Usic0draR = crate::BitReader<Usic0dra>;
impl Usic0draR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usic0dra {
        match self.bits {
            false => Usic0dra::Const0,
            true => Usic0dra::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usic0dra::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usic0dra::Const1
    }
}
#[doc = "Field `USIC0DRA` writer - Select Memory Check for USIC0"]
pub type Usic0draW<'a, REG> = crate::BitWriter<'a, REG, Usic0dra>;
impl<'a, REG> Usic0draW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usic0dra::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic0dra::Const1)
    }
}
#[doc = "Select Memory Check for USIC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic1dra {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Usic1dra> for bool {
    #[inline(always)]
    fn from(variant: Usic1dra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1DRA` reader - Select Memory Check for USIC1"]
pub type Usic1draR = crate::BitReader<Usic1dra>;
impl Usic1draR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usic1dra {
        match self.bits {
            false => Usic1dra::Const0,
            true => Usic1dra::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usic1dra::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usic1dra::Const1
    }
}
#[doc = "Field `USIC1DRA` writer - Select Memory Check for USIC1"]
pub type Usic1draW<'a, REG> = crate::BitWriter<'a, REG, Usic1dra>;
impl<'a, REG> Usic1draW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1dra::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1dra::Const1)
    }
}
#[doc = "Select Memory Check for MultiCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcandra {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Mcandra> for bool {
    #[inline(always)]
    fn from(variant: Mcandra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANDRA` reader - Select Memory Check for MultiCAN"]
pub type McandraR = crate::BitReader<Mcandra>;
impl McandraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcandra {
        match self.bits {
            false => Mcandra::Const0,
            true => Mcandra::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Mcandra::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Mcandra::Const1
    }
}
#[doc = "Field `MCANDRA` writer - Select Memory Check for MultiCAN"]
pub type McandraW<'a, REG> = crate::BitWriter<'a, REG, Mcandra>;
impl<'a, REG> McandraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mcandra::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcandra::Const1)
    }
}
#[doc = "Select Memory Check for PMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pprfdra {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Pprfdra> for bool {
    #[inline(always)]
    fn from(variant: Pprfdra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPRFDRA` reader - Select Memory Check for PMU"]
pub type PprfdraR = crate::BitReader<Pprfdra>;
impl PprfdraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pprfdra {
        match self.bits {
            false => Pprfdra::Const0,
            true => Pprfdra::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pprfdra::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pprfdra::Const1
    }
}
#[doc = "Field `PPRFDRA` writer - Select Memory Check for PMU"]
pub type PprfdraW<'a, REG> = crate::BitWriter<'a, REG, Pprfdra>;
impl<'a, REG> PprfdraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pprfdra::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pprfdra::Const1)
    }
}
#[doc = "Select Memory Check for USB SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selusb {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Selusb> for bool {
    #[inline(always)]
    fn from(variant: Selusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELUSB` reader - Select Memory Check for USB SRAM"]
pub type SelusbR = crate::BitReader<Selusb>;
impl SelusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selusb {
        match self.bits {
            false => Selusb::Const0,
            true => Selusb::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Selusb::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Selusb::Const1
    }
}
#[doc = "Field `SELUSB` writer - Select Memory Check for USB SRAM"]
pub type SelusbW<'a, REG> = crate::BitWriter<'a, REG, Selusb>;
impl<'a, REG> SelusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selusb::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selusb::Const1)
    }
}
#[doc = "Select Memory Check for ETH0 TX SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seleth0tx {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Seleth0tx> for bool {
    #[inline(always)]
    fn from(variant: Seleth0tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELETH0TX` reader - Select Memory Check for ETH0 TX SRAM"]
pub type Seleth0txR = crate::BitReader<Seleth0tx>;
impl Seleth0txR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seleth0tx {
        match self.bits {
            false => Seleth0tx::Const0,
            true => Seleth0tx::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Seleth0tx::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Seleth0tx::Const1
    }
}
#[doc = "Field `SELETH0TX` writer - Select Memory Check for ETH0 TX SRAM"]
pub type Seleth0txW<'a, REG> = crate::BitWriter<'a, REG, Seleth0tx>;
impl<'a, REG> Seleth0txW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Seleth0tx::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Seleth0tx::Const1)
    }
}
#[doc = "Select Memory Check for ETH0 RX SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seleth0rx {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Seleth0rx> for bool {
    #[inline(always)]
    fn from(variant: Seleth0rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELETH0RX` reader - Select Memory Check for ETH0 RX SRAM"]
pub type Seleth0rxR = crate::BitReader<Seleth0rx>;
impl Seleth0rxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seleth0rx {
        match self.bits {
            false => Seleth0rx::Const0,
            true => Seleth0rx::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Seleth0rx::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Seleth0rx::Const1
    }
}
#[doc = "Field `SELETH0RX` writer - Select Memory Check for ETH0 RX SRAM"]
pub type Seleth0rxW<'a, REG> = crate::BitWriter<'a, REG, Seleth0rx>;
impl<'a, REG> Seleth0rxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Seleth0rx::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Seleth0rx::Const1)
    }
}
#[doc = "Select Memory Check for SDMMC SRAM 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selsd0 {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Selsd0> for bool {
    #[inline(always)]
    fn from(variant: Selsd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELSD0` reader - Select Memory Check for SDMMC SRAM 0"]
pub type Selsd0R = crate::BitReader<Selsd0>;
impl Selsd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selsd0 {
        match self.bits {
            false => Selsd0::Const0,
            true => Selsd0::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Selsd0::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Selsd0::Const1
    }
}
#[doc = "Field `SELSD0` writer - Select Memory Check for SDMMC SRAM 0"]
pub type Selsd0W<'a, REG> = crate::BitWriter<'a, REG, Selsd0>;
impl<'a, REG> Selsd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selsd0::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selsd0::Const1)
    }
}
#[doc = "Select Memory Check for SDMMC SRAM 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selsd1 {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Selsd1> for bool {
    #[inline(always)]
    fn from(variant: Selsd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELSD1` reader - Select Memory Check for SDMMC SRAM 1"]
pub type Selsd1R = crate::BitReader<Selsd1>;
impl Selsd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selsd1 {
        match self.bits {
            false => Selsd1::Const0,
            true => Selsd1::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Selsd1::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Selsd1::Const1
    }
}
#[doc = "Field `SELSD1` writer - Select Memory Check for SDMMC SRAM 1"]
pub type Selsd1W<'a, REG> = crate::BitWriter<'a, REG, Selsd1>;
impl<'a, REG> Selsd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selsd1::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selsd1::Const1)
    }
}
#[doc = "Select Memory Check for ECAT0 SRAM 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selecat0 {
    #[doc = "0: Not selected"]
    Const0 = 0,
    #[doc = "1: Selected"]
    Const1 = 1,
}
impl From<Selecat0> for bool {
    #[inline(always)]
    fn from(variant: Selecat0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECAT0` reader - Select Memory Check for ECAT0 SRAM 1"]
pub type Selecat0R = crate::BitReader<Selecat0>;
impl Selecat0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selecat0 {
        match self.bits {
            false => Selecat0::Const0,
            true => Selecat0::Const1,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Selecat0::Const0
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Selecat0::Const1
    }
}
#[doc = "Field `SELECAT0` writer - Select Memory Check for ECAT0 SRAM 1"]
pub type Selecat0W<'a, REG> = crate::BitWriter<'a, REG, Selecat0>;
impl<'a, REG> Selecat0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selecat0::Const0)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selecat0::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&self) -> SelpsR {
        SelpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&self) -> Selds1R {
        Selds1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&self) -> Usic0draR {
        Usic0draR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&self) -> Usic1draR {
        Usic1draR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&self) -> McandraR {
        McandraR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&self) -> PprfdraR {
        PprfdraR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&self) -> SelusbR {
        SelusbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline(always)]
    pub fn seleth0tx(&self) -> Seleth0txR {
        Seleth0txR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline(always)]
    pub fn seleth0rx(&self) -> Seleth0rxR {
        Seleth0rxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline(always)]
    pub fn selsd0(&self) -> Selsd0R {
        Selsd0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline(always)]
    pub fn selsd1(&self) -> Selsd1R {
        Selsd1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline(always)]
    pub fn selecat0(&self) -> Selecat0R {
        Selecat0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn selps(&mut self) -> SelpsW<MchkconSpec> {
        SelpsW::new(self, 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn selds1(&mut self) -> Selds1W<MchkconSpec> {
        Selds1W::new(self, 1)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    #[must_use]
    pub fn usic0dra(&mut self) -> Usic0draW<MchkconSpec> {
        Usic0draW::new(self, 8)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    #[must_use]
    pub fn usic1dra(&mut self) -> Usic1draW<MchkconSpec> {
        Usic1draW::new(self, 9)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    #[must_use]
    pub fn mcandra(&mut self) -> McandraW<MchkconSpec> {
        McandraW::new(self, 12)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    #[must_use]
    pub fn pprfdra(&mut self) -> PprfdraW<MchkconSpec> {
        PprfdraW::new(self, 13)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn selusb(&mut self) -> SelusbW<MchkconSpec> {
        SelusbW::new(self, 16)
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn seleth0tx(&mut self) -> Seleth0txW<MchkconSpec> {
        Seleth0txW::new(self, 17)
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn seleth0rx(&mut self) -> Seleth0rxW<MchkconSpec> {
        Seleth0rxW::new(self, 18)
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline(always)]
    #[must_use]
    pub fn selsd0(&mut self) -> Selsd0W<MchkconSpec> {
        Selsd0W::new(self, 19)
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline(always)]
    #[must_use]
    pub fn selsd1(&mut self) -> Selsd1W<MchkconSpec> {
        Selsd1W::new(self, 20)
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline(always)]
    #[must_use]
    pub fn selecat0(&mut self) -> Selecat0W<MchkconSpec> {
        Selecat0W::new(self, 24)
    }
}
#[doc = "Memory Checking Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mchkcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mchkcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MchkconSpec;
impl crate::RegisterSpec for MchkconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mchkcon::R`](R) reader structure"]
impl crate::Readable for MchkconSpec {}
#[doc = "`write(|w| ..)` method takes [`mchkcon::W`](W) writer structure"]
impl crate::Writable for MchkconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCHKCON to value 0"]
impl crate::Resettable for MchkconSpec {
    const RESET_VALUE: u32 = 0;
}
