#[doc = "Register `OTG_HAINTMSK` reader"]
pub struct R(crate::R<OTG_HAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HAINTMSK` writer"]
pub struct W(crate::W<OTG_HAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HAINTMSK_SPEC>;
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
impl From<crate::W<OTG_HAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HAINTM` reader - HAINTM"]
pub type HAINTM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HAINTM` writer - HAINTM"]
pub type HAINTM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_HAINTMSK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - HAINTM"]
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HAINTM"]
    #[inline(always)]
    #[must_use]
    pub fn haintm(&mut self) -> HAINTM_W<0> {
        HAINTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_haintmsk](index.html) module"]
pub struct OTG_HAINTMSK_SPEC;
impl crate::RegisterSpec for OTG_HAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_haintmsk::R](R) reader structure"]
impl crate::Readable for OTG_HAINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_haintmsk::W](W) writer structure"]
impl crate::Writable for OTG_HAINTMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HAINTMSK to value 0"]
impl crate::Resettable for OTG_HAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
