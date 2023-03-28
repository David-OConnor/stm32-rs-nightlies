#[doc = "Register `DDRPHYC_PTR1` reader"]
pub struct R(crate::R<DDRPHYC_PTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_PTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_PTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_PTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_PTR1` writer"]
pub struct W(crate::W<DDRPHYC_PTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_PTR1_SPEC>;
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
impl From<crate::W<DDRPHYC_PTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_PTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDINIT0` reader - TDINIT0"]
pub type TDINIT0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TDINIT0` writer - TDINIT0"]
pub type TDINIT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_PTR1_SPEC, u32, u32, 19, O>;
#[doc = "Field `TDINIT1` reader - TDINIT1"]
pub type TDINIT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDINIT1` writer - TDINIT1"]
pub type TDINIT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_PTR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:18 - TDINIT0"]
    #[inline(always)]
    pub fn tdinit0(&self) -> TDINIT0_R {
        TDINIT0_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:26 - TDINIT1"]
    #[inline(always)]
    pub fn tdinit1(&self) -> TDINIT1_R {
        TDINIT1_R::new(((self.bits >> 19) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - TDINIT0"]
    #[inline(always)]
    #[must_use]
    pub fn tdinit0(&mut self) -> TDINIT0_W<0> {
        TDINIT0_W::new(self)
    }
    #[doc = "Bits 19:26 - TDINIT1"]
    #[inline(always)]
    #[must_use]
    pub fn tdinit1(&mut self) -> TDINIT1_W<19> {
        TDINIT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC PT register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ptr1](index.html) module"]
pub struct DDRPHYC_PTR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_PTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_ptr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_PTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ptr1::W](W) writer structure"]
impl crate::Writable for DDRPHYC_PTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PTR1 to value 0x0604_111d"]
impl crate::Resettable for DDRPHYC_PTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0604_111d;
}
