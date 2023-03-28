#[doc = "Register `DDRCTRL_PERFWR1` reader"]
pub struct R(crate::R<DDRCTRL_PERFWR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PERFWR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PERFWR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PERFWR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PERFWR1` writer"]
pub struct W(crate::W<DDRCTRL_PERFWR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PERFWR1_SPEC>;
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
impl From<crate::W<DDRCTRL_PERFWR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PERFWR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `W_MAX_STARVE` reader - W_MAX_STARVE"]
pub type W_MAX_STARVE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `W_MAX_STARVE` writer - W_MAX_STARVE"]
pub type W_MAX_STARVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PERFWR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `W_XACT_RUN_LENGTH` reader - W_XACT_RUN_LENGTH"]
pub type W_XACT_RUN_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `W_XACT_RUN_LENGTH` writer - W_XACT_RUN_LENGTH"]
pub type W_XACT_RUN_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PERFWR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - W_MAX_STARVE"]
    #[inline(always)]
    pub fn w_max_starve(&self) -> W_MAX_STARVE_R {
        W_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - W_XACT_RUN_LENGTH"]
    #[inline(always)]
    pub fn w_xact_run_length(&self) -> W_XACT_RUN_LENGTH_R {
        W_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - W_MAX_STARVE"]
    #[inline(always)]
    #[must_use]
    pub fn w_max_starve(&mut self) -> W_MAX_STARVE_W<0> {
        W_MAX_STARVE_W::new(self)
    }
    #[doc = "Bits 24:31 - W_XACT_RUN_LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn w_xact_run_length(&mut self) -> W_XACT_RUN_LENGTH_W<24> {
        W_XACT_RUN_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL write CAM register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_perfwr1](index.html) module"]
pub struct DDRCTRL_PERFWR1_SPEC;
impl crate::RegisterSpec for DDRCTRL_PERFWR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_perfwr1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PERFWR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_perfwr1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PERFWR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PERFWR1 to value 0x0f00_007f"]
impl crate::Resettable for DDRCTRL_PERFWR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_007f;
}
