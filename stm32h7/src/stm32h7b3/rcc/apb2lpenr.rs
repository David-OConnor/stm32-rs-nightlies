#[doc = "Register `APB2LPENR` reader"]
pub struct R(crate::R<APB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2LPENR` writer"]
pub struct W(crate::W<APB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2LPENR_SPEC>;
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
impl From<crate::W<APB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1LPEN` reader - TIM1 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM1LPEN_R = crate::BitReader<TIM1LPEN_A>;
#[doc = "TIM1 peripheral clock enable during CSleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1LPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<TIM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1LPEN_A {
        match self.bits {
            false => TIM1LPEN_A::Disabled,
            true => TIM1LPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1LPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1LPEN_A::Enabled
    }
}
#[doc = "Field `TIM1LPEN` writer - TIM1 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, TIM1LPEN_A, O>;
impl<'a, const O: u8> TIM1LPEN_W<'a, O> {
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::Enabled)
    }
}
#[doc = "Field `TIM8LPEN` reader - TIM8 peripheral clock enable during CSleep mode Set and reset by software."]
pub use TIM1LPEN_R as TIM8LPEN_R;
#[doc = "Field `USART1LPEN` reader - USART1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as USART1LPEN_R;
#[doc = "Field `USART6LPEN` reader - USART6 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART6 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as USART6LPEN_R;
#[doc = "Field `UART9LPEN` reader - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as UART9LPEN_R;
#[doc = "Field `USART10LPEN` reader - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as USART10LPEN_R;
#[doc = "Field `SPI1LPEN` reader - SPI1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI1 are: the kernel clock selected by I2S123SRC and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as SPI1LPEN_R;
#[doc = "Field `SPI4LPEN` reader - SPI4 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as SPI4LPEN_R;
#[doc = "Field `TIM15LPEN` reader - TIM15 peripheral clock enable during CSleep mode Set and reset by software."]
pub use TIM1LPEN_R as TIM15LPEN_R;
#[doc = "Field `TIM16LPEN` reader - TIM16 peripheral clock enable during CSleep mode Set and reset by software."]
pub use TIM1LPEN_R as TIM16LPEN_R;
#[doc = "Field `TIM17LPEN` reader - TIM17 peripheral clock enable during CSleep mode Set and reset by software."]
pub use TIM1LPEN_R as TIM17LPEN_R;
#[doc = "Field `SPI5LPEN` reader - SPI5 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as SPI5LPEN_R;
#[doc = "Field `SAI1LPEN` reader - SAI1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as SAI1LPEN_R;
#[doc = "Field `SAI2LPEN` reader - SAI2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI23EL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as SAI2LPEN_R;
#[doc = "Field `DFSDM1LPEN` reader - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_R as DFSDM1LPEN_R;
#[doc = "Field `TIM8LPEN` writer - TIM8 peripheral clock enable during CSleep mode Set and reset by software."]
pub use TIM1LPEN_W as TIM8LPEN_W;
#[doc = "Field `USART1LPEN` writer - USART1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as USART1LPEN_W;
#[doc = "Field `USART6LPEN` writer - USART6 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART6 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as USART6LPEN_W;
#[doc = "Field `UART9LPEN` writer - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as UART9LPEN_W;
#[doc = "Field `USART10LPEN` writer - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as USART10LPEN_W;
#[doc = "Field `SPI1LPEN` writer - SPI1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI1 are: the kernel clock selected by I2S123SRC and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as SPI1LPEN_W;
#[doc = "Field `SPI4LPEN` writer - SPI4 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as SPI4LPEN_W;
#[doc = "Field `TIM15LPEN` writer - TIM15 peripheral clock enable during CSleep mode Set and reset by software."]
pub use TIM1LPEN_W as TIM15LPEN_W;
#[doc = "Field `TIM16LPEN` writer - TIM16 peripheral clock enable during CSleep mode Set and reset by software."]
pub use TIM1LPEN_W as TIM16LPEN_W;
#[doc = "Field `TIM17LPEN` writer - TIM17 peripheral clock enable during CSleep mode Set and reset by software."]
pub use TIM1LPEN_W as TIM17LPEN_W;
#[doc = "Field `SPI5LPEN` writer - SPI5 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as SPI5LPEN_W;
#[doc = "Field `SAI1LPEN` writer - SAI1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as SAI1LPEN_W;
#[doc = "Field `SAI2LPEN` writer - SAI2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI23EL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as SAI2LPEN_W;
#[doc = "Field `DFSDM1LPEN` writer - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock."]
pub use TIM1LPEN_W as DFSDM1LPEN_W;
impl R {
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART6 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn uart9lpen(&self) -> UART9LPEN_R {
        UART9LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn usart10lpen(&self) -> USART10LPEN_R {
        USART10LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI1 are: the kernel clock selected by I2S123SRC and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI23EL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn dfsdm1lpen(&self) -> DFSDM1LPEN_R {
        DFSDM1LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<0> {
        TIM1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<1> {
        TIM8LPEN_W::new(self)
    }
    #[doc = "Bit 4 - USART1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<4> {
        USART1LPEN_W::new(self)
    }
    #[doc = "Bit 5 - USART6 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART6 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<5> {
        USART6LPEN_W::new(self)
    }
    #[doc = "Bit 6 - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn uart9lpen(&mut self) -> UART9LPEN_W<6> {
        UART9LPEN_W::new(self)
    }
    #[doc = "Bit 7 - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn usart10lpen(&mut self) -> USART10LPEN_W<7> {
        USART10LPEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI1 are: the kernel clock selected by I2S123SRC and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<12> {
        SPI1LPEN_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<13> {
        SPI4LPEN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<16> {
        TIM15LPEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<17> {
        TIM16LPEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<18> {
        TIM17LPEN_W::new(self)
    }
    #[doc = "Bit 20 - SPI5 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<20> {
        SPI5LPEN_W::new(self)
    }
    #[doc = "Bit 22 - SAI1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<22> {
        SAI1LPEN_W::new(self)
    }
    #[doc = "Bit 23 - SAI2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI23EL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<23> {
        SAI2LPEN_W::new(self)
    }
    #[doc = "Bit 30 - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1lpen(&mut self) -> DFSDM1LPEN_W<30> {
        DFSDM1LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2lpenr](index.html) module"]
pub struct APB2LPENR_SPEC;
impl crate::RegisterSpec for APB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2lpenr::R](R) reader structure"]
impl crate::Readable for APB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2lpenr::W](W) writer structure"]
impl crate::Writable for APB2LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2LPENR to value 0x40d7_30f3"]
impl crate::Resettable for APB2LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x40d7_30f3;
}
