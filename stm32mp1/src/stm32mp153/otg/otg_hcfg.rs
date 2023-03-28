#[doc = "Register `OTG_HCFG` reader"]
pub struct R(crate::R<OTG_HCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HCFG` writer"]
pub struct W(crate::W<OTG_HCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HCFG_SPEC>;
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
impl From<crate::W<OTG_HCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSLSPCS` reader - FSLSPCS"]
pub type FSLSPCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSLSPCS` writer - FSLSPCS"]
pub type FSLSPCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `FSLSS` reader - FSLSS"]
pub type FSLSS_R = crate::BitReader<bool>;
#[doc = "Field `DESCDMA` reader - DESCDMA"]
pub type DESCDMA_R = crate::BitReader<bool>;
#[doc = "Field `DESCDMA` writer - DESCDMA"]
pub type DESCDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCFG_SPEC, bool, O>;
#[doc = "Field `FRLSTEN` reader - FRLSTEN"]
pub type FRLSTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRLSTEN` writer - FRLSTEN"]
pub type FRLSTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `PERSSCHEDENA` reader - PERSSCHEDENA"]
pub type PERSSCHEDENA_R = crate::BitReader<bool>;
#[doc = "Field `PERSSCHEDENA` writer - PERSSCHEDENA"]
pub type PERSSCHEDENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - FSLSPCS"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FSLSS"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 23 - DESCDMA"]
    #[inline(always)]
    pub fn descdma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - FRLSTEN"]
    #[inline(always)]
    pub fn frlsten(&self) -> FRLSTEN_R {
        FRLSTEN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - PERSSCHEDENA"]
    #[inline(always)]
    pub fn persschedena(&self) -> PERSSCHEDENA_R {
        PERSSCHEDENA_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FSLSPCS"]
    #[inline(always)]
    #[must_use]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<0> {
        FSLSPCS_W::new(self)
    }
    #[doc = "Bit 23 - DESCDMA"]
    #[inline(always)]
    #[must_use]
    pub fn descdma(&mut self) -> DESCDMA_W<23> {
        DESCDMA_W::new(self)
    }
    #[doc = "Bits 24:25 - FRLSTEN"]
    #[inline(always)]
    #[must_use]
    pub fn frlsten(&mut self) -> FRLSTEN_W<24> {
        FRLSTEN_W::new(self)
    }
    #[doc = "Bit 26 - PERSSCHEDENA"]
    #[inline(always)]
    #[must_use]
    pub fn persschedena(&mut self) -> PERSSCHEDENA_W<26> {
        PERSSCHEDENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register configures the core after power-on. Do not make changes to this register after initializing the host.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcfg](index.html) module"]
pub struct OTG_HCFG_SPEC;
impl crate::RegisterSpec for OTG_HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcfg::R](R) reader structure"]
impl crate::Readable for OTG_HCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hcfg::W](W) writer structure"]
impl crate::Writable for OTG_HCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HCFG to value 0"]
impl crate::Resettable for OTG_HCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
