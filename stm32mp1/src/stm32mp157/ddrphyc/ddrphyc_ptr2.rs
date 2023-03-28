#[doc = "Register `DDRPHYC_PTR2` reader"]
pub struct R(crate::R<DDRPHYC_PTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_PTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_PTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_PTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_PTR2` writer"]
pub struct W(crate::W<DDRPHYC_PTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_PTR2_SPEC>;
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
impl From<crate::W<DDRPHYC_PTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_PTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDINIT2` reader - TDINIT2"]
pub type TDINIT2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TDINIT2` writer - TDINIT2"]
pub type TDINIT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_PTR2_SPEC, u32, u32, 17, O>;
#[doc = "Field `TDINIT3` reader - TDINIT3"]
pub type TDINIT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDINIT3` writer - TDINIT3"]
pub type TDINIT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_PTR2_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:16 - TDINIT2"]
    #[inline(always)]
    pub fn tdinit2(&self) -> TDINIT2_R {
        TDINIT2_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:26 - TDINIT3"]
    #[inline(always)]
    pub fn tdinit3(&self) -> TDINIT3_R {
        TDINIT3_R::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - TDINIT2"]
    #[inline(always)]
    #[must_use]
    pub fn tdinit2(&mut self) -> TDINIT2_W<0> {
        TDINIT2_W::new(self)
    }
    #[doc = "Bits 17:26 - TDINIT3"]
    #[inline(always)]
    #[must_use]
    pub fn tdinit3(&mut self) -> TDINIT3_W<17> {
        TDINIT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC PT register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ptr2](index.html) module"]
pub struct DDRPHYC_PTR2_SPEC;
impl crate::RegisterSpec for DDRPHYC_PTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_ptr2::R](R) reader structure"]
impl crate::Readable for DDRPHYC_PTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ptr2::W](W) writer structure"]
impl crate::Writable for DDRPHYC_PTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PTR2 to value 0x042d_a072"]
impl crate::Resettable for DDRPHYC_PTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x042d_a072;
}
