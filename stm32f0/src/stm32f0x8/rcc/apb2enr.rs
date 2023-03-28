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
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
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
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::Enabled)
    }
}
#[doc = "Field `USART6EN` reader - USART6 clock enable"]
pub use SYSCFGEN_R as USART6EN_R;
#[doc = "Field `USART7EN` reader - USART7 clock enable"]
pub use SYSCFGEN_R as USART7EN_R;
#[doc = "Field `USART8EN` reader - USART8 clock enable"]
pub use SYSCFGEN_R as USART8EN_R;
#[doc = "Field `ADCEN` reader - ADC 1 interface clock enable"]
pub use SYSCFGEN_R as ADCEN_R;
#[doc = "Field `TIM1EN` reader - TIM1 Timer clock enable"]
pub use SYSCFGEN_R as TIM1EN_R;
#[doc = "Field `SPI1EN` reader - SPI 1 clock enable"]
pub use SYSCFGEN_R as SPI1EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use SYSCFGEN_R as USART1EN_R;
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable"]
pub use SYSCFGEN_R as TIM15EN_R;
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable"]
pub use SYSCFGEN_R as TIM16EN_R;
#[doc = "Field `TIM17EN` reader - TIM17 timer clock enable"]
pub use SYSCFGEN_R as TIM17EN_R;
#[doc = "Field `DBGMCUEN` reader - MCU debug module clock enable"]
pub use SYSCFGEN_R as DBGMCUEN_R;
#[doc = "Field `USART6EN` writer - USART6 clock enable"]
pub use SYSCFGEN_W as USART6EN_W;
#[doc = "Field `USART7EN` writer - USART7 clock enable"]
pub use SYSCFGEN_W as USART7EN_W;
#[doc = "Field `USART8EN` writer - USART8 clock enable"]
pub use SYSCFGEN_W as USART8EN_W;
#[doc = "Field `ADCEN` writer - ADC 1 interface clock enable"]
pub use SYSCFGEN_W as ADCEN_W;
#[doc = "Field `TIM1EN` writer - TIM1 Timer clock enable"]
pub use SYSCFGEN_W as TIM1EN_W;
#[doc = "Field `SPI1EN` writer - SPI 1 clock enable"]
pub use SYSCFGEN_W as SPI1EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use SYSCFGEN_W as USART1EN_W;
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable"]
pub use SYSCFGEN_W as TIM15EN_W;
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable"]
pub use SYSCFGEN_W as TIM16EN_W;
#[doc = "Field `TIM17EN` writer - TIM17 timer clock enable"]
pub use SYSCFGEN_W as TIM17EN_W;
#[doc = "Field `DBGMCUEN` writer - MCU debug module clock enable"]
pub use SYSCFGEN_W as DBGMCUEN_W;
impl R {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART7 clock enable"]
    #[inline(always)]
    pub fn usart7en(&self) -> USART7EN_R {
        USART7EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART8 clock enable"]
    #[inline(always)]
    pub fn usart8en(&self) -> USART8EN_R {
        USART8EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
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
    #[doc = "Bit 22 - MCU debug module clock enable"]
    #[inline(always)]
    pub fn dbgmcuen(&self) -> DBGMCUEN_R {
        DBGMCUEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<0> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<5> {
        USART6EN_W::new(self)
    }
    #[doc = "Bit 6 - USART7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart7en(&mut self) -> USART7EN_W<6> {
        USART7EN_W::new(self)
    }
    #[doc = "Bit 7 - USART8 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart8en(&mut self) -> USART8EN_W<7> {
        USART8EN_W::new(self)
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<9> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
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
    #[doc = "Bit 22 - MCU debug module clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgmcuen(&mut self) -> DBGMCUEN_W<22> {
        DBGMCUEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
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
