#[doc = "Register `DLYB_CFGR` reader"]
pub struct R(crate::R<DLYB_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYB_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYB_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYB_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYB_CFGR` writer"]
pub struct W(crate::W<DLYB_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLYB_CFGR_SPEC>;
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
impl From<crate::W<DLYB_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLYB_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - SEL"]
pub type SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL` writer - SEL"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLYB_CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `UNIT` reader - UNIT"]
pub type UNIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UNIT` writer - UNIT"]
pub type UNIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLYB_CFGR_SPEC, u8, u8, 7, O>;
#[doc = "Field `LNG` reader - LNG"]
pub type LNG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LNGF` reader - LNGF"]
pub type LNGF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - SEL"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - UNIT"]
    #[inline(always)]
    pub fn unit(&self) -> UNIT_R {
        UNIT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - LNG"]
    #[inline(always)]
    pub fn lng(&self) -> LNG_R {
        LNG_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - LNGF"]
    #[inline(always)]
    pub fn lngf(&self) -> LNGF_R {
        LNGF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SEL"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Bits 8:14 - UNIT"]
    #[inline(always)]
    #[must_use]
    pub fn unit(&mut self) -> UNIT_W<8> {
        UNIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLYB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_cfgr](index.html) module"]
pub struct DLYB_CFGR_SPEC;
impl crate::RegisterSpec for DLYB_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlyb_cfgr::R](R) reader structure"]
impl crate::Readable for DLYB_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlyb_cfgr::W](W) writer structure"]
impl crate::Writable for DLYB_CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLYB_CFGR to value 0"]
impl crate::Resettable for DLYB_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
