#[doc = "Register `DDRPHYC_PTR0` reader"]
pub struct R(crate::R<DDRPHYC_PTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_PTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_PTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_PTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_PTR0` writer"]
pub struct W(crate::W<DDRPHYC_PTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_PTR0_SPEC>;
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
impl From<crate::W<DDRPHYC_PTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_PTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDLLSRST` reader - TDLLSRST"]
pub type TDLLSRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDLLSRST` writer - TDLLSRST"]
pub type TDLLSRST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PTR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `TDLLLOCK` reader - TDLLLOCK"]
pub type TDLLLOCK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDLLLOCK` writer - TDLLLOCK"]
pub type TDLLLOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_PTR0_SPEC, u16, u16, 12, O>;
#[doc = "Field `TITMSRST` reader - TITMSRST"]
pub type TITMSRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TITMSRST` writer - TITMSRST"]
pub type TITMSRST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PTR0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:5 - TDLLSRST"]
    #[inline(always)]
    pub fn tdllsrst(&self) -> TDLLSRST_R {
        TDLLSRST_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:17 - TDLLLOCK"]
    #[inline(always)]
    pub fn tdlllock(&self) -> TDLLLOCK_R {
        TDLLLOCK_R::new(((self.bits >> 6) & 0x0fff) as u16)
    }
    #[doc = "Bits 18:21 - TITMSRST"]
    #[inline(always)]
    pub fn titmsrst(&self) -> TITMSRST_R {
        TITMSRST_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TDLLSRST"]
    #[inline(always)]
    #[must_use]
    pub fn tdllsrst(&mut self) -> TDLLSRST_W<0> {
        TDLLSRST_W::new(self)
    }
    #[doc = "Bits 6:17 - TDLLLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn tdlllock(&mut self) -> TDLLLOCK_W<6> {
        TDLLLOCK_W::new(self)
    }
    #[doc = "Bits 18:21 - TITMSRST"]
    #[inline(always)]
    #[must_use]
    pub fn titmsrst(&mut self) -> TITMSRST_W<18> {
        TITMSRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC PT register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ptr0](index.html) module"]
pub struct DDRPHYC_PTR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_PTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_ptr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_PTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ptr0::W](W) writer structure"]
impl crate::Writable for DDRPHYC_PTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PTR0 to value 0x0022_af9b"]
impl crate::Resettable for DDRPHYC_PTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0022_af9b;
}
