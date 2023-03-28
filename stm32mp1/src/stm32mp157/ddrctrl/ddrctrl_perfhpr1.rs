#[doc = "Register `DDRCTRL_PERFHPR1` reader"]
pub struct R(crate::R<DDRCTRL_PERFHPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PERFHPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PERFHPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PERFHPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PERFHPR1` writer"]
pub struct W(crate::W<DDRCTRL_PERFHPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PERFHPR1_SPEC>;
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
impl From<crate::W<DDRCTRL_PERFHPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PERFHPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPR_MAX_STARVE` reader - HPR_MAX_STARVE"]
pub type HPR_MAX_STARVE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPR_MAX_STARVE` writer - HPR_MAX_STARVE"]
pub type HPR_MAX_STARVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PERFHPR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `HPR_XACT_RUN_LENGTH` reader - HPR_XACT_RUN_LENGTH"]
pub type HPR_XACT_RUN_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPR_XACT_RUN_LENGTH` writer - HPR_XACT_RUN_LENGTH"]
pub type HPR_XACT_RUN_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PERFHPR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - HPR_MAX_STARVE"]
    #[inline(always)]
    pub fn hpr_max_starve(&self) -> HPR_MAX_STARVE_R {
        HPR_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - HPR_XACT_RUN_LENGTH"]
    #[inline(always)]
    pub fn hpr_xact_run_length(&self) -> HPR_XACT_RUN_LENGTH_R {
        HPR_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - HPR_MAX_STARVE"]
    #[inline(always)]
    #[must_use]
    pub fn hpr_max_starve(&mut self) -> HPR_MAX_STARVE_W<0> {
        HPR_MAX_STARVE_W::new(self)
    }
    #[doc = "Bits 24:31 - HPR_XACT_RUN_LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn hpr_xact_run_length(&mut self) -> HPR_XACT_RUN_LENGTH_W<24> {
        HPR_XACT_RUN_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL high priority read CAM register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_perfhpr1](index.html) module"]
pub struct DDRCTRL_PERFHPR1_SPEC;
impl crate::RegisterSpec for DDRCTRL_PERFHPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_perfhpr1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PERFHPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_perfhpr1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PERFHPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PERFHPR1 to value 0x0f00_0001"]
impl crate::Resettable for DDRCTRL_PERFHPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_0001;
}
