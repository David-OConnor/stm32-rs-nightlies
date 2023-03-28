#[doc = "Register `APB1ENR1` reader"]
pub struct R(crate::R<APB1ENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1ENR1` writer"]
pub struct W(crate::W<APB1ENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR1_SPEC>;
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
impl From<crate::W<APB1ENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2EN` reader - CPU1 TIM2 timer clock enable"]
pub type TIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2EN` writer - CPU1 TIM2 timer clock enable"]
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
#[doc = "Field `LCDEN` reader - CPU1 LCD clock enable"]
pub type LCDEN_R = crate::BitReader<bool>;
#[doc = "Field `LCDEN` writer - CPU1 LCD clock enable"]
pub type LCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
#[doc = "Field `RTCAPBEN` reader - CPU1 RTC APB clock enable"]
pub type RTCAPBEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBEN` writer - CPU1 RTC APB clock enable"]
pub type RTCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
#[doc = "Field `WWDGEN` reader - CPU1 Window watchdog clock enable"]
pub type WWDGEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGEN` writer - CPU1 Window watchdog clock enable"]
pub type WWDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
#[doc = "Field `SPI2EN` reader - CPU1 SPI2 clock enable"]
pub type SPI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2EN` writer - CPU1 SPI2 clock enable"]
pub type SPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
#[doc = "Field `I2C1EN` reader - CPU1 I2C1 clock enable"]
pub type I2C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1EN` writer - CPU1 I2C1 clock enable"]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
#[doc = "Field `I2C3EN` reader - CPU1 I2C3 clock enable"]
pub type I2C3EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C3EN` writer - CPU1 I2C3 clock enable"]
pub type I2C3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
#[doc = "Field `CRSEN` reader - CPU1 CRS clock enable"]
pub type CRSEN_R = crate::BitReader<bool>;
#[doc = "Field `CRSEN` writer - CPU1 CRS clock enable"]
pub type CRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
#[doc = "Field `USBEN` reader - CPU1 USB clock enable"]
pub type USBEN_R = crate::BitReader<bool>;
#[doc = "Field `USBEN` writer - CPU1 USB clock enable"]
pub type USBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
#[doc = "Field `LPTIM1EN` reader - CPU1 Low power timer 1 clock enable"]
pub type LPTIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1EN` writer - CPU1 Low power timer 1 clock enable"]
pub type LPTIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU1 TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - CPU1 LCD clock enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU1 RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU1 Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU1 SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU1 I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU1 I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU1 CRS clock enable"]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU1 USB clock enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU1 Low power timer 1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 TIM2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 9 - CPU1 LCD clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LCDEN_W<9> {
        LCDEN_W::new(self)
    }
    #[doc = "Bit 10 - CPU1 RTC APB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<10> {
        RTCAPBEN_W::new(self)
    }
    #[doc = "Bit 11 - CPU1 Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 14 - CPU1 SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 21 - CPU1 I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 23 - CPU1 I2C3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<23> {
        I2C3EN_W::new(self)
    }
    #[doc = "Bit 24 - CPU1 CRS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<24> {
        CRSEN_W::new(self)
    }
    #[doc = "Bit 26 - CPU1 USB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<26> {
        USBEN_W::new(self)
    }
    #[doc = "Bit 31 - CPU1 Low power timer 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<31> {
        LPTIM1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1ENR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr1](index.html) module"]
pub struct APB1ENR1_SPEC;
impl crate::RegisterSpec for APB1ENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1enr1::R](R) reader structure"]
impl crate::Readable for APB1ENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1enr1::W](W) writer structure"]
impl crate::Writable for APB1ENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1ENR1 to value 0x0400"]
impl crate::Resettable for APB1ENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
