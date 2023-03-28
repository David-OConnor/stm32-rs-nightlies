#[doc = "Register `CCIPR` reader"]
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR` writer"]
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub type USART1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub type USART1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub type USART2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub type USART2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `USART3SEL` reader - USART3 clock source selection"]
pub type USART3SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART3SEL` writer - USART3 clock source selection"]
pub type USART3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CECSEL` reader - HDMI CEC clock source selection"]
pub type CECSEL_R = crate::BitReader<bool>;
#[doc = "Field `CECSEL` writer - HDMI CEC clock source selection"]
pub type CECSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `LPUART2SEL` reader - LPUART2 clock source selection"]
pub type LPUART2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPUART2SEL` writer - LPUART2 clock source selection"]
pub type LPUART2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub type LPUART1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub type LPUART1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2C1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2C1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2S2SEL` reader - I2S1 clock source selection"]
pub type I2S2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S2SEL` writer - I2S1 clock source selection"]
pub type I2S2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 clock source selection"]
pub type LPTIM1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 clock source selection"]
pub type LPTIM1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 clock source selection"]
pub type LPTIM2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 clock source selection"]
pub type LPTIM2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIM1SEL` reader - TIM1 clock source selection"]
pub type TIM1SEL_R = crate::BitReader<bool>;
#[doc = "Field `TIM1SEL` writer - TIM1 clock source selection"]
pub type TIM1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `TIM15SEL` reader - TIM15 clock source selection"]
pub type TIM15SEL_R = crate::BitReader<bool>;
#[doc = "Field `TIM15SEL` writer - TIM15 clock source selection"]
pub type TIM15SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `ADCSEL` reader - ADCs clock source selection"]
pub type ADCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCSEL` writer - ADCs clock source selection"]
pub type ADCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - LPUART2 clock source selection"]
    #[inline(always)]
    pub fn lpuart2sel(&self) -> LPUART2SEL_R {
        LPUART2SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2S1 clock source selection"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - LPTIM2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - TIM1 clock source selection"]
    #[inline(always)]
    pub fn tim1sel(&self) -> TIM1SEL_R {
        TIM1SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TIM15 clock source selection"]
    #[inline(always)]
    pub fn tim15sel(&self) -> TIM15SEL_R {
        TIM15SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 30:31 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<0> {
        USART1SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<2> {
        USART2SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<4> {
        USART3SEL_W::new(self)
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn cecsel(&mut self) -> CECSEL_W<6> {
        CECSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - LPUART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2sel(&mut self) -> LPUART2SEL_W<8> {
        LPUART2SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<10> {
        LPUART1SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<12> {
        I2C1SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - I2S1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W<14> {
        I2S2SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<18> {
        LPTIM1SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - LPTIM2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<20> {
        LPTIM2SEL_W::new(self)
    }
    #[doc = "Bit 22 - TIM1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sel(&mut self) -> TIM1SEL_W<22> {
        TIM1SEL_W::new(self)
    }
    #[doc = "Bit 24 - TIM15 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim15sel(&mut self) -> TIM15SEL_W<24> {
        TIM15SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - ADCs clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<30> {
        ADCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](index.html) module"]
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr::R](R) reader structure"]
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr::W](W) writer structure"]
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
