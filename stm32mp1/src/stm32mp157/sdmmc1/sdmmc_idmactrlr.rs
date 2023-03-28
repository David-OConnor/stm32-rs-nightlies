#[doc = "Register `SDMMC_IDMACTRLR` reader"]
pub struct R(crate::R<SDMMC_IDMACTRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMACTRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMACTRLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMACTRLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_IDMACTRLR` writer"]
pub struct W(crate::W<SDMMC_IDMACTRLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMACTRLR_SPEC>;
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
impl From<crate::W<SDMMC_IDMACTRLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMACTRLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMAEN` reader - IDMAEN"]
pub type IDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `IDMAEN` writer - IDMAEN"]
pub type IDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_IDMACTRLR_SPEC, bool, O>;
#[doc = "Field `IDMABMODE` reader - IDMABMODE"]
pub type IDMABMODE_R = crate::BitReader<bool>;
#[doc = "Field `IDMABMODE` writer - IDMABMODE"]
pub type IDMABMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_IDMACTRLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IDMAEN"]
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IDMABMODE"]
    #[inline(always)]
    pub fn idmabmode(&self) -> IDMABMODE_R {
        IDMABMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn idmaen(&mut self) -> IDMAEN_W<0> {
        IDMAEN_W::new(self)
    }
    #[doc = "Bit 1 - IDMABMODE"]
    #[inline(always)]
    #[must_use]
    pub fn idmabmode(&mut self) -> IDMABMODE_W<1> {
        IDMABMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmactrlr](index.html) module"]
pub struct SDMMC_IDMACTRLR_SPEC;
impl crate::RegisterSpec for SDMMC_IDMACTRLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_idmactrlr::R](R) reader structure"]
impl crate::Readable for SDMMC_IDMACTRLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmactrlr::W](W) writer structure"]
impl crate::Writable for SDMMC_IDMACTRLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_IDMACTRLR to value 0"]
impl crate::Resettable for SDMMC_IDMACTRLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
