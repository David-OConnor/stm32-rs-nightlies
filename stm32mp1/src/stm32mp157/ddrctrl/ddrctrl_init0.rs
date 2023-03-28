#[doc = "Register `DDRCTRL_INIT0` reader"]
pub struct R(crate::R<DDRCTRL_INIT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_INIT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_INIT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_INIT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_INIT0` writer"]
pub struct W(crate::W<DDRCTRL_INIT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_INIT0_SPEC>;
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
impl From<crate::W<DDRCTRL_INIT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_INIT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_CKE_X1024` reader - PRE_CKE_X1024"]
pub type PRE_CKE_X1024_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRE_CKE_X1024` writer - PRE_CKE_X1024"]
pub type PRE_CKE_X1024_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_INIT0_SPEC, u16, u16, 12, O>;
#[doc = "Field `POST_CKE_X1024` reader - POST_CKE_X1024"]
pub type POST_CKE_X1024_R = crate::FieldReader<u16, u16>;
#[doc = "Field `POST_CKE_X1024` writer - POST_CKE_X1024"]
pub type POST_CKE_X1024_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_INIT0_SPEC, u16, u16, 10, O>;
#[doc = "Field `SKIP_DRAM_INIT` reader - SKIP_DRAM_INIT"]
pub type SKIP_DRAM_INIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SKIP_DRAM_INIT` writer - SKIP_DRAM_INIT"]
pub type SKIP_DRAM_INIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_INIT0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:11 - PRE_CKE_X1024"]
    #[inline(always)]
    pub fn pre_cke_x1024(&self) -> PRE_CKE_X1024_R {
        PRE_CKE_X1024_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - POST_CKE_X1024"]
    #[inline(always)]
    pub fn post_cke_x1024(&self) -> POST_CKE_X1024_R {
        POST_CKE_X1024_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - SKIP_DRAM_INIT"]
    #[inline(always)]
    pub fn skip_dram_init(&self) -> SKIP_DRAM_INIT_R {
        SKIP_DRAM_INIT_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - PRE_CKE_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn pre_cke_x1024(&mut self) -> PRE_CKE_X1024_W<0> {
        PRE_CKE_X1024_W::new(self)
    }
    #[doc = "Bits 16:25 - POST_CKE_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn post_cke_x1024(&mut self) -> POST_CKE_X1024_W<16> {
        POST_CKE_X1024_W::new(self)
    }
    #[doc = "Bits 30:31 - SKIP_DRAM_INIT"]
    #[inline(always)]
    #[must_use]
    pub fn skip_dram_init(&mut self) -> SKIP_DRAM_INIT_W<30> {
        SKIP_DRAM_INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM initialization register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init0](index.html) module"]
pub struct DDRCTRL_INIT0_SPEC;
impl crate::RegisterSpec for DDRCTRL_INIT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_init0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT0 to value 0x0002_004e"]
impl crate::Resettable for DDRCTRL_INIT0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_004e;
}
