#[doc = "Register `SECCFGR1` reader"]
pub struct R(crate::R<SECCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR1` writer"]
pub struct W(crate::W<SECCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR1_SPEC>;
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
impl From<crate::W<SECCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESSEC` reader - AESSEC"]
pub type AESSEC_R = crate::BitReader<bool>;
#[doc = "Field `AESSEC` writer - AESSEC"]
pub type AESSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `RNGSEC` reader - RNGSEC"]
pub type RNGSEC_R = crate::BitReader<bool>;
#[doc = "Field `RNGSEC` writer - RNGSEC"]
pub type RNGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `PKASEC` reader - PKASEC"]
pub type PKASEC_R = crate::BitReader<bool>;
#[doc = "Field `PKASEC` writer - PKASEC"]
pub type PKASEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AESSEC"]
    #[inline(always)]
    #[must_use]
    pub fn aessec(&mut self) -> AESSEC_W<2> {
        AESSEC_W::new(self)
    }
    #[doc = "Bit 3 - RNGSEC"]
    #[inline(always)]
    #[must_use]
    pub fn rngsec(&mut self) -> RNGSEC_W<3> {
        RNGSEC_W::new(self)
    }
    #[doc = "Bit 13 - PKASEC"]
    #[inline(always)]
    #[must_use]
    pub fn pkasec(&mut self) -> PKASEC_W<13> {
        PKASEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC security configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr1](index.html) module"]
pub struct SECCFGR1_SPEC;
impl crate::RegisterSpec for SECCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr1::R](R) reader structure"]
impl crate::Readable for SECCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr1::W](W) writer structure"]
impl crate::Writable for SECCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCFGR1 to value 0"]
impl crate::Resettable for SECCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
