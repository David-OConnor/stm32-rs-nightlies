#[doc = "Register `OTR` reader"]
pub struct R(crate::R<OTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTR` writer"]
pub struct W(crate::W<OTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTR_SPEC>;
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
impl From<crate::W<OTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AO1_OPT_OFFSET_TRIM` reader - OPAMP1, 10-bit offset trim value for normal mode"]
pub type AO1_OPT_OFFSET_TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AO1_OPT_OFFSET_TRIM` writer - OPAMP1, 10-bit offset trim value for normal mode"]
pub type AO1_OPT_OFFSET_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTR_SPEC, u16, u16, 10, O>;
#[doc = "Field `AO2_OPT_OFFSET_TRIM` reader - OPAMP2, 10-bit offset trim value for normal mode"]
pub type AO2_OPT_OFFSET_TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AO2_OPT_OFFSET_TRIM` writer - OPAMP2, 10-bit offset trim value for normal mode"]
pub type AO2_OPT_OFFSET_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTR_SPEC, u16, u16, 10, O>;
#[doc = "Field `AO3_OPT_OFFSET_TRIM` reader - OPAMP3, 10-bit offset trim value for normal mode"]
pub type AO3_OPT_OFFSET_TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AO3_OPT_OFFSET_TRIM` writer - OPAMP3, 10-bit offset trim value for normal mode"]
pub type AO3_OPT_OFFSET_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTR_SPEC, u16, u16, 10, O>;
#[doc = "Field `OT_USER` reader - Select user or factory trimming value"]
pub type OT_USER_R = crate::BitReader<bool>;
#[doc = "Field `OT_USER` writer - Select user or factory trimming value"]
pub type OT_USER_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao1_opt_offset_trim(&self) -> AO1_OPT_OFFSET_TRIM_R {
        AO1_OPT_OFFSET_TRIM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao2_opt_offset_trim(&self) -> AO2_OPT_OFFSET_TRIM_R {
        AO2_OPT_OFFSET_TRIM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao3_opt_offset_trim(&self) -> AO3_OPT_OFFSET_TRIM_R {
        AO3_OPT_OFFSET_TRIM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Select user or factory trimming value"]
    #[inline(always)]
    pub fn ot_user(&self) -> OT_USER_R {
        OT_USER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn ao1_opt_offset_trim(&mut self) -> AO1_OPT_OFFSET_TRIM_W<0> {
        AO1_OPT_OFFSET_TRIM_W::new(self)
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn ao2_opt_offset_trim(&mut self) -> AO2_OPT_OFFSET_TRIM_W<10> {
        AO2_OPT_OFFSET_TRIM_W::new(self)
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn ao3_opt_offset_trim(&mut self) -> AO3_OPT_OFFSET_TRIM_W<20> {
        AO3_OPT_OFFSET_TRIM_W::new(self)
    }
    #[doc = "Bit 31 - Select user or factory trimming value"]
    #[inline(always)]
    #[must_use]
    pub fn ot_user(&mut self) -> OT_USER_W<31> {
        OT_USER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset trimming register for normal mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otr](index.html) module"]
pub struct OTR_SPEC;
impl crate::RegisterSpec for OTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otr::R](R) reader structure"]
impl crate::Readable for OTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otr::W](W) writer structure"]
impl crate::Writable for OTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTR to value 0"]
impl crate::Resettable for OTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
