#[doc = "Register `CRYP_CSGCMCCM3R` reader"]
pub struct R(crate::R<CRYP_CSGCMCCM3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_CSGCMCCM3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_CSGCMCCM3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_CSGCMCCM3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_CSGCMCCM3R` writer"]
pub struct W(crate::W<CRYP_CSGCMCCM3R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_CSGCMCCM3R_SPEC>;
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
impl From<crate::W<CRYP_CSGCMCCM3R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_CSGCMCCM3R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCMCCM3` reader - CSGCMCCM3"]
pub type CSGCMCCM3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCMCCM3` writer - CSGCMCCM3"]
pub type CSGCMCCM3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRYP_CSGCMCCM3R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM3"]
    #[inline(always)]
    pub fn csgcmccm3(&self) -> CSGCMCCM3_R {
        CSGCMCCM3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM3"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm3(&mut self) -> CSGCMCCM3_W<0> {
        CSGCMCCM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm3r](index.html) module"]
pub struct CRYP_CSGCMCCM3R_SPEC;
impl crate::RegisterSpec for CRYP_CSGCMCCM3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_csgcmccm3r::R](R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM3R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm3r::W](W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM3R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYP_CSGCMCCM3R to value 0"]
impl crate::Resettable for CRYP_CSGCMCCM3R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}