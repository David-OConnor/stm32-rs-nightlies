#[doc = "Register `IDMABASE1R` reader"]
pub struct R(crate::R<IDMABASE1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDMABASE1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDMABASE1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDMABASE1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDMABASE1R` writer"]
pub struct W(crate::W<IDMABASE1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDMABASE1R_SPEC>;
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
impl From<crate::W<IDMABASE1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDMABASE1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMABASE1` reader - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
pub type IDMABASE1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDMABASE1` writer - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
pub type IDMABASE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IDMABASE1R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
    #[inline(always)]
    pub fn idmabase1(&self) -> IDMABASE1_R {
        IDMABASE1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn idmabase1(&mut self) -> IDMABASE1_W<0> {
        IDMABASE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmabase1r](index.html) module"]
pub struct IDMABASE1R_SPEC;
impl crate::RegisterSpec for IDMABASE1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idmabase1r::R](R) reader structure"]
impl crate::Readable for IDMABASE1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idmabase1r::W](W) writer structure"]
impl crate::Writable for IDMABASE1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDMABASE1R to value 0"]
impl crate::Resettable for IDMABASE1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
