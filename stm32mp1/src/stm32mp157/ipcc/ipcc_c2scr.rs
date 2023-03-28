#[doc = "Register `IPCC_C2SCR` reader"]
pub struct R(crate::R<IPCC_C2SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_C2SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_C2SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_C2SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCC_C2SCR` writer"]
pub struct W(crate::W<IPCC_C2SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCC_C2SCR_SPEC>;
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
impl From<crate::W<IPCC_C2SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCC_C2SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHxC` reader - CHxC"]
pub type CHX_C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHxC` writer - CHxC"]
pub type CHX_C_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCC_C2SCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CHxS` reader - CHxS"]
pub type CHX_S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHxS` writer - CHxS"]
pub type CHX_S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCC_C2SCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - CHxC"]
    #[inline(always)]
    pub fn chx_c(&self) -> CHX_C_R {
        CHX_C_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CHxS"]
    #[inline(always)]
    pub fn chx_s(&self) -> CHX_S_R {
        CHX_S_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CHxC"]
    #[inline(always)]
    #[must_use]
    pub fn chx_c(&mut self) -> CHX_C_W<0> {
        CHX_C_W::new(self)
    }
    #[doc = "Bits 16:21 - CHxS"]
    #[inline(always)]
    #[must_use]
    pub fn chx_s(&mut self) -> CHX_S_W<16> {
        CHX_S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reading this register will always return 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c2scr](index.html) module"]
pub struct IPCC_C2SCR_SPEC;
impl crate::RegisterSpec for IPCC_C2SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcc_c2scr::R](R) reader structure"]
impl crate::Readable for IPCC_C2SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcc_c2scr::W](W) writer structure"]
impl crate::Writable for IPCC_C2SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCC_C2SCR to value 0"]
impl crate::Resettable for IPCC_C2SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
