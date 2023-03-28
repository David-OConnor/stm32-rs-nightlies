#[doc = "Register `DDRPERFM_CFG` reader"]
pub struct R(crate::R<DDRPERFM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPERFM_CFG` writer"]
pub struct W(crate::W<DDRPERFM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPERFM_CFG_SPEC>;
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
impl From<crate::W<DDRPERFM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPERFM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPERFM_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `SEL` reader - SEL"]
pub type SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL` writer - SEL"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPERFM_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - SEL"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 16:17 - SEL"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<16> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPERFM configurationl register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cfg](index.html) module"]
pub struct DDRPERFM_CFG_SPEC;
impl crate::RegisterSpec for DDRPERFM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_cfg::R](R) reader structure"]
impl crate::Readable for DDRPERFM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrperfm_cfg::W](W) writer structure"]
impl crate::Writable for DDRPERFM_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPERFM_CFG to value 0"]
impl crate::Resettable for DDRPERFM_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
