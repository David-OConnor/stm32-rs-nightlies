#[doc = "Register `SYSCFG_CBR` reader"]
pub struct R(crate::R<SYSCFG_CBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_CBR` writer"]
pub struct W(crate::W<SYSCFG_CBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CBR_SPEC>;
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
impl From<crate::W<SYSCFG_CBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLL` reader - CLL"]
pub type CLL_R = crate::BitReader<bool>;
#[doc = "Field `CLL` writer - CLL"]
pub type CLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_CBR_SPEC, bool, O>;
#[doc = "Field `PVDL` reader - PVDL"]
pub type PVDL_R = crate::BitReader<bool>;
#[doc = "Field `PVDL` writer - PVDL"]
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_CBR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CLL"]
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PVDL"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLL"]
    #[inline(always)]
    #[must_use]
    pub fn cll(&mut self) -> CLL_W<0> {
        CLL_W::new(self)
    }
    #[doc = "Bit 2 - PVDL"]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<2> {
        PVDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG control timer break register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cbr](index.html) module"]
pub struct SYSCFG_CBR_SPEC;
impl crate::RegisterSpec for SYSCFG_CBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_cbr::R](R) reader structure"]
impl crate::Readable for SYSCFG_CBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_cbr::W](W) writer structure"]
impl crate::Writable for SYSCFG_CBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_CBR to value 0"]
impl crate::Resettable for SYSCFG_CBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
