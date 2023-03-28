#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GIE` reader - Global interrupt enable"]
pub type GIE_R = crate::BitReader<bool>;
#[doc = "Field `GIE` writer - Global interrupt enable"]
pub type GIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `GECCSEIE` reader - Global ECC single error interrupt enable"]
pub type GECCSEIE_R = crate::BitReader<bool>;
#[doc = "Field `GECCSEIE` writer - Global ECC single error interrupt enable"]
pub type GECCSEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `GECCDEIE` reader - Global ECC double error interrupt enable"]
pub type GECCDEIE_R = crate::BitReader<bool>;
#[doc = "Field `GECCDEIE` writer - Global ECC double error interrupt enable"]
pub type GECCDEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `GECCDEBWIE` reader - Global ECC double error on byte write interrupt enable"]
pub type GECCDEBWIE_R = crate::BitReader<bool>;
#[doc = "Field `GECCDEBWIE` writer - Global ECC double error on byte write interrupt enable"]
pub type GECCDEBWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn gie(&self) -> GIE_R {
        GIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    pub fn geccseie(&self) -> GECCSEIE_R {
        GECCSEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    pub fn geccdeie(&self) -> GECCDEIE_R {
        GECCDEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global ECC double error on byte write interrupt enable"]
    #[inline(always)]
    pub fn geccdebwie(&self) -> GECCDEBWIE_R {
        GECCDEBWIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gie(&mut self) -> GIE_W<0> {
        GIE_W::new(self)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn geccseie(&mut self) -> GECCSEIE_W<1> {
        GECCSEIE_W::new(self)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn geccdeie(&mut self) -> GECCDEIE_W<2> {
        GECCDEIE_W::new(self)
    }
    #[doc = "Bit 3 - Global ECC double error on byte write interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn geccdebwie(&mut self) -> GECCDEBWIE_W<3> {
        GECCDEBWIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
