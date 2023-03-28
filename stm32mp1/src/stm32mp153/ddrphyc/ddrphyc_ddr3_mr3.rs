#[doc = "Register `DDRPHYC_DDR3_MR3` reader"]
pub struct R(crate::R<DDRPHYC_DDR3_MR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DDR3_MR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DDR3_MR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DDR3_MR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DDR3_MR3` writer"]
pub struct W(crate::W<DDRPHYC_DDR3_MR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DDR3_MR3_SPEC>;
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
impl From<crate::W<DDRPHYC_DDR3_MR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DDR3_MR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPRLOC` reader - MPRLOC"]
pub type MPRLOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MPRLOC` writer - MPRLOC"]
pub type MPRLOC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, DDRPHYC_DDR3_MR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `MPR` reader - MPR"]
pub type MPR_R = crate::BitReader<bool>;
#[doc = "Field `MPR` writer - MPR"]
pub type MPR_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRPHYC_DDR3_MR3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - MPRLOC"]
    #[inline(always)]
    pub fn mprloc(&self) -> MPRLOC_R {
        MPRLOC_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - MPR"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MPRLOC"]
    #[inline(always)]
    #[must_use]
    pub fn mprloc(&mut self) -> MPRLOC_W<0> {
        MPRLOC_W::new(self)
    }
    #[doc = "Bit 2 - MPR"]
    #[inline(always)]
    #[must_use]
    pub fn mpr(&mut self) -> MPR_W<2> {
        MPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC MR3 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr3](index.html) module"]
pub struct DDRPHYC_DDR3_MR3_SPEC;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ddrphyc_ddr3_mr3::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr3::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR3 to value 0"]
impl crate::Resettable for DDRPHYC_DDR3_MR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
