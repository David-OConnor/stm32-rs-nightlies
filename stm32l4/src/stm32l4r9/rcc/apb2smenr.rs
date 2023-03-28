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
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SYSCFGSMEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SYSCFGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type TIM1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type TIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clocks enable during Sleep and Stop modes"]
pub type SPI1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1SMEN` writer - SPI1 clocks enable during Sleep and Stop modes"]
pub type SPI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `TIM8SMEN` reader - TIM8 timer clocks enable during Sleep and Stop modes"]
pub type TIM8SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM8SMEN` writer - TIM8 timer clocks enable during Sleep and Stop modes"]
pub type TIM8SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `USART1SMEN` reader - USART1clocks enable during Sleep and Stop modes"]
pub type USART1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `USART1SMEN` writer - USART1clocks enable during Sleep and Stop modes"]
pub type USART1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clocks enable during Sleep and Stop modes"]
pub type TIM15SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clocks enable during Sleep and Stop modes"]
pub type TIM15SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clocks enable during Sleep and Stop modes"]
pub type TIM16SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clocks enable during Sleep and Stop modes"]
pub type TIM16SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clocks enable during Sleep and Stop modes"]
pub type TIM17SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clocks enable during Sleep and Stop modes"]
pub type TIM17SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `SAI1SMEN` reader - SAI1 clocks enable during Sleep and Stop modes"]
pub type SAI1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SAI1SMEN` writer - SAI1 clocks enable during Sleep and Stop modes"]
pub type SAI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `SAI2SMEN` reader - SAI2 clocks enable during Sleep and Stop modes"]
pub type SAI2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SAI2SMEN` writer - SAI2 clocks enable during Sleep and Stop modes"]
pub type SAI2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `DFSDM1SMEN` reader - DFSDM timer clocks enable during Sleep and Stop modes"]
pub type DFSDM1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DFSDM1SMEN` writer - DFSDM timer clocks enable during Sleep and Stop modes"]
pub type DFSDM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `LTDCSMEN` reader - LCD-TFT timer clocks enable during Sleep and Stop modes"]
pub type LTDCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `LTDCSMEN` writer - LCD-TFT timer clocks enable during Sleep and Stop modes"]
pub type LTDCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
#[doc = "Field `DSISMEN` reader - DSI clocks enable during Sleep and Stop modes"]
pub type DSISMEN_R = crate::BitReader<bool>;
#[doc = "Field `DSISMEN` writer - DSI clocks enable during Sleep and Stop modes"]
pub type DSISMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2SMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim8smen(&self) -> TIM8SMEN_R {
        TIM8SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai2smen(&self) -> SAI2SMEN_R {
        SAI2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dfsdm1smen(&self) -> DFSDM1SMEN_R {
        DFSDM1SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ltdcsmen(&self) -> LTDCSMEN_R {
        LTDCSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dsismen(&self) -> DSISMEN_R {
        DSISMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<0> {
        SYSCFGSMEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<11> {
        TIM1SMEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<12> {
        SPI1SMEN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim8smen(&mut self) -> TIM8SMEN_W<13> {
        TIM8SMEN_W::new(self)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<14> {
        USART1SMEN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W<16> {
        TIM15SMEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<17> {
        TIM16SMEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<18> {
        TIM17SMEN_W::new(self)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W<21> {
        SAI1SMEN_W::new(self)
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sai2smen(&mut self) -> SAI2SMEN_W<22> {
        SAI2SMEN_W::new(self)
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1smen(&mut self) -> DFSDM1SMEN_W<24> {
        DFSDM1SMEN_W::new(self)
    }
    #[doc = "Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcsmen(&mut self) -> LTDCSMEN_W<26> {
        LTDCSMEN_W::new(self)
    }
    #[doc = "Bit 27 - DSI clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dsismen(&mut self) -> DSISMEN_W<27> {
        DSISMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2SMENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2smenr](index.html) module"]
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
#[doc = "`reset()` method sets APB2SMENR to value 0x0167_7c01"]
impl crate::Resettable for APB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0167_7c01;
}
