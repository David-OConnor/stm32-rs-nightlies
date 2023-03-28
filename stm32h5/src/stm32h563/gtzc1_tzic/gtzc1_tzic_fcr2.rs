#[doc = "Register `GTZC1_TZIC_FCR2` writer"]
pub struct W(crate::W<GTZC1_TZIC_FCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZIC_FCR2_SPEC>;
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
impl From<crate::W<GTZC1_TZIC_FCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZIC_FCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFDCAN1F` writer - clear the illegal access flag for FDCAN1"]
pub type CFDCAN1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CFDCAN2F` writer - clear the illegal access flag for FDCAN2"]
pub type CFDCAN2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CUCPDF` writer - clear the illegal access flag for UCPD"]
pub type CUCPDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CTIM1F` writer - clear the illegal access flag for TIM1"]
pub type CTIM1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CSPI1F` writer - clear the illegal access flag for SPI1"]
pub type CSPI1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CTIM8F` writer - clear the illegal access flag for TIM8"]
pub type CTIM8F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CUSART1F` writer - clear the illegal access flag for USART1"]
pub type CUSART1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CTIM15F` writer - clear the illegal access flag for TIM15"]
pub type CTIM15F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CTIM16F` writer - clear the illegal access flag for TIM16"]
pub type CTIM16F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CTIM17F` writer - clear the illegal access flag for TIM17"]
pub type CTIM17F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CSPI4F` writer - clear the illegal access flag for SPI4"]
pub type CSPI4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CSPI6F` writer - clear the illegal access flag for SPI6"]
pub type CSPI6F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CSAI1F` writer - clear the illegal access flag for SAI1"]
pub type CSAI1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CSAI2F` writer - clear the illegal access flag for SAI2"]
pub type CSAI2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CUSBF` writer - clear the illegal access flag for USB"]
pub type CUSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CSPI5F` writer - clear the illegal access flag for SPI5"]
pub type CSPI5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CLPUART1F` writer - clear the illegal access flag for LPUART"]
pub type CLPUART1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CI2C3F` writer - clear the illegal access flag for I2C3"]
pub type CI2C3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CI2C4F` writer - clear the illegal access flag for I2C4"]
pub type CI2C4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CLPTIM1F` writer - clear the illegal access flag for LPTIM1"]
pub type CLPTIM1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CLPTIM3F` writer - clear the illegal access flag for LPTIM3"]
pub type CLPTIM3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CLPTIM4F` writer - clear the illegal access flag for LPTIM4"]
pub type CLPTIM4F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
#[doc = "Field `CLPTIM5F` writer - clear the illegal access flag for LPTIM5"]
pub type CLPTIM5F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR2_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn cfdcan1f(&mut self) -> CFDCAN1F_W<0> {
        CFDCAN1F_W::new(self)
    }
    #[doc = "Bit 1 - clear the illegal access flag for FDCAN2"]
    #[inline(always)]
    #[must_use]
    pub fn cfdcan2f(&mut self) -> CFDCAN2F_W<1> {
        CFDCAN2F_W::new(self)
    }
    #[doc = "Bit 2 - clear the illegal access flag for UCPD"]
    #[inline(always)]
    #[must_use]
    pub fn cucpdf(&mut self) -> CUCPDF_W<2> {
        CUCPDF_W::new(self)
    }
    #[doc = "Bit 8 - clear the illegal access flag for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn ctim1f(&mut self) -> CTIM1F_W<8> {
        CTIM1F_W::new(self)
    }
    #[doc = "Bit 9 - clear the illegal access flag for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn cspi1f(&mut self) -> CSPI1F_W<9> {
        CSPI1F_W::new(self)
    }
    #[doc = "Bit 10 - clear the illegal access flag for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn ctim8f(&mut self) -> CTIM8F_W<10> {
        CTIM8F_W::new(self)
    }
    #[doc = "Bit 11 - clear the illegal access flag for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn cusart1f(&mut self) -> CUSART1F_W<11> {
        CUSART1F_W::new(self)
    }
    #[doc = "Bit 12 - clear the illegal access flag for TIM15"]
    #[inline(always)]
    #[must_use]
    pub fn ctim15f(&mut self) -> CTIM15F_W<12> {
        CTIM15F_W::new(self)
    }
    #[doc = "Bit 13 - clear the illegal access flag for TIM16"]
    #[inline(always)]
    #[must_use]
    pub fn ctim16f(&mut self) -> CTIM16F_W<13> {
        CTIM16F_W::new(self)
    }
    #[doc = "Bit 14 - clear the illegal access flag for TIM17"]
    #[inline(always)]
    #[must_use]
    pub fn ctim17f(&mut self) -> CTIM17F_W<14> {
        CTIM17F_W::new(self)
    }
    #[doc = "Bit 15 - clear the illegal access flag for SPI4"]
    #[inline(always)]
    #[must_use]
    pub fn cspi4f(&mut self) -> CSPI4F_W<15> {
        CSPI4F_W::new(self)
    }
    #[doc = "Bit 16 - clear the illegal access flag for SPI6"]
    #[inline(always)]
    #[must_use]
    pub fn cspi6f(&mut self) -> CSPI6F_W<16> {
        CSPI6F_W::new(self)
    }
    #[doc = "Bit 17 - clear the illegal access flag for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn csai1f(&mut self) -> CSAI1F_W<17> {
        CSAI1F_W::new(self)
    }
    #[doc = "Bit 18 - clear the illegal access flag for SAI2"]
    #[inline(always)]
    #[must_use]
    pub fn csai2f(&mut self) -> CSAI2F_W<18> {
        CSAI2F_W::new(self)
    }
    #[doc = "Bit 19 - clear the illegal access flag for USB"]
    #[inline(always)]
    #[must_use]
    pub fn cusbf(&mut self) -> CUSBF_W<19> {
        CUSBF_W::new(self)
    }
    #[doc = "Bit 24 - clear the illegal access flag for SPI5"]
    #[inline(always)]
    #[must_use]
    pub fn cspi5f(&mut self) -> CSPI5F_W<24> {
        CSPI5F_W::new(self)
    }
    #[doc = "Bit 25 - clear the illegal access flag for LPUART"]
    #[inline(always)]
    #[must_use]
    pub fn clpuart1f(&mut self) -> CLPUART1F_W<25> {
        CLPUART1F_W::new(self)
    }
    #[doc = "Bit 26 - clear the illegal access flag for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c3f(&mut self) -> CI2C3F_W<26> {
        CI2C3F_W::new(self)
    }
    #[doc = "Bit 27 - clear the illegal access flag for I2C4"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c4f(&mut self) -> CI2C4F_W<27> {
        CI2C4F_W::new(self)
    }
    #[doc = "Bit 28 - clear the illegal access flag for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn clptim1f(&mut self) -> CLPTIM1F_W<28> {
        CLPTIM1F_W::new(self)
    }
    #[doc = "Bit 29 - clear the illegal access flag for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn clptim3f(&mut self) -> CLPTIM3F_W<29> {
        CLPTIM3F_W::new(self)
    }
    #[doc = "Bit 30 - clear the illegal access flag for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn clptim4f(&mut self) -> CLPTIM4F_W<30> {
        CLPTIM4F_W::new(self)
    }
    #[doc = "Bit 31 - clear the illegal access flag for LPTIM5"]
    #[inline(always)]
    #[must_use]
    pub fn clptim5f(&mut self) -> CLPTIM5F_W<31> {
        CLPTIM5F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC flag clear register 2\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzic_fcr2](index.html) module"]
pub struct GTZC1_TZIC_FCR2_SPEC;
impl crate::RegisterSpec for GTZC1_TZIC_FCR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzic_fcr2::W](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_FCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_FCR2 to value 0"]
impl crate::Resettable for GTZC1_TZIC_FCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
