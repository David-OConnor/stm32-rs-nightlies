#[doc = "Register `APB2SMENR` reader"]
pub struct R(crate::R<APB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2SMENR` writer"]
pub struct W(crate::W<APB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2SMENR_SPEC>;
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
impl From<crate::W<APB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGSMEN` reader - System configuration controller clock enable during sleep mode bit"]
pub type SYSCFGSMEN_R = crate::BitReader<SYSCFGSMEN_A>;
#[doc = "System configuration controller clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGSMEN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<SYSCFGSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGSMEN_A {
        match self.bits {
            false => SYSCFGSMEN_A::Disabled,
            true => SYSCFGSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGSMEN_A::Enabled
    }
}
#[doc = "Field `SYSCFGSMEN` writer - System configuration controller clock enable during sleep mode bit"]
pub type SYSCFGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, SYSCFGSMEN_A, O>;
impl<'a, const O: u8> SYSCFGSMEN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGSMEN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGSMEN_A::Enabled)
    }
}
#[doc = "Field `TIM21SMEN` reader - TIM21 timer clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as TIM21SMEN_R;
#[doc = "Field `TIM22SMEN` reader - TIM22 timer clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as TIM22SMEN_R;
#[doc = "Field `ADCSMEN` reader - ADC clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as ADCSMEN_R;
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as SPI1SMEN_R;
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as USART1SMEN_R;
#[doc = "Field `DBGSMEN` reader - DBG clock enable during sleep mode bit"]
pub use SYSCFGSMEN_R as DBGSMEN_R;
#[doc = "Field `TIM21SMEN` writer - TIM21 timer clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as TIM21SMEN_W;
#[doc = "Field `TIM22SMEN` writer - TIM22 timer clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as TIM22SMEN_W;
#[doc = "Field `ADCSMEN` writer - ADC clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as ADCSMEN_W;
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as SPI1SMEN_W;
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as USART1SMEN_W;
#[doc = "Field `DBGSMEN` writer - DBG clock enable during sleep mode bit"]
pub use SYSCFGSMEN_W as DBGSMEN_W;
impl R {
    #[doc = "Bit 0 - System configuration controller clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TIM21 timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim21smen(&self) -> TIM21SMEN_R {
        TIM21SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM22 timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim22smen(&self) -> TIM22SMEN_R {
        TIM22SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - DBG clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration controller clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<0> {
        SYSCFGSMEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM21 timer clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim21smen(&mut self) -> TIM21SMEN_W<2> {
        TIM21SMEN_W::new(self)
    }
    #[doc = "Bit 5 - TIM22 timer clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim22smen(&mut self) -> TIM22SMEN_W<5> {
        TIM22SMEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<9> {
        ADCSMEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<12> {
        SPI1SMEN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<14> {
        USART1SMEN_W::new(self)
    }
    #[doc = "Bit 22 - DBG clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<22> {
        DBGSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2smenr](index.html) module"]
pub struct APB2SMENR_SPEC;
impl crate::RegisterSpec for APB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2smenr::R](R) reader structure"]
impl crate::Readable for APB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2smenr::W](W) writer structure"]
impl crate::Writable for APB2SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2SMENR to value 0x0040_5225"]
impl crate::Resettable for APB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_5225;
}
