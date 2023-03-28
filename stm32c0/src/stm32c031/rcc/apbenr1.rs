#[doc = "Register `APBENR1` reader"]
pub struct R(crate::R<APBENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBENR1` writer"]
pub struct W(crate::W<APBENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBENR1_SPEC>;
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
impl From<crate::W<APBENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable Set and cleared by software."]
pub type TIM3EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable Set and cleared by software."]
pub type TIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable Set and cleared by software."]
pub type RTCAPBEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable Set and cleared by software."]
pub type RTCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `WWDGEN` reader - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
pub type WWDGEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGEN` writer - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
pub type WWDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `USART2EN` reader - USART2 clock enable Set and cleared by software."]
pub type USART2EN_R = crate::BitReader<bool>;
#[doc = "Field `USART2EN` writer - USART2 clock enable Set and cleared by software."]
pub type USART2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable Set and cleared by software."]
pub type I2C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable Set and cleared by software."]
pub type I2C1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `DBGEN` reader - Debug support clock enable Set and cleared by software."]
pub type DBGEN_R = crate::BitReader<bool>;
#[doc = "Field `DBGEN` writer - Debug support clock enable Set and cleared by software."]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
#[doc = "Field `PWREN` reader - Power interface clock enable Set and cleared by software."]
pub type PWREN_R = crate::BitReader<bool>;
#[doc = "Field `PWREN` writer - Power interface clock enable Set and cleared by software."]
pub type PWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBENR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<10> {
        RTCAPBEN_W::new(self)
    }
    #[doc = "Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 27 - Debug support clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<27> {
        DBGEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB peripheral clock enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbenr1](index.html) module"]
pub struct APBENR1_SPEC;
impl crate::RegisterSpec for APBENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbenr1::R](R) reader structure"]
impl crate::Readable for APBENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbenr1::W](W) writer structure"]
impl crate::Writable for APBENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBENR1 to value 0"]
impl crate::Resettable for APBENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
