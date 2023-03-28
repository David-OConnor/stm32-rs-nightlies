#[doc = "Register `QUADSPI_DCR` reader"]
pub struct R(crate::R<QUADSPI_DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUADSPI_DCR` writer"]
pub struct W(crate::W<QUADSPI_DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_DCR_SPEC>;
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
impl From<crate::W<QUADSPI_DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKMODE` reader - CKMODE"]
pub type CKMODE_R = crate::BitReader<bool>;
#[doc = "Field `CKMODE` writer - CKMODE"]
pub type CKMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_DCR_SPEC, bool, O>;
#[doc = "Field `CSHT` reader - CSHT"]
pub type CSHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSHT` writer - CSHT"]
pub type CSHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_DCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FSIZE` reader - FSIZE"]
pub type FSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSIZE` writer - FSIZE"]
pub type FSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_DCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - CSHT"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - FSIZE"]
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CKMODE"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<0> {
        CKMODE_W::new(self)
    }
    #[doc = "Bits 8:10 - CSHT"]
    #[inline(always)]
    #[must_use]
    pub fn csht(&mut self) -> CSHT_W<8> {
        CSHT_W::new(self)
    }
    #[doc = "Bits 16:20 - FSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn fsize(&mut self) -> FSIZE_W<16> {
        FSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_dcr](index.html) module"]
pub struct QUADSPI_DCR_SPEC;
impl crate::RegisterSpec for QUADSPI_DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_dcr::R](R) reader structure"]
impl crate::Readable for QUADSPI_DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quadspi_dcr::W](W) writer structure"]
impl crate::Writable for QUADSPI_DCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QUADSPI_DCR to value 0"]
impl crate::Resettable for QUADSPI_DCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
