#[doc = "Register `GPWRDN` reader"]
pub struct R(crate::R<GPWRDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPWRDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPWRDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPWRDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPWRDN` writer"]
pub struct W(crate::W<GPWRDN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPWRDN_SPEC>;
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
impl From<crate::W<GPWRDN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPWRDN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADPMEN` reader - ADP module enable"]
pub type ADPMEN_R = crate::BitReader<bool>;
#[doc = "Field `ADPMEN` writer - ADP module enable"]
pub type ADPMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPWRDN_SPEC, bool, O>;
#[doc = "Field `ADPIF` reader - ADP interrupt flag"]
pub type ADPIF_R = crate::BitReader<bool>;
#[doc = "Field `ADPIF` writer - ADP interrupt flag"]
pub type ADPIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPWRDN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&self) -> ADPMEN_R {
        ADPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    pub fn adpif(&self) -> ADPIF_R {
        ADPIF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    #[must_use]
    pub fn adpmen(&mut self) -> ADPMEN_W<0> {
        ADPMEN_W::new(self)
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpif(&mut self) -> ADPIF_W<23> {
        ADPIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG power down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpwrdn](index.html) module"]
pub struct GPWRDN_SPEC;
impl crate::RegisterSpec for GPWRDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpwrdn::R](R) reader structure"]
impl crate::Readable for GPWRDN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpwrdn::W](W) writer structure"]
impl crate::Writable for GPWRDN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPWRDN to value 0"]
impl crate::Resettable for GPWRDN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
