#[doc = "Register `GTZC1_TZIC_FCR1` writer"]
pub struct W(crate::W<GTZC1_TZIC_FCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZIC_FCR1_SPEC>;
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
impl From<crate::W<GTZC1_TZIC_FCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZIC_FCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIM2F` writer - clear the illegal access flag for TIM2"]
pub type CTIM2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CTIM3F` writer - clear the illegal access flag for TIM3"]
pub type CTIM3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CTIM4F` writer - clear the illegal access flag for TIM4"]
pub type CTIM4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CTIM5F` writer - clear the illegal access flag for TIM5"]
pub type CTIM5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CTIM6F` writer - clear the illegal access flag for TIM6"]
pub type CTIM6F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CTIM7F` writer - clear the illegal access flag for TIM7"]
pub type CTIM7F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CTIM12F` writer - clear the illegal access flag for TIM12"]
pub type CTIM12F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CTIM13F` writer - clear the illegal access flag for TIM13"]
pub type CTIM13F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CTIM14F` writer - clear the illegal access flag for TIM14"]
pub type CTIM14F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CWWDGF` writer - clear the illegal access flag for WWDG"]
pub type CWWDGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CIWDGF` writer - clear the illegal access flag for IWDG"]
pub type CIWDGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CSPI2F` writer - clear the illegal access flag for SPI2"]
pub type CSPI2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CSPI3F` writer - clear the illegal access flag for SPI3"]
pub type CSPI3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUSART2F` writer - clear the illegal access flag for USART2"]
pub type CUSART2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUSART3F` writer - clear the illegal access flag for USART3"]
pub type CUSART3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUART4F` writer - clear the illegal access flag for UART4"]
pub type CUART4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUART5F` writer - clear the illegal access flag for UART5"]
pub type CUART5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CI2C1F` writer - clear the illegal access flag for I2C1"]
pub type CI2C1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CI2C2F` writer - clear the illegal access flag for I2C2"]
pub type CI2C2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CI3C1F` writer - clear the illegal access flag for I3C1"]
pub type CI3C1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CCRSF` writer - clear the illegal access flag for CRS"]
pub type CCRSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUSART6F` writer - clear the illegal access flag for USART6"]
pub type CUSART6F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUSART10F` writer - clear the illegal access flag for USART10"]
pub type CUSART10F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUSART11F` writer - clear the illegal access flag for USART11"]
pub type CUSART11F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CHDMICECF` writer - clear the illegal access flag for HDMICEC"]
pub type CHDMICECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CDAC1F` writer - clear the illegal access flag for DAC1"]
pub type CDAC1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUART7F` writer - clear the illegal access flag for UART7"]
pub type CUART7F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUART8F` writer - clear the illegal access flag for UART8"]
pub type CUART8F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUART9F` writer - clear the illegal access flag for UART9"]
pub type CUART9F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CUART12F` writer - clear the illegal access flag for UART12"]
pub type CUART12F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CDTSF` writer - clear the illegal access flag for DTS"]
pub type CDTSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
#[doc = "Field `CLPTIM2F` writer - clear the illegal access flag for LPTIM2"]
pub type CLPTIM2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR1_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for TIM2"]
    #[inline(always)]
    #[must_use]
    pub fn ctim2f(&mut self) -> CTIM2F_W<0> {
        CTIM2F_W::new(self)
    }
    #[doc = "Bit 1 - clear the illegal access flag for TIM3"]
    #[inline(always)]
    #[must_use]
    pub fn ctim3f(&mut self) -> CTIM3F_W<1> {
        CTIM3F_W::new(self)
    }
    #[doc = "Bit 2 - clear the illegal access flag for TIM4"]
    #[inline(always)]
    #[must_use]
    pub fn ctim4f(&mut self) -> CTIM4F_W<2> {
        CTIM4F_W::new(self)
    }
    #[doc = "Bit 3 - clear the illegal access flag for TIM5"]
    #[inline(always)]
    #[must_use]
    pub fn ctim5f(&mut self) -> CTIM5F_W<3> {
        CTIM5F_W::new(self)
    }
    #[doc = "Bit 4 - clear the illegal access flag for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn ctim6f(&mut self) -> CTIM6F_W<4> {
        CTIM6F_W::new(self)
    }
    #[doc = "Bit 5 - clear the illegal access flag for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn ctim7f(&mut self) -> CTIM7F_W<5> {
        CTIM7F_W::new(self)
    }
    #[doc = "Bit 6 - clear the illegal access flag for TIM12"]
    #[inline(always)]
    #[must_use]
    pub fn ctim12f(&mut self) -> CTIM12F_W<6> {
        CTIM12F_W::new(self)
    }
    #[doc = "Bit 7 - clear the illegal access flag for TIM13"]
    #[inline(always)]
    #[must_use]
    pub fn ctim13f(&mut self) -> CTIM13F_W<7> {
        CTIM13F_W::new(self)
    }
    #[doc = "Bit 8 - clear the illegal access flag for TIM14"]
    #[inline(always)]
    #[must_use]
    pub fn ctim14f(&mut self) -> CTIM14F_W<8> {
        CTIM14F_W::new(self)
    }
    #[doc = "Bit 9 - clear the illegal access flag for WWDG"]
    #[inline(always)]
    #[must_use]
    pub fn cwwdgf(&mut self) -> CWWDGF_W<9> {
        CWWDGF_W::new(self)
    }
    #[doc = "Bit 10 - clear the illegal access flag for IWDG"]
    #[inline(always)]
    #[must_use]
    pub fn ciwdgf(&mut self) -> CIWDGF_W<10> {
        CIWDGF_W::new(self)
    }
    #[doc = "Bit 11 - clear the illegal access flag for SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn cspi2f(&mut self) -> CSPI2F_W<11> {
        CSPI2F_W::new(self)
    }
    #[doc = "Bit 12 - clear the illegal access flag for SPI3"]
    #[inline(always)]
    #[must_use]
    pub fn cspi3f(&mut self) -> CSPI3F_W<12> {
        CSPI3F_W::new(self)
    }
    #[doc = "Bit 13 - clear the illegal access flag for USART2"]
    #[inline(always)]
    #[must_use]
    pub fn cusart2f(&mut self) -> CUSART2F_W<13> {
        CUSART2F_W::new(self)
    }
    #[doc = "Bit 14 - clear the illegal access flag for USART3"]
    #[inline(always)]
    #[must_use]
    pub fn cusart3f(&mut self) -> CUSART3F_W<14> {
        CUSART3F_W::new(self)
    }
    #[doc = "Bit 15 - clear the illegal access flag for UART4"]
    #[inline(always)]
    #[must_use]
    pub fn cuart4f(&mut self) -> CUART4F_W<15> {
        CUART4F_W::new(self)
    }
    #[doc = "Bit 16 - clear the illegal access flag for UART5"]
    #[inline(always)]
    #[must_use]
    pub fn cuart5f(&mut self) -> CUART5F_W<16> {
        CUART5F_W::new(self)
    }
    #[doc = "Bit 17 - clear the illegal access flag for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c1f(&mut self) -> CI2C1F_W<17> {
        CI2C1F_W::new(self)
    }
    #[doc = "Bit 18 - clear the illegal access flag for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c2f(&mut self) -> CI2C2F_W<18> {
        CI2C2F_W::new(self)
    }
    #[doc = "Bit 19 - clear the illegal access flag for I3C1"]
    #[inline(always)]
    #[must_use]
    pub fn ci3c1f(&mut self) -> CI3C1F_W<19> {
        CI3C1F_W::new(self)
    }
    #[doc = "Bit 20 - clear the illegal access flag for CRS"]
    #[inline(always)]
    #[must_use]
    pub fn ccrsf(&mut self) -> CCRSF_W<20> {
        CCRSF_W::new(self)
    }
    #[doc = "Bit 21 - clear the illegal access flag for USART6"]
    #[inline(always)]
    #[must_use]
    pub fn cusart6f(&mut self) -> CUSART6F_W<21> {
        CUSART6F_W::new(self)
    }
    #[doc = "Bit 22 - clear the illegal access flag for USART10"]
    #[inline(always)]
    #[must_use]
    pub fn cusart10f(&mut self) -> CUSART10F_W<22> {
        CUSART10F_W::new(self)
    }
    #[doc = "Bit 23 - clear the illegal access flag for USART11"]
    #[inline(always)]
    #[must_use]
    pub fn cusart11f(&mut self) -> CUSART11F_W<23> {
        CUSART11F_W::new(self)
    }
    #[doc = "Bit 24 - clear the illegal access flag for HDMICEC"]
    #[inline(always)]
    #[must_use]
    pub fn chdmicecf(&mut self) -> CHDMICECF_W<24> {
        CHDMICECF_W::new(self)
    }
    #[doc = "Bit 25 - clear the illegal access flag for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn cdac1f(&mut self) -> CDAC1F_W<25> {
        CDAC1F_W::new(self)
    }
    #[doc = "Bit 26 - clear the illegal access flag for UART7"]
    #[inline(always)]
    #[must_use]
    pub fn cuart7f(&mut self) -> CUART7F_W<26> {
        CUART7F_W::new(self)
    }
    #[doc = "Bit 27 - clear the illegal access flag for UART8"]
    #[inline(always)]
    #[must_use]
    pub fn cuart8f(&mut self) -> CUART8F_W<27> {
        CUART8F_W::new(self)
    }
    #[doc = "Bit 28 - clear the illegal access flag for UART9"]
    #[inline(always)]
    #[must_use]
    pub fn cuart9f(&mut self) -> CUART9F_W<28> {
        CUART9F_W::new(self)
    }
    #[doc = "Bit 29 - clear the illegal access flag for UART12"]
    #[inline(always)]
    #[must_use]
    pub fn cuart12f(&mut self) -> CUART12F_W<29> {
        CUART12F_W::new(self)
    }
    #[doc = "Bit 30 - clear the illegal access flag for DTS"]
    #[inline(always)]
    #[must_use]
    pub fn cdtsf(&mut self) -> CDTSF_W<30> {
        CDTSF_W::new(self)
    }
    #[doc = "Bit 31 - clear the illegal access flag for LPTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn clptim2f(&mut self) -> CLPTIM2F_W<31> {
        CLPTIM2F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC flag clear register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzic_fcr1](index.html) module"]
pub struct GTZC1_TZIC_FCR1_SPEC;
impl crate::RegisterSpec for GTZC1_TZIC_FCR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzic_fcr1::W](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_FCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_FCR1 to value 0"]
impl crate::Resettable for GTZC1_TZIC_FCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
