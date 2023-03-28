#[doc = "Register `SPDIFRX_IFCR` reader"]
pub struct R(crate::R<SPDIFRX_IFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_IFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_IFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDIFRX_IFCR` writer"]
pub struct W(crate::W<SPDIFRX_IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDIFRX_IFCR_SPEC>;
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
impl From<crate::W<SPDIFRX_IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDIFRX_IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERRCF` writer - PERRCF"]
pub type PERRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IFCR_SPEC, bool, O>;
#[doc = "Field `OVRCF` writer - OVRCF"]
pub type OVRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IFCR_SPEC, bool, O>;
#[doc = "Field `SBDCF` writer - SBDCF"]
pub type SBDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IFCR_SPEC, bool, O>;
#[doc = "Field `SYNCDCF` writer - SYNCDCF"]
pub type SYNCDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_IFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 2 - PERRCF"]
    #[inline(always)]
    #[must_use]
    pub fn perrcf(&mut self) -> PERRCF_W<2> {
        PERRCF_W::new(self)
    }
    #[doc = "Bit 3 - OVRCF"]
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<3> {
        OVRCF_W::new(self)
    }
    #[doc = "Bit 4 - SBDCF"]
    #[inline(always)]
    #[must_use]
    pub fn sbdcf(&mut self) -> SBDCF_W<4> {
        SBDCF_W::new(self)
    }
    #[doc = "Bit 5 - SYNCDCF"]
    #[inline(always)]
    #[must_use]
    pub fn syncdcf(&mut self) -> SYNCDCF_W<5> {
        SYNCDCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_ifcr](index.html) module"]
pub struct SPDIFRX_IFCR_SPEC;
impl crate::RegisterSpec for SPDIFRX_IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdifrx_ifcr::R](R) reader structure"]
impl crate::Readable for SPDIFRX_IFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdifrx_ifcr::W](W) writer structure"]
impl crate::Writable for SPDIFRX_IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDIFRX_IFCR to value 0"]
impl crate::Resettable for SPDIFRX_IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
