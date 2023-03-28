#[doc = "Register `RCC_MP_APB2ENSETR` reader"]
pub struct R(crate::R<RCC_MP_APB2ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB2ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB2ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB2ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB2ENSETR` writer"]
pub struct W(crate::W<RCC_MP_APB2ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB2ENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_APB2ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB2ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1EN` reader - TIM1EN"]
pub type TIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1EN` writer - TIM1EN"]
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `TIM8EN` reader - TIM8EN"]
pub type TIM8EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM8EN` writer - TIM8EN"]
pub type TIM8EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `TIM15EN` reader - TIM15EN"]
pub type TIM15EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM15EN` writer - TIM15EN"]
pub type TIM15EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `TIM16EN` reader - TIM16EN"]
pub type TIM16EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM16EN` writer - TIM16EN"]
pub type TIM16EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `TIM17EN` reader - TIM17EN"]
pub type TIM17EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM17EN` writer - TIM17EN"]
pub type TIM17EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `SPI1EN` reader - SPI1EN"]
pub type SPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1EN` writer - SPI1EN"]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `SPI4EN` reader - SPI4EN"]
pub type SPI4EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI4EN` writer - SPI4EN"]
pub type SPI4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `SPI5EN` reader - SPI5EN"]
pub type SPI5EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI5EN` writer - SPI5EN"]
pub type SPI5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `USART6EN` reader - USART6EN"]
pub type USART6EN_R = crate::BitReader<bool>;
#[doc = "Field `USART6EN` writer - USART6EN"]
pub type USART6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `SAI1EN` reader - SAI1EN"]
pub type SAI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SAI1EN` writer - SAI1EN"]
pub type SAI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `SAI2EN` reader - SAI2EN"]
pub type SAI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SAI2EN` writer - SAI2EN"]
pub type SAI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `SAI3EN` reader - SAI3EN"]
pub type SAI3EN_R = crate::BitReader<bool>;
#[doc = "Field `SAI3EN` writer - SAI3EN"]
pub type SAI3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `DFSDMEN` reader - DFSDMEN"]
pub type DFSDMEN_R = crate::BitReader<bool>;
#[doc = "Field `DFSDMEN` writer - DFSDMEN"]
pub type DFSDMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `ADFSDMEN` reader - ADFSDMEN"]
pub type ADFSDMEN_R = crate::BitReader<bool>;
#[doc = "Field `ADFSDMEN` writer - ADFSDMEN"]
pub type ADFSDMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
#[doc = "Field `FDCANEN` reader - FDCANEN"]
pub type FDCANEN_R = crate::BitReader<bool>;
#[doc = "Field `FDCANEN` writer - FDCANEN"]
pub type FDCANEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB2ENSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15EN"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16EN"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17EN"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI4EN"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI5EN"]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - USART6EN"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - SAI1EN"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI2EN"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SAI3EN"]
    #[inline(always)]
    pub fn sai3en(&self) -> SAI3EN_R {
        SAI3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADFSDMEN"]
    #[inline(always)]
    pub fn adfsdmen(&self) -> ADFSDMEN_R {
        ADFSDMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - FDCANEN"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<0> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 1 - TIM8EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<1> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 2 - TIM15EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<2> {
        TIM15EN_W::new(self)
    }
    #[doc = "Bit 3 - TIM16EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<3> {
        TIM16EN_W::new(self)
    }
    #[doc = "Bit 4 - TIM17EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<4> {
        TIM17EN_W::new(self)
    }
    #[doc = "Bit 8 - SPI1EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<8> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 9 - SPI4EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<9> {
        SPI4EN_W::new(self)
    }
    #[doc = "Bit 10 - SPI5EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi5en(&mut self) -> SPI5EN_W<10> {
        SPI5EN_W::new(self)
    }
    #[doc = "Bit 13 - USART6EN"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<13> {
        USART6EN_W::new(self)
    }
    #[doc = "Bit 16 - SAI1EN"]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<16> {
        SAI1EN_W::new(self)
    }
    #[doc = "Bit 17 - SAI2EN"]
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<17> {
        SAI2EN_W::new(self)
    }
    #[doc = "Bit 18 - SAI3EN"]
    #[inline(always)]
    #[must_use]
    pub fn sai3en(&mut self) -> SAI3EN_W<18> {
        SAI3EN_W::new(self)
    }
    #[doc = "Bit 20 - DFSDMEN"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<20> {
        DFSDMEN_W::new(self)
    }
    #[doc = "Bit 21 - ADFSDMEN"]
    #[inline(always)]
    #[must_use]
    pub fn adfsdmen(&mut self) -> ADFSDMEN_W<21> {
        ADFSDMEN_W::new(self)
    }
    #[doc = "Bit 24 - FDCANEN"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<24> {
        FDCANEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb2ensetr](index.html) module"]
pub struct RCC_MP_APB2ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB2ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb2ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB2ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb2ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB2ENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_APB2ENSETR to value 0"]
impl crate::Resettable for RCC_MP_APB2ENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
