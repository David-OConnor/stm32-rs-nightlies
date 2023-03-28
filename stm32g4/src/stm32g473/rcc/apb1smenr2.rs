#[doc = "Register `APB1SMENR2` reader"]
pub struct R(crate::R<APB1SMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SMENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1SMENR2` writer"]
pub struct W(crate::W<APB1SMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1SMENR2_SPEC>;
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
impl From<crate::W<APB1SMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1SMENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes"]
pub type LPUART1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes"]
pub type LPUART1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR2_SPEC, bool, O>;
#[doc = "Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes"]
pub type I2C4SMEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes"]
pub type I2C4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR2_SPEC, bool, O>;
#[doc = "Field `UCPD1SMEN` reader - UCPD1 clocks enable during Sleep and Stop modes"]
pub type UCPD1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `UCPD1SMEN` writer - UCPD1 clocks enable during Sleep and Stop modes"]
pub type UCPD1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1SMENR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - UCPD1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<0> {
        LPUART1SMEN_W::new(self)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<1> {
        I2C4SMEN_W::new(self)
    }
    #[doc = "Bit 8 - UCPD1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<8> {
        UCPD1SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr2](index.html) module"]
pub struct APB1SMENR2_SPEC;
impl crate::RegisterSpec for APB1SMENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1smenr2::R](R) reader structure"]
impl crate::Readable for APB1SMENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1smenr2::W](W) writer structure"]
impl crate::Writable for APB1SMENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1SMENR2 to value 0x0103"]
impl crate::Resettable for APB1SMENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0103;
}
