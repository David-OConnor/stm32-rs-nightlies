#[doc = "Register `FMC_SDRTR` reader"]
pub struct R(crate::R<FMC_SDRTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_SDRTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_SDRTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_SDRTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_SDRTR` writer"]
pub struct W(crate::W<FMC_SDRTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_SDRTR_SPEC>;
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
impl From<crate::W<FMC_SDRTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_SDRTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRE` writer - Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register."]
pub type CRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SDRTR_SPEC, bool, O>;
#[doc = "Field `COUNT` reader - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20"]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_SDRTR_SPEC, u16, u16, 13, O>;
#[doc = "Field `REIE` reader - RES Interrupt Enable"]
pub type REIE_R = crate::BitReader<bool>;
#[doc = "Field `REIE` writer - RES Interrupt Enable"]
pub type REIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SDRTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:13 - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - RES Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register."]
    #[inline(always)]
    #[must_use]
    pub fn cre(&mut self) -> CRE_W<0> {
        CRE_W::new(self)
    }
    #[doc = "Bits 1:13 - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<1> {
        COUNT_W::new(self)
    }
    #[doc = "Bit 14 - RES Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reie(&mut self) -> REIE_W<14> {
        REIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM refresh timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sdrtr](index.html) module"]
pub struct FMC_SDRTR_SPEC;
impl crate::RegisterSpec for FMC_SDRTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_sdrtr::R](R) reader structure"]
impl crate::Readable for FMC_SDRTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_sdrtr::W](W) writer structure"]
impl crate::Writable for FMC_SDRTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_SDRTR to value 0"]
impl crate::Resettable for FMC_SDRTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
