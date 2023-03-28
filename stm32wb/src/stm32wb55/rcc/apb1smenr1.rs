#[doc = "Register `APB1SMENR1` reader"]
pub struct R(crate::R<APB1SMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SMENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1SMENR1` writer"]
pub struct W(crate::W<APB1SMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1SMENR1_SPEC>;
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
impl From<crate::W<APB1SMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1SMENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clocks enable during CPU1 Sleep mode"]
pub type TIM2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clocks enable during CPU1 Sleep mode"]
pub type TIM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `LCDSMEN` reader - LCD clocks enable during CPU1 Sleep mode"]
pub type LCDSMEN_R = crate::BitReader<bool>;
#[doc = "Field `LCDSMEN` writer - LCD clocks enable during CPU1 Sleep mode"]
pub type LCDSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clocks enable during CPU1 Sleep mode"]
pub type RTCAPBSMEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clocks enable during CPU1 Sleep mode"]
pub type RTCAPBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `WWDGSMEN` reader - Window watchdog clocks enable during CPU1 Sleep mode"]
pub type WWDGSMEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGSMEN` writer - Window watchdog clocks enable during CPU1 Sleep mode"]
pub type WWDGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `SPI2SMEN` reader - SPI2 clocks enable during CPU1 Sleep mode"]
pub type SPI2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2SMEN` writer - SPI2 clocks enable during CPU1 Sleep mode"]
pub type SPI2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `I2C1SMEN` reader - I2C1 clocks enable during CPU1 Sleep mode"]
pub type I2C1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1SMEN` writer - I2C1 clocks enable during CPU1 Sleep mode"]
pub type I2C1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `I2C3SMEN` reader - I2C3 clocks enable during CPU1 Sleep mode"]
pub type I2C3SMEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C3SMEN` writer - I2C3 clocks enable during CPU1 Sleep mode"]
pub type I2C3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `CRSMEN` reader - CRS clocks enable during CPU1 Sleep mode"]
pub type CRSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CRSMEN` writer - CRS clocks enable during CPU1 Sleep mode"]
pub type CRSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `USBSMEN` reader - USB FS clocks enable during CPU1 Sleep mode"]
pub type USBSMEN_R = crate::BitReader<bool>;
#[doc = "Field `USBSMEN` writer - USB FS clocks enable during CPU1 Sleep mode"]
pub type USBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
#[doc = "Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during CPU1 Sleep mode"]
pub type LPTIM1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during CPU1 Sleep mode"]
pub type LPTIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn lcdsmen(&self) -> LCDSMEN_R {
        LCDSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn crsmen(&self) -> CRSMEN_R {
        CRSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - USB FS clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<0> {
        TIM2SMEN_W::new(self)
    }
    #[doc = "Bit 9 - LCD clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcdsmen(&mut self) -> LCDSMEN_W<9> {
        LCDSMEN_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<10> {
        RTCAPBSMEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<11> {
        WWDGSMEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<14> {
        SPI2SMEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<21> {
        I2C1SMEN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<23> {
        I2C3SMEN_W::new(self)
    }
    #[doc = "Bit 24 - CRS clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crsmen(&mut self) -> CRSMEN_W<24> {
        CRSMEN_W::new(self)
    }
    #[doc = "Bit 26 - USB FS clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbsmen(&mut self) -> USBSMEN_W<26> {
        USBSMEN_W::new(self)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<31> {
        LPTIM1SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1SMENR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr1](index.html) module"]
pub struct APB1SMENR1_SPEC;
impl crate::RegisterSpec for APB1SMENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1smenr1::R](R) reader structure"]
impl crate::Readable for APB1SMENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1smenr1::W](W) writer structure"]
impl crate::Writable for APB1SMENR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1SMENR1 to value 0x85a0_4e01"]
impl crate::Resettable for APB1SMENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x85a0_4e01;
}
