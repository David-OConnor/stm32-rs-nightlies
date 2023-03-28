#[doc = "Register `FDCAN_TTMLM` reader"]
pub struct R(crate::R<FDCAN_TTMLM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTMLM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTMLM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTMLM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTMLM` writer"]
pub struct W(crate::W<FDCAN_TTMLM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTMLM_SPEC>;
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
impl From<crate::W<FDCAN_TTMLM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTMLM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCM` reader - CCM"]
pub type CCM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCM` writer - CCM"]
pub type CCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTMLM_SPEC, u8, u8, 6, O>;
#[doc = "Field `CSS` reader - CSS"]
pub type CSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSS` writer - CSS"]
pub type CSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTMLM_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXEW` reader - TXEW"]
pub type TXEW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXEW` writer - TXEW"]
pub type TXEW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTMLM_SPEC, u8, u8, 4, O>;
#[doc = "Field `ENTT` reader - ENTT"]
pub type ENTT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ENTT` writer - ENTT"]
pub type ENTT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTMLM_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:5 - CCM"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - CSS"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - TXEW"]
    #[inline(always)]
    pub fn txew(&self) -> TXEW_R {
        TXEW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - ENTT"]
    #[inline(always)]
    pub fn entt(&self) -> ENTT_R {
        ENTT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - CCM"]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<0> {
        CCM_W::new(self)
    }
    #[doc = "Bits 6:7 - CSS"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<6> {
        CSS_W::new(self)
    }
    #[doc = "Bits 8:11 - TXEW"]
    #[inline(always)]
    #[must_use]
    pub fn txew(&mut self) -> TXEW_W<8> {
        TXEW_W::new(self)
    }
    #[doc = "Bits 16:27 - ENTT"]
    #[inline(always)]
    #[must_use]
    pub fn entt(&mut self) -> ENTT_W<16> {
        ENTT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT matrix limits register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttmlm](index.html) module"]
pub struct FDCAN_TTMLM_SPEC;
impl crate::RegisterSpec for FDCAN_TTMLM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttmlm::R](R) reader structure"]
impl crate::Readable for FDCAN_TTMLM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttmlm::W](W) writer structure"]
impl crate::Writable for FDCAN_TTMLM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTMLM to value 0"]
impl crate::Resettable for FDCAN_TTMLM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
