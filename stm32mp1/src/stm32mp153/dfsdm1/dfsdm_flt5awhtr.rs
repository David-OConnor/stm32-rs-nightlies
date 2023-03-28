#[doc = "Register `DFSDM_FLT5AWHTR` reader"]
pub struct R(crate::R<DFSDM_FLT5AWHTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT5AWHTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT5AWHTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT5AWHTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_FLT5AWHTR` writer"]
pub struct W(crate::W<DFSDM_FLT5AWHTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT5AWHTR_SPEC>;
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
impl From<crate::W<DFSDM_FLT5AWHTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT5AWHTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKAWH` reader - BKAWH"]
pub type BKAWH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKAWH` writer - BKAWH"]
pub type BKAWH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_FLT5AWHTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWHT` reader - AWHT"]
pub type AWHT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AWHT` writer - AWHT"]
pub type AWHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_FLT5AWHTR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - BKAWH"]
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - AWHT"]
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - BKAWH"]
    #[inline(always)]
    #[must_use]
    pub fn bkawh(&mut self) -> BKAWH_W<0> {
        BKAWH_W::new(self)
    }
    #[doc = "Bits 8:31 - AWHT"]
    #[inline(always)]
    #[must_use]
    pub fn awht(&mut self) -> AWHT_W<8> {
        AWHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM filter 5 analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt5awhtr](index.html) module"]
pub struct DFSDM_FLT5AWHTR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT5AWHTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt5awhtr::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT5AWHTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt5awhtr::W](W) writer structure"]
impl crate::Writable for DFSDM_FLT5AWHTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT5AWHTR to value 0"]
impl crate::Resettable for DFSDM_FLT5AWHTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
