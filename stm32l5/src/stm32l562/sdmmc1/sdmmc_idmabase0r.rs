#[doc = "Register `SDMMC_IDMABASE0R` reader"]
pub struct R(crate::R<SDMMC_IDMABASE0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMABASE0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMABASE0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMABASE0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_IDMABASE0R` writer"]
pub struct W(crate::W<SDMMC_IDMABASE0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMABASE0R_SPEC>;
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
impl From<crate::W<SDMMC_IDMABASE0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMABASE0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMABASE0` reader - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
pub type IDMABASE0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDMABASE0` writer - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
pub type IDMABASE0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_IDMABASE0R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
    #[inline(always)]
    pub fn idmabase0(&self) -> IDMABASE0_R {
        IDMABASE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 0 is inactive (IDMABACT = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn idmabase0(&mut self) -> IDMABASE0_W<0> {
        IDMABASE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabase0r](index.html) module"]
pub struct SDMMC_IDMABASE0R_SPEC;
impl crate::RegisterSpec for SDMMC_IDMABASE0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_idmabase0r::R](R) reader structure"]
impl crate::Readable for SDMMC_IDMABASE0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabase0r::W](W) writer structure"]
impl crate::Writable for SDMMC_IDMABASE0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_IDMABASE0R to value 0"]
impl crate::Resettable for SDMMC_IDMABASE0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
