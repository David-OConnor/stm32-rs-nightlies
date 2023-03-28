#[doc = "Register `SDMMC_IDMABASER` reader"]
pub struct R(crate::R<SDMMC_IDMABASER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMABASER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMABASER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMABASER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_IDMABASER` writer"]
pub struct W(crate::W<SDMMC_IDMABASER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMABASER_SPEC>;
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
impl From<crate::W<SDMMC_IDMABASER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMABASER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMABASE` reader - IDMABASE"]
pub type IDMABASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDMABASE` writer - IDMABASE"]
pub type IDMABASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_IDMABASER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IDMABASE"]
    #[inline(always)]
    pub fn idmabase(&self) -> IDMABASE_R {
        IDMABASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IDMABASE"]
    #[inline(always)]
    #[must_use]
    pub fn idmabase(&mut self) -> IDMABASE_W<0> {
        IDMABASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabaser](index.html) module"]
pub struct SDMMC_IDMABASER_SPEC;
impl crate::RegisterSpec for SDMMC_IDMABASER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_idmabaser::R](R) reader structure"]
impl crate::Readable for SDMMC_IDMABASER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabaser::W](W) writer structure"]
impl crate::Writable for SDMMC_IDMABASER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_IDMABASER to value 0"]
impl crate::Resettable for SDMMC_IDMABASER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
