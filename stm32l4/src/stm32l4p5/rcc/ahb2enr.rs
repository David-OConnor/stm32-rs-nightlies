#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
#[doc = "IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN_A {
    #[doc = "0: IO port x clock disabled"]
    Disabled = 0,
    #[doc = "1: IO port x clock enabled"]
    Enabled = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::Disabled,
            true => GPIOAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN_A::Enabled
    }
}
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, GPIOAEN_A, O>;
impl<'a, const O: u8> GPIOAEN_W<'a, O> {
    #[doc = "IO port x clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Disabled)
    }
    #[doc = "IO port x clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Enabled)
    }
}
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub use GPIOAEN_R as GPIODEN_R;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub use GPIOAEN_R as GPIOEEN_R;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub use GPIOAEN_R as GPIOFEN_R;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub use GPIOAEN_R as GPIOGEN_R;
#[doc = "Field `GPIOHEN` reader - IO port H clock enable"]
pub use GPIOAEN_R as GPIOHEN_R;
#[doc = "Field `GPIOIEN` reader - IO port I clock enable"]
pub use GPIOAEN_R as GPIOIEN_R;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub use GPIOAEN_W as GPIODEN_W;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub use GPIOAEN_W as GPIOEEN_W;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub use GPIOAEN_W as GPIOFEN_W;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub use GPIOAEN_W as GPIOGEN_W;
#[doc = "Field `GPIOHEN` writer - IO port H clock enable"]
pub use GPIOAEN_W as GPIOHEN_W;
#[doc = "Field `GPIOIEN` writer - IO port I clock enable"]
pub use GPIOAEN_W as GPIOIEN_W;
#[doc = "Field `OTGFSEN` reader - OTG full speed clock enable"]
pub type OTGFSEN_R = crate::BitReader<OTGFSEN_A>;
#[doc = "OTG full speed clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSEN_A {
    #[doc = "0: USB OTG full speed clock disabled"]
    Disabled = 0,
    #[doc = "1: USB OTG full speed clock enabled"]
    Enabled = 1,
}
impl From<OTGFSEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OTGFSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGFSEN_A {
        match self.bits {
            false => OTGFSEN_A::Disabled,
            true => OTGFSEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OTGFSEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OTGFSEN_A::Enabled
    }
}
#[doc = "Field `OTGFSEN` writer - OTG full speed clock enable"]
pub type OTGFSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, OTGFSEN_A, O>;
impl<'a, const O: u8> OTGFSEN_W<'a, O> {
    #[doc = "USB OTG full speed clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGFSEN_A::Disabled)
    }
    #[doc = "USB OTG full speed clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGFSEN_A::Enabled)
    }
}
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type ADCEN_R = crate::BitReader<ADCEN_A>;
#[doc = "ADC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN_A {
    #[doc = "0: ADC clock disabled"]
    Disabled = 0,
    #[doc = "1: ADC clock enabled"]
    Enabled = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::Disabled,
            true => ADCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN_A::Enabled
    }
}
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type ADCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, ADCEN_A, O>;
impl<'a, const O: u8> ADCEN_W<'a, O> {
    #[doc = "ADC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCEN_A::Disabled)
    }
    #[doc = "ADC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCEN_A::Enabled)
    }
}
#[doc = "Field `DCMIEN` reader - DCMI clock enable"]
pub type DCMIEN_R = crate::BitReader<DCMIEN_A>;
#[doc = "DCMI clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIEN_A {
    #[doc = "0: DCMI/PSSI clock disabled"]
    Disabled = 0,
    #[doc = "1: DCMI/PSSI clock enabled"]
    Enabled = 1,
}
impl From<DCMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMIEN_A {
        match self.bits {
            false => DCMIEN_A::Disabled,
            true => DCMIEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMIEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMIEN_A::Enabled
    }
}
#[doc = "Field `DCMIEN` writer - DCMI clock enable"]
pub type DCMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, DCMIEN_A, O>;
impl<'a, const O: u8> DCMIEN_W<'a, O> {
    #[doc = "DCMI/PSSI clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMIEN_A::Disabled)
    }
    #[doc = "DCMI/PSSI clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMIEN_A::Enabled)
    }
}
#[doc = "Field `PKAEN` reader - PKA clock enable"]
pub type PKAEN_R = crate::BitReader<PKAEN_A>;
#[doc = "PKA clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKAEN_A {
    #[doc = "0: PKA clock disabled"]
    Disabled = 0,
    #[doc = "1: PKA clock enabled"]
    Enabled = 1,
}
impl From<PKAEN_A> for bool {
    #[inline(always)]
    fn from(variant: PKAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PKAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PKAEN_A {
        match self.bits {
            false => PKAEN_A::Disabled,
            true => PKAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKAEN_A::Enabled
    }
}
#[doc = "Field `PKAEN` writer - PKA clock enable"]
pub type PKAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, PKAEN_A, O>;
impl<'a, const O: u8> PKAEN_W<'a, O> {
    #[doc = "PKA clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PKAEN_A::Disabled)
    }
    #[doc = "PKA clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PKAEN_A::Enabled)
    }
}
#[doc = "Field `AESEN` reader - AES accelerator clock enable"]
pub type AESEN_R = crate::BitReader<AESEN_A>;
#[doc = "AES accelerator clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESEN_A {
    #[doc = "0: AES clock disabled"]
    Disabled = 0,
    #[doc = "1: AES clock enabled"]
    Enabled = 1,
}
impl From<AESEN_A> for bool {
    #[inline(always)]
    fn from(variant: AESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESEN_A {
        match self.bits {
            false => AESEN_A::Disabled,
            true => AESEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AESEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AESEN_A::Enabled
    }
}
#[doc = "Field `AESEN` writer - AES accelerator clock enable"]
pub type AESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, AESEN_A, O>;
impl<'a, const O: u8> AESEN_W<'a, O> {
    #[doc = "AES clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AESEN_A::Disabled)
    }
    #[doc = "AES clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AESEN_A::Enabled)
    }
}
#[doc = "Field `HASHEN` reader - HASH clock enable"]
pub type HASHEN_R = crate::BitReader<HASHEN_A>;
#[doc = "HASH clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHEN_A {
    #[doc = "0: HASH clock disabled"]
    Disabled = 0,
    #[doc = "1: HASH clock enabled"]
    Enabled = 1,
}
impl From<HASHEN_A> for bool {
    #[inline(always)]
    fn from(variant: HASHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HASHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASHEN_A {
        match self.bits {
            false => HASHEN_A::Disabled,
            true => HASHEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HASHEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HASHEN_A::Enabled
    }
}
#[doc = "Field `HASHEN` writer - HASH clock enable"]
pub type HASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, HASHEN_A, O>;
impl<'a, const O: u8> HASHEN_W<'a, O> {
    #[doc = "HASH clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HASHEN_A::Disabled)
    }
    #[doc = "HASH clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HASHEN_A::Enabled)
    }
}
#[doc = "Field `RNGEN` reader - Random Number Generator clock enable"]
pub type RNGEN_R = crate::BitReader<RNGEN_A>;
#[doc = "Random Number Generator clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN_A {
    #[doc = "0: Random Number Generator clock disabled"]
    Disabled = 0,
    #[doc = "1: Random Number Generator clock enabled"]
    Enabled = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::Disabled,
            true => RNGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN_A::Enabled
    }
}
#[doc = "Field `RNGEN` writer - Random Number Generator clock enable"]
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, RNGEN_A, O>;
impl<'a, const O: u8> RNGEN_W<'a, O> {
    #[doc = "Random Number Generator clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Disabled)
    }
    #[doc = "Random Number Generator clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Enabled)
    }
}
#[doc = "Field `OSPIMEN` reader - OctoSPI IO manager clock enable"]
pub type OSPIMEN_R = crate::BitReader<OSPIMEN_A>;
#[doc = "OctoSPI IO manager clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMEN_A {
    #[doc = "0: OctoSPI IO manager clock disabled"]
    Disabled = 0,
    #[doc = "1: OctoSPI IO manager clock enabled"]
    Enabled = 1,
}
impl From<OSPIMEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSPIMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OSPIMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPIMEN_A {
        match self.bits {
            false => OSPIMEN_A::Disabled,
            true => OSPIMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPIMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPIMEN_A::Enabled
    }
}
#[doc = "Field `OSPIMEN` writer - OctoSPI IO manager clock enable"]
pub type OSPIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, OSPIMEN_A, O>;
impl<'a, const O: u8> OSPIMEN_W<'a, O> {
    #[doc = "OctoSPI IO manager clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSPIMEN_A::Disabled)
    }
    #[doc = "OctoSPI IO manager clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSPIMEN_A::Enabled)
    }
}
#[doc = "Field `SDMMC1EN` reader - SDMMC1 clock enable"]
pub type SDMMC1EN_R = crate::BitReader<SDMMC1EN_A>;
#[doc = "SDMMC1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1EN_A {
    #[doc = "0: SDMMCx clock disabled"]
    Disabled = 0,
    #[doc = "1: SDMMCx clock enabled"]
    Enabled = 1,
}
impl From<SDMMC1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMMC1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC1EN_A {
        match self.bits {
            false => SDMMC1EN_A::Disabled,
            true => SDMMC1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1EN_A::Enabled
    }
}
#[doc = "Field `SDMMC1EN` writer - SDMMC1 clock enable"]
pub type SDMMC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, SDMMC1EN_A, O>;
impl<'a, const O: u8> SDMMC1EN_W<'a, O> {
    #[doc = "SDMMCx clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDMMC1EN_A::Disabled)
    }
    #[doc = "SDMMCx clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDMMC1EN_A::Enabled)
    }
}
#[doc = "Field `SDMMC2EN` reader - SDMMC2 clock enable"]
pub use SDMMC1EN_R as SDMMC2EN_R;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 clock enable"]
pub use SDMMC1EN_W as SDMMC2EN_W;
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKA clock enable"]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clock enable"]
    #[inline(always)]
    pub fn ospimen(&self) -> OSPIMEN_R {
        OSPIMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMMC1 clock enable"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SDMMC2 clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<8> {
        GPIOIEN_W::new(self)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<12> {
        OTGFSEN_W::new(self)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<13> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcmien(&mut self) -> DCMIEN_W<14> {
        DCMIEN_W::new(self)
    }
    #[doc = "Bit 15 - PKA clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<15> {
        PKAEN_W::new(self)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<16> {
        AESEN_W::new(self)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<17> {
        HASHEN_W::new(self)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<18> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ospimen(&mut self) -> OSPIMEN_W<20> {
        OSPIMEN_W::new(self)
    }
    #[doc = "Bit 22 - SDMMC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<22> {
        SDMMC1EN_W::new(self)
    }
    #[doc = "Bit 23 - SDMMC2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<23> {
        SDMMC2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
