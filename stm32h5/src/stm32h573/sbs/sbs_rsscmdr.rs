#[doc = "Register `SBS_RSSCMDR` reader"]
pub struct R(crate::R<SBS_RSSCMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_RSSCMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_RSSCMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_RSSCMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_RSSCMDR` writer"]
pub struct W(crate::W<SBS_RSSCMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_RSSCMDR_SPEC>;
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
impl From<crate::W<SBS_RSSCMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_RSSCMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSSCMD` reader - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value."]
pub type RSSCMD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSSCMD` writer - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value."]
pub type RSSCMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SBS_RSSCMDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value."]
    #[inline(always)]
    pub fn rsscmd(&self) -> RSSCMD_R {
        RSSCMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value."]
    #[inline(always)]
    #[must_use]
    pub fn rsscmd(&mut self) -> RSSCMD_W<0> {
        RSSCMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS RSS command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_rsscmdr](index.html) module"]
pub struct SBS_RSSCMDR_SPEC;
impl crate::RegisterSpec for SBS_RSSCMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_rsscmdr::R](R) reader structure"]
impl crate::Readable for SBS_RSSCMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_rsscmdr::W](W) writer structure"]
impl crate::Writable for SBS_RSSCMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_RSSCMDR to value 0"]
impl crate::Resettable for SBS_RSSCMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
