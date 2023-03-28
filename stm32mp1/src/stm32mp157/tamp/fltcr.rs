#[doc = "Register `FLTCR` reader"]
pub struct R(crate::R<FLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTCR` writer"]
pub struct W(crate::W<FLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTCR_SPEC>;
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
impl From<crate::W<FLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMPFREQ` reader - TAMPFREQ"]
pub type TAMPFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAMPFREQ` writer - TAMPFREQ"]
pub type TAMPFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TAMPFLT` reader - TAMPFLT"]
pub type TAMPFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAMPFLT` writer - TAMPFLT"]
pub type TAMPFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TAMPPRCH` reader - TAMPPRCH"]
pub type TAMPPRCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAMPPRCH` writer - TAMPPRCH"]
pub type TAMPPRCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TAMPPUDIS` reader - TAMPPUDIS"]
pub type TAMPPUDIS_R = crate::BitReader<bool>;
#[doc = "Field `TAMPPUDIS` writer - TAMPPUDIS"]
pub type TAMPPUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<0> {
        TAMPFREQ_W::new(self)
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<3> {
        TAMPFLT_W::new(self)
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<5> {
        TAMPPRCH_W::new(self)
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<7> {
        TAMPPUDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltcr](index.html) module"]
pub struct FLTCR_SPEC;
impl crate::RegisterSpec for FLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltcr::R](R) reader structure"]
impl crate::Readable for FLTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltcr::W](W) writer structure"]
impl crate::Writable for FLTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTCR to value 0"]
impl crate::Resettable for FLTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
