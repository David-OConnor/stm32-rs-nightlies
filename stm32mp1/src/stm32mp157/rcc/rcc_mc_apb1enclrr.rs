#[doc = "Register `RCC_MC_APB1ENCLRR` reader"]
pub struct R(crate::R<RCC_MC_APB1ENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_APB1ENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_APB1ENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_APB1ENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_APB1ENCLRR` writer"]
pub struct W(crate::W<RCC_MC_APB1ENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_APB1ENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_APB1ENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_APB1ENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2EN` reader - TIM2EN"]
pub type TIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2EN` writer - TIM2EN"]
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM3EN` reader - TIM3EN"]
pub type TIM3EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3EN` writer - TIM3EN"]
pub type TIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM4EN` reader - TIM4EN"]
pub type TIM4EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM4EN` writer - TIM4EN"]
pub type TIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM5EN` reader - TIM5EN"]
pub type TIM5EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM5EN` writer - TIM5EN"]
pub type TIM5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM6EN` reader - TIM6EN"]
pub type TIM6EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM6EN` writer - TIM6EN"]
pub type TIM6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM7EN` reader - TIM7EN"]
pub type TIM7EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM7EN` writer - TIM7EN"]
pub type TIM7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM12EN` reader - TIM12EN"]
pub type TIM12EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM12EN` writer - TIM12EN"]
pub type TIM12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM13EN` reader - TIM13EN"]
pub type TIM13EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM13EN` writer - TIM13EN"]
pub type TIM13EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `TIM14EN` reader - TIM14EN"]
pub type TIM14EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM14EN` writer - TIM14EN"]
pub type TIM14EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `LPTIM1EN` reader - LPTIM1EN"]
pub type LPTIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1EN` writer - LPTIM1EN"]
pub type LPTIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `SPI2EN` reader - SPI2EN"]
pub type SPI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2EN` writer - SPI2EN"]
pub type SPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `SPI3EN` reader - SPI3EN"]
pub type SPI3EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3EN` writer - SPI3EN"]
pub type SPI3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `USART2EN` reader - USART2EN"]
pub type USART2EN_R = crate::BitReader<bool>;
#[doc = "Field `USART2EN` writer - USART2EN"]
pub type USART2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `USART3EN` reader - USART3EN"]
pub type USART3EN_R = crate::BitReader<bool>;
#[doc = "Field `USART3EN` writer - USART3EN"]
pub type USART3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `UART4EN` reader - UART4EN"]
pub type UART4EN_R = crate::BitReader<bool>;
#[doc = "Field `UART4EN` writer - UART4EN"]
pub type UART4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `UART5EN` reader - UART5EN"]
pub type UART5EN_R = crate::BitReader<bool>;
#[doc = "Field `UART5EN` writer - UART5EN"]
pub type UART5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `UART7EN` reader - UART7EN"]
pub type UART7EN_R = crate::BitReader<bool>;
#[doc = "Field `UART7EN` writer - UART7EN"]
pub type UART7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `UART8EN` reader - UART8EN"]
pub type UART8EN_R = crate::BitReader<bool>;
#[doc = "Field `UART8EN` writer - UART8EN"]
pub type UART8EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `I2C1EN` reader - I2C1EN"]
pub type I2C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1EN` writer - I2C1EN"]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `I2C2EN` reader - I2C2EN"]
pub type I2C2EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C2EN` writer - I2C2EN"]
pub type I2C2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `I2C3EN` reader - I2C3EN"]
pub type I2C3EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C3EN` writer - I2C3EN"]
pub type I2C3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `I2C5EN` reader - I2C5EN"]
pub type I2C5EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C5EN` writer - I2C5EN"]
pub type I2C5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `SPDIFEN` reader - SPDIFEN"]
pub type SPDIFEN_R = crate::BitReader<bool>;
#[doc = "Field `SPDIFEN` writer - SPDIFEN"]
pub type SPDIFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `CECEN` reader - CECEN"]
pub type CECEN_R = crate::BitReader<bool>;
#[doc = "Field `CECEN` writer - CECEN"]
pub type CECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `DAC12EN` reader - DAC12EN"]
pub type DAC12EN_R = crate::BitReader<bool>;
#[doc = "Field `DAC12EN` writer - DAC12EN"]
pub type DAC12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
#[doc = "Field `MDIOSEN` reader - MDIOSEN"]
pub type MDIOSEN_R = crate::BitReader<bool>;
#[doc = "Field `MDIOSEN` writer - MDIOSEN"]
pub type MDIOSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB1ENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2EN"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3EN"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4EN"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5EN"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6EN"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7EN"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12EN"]
    #[inline(always)]
    pub fn tim12en(&self) -> TIM12EN_R {
        TIM12EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13EN"]
    #[inline(always)]
    pub fn tim13en(&self) -> TIM13EN_R {
        TIM13EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14EN"]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1EN"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI2EN"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI3EN"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART2EN"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USART3EN"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - UART4EN"]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - UART5EN"]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART7EN"]
    #[inline(always)]
    pub fn uart7en(&self) -> UART7EN_R {
        UART7EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART8EN"]
    #[inline(always)]
    pub fn uart8en(&self) -> UART8EN_R {
        UART8EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2EN"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3EN"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C5EN"]
    #[inline(always)]
    pub fn i2c5en(&self) -> I2C5EN_R {
        I2C5EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - SPDIFEN"]
    #[inline(always)]
    pub fn spdifen(&self) -> SPDIFEN_R {
        SPDIFEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CECEN"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC12EN"]
    #[inline(always)]
    pub fn dac12en(&self) -> DAC12EN_R {
        DAC12EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - MDIOSEN"]
    #[inline(always)]
    pub fn mdiosen(&self) -> MDIOSEN_R {
        MDIOSEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<2> {
        TIM4EN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<3> {
        TIM5EN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<5> {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 6 - TIM12EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim12en(&mut self) -> TIM12EN_W<6> {
        TIM12EN_W::new(self)
    }
    #[doc = "Bit 7 - TIM13EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim13en(&mut self) -> TIM13EN_W<7> {
        TIM13EN_W::new(self)
    }
    #[doc = "Bit 8 - TIM14EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> TIM14EN_W<8> {
        TIM14EN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<9> {
        LPTIM1EN_W::new(self)
    }
    #[doc = "Bit 11 - SPI2EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<11> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI3EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<12> {
        SPI3EN_W::new(self)
    }
    #[doc = "Bit 14 - USART2EN"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<14> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 15 - USART3EN"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<15> {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 16 - UART4EN"]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<16> {
        UART4EN_W::new(self)
    }
    #[doc = "Bit 17 - UART5EN"]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<17> {
        UART5EN_W::new(self)
    }
    #[doc = "Bit 18 - UART7EN"]
    #[inline(always)]
    #[must_use]
    pub fn uart7en(&mut self) -> UART7EN_W<18> {
        UART7EN_W::new(self)
    }
    #[doc = "Bit 19 - UART8EN"]
    #[inline(always)]
    #[must_use]
    pub fn uart8en(&mut self) -> UART8EN_W<19> {
        UART8EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<23> {
        I2C3EN_W::new(self)
    }
    #[doc = "Bit 24 - I2C5EN"]
    #[inline(always)]
    #[must_use]
    pub fn i2c5en(&mut self) -> I2C5EN_W<24> {
        I2C5EN_W::new(self)
    }
    #[doc = "Bit 26 - SPDIFEN"]
    #[inline(always)]
    #[must_use]
    pub fn spdifen(&mut self) -> SPDIFEN_W<26> {
        SPDIFEN_W::new(self)
    }
    #[doc = "Bit 27 - CECEN"]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<27> {
        CECEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC12EN"]
    #[inline(always)]
    #[must_use]
    pub fn dac12en(&mut self) -> DAC12EN_W<29> {
        DAC12EN_W::new(self)
    }
    #[doc = "Bit 31 - MDIOSEN"]
    #[inline(always)]
    #[must_use]
    pub fn mdiosen(&mut self) -> MDIOSEN_W<31> {
        MDIOSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb1enclrr](index.html) module"]
pub struct RCC_MC_APB1ENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_APB1ENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_apb1enclrr::R](R) reader structure"]
impl crate::Readable for RCC_MC_APB1ENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb1enclrr::W](W) writer structure"]
impl crate::Writable for RCC_MC_APB1ENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MC_APB1ENCLRR to value 0"]
impl crate::Resettable for RCC_MC_APB1ENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
