#[doc = "Register `LPTIM_CR` reader"]
pub struct R(crate::R<LPTIM_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CR` writer"]
pub struct W(crate::W<LPTIM_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CR_SPEC>;
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
impl From<crate::W<LPTIM_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - LPTIM enable The ENABLE bit is set and cleared by software."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - LPTIM enable The ENABLE bit is set and cleared by software."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CR_SPEC, bool, O>;
#[doc = "Field `SNGSTRT` reader - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
pub type SNGSTRT_R = crate::BitReader<bool>;
#[doc = "Field `SNGSTRT` writer - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
pub type SNGSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CR_SPEC, bool, O>;
#[doc = "Field `CNTSTRT` reader - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
pub type CNTSTRT_R = crate::BitReader<bool>;
#[doc = "Field `CNTSTRT` writer - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
pub type CNTSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CR_SPEC, bool, O>;
#[doc = "Field `COUNTRST` reader - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
pub type COUNTRST_R = crate::BitReader<bool>;
#[doc = "Field `COUNTRST` writer - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
pub type COUNTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CR_SPEC, bool, O>;
#[doc = "Field `RSTARE` reader - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
pub type RSTARE_R = crate::BitReader<bool>;
#[doc = "Field `RSTARE` writer - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
pub type RSTARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<1> {
        SNGSTRT_W::new(self)
    }
    #[doc = "Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<2> {
        CNTSTRT_W::new(self)
    }
    #[doc = "Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn countrst(&mut self) -> COUNTRST_W<3> {
        COUNTRST_W::new(self)
    }
    #[doc = "Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rstare(&mut self) -> RSTARE_W<4> {
        RSTARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cr](index.html) module"]
pub struct LPTIM_CR_SPEC;
impl crate::RegisterSpec for LPTIM_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_cr::R](R) reader structure"]
impl crate::Readable for LPTIM_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_cr::W](W) writer structure"]
impl crate::Writable for LPTIM_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_CR to value 0"]
impl crate::Resettable for LPTIM_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
