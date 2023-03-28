#[doc = "Register `M4SR` reader"]
pub struct R(crate::R<M4SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4SR` writer"]
pub struct W(crate::W<M4SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4SR_SPEC>;
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
impl From<crate::W<M4SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEDCF` reader - ECC single error detected and corrected flag"]
pub type SEDCF_R = crate::BitReader<bool>;
#[doc = "Field `SEDCF` writer - ECC single error detected and corrected flag"]
pub type SEDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, M4SR_SPEC, bool, O>;
#[doc = "Field `DEDF` reader - ECC double error detected flag"]
pub type DEDF_R = crate::BitReader<bool>;
#[doc = "Field `DEDF` writer - ECC double error detected flag"]
pub type DEDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, M4SR_SPEC, bool, O>;
#[doc = "Field `DEBWDF` reader - ECC double error on byte write (BW) detected flag"]
pub type DEBWDF_R = crate::BitReader<bool>;
#[doc = "Field `DEBWDF` writer - ECC double error on byte write (BW) detected flag"]
pub type DEBWDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, M4SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ECC single error detected and corrected flag"]
    #[inline(always)]
    pub fn sedcf(&self) -> SEDCF_R {
        SEDCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC double error on byte write (BW) detected flag"]
    #[inline(always)]
    pub fn debwdf(&self) -> DEBWDF_R {
        DEBWDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC single error detected and corrected flag"]
    #[inline(always)]
    #[must_use]
    pub fn sedcf(&mut self) -> SEDCF_W<0> {
        SEDCF_W::new(self)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    #[must_use]
    pub fn dedf(&mut self) -> DEDF_W<1> {
        DEDF_W::new(self)
    }
    #[doc = "Bit 2 - ECC double error on byte write (BW) detected flag"]
    #[inline(always)]
    #[must_use]
    pub fn debwdf(&mut self) -> DEBWDF_W<2> {
        DEBWDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor x status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4sr](index.html) module"]
pub struct M4SR_SPEC;
impl crate::RegisterSpec for M4SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4sr::R](R) reader structure"]
impl crate::Readable for M4SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4sr::W](W) writer structure"]
impl crate::Writable for M4SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M4SR to value 0"]
impl crate::Resettable for M4SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
