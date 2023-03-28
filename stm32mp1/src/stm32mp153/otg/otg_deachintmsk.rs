#[doc = "Register `OTG_DEACHINTMSK` reader"]
pub struct R(crate::R<OTG_DEACHINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DEACHINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DEACHINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DEACHINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DEACHINTMSK` writer"]
pub struct W(crate::W<OTG_DEACHINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DEACHINTMSK_SPEC>;
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
impl From<crate::W<OTG_DEACHINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DEACHINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEP1INTM` reader - IEP1INTM"]
pub type IEP1INTM_R = crate::BitReader<bool>;
#[doc = "Field `IEP1INTM` writer - IEP1INTM"]
pub type IEP1INTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DEACHINTMSK_SPEC, bool, O>;
#[doc = "Field `OEP1INTM` reader - OEP1INTM"]
pub type OEP1INTM_R = crate::BitReader<bool>;
#[doc = "Field `OEP1INTM` writer - OEP1INTM"]
pub type OEP1INTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DEACHINTMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - IEP1INTM"]
    #[inline(always)]
    pub fn iep1intm(&self) -> IEP1INTM_R {
        IEP1INTM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OEP1INTM"]
    #[inline(always)]
    pub fn oep1intm(&self) -> OEP1INTM_R {
        OEP1INTM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IEP1INTM"]
    #[inline(always)]
    #[must_use]
    pub fn iep1intm(&mut self) -> IEP1INTM_W<1> {
        IEP1INTM_W::new(self)
    }
    #[doc = "Bit 17 - OEP1INTM"]
    #[inline(always)]
    #[must_use]
    pub fn oep1intm(&mut self) -> OEP1INTM_W<17> {
        OEP1INTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_deachintmsk](index.html) module"]
pub struct OTG_DEACHINTMSK_SPEC;
impl crate::RegisterSpec for OTG_DEACHINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_deachintmsk::R](R) reader structure"]
impl crate::Readable for OTG_DEACHINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_deachintmsk::W](W) writer structure"]
impl crate::Writable for OTG_DEACHINTMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_DEACHINTMSK to value 0"]
impl crate::Resettable for OTG_DEACHINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
