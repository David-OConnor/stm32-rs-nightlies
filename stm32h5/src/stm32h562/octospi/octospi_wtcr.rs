#[doc = "Register `OCTOSPI_WTCR` reader"]
pub struct R(crate::R<OCTOSPI_WTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_WTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_WTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_WTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_WTCR` writer"]
pub struct W(crate::W<OCTOSPI_WTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_WTCR_SPEC>;
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
impl From<crate::W<OCTOSPI_WTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_WTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
pub type DCYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
pub type DCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_WTCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<0> {
        DCYC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI write timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_wtcr](index.html) module"]
pub struct OCTOSPI_WTCR_SPEC;
impl crate::RegisterSpec for OCTOSPI_WTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_wtcr::R](R) reader structure"]
impl crate::Readable for OCTOSPI_WTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_wtcr::W](W) writer structure"]
impl crate::Writable for OCTOSPI_WTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_WTCR to value 0"]
impl crate::Resettable for OCTOSPI_WTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
