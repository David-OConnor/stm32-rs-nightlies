#[doc = "Register `DLYB_CR` reader"]
pub struct R(crate::R<DLYB_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYB_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYB_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYB_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYB_CR` writer"]
pub struct W(crate::W<DLYB_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLYB_CR_SPEC>;
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
impl From<crate::W<DLYB_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLYB_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEN` reader - DEN"]
pub type DEN_R = crate::BitReader<bool>;
#[doc = "Field `DEN` writer - DEN"]
pub type DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLYB_CR_SPEC, bool, O>;
#[doc = "Field `SEN` reader - SEN"]
pub type SEN_R = crate::BitReader<bool>;
#[doc = "Field `SEN` writer - SEN"]
pub type SEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLYB_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DEN"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SEN"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DEN"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<0> {
        DEN_W::new(self)
    }
    #[doc = "Bit 1 - SEN"]
    #[inline(always)]
    #[must_use]
    pub fn sen(&mut self) -> SEN_W<1> {
        SEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLYB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_cr](index.html) module"]
pub struct DLYB_CR_SPEC;
impl crate::RegisterSpec for DLYB_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlyb_cr::R](R) reader structure"]
impl crate::Readable for DLYB_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlyb_cr::W](W) writer structure"]
impl crate::Writable for DLYB_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLYB_CR to value 0"]
impl crate::Resettable for DLYB_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
