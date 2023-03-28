#[doc = "Register `APB2ENR` reader"]
pub struct R(crate::R<APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2ENR` writer"]
pub struct W(crate::W<APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2ENR_SPEC>;
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
impl From<crate::W<APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGEN` reader - SYSCFG clock enable"]
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN_A>;
#[doc = "SYSCFG clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN_A {
    #[doc = "0: SYSCFG + COMP + VREFBUF clock disabled"]
    Disabled = 0,
    #[doc = "1: SYSCFG + COMP + VREFBUF clock enabled"]
    Enabled = 1,
}
impl From<SYSCFGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGEN_A {
        match self.bits {
            false => SYSCFGEN_A::Disabled,
            true => SYSCFGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN_A::Enabled
    }
}
#[doc = "Field `SYSCFGEN` writer - SYSCFG clock enable"]
pub type SYSCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, SYSCFGEN_A, O>;
impl<'a, const O: u8> SYSCFGEN_W<'a, O> {
    #[doc = "SYSCFG + COMP + VREFBUF clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::Disabled)
    }
    #[doc = "SYSCFG + COMP + VREFBUF clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::Enabled)
    }
}
#[doc = "Field `FWEN` reader - Firewall clock enable"]
pub type FWEN_R = crate::BitReader<FWEN_A>;
#[doc = "Firewall clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWEN_A {
    #[doc = "0: Firewall clock disabled"]
    Disabled = 0,
    #[doc = "1: Firewall clock enabled"]
    Enabled = 1,
}
impl From<FWEN_A> for bool {
    #[inline(always)]
    fn from(variant: FWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWEN_A {
        match self.bits {
            false => FWEN_A::Disabled,
            true => FWEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWEN_A::Enabled
    }
}
#[doc = "Field `FWEN` writer - Firewall clock enable"]
pub type FWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, FWEN_A, O>;
impl<'a, const O: u8> FWEN_W<'a, O> {
    #[doc = "Firewall clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FWEN_A::Disabled)
    }
    #[doc = "Firewall clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FWEN_A::Enabled)
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable"]
pub type TIM1EN_R = crate::BitReader<TIM1EN_A>;
#[doc = "TIM1 timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN_A {
    #[doc = "0: TIMx clock disabled"]
    Disabled = 0,
    #[doc = "1: TIMx clock enabled"]
    Enabled = 1,
}
impl From<TIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1EN_A {
        match self.bits {
            false => TIM1EN_A::Disabled,
            true => TIM1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN_A::Enabled
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable"]
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, TIM1EN_A, O>;
impl<'a, const O: u8> TIM1EN_W<'a, O> {
    #[doc = "TIMx clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::Disabled)
    }
    #[doc = "TIMx clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::Enabled)
    }
}
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader<SPI1EN_A>;
#[doc = "SPI1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1EN_A {
    #[doc = "0: SPI1 clock disabled"]
    Disabled = 0,
    #[doc = "1: SPI1 clock enabled"]
    Enabled = 1,
}
impl From<SPI1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1EN_A {
        match self.bits {
            false => SPI1EN_A::Disabled,
            true => SPI1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI1EN_A::Enabled
    }
}
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, SPI1EN_A, O>;
impl<'a, const O: u8> SPI1EN_W<'a, O> {
    #[doc = "SPI1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI1EN_A::Disabled)
    }
    #[doc = "SPI1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI1EN_A::Enabled)
    }
}
#[doc = "Field `TIM8EN` reader - TIM8 timer clock enable"]
pub use TIM1EN_R as TIM8EN_R;
#[doc = "Field `TIM8EN` writer - TIM8 timer clock enable"]
pub use TIM1EN_W as TIM8EN_W;
#[doc = "Field `USART1EN` reader - USART1clock enable"]
pub type USART1EN_R = crate::BitReader<USART1EN_A>;
#[doc = "USART1clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1EN_A {
    #[doc = "0: USART1 clock disabled"]
    Disabled = 0,
    #[doc = "1: USART1 clock enabled"]
    Enabled = 1,
}
impl From<USART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1EN_A {
        match self.bits {
            false => USART1EN_A::Disabled,
            true => USART1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART1EN_A::Enabled
    }
}
#[doc = "Field `USART1EN` writer - USART1clock enable"]
pub type USART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, USART1EN_A, O>;
impl<'a, const O: u8> USART1EN_W<'a, O> {
    #[doc = "USART1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1EN_A::Disabled)
    }
    #[doc = "USART1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1EN_A::Enabled)
    }
}
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable"]
pub use TIM1EN_R as TIM15EN_R;
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable"]
pub use TIM1EN_R as TIM16EN_R;
#[doc = "Field `TIM17EN` reader - TIM17 timer clock enable"]
pub use TIM1EN_R as TIM17EN_R;
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable"]
pub use TIM1EN_W as TIM15EN_W;
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable"]
pub use TIM1EN_W as TIM16EN_W;
#[doc = "Field `TIM17EN` writer - TIM17 timer clock enable"]
pub use TIM1EN_W as TIM17EN_W;
#[doc = "Field `SAI1EN` reader - SAI1 clock enable"]
pub type SAI1EN_R = crate::BitReader<SAI1EN_A>;
#[doc = "SAI1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1EN_A {
    #[doc = "0: SAIx clock disabled"]
    Disabled = 0,
    #[doc = "1: SAIx clock enabled"]
    Enabled = 1,
}
impl From<SAI1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1EN_A {
        match self.bits {
            false => SAI1EN_A::Disabled,
            true => SAI1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAI1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAI1EN_A::Enabled
    }
}
#[doc = "Field `SAI1EN` writer - SAI1 clock enable"]
pub type SAI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, SAI1EN_A, O>;
impl<'a, const O: u8> SAI1EN_W<'a, O> {
    #[doc = "SAIx clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAI1EN_A::Disabled)
    }
    #[doc = "SAIx clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAI1EN_A::Enabled)
    }
}
#[doc = "Field `SAI2EN` reader - SAI2 clock enable"]
pub use SAI1EN_R as SAI2EN_R;
#[doc = "Field `SAI2EN` writer - SAI2 clock enable"]
pub use SAI1EN_W as SAI2EN_W;
#[doc = "Field `DFSDM1EN` reader - DFSDM timer clock enable"]
pub type DFSDM1EN_R = crate::BitReader<DFSDM1EN_A>;
#[doc = "DFSDM timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1EN_A {
    #[doc = "0: DFSDM1 clock disabled"]
    Disabled = 0,
    #[doc = "1: DFSDM1 clock enabled"]
    Enabled = 1,
}
impl From<DFSDM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DFSDM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDM1EN_A {
        match self.bits {
            false => DFSDM1EN_A::Disabled,
            true => DFSDM1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFSDM1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFSDM1EN_A::Enabled
    }
}
#[doc = "Field `DFSDM1EN` writer - DFSDM timer clock enable"]
pub type DFSDM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, DFSDM1EN_A, O>;
impl<'a, const O: u8> DFSDM1EN_W<'a, O> {
    #[doc = "DFSDM1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DFSDM1EN_A::Disabled)
    }
    #[doc = "DFSDM1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DFSDM1EN_A::Enabled)
    }
}
#[doc = "Field `LTDCEN` reader - LCD-TFT clock enable"]
pub type LTDCEN_R = crate::BitReader<LTDCEN_A>;
#[doc = "LCD-TFT clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN_A {
    #[doc = "0: LTDC clock disabled"]
    Disabled = 0,
    #[doc = "1: LTDC clock enabled"]
    Enabled = 1,
}
impl From<LTDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LTDCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCEN_A {
        match self.bits {
            false => LTDCEN_A::Disabled,
            true => LTDCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN_A::Enabled
    }
}
#[doc = "Field `LTDCEN` writer - LCD-TFT clock enable"]
pub type LTDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, LTDCEN_A, O>;
impl<'a, const O: u8> LTDCEN_W<'a, O> {
    #[doc = "LTDC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::Disabled)
    }
    #[doc = "LTDC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::Enabled)
    }
}
#[doc = "Field `DSIEN` reader - DSI clock enable"]
pub type DSIEN_R = crate::BitReader<DSIEN_A>;
#[doc = "DSI clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIEN_A {
    #[doc = "0: DSI clock disabled"]
    Disabled = 0,
    #[doc = "1: DSI clock enabled"]
    Enabled = 1,
}
impl From<DSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DSIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DSIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSIEN_A {
        match self.bits {
            false => DSIEN_A::Disabled,
            true => DSIEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSIEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSIEN_A::Enabled
    }
}
#[doc = "Field `DSIEN` writer - DSI clock enable"]
pub type DSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, DSIEN_A, O>;
impl<'a, const O: u8> DSIEN_W<'a, O> {
    #[doc = "DSI clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DSIEN_A::Disabled)
    }
    #[doc = "DSI clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DSIEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Firewall clock enable"]
    #[inline(always)]
    pub fn fwen(&self) -> FWEN_R {
        FWEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clock enable"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DFSDM timer clock enable"]
    #[inline(always)]
    pub fn dfsdm1en(&self) -> DFSDM1EN_R {
        DFSDM1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD-TFT clock enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI clock enable"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<0> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 7 - Firewall clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwen(&mut self) -> FWEN_W<7> {
        FWEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<13> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<16> {
        TIM15EN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<17> {
        TIM16EN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<18> {
        TIM17EN_W::new(self)
    }
    #[doc = "Bit 21 - SAI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<21> {
        SAI1EN_W::new(self)
    }
    #[doc = "Bit 22 - SAI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<22> {
        SAI2EN_W::new(self)
    }
    #[doc = "Bit 24 - DFSDM timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1en(&mut self) -> DFSDM1EN_W<24> {
        DFSDM1EN_W::new(self)
    }
    #[doc = "Bit 26 - LCD-TFT clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<26> {
        LTDCEN_W::new(self)
    }
    #[doc = "Bit 27 - DSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<27> {
        DSIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2ENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2enr::R](R) reader structure"]
impl crate::Readable for APB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](W) writer structure"]
impl crate::Writable for APB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
