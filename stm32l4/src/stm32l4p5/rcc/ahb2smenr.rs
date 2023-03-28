#[doc = "Register `AHB2SMENR` reader"]
pub struct R(crate::R<AHB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2SMENR` writer"]
pub struct W(crate::W<AHB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2SMENR_SPEC>;
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
impl From<crate::W<AHB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_R = crate::BitReader<GPIOASMEN_A>;
#[doc = "IO port A clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOASMEN_A {
    #[doc = "0: IO port x clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: IO port x clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<GPIOASMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOASMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOASMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOASMEN_A {
        match self.bits {
            false => GPIOASMEN_A::Disabled,
            true => GPIOASMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOASMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOASMEN_A::Enabled
    }
}
#[doc = "Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, GPIOASMEN_A, O>;
impl<'a, const O: u8> GPIOASMEN_W<'a, O> {
    #[doc = "IO port x clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOASMEN_A::Disabled)
    }
    #[doc = "IO port x clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOASMEN_A::Enabled)
    }
}
#[doc = "Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOBSMEN_R;
#[doc = "Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOCSMEN_R;
#[doc = "Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIODSMEN_R;
#[doc = "Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOESMEN_R;
#[doc = "Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOFSMEN_R;
#[doc = "Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOGSMEN_R;
#[doc = "Field `GPIOHSMEN` reader - IO port H clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOHSMEN_R;
#[doc = "Field `GPIOISMEN` reader - IO port I clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOISMEN_R;
#[doc = "Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOBSMEN_W;
#[doc = "Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOCSMEN_W;
#[doc = "Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIODSMEN_W;
#[doc = "Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOESMEN_W;
#[doc = "Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOFSMEN_W;
#[doc = "Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOGSMEN_W;
#[doc = "Field `GPIOHSMEN` writer - IO port H clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOHSMEN_W;
#[doc = "Field `GPIOISMEN` writer - IO port I clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOISMEN_W;
#[doc = "Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_R = crate::BitReader<SRAM2SMEN_A>;
#[doc = "SRAM2 interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2SMEN_A {
    #[doc = "0: SRAMx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SRAMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SRAM2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM2SMEN_A {
        match self.bits {
            false => SRAM2SMEN_A::Disabled,
            true => SRAM2SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM2SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM2SMEN_A::Enabled
    }
}
#[doc = "Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, SRAM2SMEN_A, O>;
impl<'a, const O: u8> SRAM2SMEN_W<'a, O> {
    #[doc = "SRAMx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM2SMEN_A::Disabled)
    }
    #[doc = "SRAMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM2SMEN_A::Enabled)
    }
}
#[doc = "Field `SRAM3SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub use SRAM2SMEN_R as SRAM3SMEN_R;
#[doc = "Field `SRAM3SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub use SRAM2SMEN_W as SRAM3SMEN_W;
#[doc = "Field `OTGFSSMEN` reader - OTG full speed clocks enable during Sleep and Stop modes"]
pub type OTGFSSMEN_R = crate::BitReader<OTGFSSMEN_A>;
#[doc = "OTG full speed clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSSMEN_A {
    #[doc = "0: USB OTG full speed clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: USB OTG full speed clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OTGFSSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OTGFSSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGFSSMEN_A {
        match self.bits {
            false => OTGFSSMEN_A::Disabled,
            true => OTGFSSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OTGFSSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OTGFSSMEN_A::Enabled
    }
}
#[doc = "Field `OTGFSSMEN` writer - OTG full speed clocks enable during Sleep and Stop modes"]
pub type OTGFSSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, OTGFSSMEN_A, O>;
impl<'a, const O: u8> OTGFSSMEN_W<'a, O> {
    #[doc = "USB OTG full speed clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGFSSMEN_A::Disabled)
    }
    #[doc = "USB OTG full speed clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGFSSMEN_A::Enabled)
    }
}
#[doc = "Field `ADCSMEN` reader - ADC clocks enable during Sleep and Stop modes"]
pub type ADCSMEN_R = crate::BitReader<ADCSMEN_A>;
#[doc = "ADC clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCSMEN_A {
    #[doc = "0: ADC clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: ADC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<ADCSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSMEN_A {
        match self.bits {
            false => ADCSMEN_A::Disabled,
            true => ADCSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCSMEN_A::Enabled
    }
}
#[doc = "Field `ADCSMEN` writer - ADC clocks enable during Sleep and Stop modes"]
pub type ADCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, ADCSMEN_A, O>;
impl<'a, const O: u8> ADCSMEN_W<'a, O> {
    #[doc = "ADC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCSMEN_A::Disabled)
    }
    #[doc = "ADC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCSMEN_A::Enabled)
    }
}
#[doc = "Field `DCMISMEN` reader - DCMI clock enable during Sleep and Stop modes"]
pub type DCMISMEN_R = crate::BitReader<DCMISMEN_A>;
#[doc = "DCMI clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMISMEN_A {
    #[doc = "0: DCMI/PSSI clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DCMI/PSSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DCMISMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMISMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMISMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMISMEN_A {
        match self.bits {
            false => DCMISMEN_A::Disabled,
            true => DCMISMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMISMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMISMEN_A::Enabled
    }
}
#[doc = "Field `DCMISMEN` writer - DCMI clock enable during Sleep and Stop modes"]
pub type DCMISMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, DCMISMEN_A, O>;
impl<'a, const O: u8> DCMISMEN_W<'a, O> {
    #[doc = "DCMI/PSSI clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMISMEN_A::Disabled)
    }
    #[doc = "DCMI/PSSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMISMEN_A::Enabled)
    }
}
#[doc = "Field `PKASMEN` reader - PKA clocks enable during Sleep and Stop modes"]
pub type PKASMEN_R = crate::BitReader<PKASMEN_A>;
#[doc = "PKA clocks enable during Sleep and Stop modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKASMEN_A {
    #[doc = "0: PKA clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: PKA clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<PKASMEN_A> for bool {
    #[inline(always)]
    fn from(variant: PKASMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PKASMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PKASMEN_A {
        match self.bits {
            false => PKASMEN_A::Disabled,
            true => PKASMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKASMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKASMEN_A::Enabled
    }
}
#[doc = "Field `PKASMEN` writer - PKA clocks enable during Sleep and Stop modes"]
pub type PKASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, PKASMEN_A, O>;
impl<'a, const O: u8> PKASMEN_W<'a, O> {
    #[doc = "PKA clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PKASMEN_A::Disabled)
    }
    #[doc = "PKA clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PKASMEN_A::Enabled)
    }
}
#[doc = "Field `AESSMEN` reader - AES accelerator clocks enable during Sleep and Stop modes"]
pub type AESSMEN_R = crate::BitReader<AESSMEN_A>;
#[doc = "AES accelerator clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESSMEN_A {
    #[doc = "0: AES clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: AES clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<AESSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: AESSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AESSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESSMEN_A {
        match self.bits {
            false => AESSMEN_A::Disabled,
            true => AESSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AESSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AESSMEN_A::Enabled
    }
}
#[doc = "Field `AESSMEN` writer - AES accelerator clocks enable during Sleep and Stop modes"]
pub type AESSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, AESSMEN_A, O>;
impl<'a, const O: u8> AESSMEN_W<'a, O> {
    #[doc = "AES clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AESSMEN_A::Disabled)
    }
    #[doc = "AES clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AESSMEN_A::Enabled)
    }
}
#[doc = "Field `HASHSMEN` reader - HASH clock enable during Sleep and Stop modes"]
pub type HASHSMEN_R = crate::BitReader<HASHSMEN_A>;
#[doc = "HASH clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHSMEN_A {
    #[doc = "0: HASH clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: HASH clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<HASHSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: HASHSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HASHSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASHSMEN_A {
        match self.bits {
            false => HASHSMEN_A::Disabled,
            true => HASHSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HASHSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HASHSMEN_A::Enabled
    }
}
#[doc = "Field `HASHSMEN` writer - HASH clock enable during Sleep and Stop modes"]
pub type HASHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, HASHSMEN_A, O>;
impl<'a, const O: u8> HASHSMEN_W<'a, O> {
    #[doc = "HASH clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HASHSMEN_A::Disabled)
    }
    #[doc = "HASH clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HASHSMEN_A::Enabled)
    }
}
#[doc = "Field `RNGSMEN` reader - Random Number Generator clocks enable during Sleep and Stop modes"]
pub type RNGSMEN_R = crate::BitReader<RNGSMEN_A>;
#[doc = "Random Number Generator clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGSMEN_A {
    #[doc = "0: Random Number Generator clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: Random Number Generator clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<RNGSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGSMEN_A {
        match self.bits {
            false => RNGSMEN_A::Disabled,
            true => RNGSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGSMEN_A::Enabled
    }
}
#[doc = "Field `RNGSMEN` writer - Random Number Generator clocks enable during Sleep and Stop modes"]
pub type RNGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, RNGSMEN_A, O>;
impl<'a, const O: u8> RNGSMEN_W<'a, O> {
    #[doc = "Random Number Generator clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGSMEN_A::Disabled)
    }
    #[doc = "Random Number Generator clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGSMEN_A::Enabled)
    }
}
#[doc = "Field `OSPIMSMEN` reader - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
pub type OSPIMSMEN_R = crate::BitReader<OSPIMSMEN_A>;
#[doc = "OctoSPI IO manager clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMSMEN_A {
    #[doc = "0: OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: OCTOSPIM clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OSPIMSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSPIMSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OSPIMSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPIMSMEN_A {
        match self.bits {
            false => OSPIMSMEN_A::Disabled,
            true => OSPIMSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPIMSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPIMSMEN_A::Enabled
    }
}
#[doc = "Field `OSPIMSMEN` writer - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
pub type OSPIMSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, OSPIMSMEN_A, O>;
impl<'a, const O: u8> OSPIMSMEN_W<'a, O> {
    #[doc = "OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSPIMSMEN_A::Disabled)
    }
    #[doc = "OCTOSPIM clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSPIMSMEN_A::Enabled)
    }
}
#[doc = "Field `SDMMC1SMEN` reader - SDMMC1 clocks enable during Sleep and Stop modes"]
pub type SDMMC1SMEN_R = crate::BitReader<SDMMC1SMEN_A>;
#[doc = "SDMMC1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1SMEN_A {
    #[doc = "0: SDMMCx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SDMMCx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SDMMC1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMMC1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC1SMEN_A {
        match self.bits {
            false => SDMMC1SMEN_A::Disabled,
            true => SDMMC1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1SMEN_A::Enabled
    }
}
#[doc = "Field `SDMMC1SMEN` writer - SDMMC1 clocks enable during Sleep and Stop modes"]
pub type SDMMC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, SDMMC1SMEN_A, O>;
impl<'a, const O: u8> SDMMC1SMEN_W<'a, O> {
    #[doc = "SDMMCx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDMMC1SMEN_A::Disabled)
    }
    #[doc = "SDMMCx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDMMC1SMEN_A::Enabled)
    }
}
#[doc = "Field `SDMMC2SMEN` reader - SDMMC2 clocks enable during Sleep and Stop modes"]
pub use SDMMC1SMEN_R as SDMMC2SMEN_R;
#[doc = "Field `SDMMC2SMEN` writer - SDMMC2 clocks enable during Sleep and Stop modes"]
pub use SDMMC1SMEN_W as SDMMC2SMEN_W;
impl R {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioismen(&self) -> GPIOISMEN_R {
        GPIOISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram3smen(&self) -> SRAM3SMEN_R {
        SRAM3SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn otgfssmen(&self) -> OTGFSSMEN_R {
        OTGFSSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dcmismen(&self) -> DCMISMEN_R {
        DCMISMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKA clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn hashsmen(&self) -> HASHSMEN_R {
        HASHSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ospimsmen(&self) -> OSPIMSMEN_R {
        OSPIMSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sdmmc1smen(&self) -> SDMMC1SMEN_R {
        SDMMC1SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SDMMC2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sdmmc2smen(&self) -> SDMMC2SMEN_R {
        SDMMC2SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<3> {
        GPIODSMEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<4> {
        GPIOESMEN_W::new(self)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<5> {
        GPIOFSMEN_W::new(self)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W<6> {
        GPIOGSMEN_W::new(self)
    }
    #[doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<7> {
        GPIOHSMEN_W::new(self)
    }
    #[doc = "Bit 8 - IO port I clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioismen(&mut self) -> GPIOISMEN_W<8> {
        GPIOISMEN_W::new(self)
    }
    #[doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<9> {
        SRAM2SMEN_W::new(self)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram3smen(&mut self) -> SRAM3SMEN_W<10> {
        SRAM3SMEN_W::new(self)
    }
    #[doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn otgfssmen(&mut self) -> OTGFSSMEN_W<12> {
        OTGFSSMEN_W::new(self)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<13> {
        ADCSMEN_W::new(self)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dcmismen(&mut self) -> DCMISMEN_W<14> {
        DCMISMEN_W::new(self)
    }
    #[doc = "Bit 15 - PKA clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn pkasmen(&mut self) -> PKASMEN_W<15> {
        PKASMEN_W::new(self)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn aessmen(&mut self) -> AESSMEN_W<16> {
        AESSMEN_W::new(self)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn hashsmen(&mut self) -> HASHSMEN_W<17> {
        HASHSMEN_W::new(self)
    }
    #[doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<18> {
        RNGSMEN_W::new(self)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ospimsmen(&mut self) -> OSPIMSMEN_W<20> {
        OSPIMSMEN_W::new(self)
    }
    #[doc = "Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1smen(&mut self) -> SDMMC1SMEN_W<22> {
        SDMMC1SMEN_W::new(self)
    }
    #[doc = "Bit 23 - SDMMC2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2smen(&mut self) -> SDMMC2SMEN_W<23> {
        SDMMC2SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2smenr](index.html) module"]
pub struct AHB2SMENR_SPEC;
impl crate::RegisterSpec for AHB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2smenr::R](R) reader structure"]
impl crate::Readable for AHB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2smenr::W](W) writer structure"]
impl crate::Writable for AHB2SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2SMENR to value 0x0057_77ff"]
impl crate::Resettable for AHB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0057_77ff;
}
