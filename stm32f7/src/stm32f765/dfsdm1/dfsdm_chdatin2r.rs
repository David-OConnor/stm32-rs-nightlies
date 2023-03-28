#[doc = "Register `DFSDM_CHDATIN2R` reader"]
pub struct R(crate::R<DFSDM_CHDATIN2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_CHDATIN2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_CHDATIN2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_CHDATIN2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_CHDATIN2R` writer"]
pub struct W(crate::W<DFSDM_CHDATIN2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_CHDATIN2R_SPEC>;
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
impl From<crate::W<DFSDM_CHDATIN2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_CHDATIN2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDAT0` reader - Input data for channel 2"]
pub type INDAT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INDAT0` writer - Input data for channel 2"]
pub type INDAT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_CHDATIN2R_SPEC, u16, u16, 16, O>;
#[doc = "Field `INDAT1` reader - Input data for channel 3"]
pub type INDAT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INDAT1` writer - Input data for channel 3"]
pub type INDAT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_CHDATIN2R_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Input data for channel 2"]
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Input data for channel 3"]
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input data for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn indat0(&mut self) -> INDAT0_W<0> {
        INDAT0_W::new(self)
    }
    #[doc = "Bits 16:31 - Input data for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn indat1(&mut self) -> INDAT1_W<16> {
        INDAT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chdatin2r](index.html) module"]
pub struct DFSDM_CHDATIN2R_SPEC;
impl crate::RegisterSpec for DFSDM_CHDATIN2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_chdatin2r::R](R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_chdatin2r::W](W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSDM_CHDATIN2R to value 0"]
impl crate::Resettable for DFSDM_CHDATIN2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
