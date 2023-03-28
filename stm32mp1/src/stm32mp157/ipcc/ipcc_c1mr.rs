#[doc = "Register `IPCC_C1MR` reader"]
pub struct R(crate::R<IPCC_C1MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_C1MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_C1MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_C1MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCC_C1MR` writer"]
pub struct W(crate::W<IPCC_C1MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCC_C1MR_SPEC>;
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
impl From<crate::W<IPCC_C1MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCC_C1MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHxOM` reader - CHxOM"]
pub type CHX_OM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHxOM` writer - CHxOM"]
pub type CHX_OM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCC_C1MR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CHxFM` reader - CHxFM"]
pub type CHX_FM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHxFM` writer - CHxFM"]
pub type CHX_FM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCC_C1MR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - CHxOM"]
    #[inline(always)]
    pub fn chx_om(&self) -> CHX_OM_R {
        CHX_OM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CHxFM"]
    #[inline(always)]
    pub fn chx_fm(&self) -> CHX_FM_R {
        CHX_FM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CHxOM"]
    #[inline(always)]
    #[must_use]
    pub fn chx_om(&mut self) -> CHX_OM_W<0> {
        CHX_OM_W::new(self)
    }
    #[doc = "Bits 16:21 - CHxFM"]
    #[inline(always)]
    #[must_use]
    pub fn chx_fm(&mut self) -> CHX_FM_W<16> {
        CHX_FM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPCC Processor 1 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c1mr](index.html) module"]
pub struct IPCC_C1MR_SPEC;
impl crate::RegisterSpec for IPCC_C1MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcc_c1mr::R](R) reader structure"]
impl crate::Readable for IPCC_C1MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcc_c1mr::W](W) writer structure"]
impl crate::Writable for IPCC_C1MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCC_C1MR to value 0xffff_ffff"]
impl crate::Resettable for IPCC_C1MR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
