#[doc = "Register `APB2RSTR` reader"]
pub struct R(crate::R<APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RSTR` writer"]
pub struct W(crate::W<APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RSTR_SPEC>;
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
impl From<crate::W<APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFIORST` reader - Alternate function I/O reset"]
pub type AFIORST_R = crate::BitReader<AFIORST_A>;
#[doc = "Alternate function I/O reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFIORST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<AFIORST_A> for bool {
    #[inline(always)]
    fn from(variant: AFIORST_A) -> Self {
        variant as u8 != 0
    }
}
impl AFIORST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AFIORST_A> {
        match self.bits {
            true => Some(AFIORST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AFIORST_A::Reset
    }
}
#[doc = "Field `AFIORST` writer - Alternate function I/O reset"]
pub type AFIORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, AFIORST_A, O>;
impl<'a, const O: u8> AFIORST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(AFIORST_A::Reset)
    }
}
#[doc = "Field `IOPARST` reader - IO port A reset"]
pub use AFIORST_R as IOPARST_R;
#[doc = "Field `IOPBRST` reader - IO port B reset"]
pub use AFIORST_R as IOPBRST_R;
#[doc = "Field `IOPCRST` reader - IO port C reset"]
pub use AFIORST_R as IOPCRST_R;
#[doc = "Field `IOPDRST` reader - IO port D reset"]
pub use AFIORST_R as IOPDRST_R;
#[doc = "Field `IOPERST` reader - IO port E reset"]
pub use AFIORST_R as IOPERST_R;
#[doc = "Field `ADC1RST` reader - ADC 1 interface reset"]
pub use AFIORST_R as ADC1RST_R;
#[doc = "Field `ADC2RST` reader - ADC 2 interface reset"]
pub use AFIORST_R as ADC2RST_R;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub use AFIORST_R as TIM1RST_R;
#[doc = "Field `SPI1RST` reader - SPI 1 reset"]
pub use AFIORST_R as SPI1RST_R;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use AFIORST_R as USART1RST_R;
#[doc = "Field `IOPARST` writer - IO port A reset"]
pub use AFIORST_W as IOPARST_W;
#[doc = "Field `IOPBRST` writer - IO port B reset"]
pub use AFIORST_W as IOPBRST_W;
#[doc = "Field `IOPCRST` writer - IO port C reset"]
pub use AFIORST_W as IOPCRST_W;
#[doc = "Field `IOPDRST` writer - IO port D reset"]
pub use AFIORST_W as IOPDRST_W;
#[doc = "Field `IOPERST` writer - IO port E reset"]
pub use AFIORST_W as IOPERST_W;
#[doc = "Field `ADC1RST` writer - ADC 1 interface reset"]
pub use AFIORST_W as ADC1RST_W;
#[doc = "Field `ADC2RST` writer - ADC 2 interface reset"]
pub use AFIORST_W as ADC2RST_W;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub use AFIORST_W as TIM1RST_W;
#[doc = "Field `SPI1RST` writer - SPI 1 reset"]
pub use AFIORST_W as SPI1RST_W;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use AFIORST_W as USART1RST_W;
impl R {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    pub fn afiorst(&self) -> AFIORST_R {
        AFIORST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC 1 interface reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC 2 interface reset"]
    #[inline(always)]
    pub fn adc2rst(&self) -> ADC2RST_R {
        ADC2RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    #[must_use]
    pub fn afiorst(&mut self) -> AFIORST_W<0> {
        AFIORST_W::new(self)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioparst(&mut self) -> IOPARST_W<2> {
        IOPARST_W::new(self)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopbrst(&mut self) -> IOPBRST_W<3> {
        IOPBRST_W::new(self)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopcrst(&mut self) -> IOPCRST_W<4> {
        IOPCRST_W::new(self)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopdrst(&mut self) -> IOPDRST_W<5> {
        IOPDRST_W::new(self)
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioperst(&mut self) -> IOPERST_W<6> {
        IOPERST_W::new(self)
    }
    #[doc = "Bit 9 - ADC 1 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<9> {
        ADC1RST_W::new(self)
    }
    #[doc = "Bit 10 - ADC 2 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc2rst(&mut self) -> ADC2RST_W<10> {
        ADC2RST_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rstr::R](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
